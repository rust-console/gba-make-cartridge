#![deny(warnings)]

extern crate clap;
use std::path::Path;
use std::fs::{OpenOptions};
use clap::{Arg, App};
use std::process::{Command, Stdio};

extern crate tempfile;
use tempfile::NamedTempFile;

#[macro_use]
extern crate error_chain;

mod errors {
    error_chain!{}
}
use errors::*;

mod fix_header;
fn main() {
    if let Err(ref e) = run() {
        use std::io::Write;
        let stderr = &mut ::std::io::stderr();
        let errmsg = "Error writing to stderr";

        writeln!(stderr, "error: {}", e).expect(errmsg);

        for e in e.iter().skip(1) {
            writeln!(stderr, "caused by: {}", e).expect(errmsg);
        }

        // The backtrace is not always generated. Try to run this example
        // with `RUST_BACKTRACE=1`.
        if let Some(backtrace) = e.backtrace() {
            writeln!(stderr, "backtrace: {:?}", backtrace).expect(errmsg);
        }

        ::std::process::exit(1);
    }
}

fn objcopy(input_path_str: &str, output_path_str: &str) -> Result<()> {
    let copy_prog = "armv4t-none-eabi-objcopy";
    let copy_args = [input_path_str, "-O", "binary", output_path_str, "-S"];

    let output = Command::new(copy_prog).args(&copy_args)
        .stdout(Stdio::null())
        .stderr(Stdio::piped())
        .output()
        .chain_err(|| "failed to execute armv4t-none-eabi-objcopy")?;

    if !output.status.success() {
        let failure_msg = output
            .status
            .code()
            .map(|c| format!("failed with code {}", c))
            .unwrap_or_else(|| String::from("failed"));

        bail!(format!("failed to extract binary image:\n`{} {}` {}:\n{}",
                      copy_prog,
                      copy_args.join(" "),
                      failure_msg,
                      String::from_utf8_lossy(&output.stderr)));
    }

    Ok(())
}

fn fix_header_for_path(path: &Path,
                       title: &str,
                       game_code: &str,
                       manu_code: &str,
                       version: u8)
                       -> Result<()> {
    let file = OpenOptions::new().read(true)
        .write(true)
        .create(false)
        .open(path)
        .chain_err(|| "could not open temporary file for writing")?;

    let mut title_buf = [0u8; 12];
    title_buf[..title.len()].clone_from_slice(title.as_bytes());
    let mut game_code_buf = [0u8; 4];
    game_code_buf.clone_from_slice(game_code.as_bytes());
    let mut manu_code_buf = [0u8; 2];
    manu_code_buf.clone_from_slice(manu_code.as_bytes());

    fix_header::fix_header(
        &file,
        &title_buf,
        &game_code_buf,
        &manu_code_buf,
        version
    ).chain_err(|| "failed fixing up the GBA header")?;
    Ok(())
}

fn validate_header_field(s: &str,
                         max_chars: usize,
                         okay_less_than_max: bool)
                         -> std::result::Result<(), String> {
    if s.len() > max_chars {
        return Err(String::from("too many characters"));
    }

    if !okay_less_than_max && (s.len() < max_chars) {
        return Err(String::from("too few characters"));
    }

    if s.bytes().any(|c| c < b'A' || c > b'Z') {
        return Err(String::from("all characters must be capital letters"));
    }

    Ok(())
}

fn run() -> Result<()> {
    let matches = App::new("gba-make-cartridge")
        .version("1.0")
        .author("Russell McClellan <rmcclellan@gmail.com>")
        .about("Creates a gameboy cartridge from an ELF file")
        .arg(Arg::with_name("input")
                 .help("ELF file containing code")
                 .required(true)
                 .index(1))
        .arg(Arg::with_name("output")
                 .short("o")
                 .long("output")
                 .value_name("output")
                 .help("output cartridge file")
                 .takes_value(true)
                 .default_value("a.gba"))
        .arg(Arg::with_name("title")
                 .short("t")
                 .long("title")
                 .value_name("title")
                 .help("title of game, 12 upper case letters or less")
                 .takes_value(true)
                 .default_value("MYGAME")
                 .validator(|s| validate_header_field(&s, 12, true)))
        .arg(Arg::with_name("game_code")
                 .short("g")
                 .long("game_code")
                 .value_name("game_code")
                 .help("title of game, exactly 4 upper case letters")
                 .takes_value(true)
                 .default_value("BMGE")
                 .validator(|s| validate_header_field(&s, 4, false)))
        .arg(Arg::with_name("manu_code")
                 .short("m")
                 .long("manu_code")
                 .value_name("manufacturer_code")
                 .help("manufacturer's code, exactly 2 upper case letters")
                 .takes_value(true)
                 .default_value("RM")
                 .validator(|s| validate_header_field(&s, 2, false)))
        .arg(Arg::with_name("version")
                 .short("v")
                 .long("version")
                 .value_name("version")
                 .help("version number, less than 256")
                 .takes_value(true)
                 .default_value("0")
                 .validator(|s| {
                                s.parse::<u8>()
                                    .map(|_| ())
                                    .map_err(|_| String::from("not a number less than 256"))
                            }))
        .get_matches();

    let input = matches.value_of("input").unwrap();
    let output = matches.value_of("output").unwrap();
    let title = matches.value_of("title").unwrap();
    let game_code = matches.value_of("game_code").unwrap();
    let manu_code = matches.value_of("manu_code").unwrap();
    let version = matches
        .value_of("version")
        .unwrap()
        .parse::<u8>()
        .unwrap();

    let temp_file = NamedTempFile::new().chain_err(|| "failed to create temporary file")?;

    objcopy(input,
            temp_file.path()
            .to_str()
            .ok_or("could not get temporary file path")?)?;

    fix_header_for_path(temp_file.path(), title, game_code, manu_code, version)?;

    std::fs::copy(
        &temp_file.path(),
        Path::new(output)).chain_err(|| "failed to copy cartridge to output path")?;
    Ok(())
}
