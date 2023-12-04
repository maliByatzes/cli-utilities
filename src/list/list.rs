use std::fs;
use lexopt::prelude::*;

struct Args {
    directory: String,
}

/* TODO: Structure in a way where no args are neeede just `list` */

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
        directory: directory.ok_or("Missing argument [DIRECTORY]")?,
    })
}

fn list_files(directory: &str) -> Result<(), Box<dyn std::error::Error>> {
    for entry in fs::read_dir(directory)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            println!("{}/", path.display());
        } else {
            println!("{}", path.display());
        }
    }

    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = parse_args()?;
    list_files(&args.directory)?;
    Ok(())
}
