use std::fs;
use std::io::stdin;
use std::io::stdout;
use std::io::Write;
use std::process;

struct Args {
    files: Vec<String>,
    args_prov: bool,
}

fn parse_args() -> Result<Args, lexopt::Error> {
    use lexopt::prelude::*;

    let mut files = Vec::new();
    let mut args_prov = false;
    let mut parser = lexopt::Parser::from_env();

    while let Some(arg) = parser.next()? {
        match arg {
            Value(val) => {
                files.push(val.string()?);
            }
            _ => return Err(arg.unexpected()),
        }
    }

    if !files.is_empty() {
        args_prov = true;
    }

    Ok(Args { files, args_prov })
}

fn print_files(args: &Args) -> Result<(), lexopt::Error> {
    match args.args_prov {
        true => {
            for file_path in &args.files {
                /* Not ideal solution for now */
                if file_path.trim() == "-" {
                    loop {
                        print!("> ");
                        let mut buffer = String::new();
                        stdout().flush().unwrap_or_else(|err| {
                            println!("Cannot flush stdout: {err}");
                            process::exit(2);
                        });

                        stdin().read_line(&mut buffer).unwrap_or_else(|err| {
                            println!("Cannot read user input: {err}");
                            process::exit(3);
                        });

                        print!("{buffer}");
                    }
                }
                let contents = fs::read_to_string(file_path).unwrap_or_else(|err| {
                    println!("Cannot read file: {err}");
                    process::exit(1);
                });
                println!("{}", contents);
            }
        }
        false => loop {
            print!("> ");
            let mut buffer = String::new();
            stdout().flush().unwrap_or_else(|err| {
                println!("Cannot flush stdout: {err}");
                process::exit(2);
            });

            stdin().read_line(&mut buffer).unwrap_or_else(|err| {
                println!("Cannot read user input: {err}");
                process::exit(3);
            });

            print!("{buffer}");
        },
    }

    Ok(())
}

fn main() -> Result<(), lexopt::Error> {
    let args = parse_args()?;

    print_files(&args)?;

    Ok(())
}
