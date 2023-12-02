use std::fs;
use std::io::ErrorKind;
use std::io::stdout;
use std::io::stdin;
use std::io::Write;

struct Args {
    file_path: String,
    valid_file_path: bool,
    args_prov: bool,
}

fn parse_args() -> Result<Args, lexopt::Error> {
    use lexopt::prelude::*;

    let mut file_path = None;
    let valid_file_path = false;
    let mut args_prov = false;
    let mut parser = lexopt::Parser::from_env();
    while let Some(arg) = parser.next()? {
        match arg {
            Value(val) => {
                file_path = Some(val.string()?);
                args_prov = true;
            }
            _ => return Err(arg.unexpected()),
        }
    }

    Ok(Args {
        file_path: match file_path {
            Some(f) => f,
            _ => "".to_owned(),
        },
        valid_file_path,
        args_prov,
    })
}

fn main() -> Result<(), lexopt::Error> {
    let mut args = parse_args()?;
    let file_path = args.file_path;

    println!("{}", file_path);

    let file_contents = match fs::read_to_string(file_path) {
        Ok(contents) => contents,
        Err(err) => match err.kind() {
            ErrorKind::NotFound => {
                args.valid_file_path = false;
                "".to_owned()
            },
            _ => "".to_owned(),
        },
    };

    if args.args_prov {
        if args.valid_file_path {
            println!("{}", file_contents);
        }
    } else {
        loop {
            print!("> ");
            stdout().flush().expect("Cannot flush output");

            let mut buffer = String::new();
            stdin().read_line(&mut buffer)
                .expect("Cannot read user input");

            println!("{buffer}");
        }
    }

    Ok(())
}
