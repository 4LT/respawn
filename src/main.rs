use quake_util::qmap;
use std::{env, fs, io, fmt};
use std::process::exit;
use std::error::Error;
use std::path::Path;
use fmt::{Display, Formatter};
use std::ffi::OsString;

mod filters;
use filters::*;

#[derive(Debug)]
struct AppError(String);

impl AppError {
    pub fn new<E: Error>(error: E) -> AppError {
        AppError(format!("{}", error))
    }
}

impl Display for AppError {
    fn fmt(&self, formatter: &mut Formatter) -> Result<(), fmt::Error> {
        write!(formatter, "{}", self.0)?;
        Ok(())
    }
}

impl Error for AppError {
}

fn main() {
    if let Err(e) = run() {
        eprintln!("Error: {}", e);
        exit(1);
    }
}

fn run() -> Result<(), AppError> {
    let suffix = OsString::from("-post");
    let mut args = env::args_os();
    let arg1 = args.nth(1);

    let read_path = match &arg1 {
        None => {
            return Err(AppError(String::from("No arguments")));
        },
        Some(path) => Path::new(path)
    };

    let read_file = fs::File::open(read_path).map_err(AppError::new)?;

    let reader = io::BufReader::new(read_file);

    let ext = read_path.extension();
    let stem = read_path.file_stem();
    let mut write_file_name = OsString::from(stem.unwrap());
    write_file_name.push(suffix);

    match ext {
        None => {
            write_file_name.push(".map");
        },
        Some(ext) => {
            write_file_name.push(".");
            write_file_name.push(ext);
        }
    }

    let write_path = read_path.with_file_name(write_file_name.clone());
    let write_file = fs::File::create(write_path).map_err(
        |e| AppError(format!("{:#?}: {}", write_file_name, e))
    )?;
    let mut writer = io::BufWriter::new(write_file);
    let mut map = qmap::parse(reader).map_err(AppError::new)?;

    patch_skill(&mut map);

    map.write_to(&mut writer).map_err(AppError::new)?;

    Ok(())
}
