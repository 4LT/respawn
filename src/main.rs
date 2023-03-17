use quake_util::qmap;
use std::{env, fs, io, fmt};
use std::process::exit;
use std::error::Error;
use std::path::{Path, PathBuf};
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

const NO_ARGS: &str = "No arguments";
const USAGE: &str =
    "Usage: respawn <input-file> [<output-file>]";

fn main() {
    let cleanup_path = &mut None;

    if let Err(e) = run(cleanup_path) {
        eprintln!("Error: {}", e);

        if &e.0 == NO_ARGS {
            eprintln!("{}", USAGE);
        }
        
        if let Some(path) = cleanup_path {
            fs::remove_file(path).unwrap();
        }

        exit(1);
    }
}

fn run(cleanup_path: &mut Option<PathBuf>) -> Result<(), AppError> {
    let suffix = OsString::from("-post");
    let mut args = env::args_os();
    let mut write_file_name;
    let arg1 = args.nth(1);
    let arg2 = args.next();

    let read_path = match &arg1 {
        None => {
            return Err(AppError(String::from(NO_ARGS)));
        },
        Some(path_str) => Path::new(path_str)
    };

    let read_file = fs::File::open(read_path).map_err(AppError::new)?;

    let reader = io::BufReader::new(read_file);

    let write_path = match &arg2 {
        None => {
            let ext = read_path.extension();
            let stem = read_path.file_stem();
            write_file_name = OsString::from(stem.unwrap());
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

            read_path.with_file_name(&write_file_name)
        },
        Some(path_str) => {
            let path = PathBuf::from(path_str);

            write_file_name = path.file_name().ok_or(
                AppError(format!("No file in \"{:#?}\"", path_str))
            )?.into();
            
            path
        },
    };

    let mut map = qmap::parse(reader).map_err(AppError::new)?;

    let write_file = fs::File::create(&write_path).map_err(
        |e| AppError(format!("{:#?}: {}", write_file_name, e))
    )?;

    *cleanup_path = Some(write_path.clone());

    let mut writer = io::BufWriter::new(write_file);

    patch_skill(&mut map);

    map.write_to(&mut writer).map_err(AppError::new)?;

    Ok(())
}
