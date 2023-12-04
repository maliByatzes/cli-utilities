use lexopt::prelude::*;
use std::{env, fs, path::PathBuf};

// TODO: Display items better
// TODO: Add color schemes for files and directories

struct Args {
    directory: Option<String>,
}

fn parse_args() -> Result<Args, lexopt::Error> {
    let mut directory = None;
    let mut parser = lexopt::Parser::from_env();

    while let Some(arg) = parser.next()? {
        match arg {
            Value(val) if directory.is_none() => {
                directory = Some(val.string()?);
            }
            _ => return Err(arg.unexpected()),
        }
    }

    Ok(Args {
        directory,
    })
}

fn list_files(path: &PathBuf, args: &Args) -> Result<(), Box<dyn std::error::Error>> {
    if args.directory.is_none() {
        for entry in fs::read_dir(path)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                println!("{}/", path.display());
            } else {
                println!("{}", path.display());
            }
        }
    } else {
        // Error here is possibly unreachable??
        let directory = args.directory.clone().ok_or("Missing argument [DIRECTORY]")?; /* TODO: not use clone() */
        for entry in fs::read_dir(directory)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                println!("{}/", path.display());
            } else {
                println!("{}", path.display());
            }
        }
    }

    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path = env::current_dir()?;
    let args = parse_args()?;
    list_files(&path, &args)?;
    Ok(())
}
