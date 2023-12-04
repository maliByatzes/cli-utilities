use std::{fs, env, path::PathBuf};

// TODO: Add args for FILE
// TODO: Display items better

/*
struct Args {
    directory: String,
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
        directory: directory.ok_or("Missing argument [DIRECTORY]")?,
    })
}
*/

fn list_files(path: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    for entry in fs::read_dir(path)? {
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
    let path = env::current_dir()?;
    // let args = parse_args()?;
    list_files(&path)?;
    Ok(())
}
