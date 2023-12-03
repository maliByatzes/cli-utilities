use std::fs;
use std::io::stdin;
use std::io::stdout;
use std::io::Write;
use std::process;

struct Args {
    files: Vec<String>,
    args_prov: bool,
    number: bool,
    show_all: bool,
}

fn parse_args() -> Result<Args, lexopt::Error> {
    use lexopt::prelude::*;

    let mut files = Vec::new();
    let mut args_prov = false;
    let mut number = false;
    let mut show_all = false;
    let mut parser = lexopt::Parser::from_env();

    while let Some(arg) = parser.next()? {
        match arg {
            Short('A') | Long("--show-all") => {
                show_all = true;
            }
            Short('n') | Long("--number") => {
                number = true;
            }
            Value(val) => {
                files.push(val.string()?);
            }
            _ => return Err(arg.unexpected()),
        }
    }

    if !files.is_empty() {
        args_prov = true;
    }

    Ok(Args {
        files,
        args_prov,
        number,
        show_all,
    })
}

fn read_user_input() {
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

fn print_files(args: &Args) -> Result<(), lexopt::Error> {
    match args.args_prov {
        true => {
            for file_path in &args.files {
                let contents = fs::read_to_string(file_path).unwrap_or_else(|err| {
                    eprintln!("Cannot read file: {err}");
                    process::exit(1);
                });

                let lines = contents.lines().enumerate();
                let print_line = match (args.number, args.show_all) {
                    (true, true) => |(index, line)| println!("    {}  {}$", (index + 1), line),
                    (true, false) => |(index, line)| println!("    {}  {}", (index + 1), line),
                    (false, true) => |(_, line)| println!("{}$", line),
                    (false, false) => |(_, line)| println!("{}", line),
                };

                lines.for_each(print_line);
                println!();
            }
        }
        false => read_user_input(),
    }

    Ok(())
}

fn main() -> Result<(), lexopt::Error> {
    let args = parse_args()?;

    print_files(&args)?;

    Ok(())
}
