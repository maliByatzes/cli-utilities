struct Args {
    suppress_newline: bool,
    interpret_escapes: bool,
    message: Vec<String>,
}

fn parse_args() -> Result<Args, lexopt::Error> {
    use lexopt::prelude::*;

    let mut suppress_newline = false;
    let mut interpret_escapes = false;
    let mut message = Vec::new();
    let mut parser = lexopt::Parser::from_env();
    while let Some(arg) = parser.next()? {
        match arg {
            Short('n') | Long("no-newline") => {
                suppress_newline = true;
            }
            Short('e') | Long("interpret-escapes") => {
                interpret_escapes = true;
            }
            Value(val) => {
                message.push(val.string()?);
            }
            Long("help") => {
                println!("Usage: echom [-n|--no-newline] [-e|--interpret-escapes] MESSAGE");
                std::process::exit(0);
            }
            _ => return Err(arg.unexpected()),
        }
    }

    Ok(Args {
        suppress_newline,
        interpret_escapes,
        message,
    })
}

fn main() -> Result<(), lexopt::Error> {
    let args = parse_args()?;
    let mut message = args.message.join(" ");
    if args.interpret_escapes {
        // TODO: Implement escape characters
    }

    if !args.suppress_newline {
        message.push('\n');
    }
    print!("{}", message);
    Ok(())
}
