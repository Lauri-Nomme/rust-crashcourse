#[derive(Debug)]
struct Frame {
    width: u32,
    height: u32,
}

#[derive(Debug)]
enum ParseError {
    TooFewArgs,
    TooManyArgs,
    InvalidInteger(String),
}

struct ParseArgs(std::env::Args);

impl ParseArgs {
    fn new() -> ParseArgs {
        ParseArgs(std::env::args())
    }

    fn require_arg(&mut self) -> Result<String, ParseError> {
        match self.0.next() {
            None => Err(ParseError::TooFewArgs),
            Some(arg) => Ok(arg),
        }
    }

    fn require_no_args(&mut self) -> Result<(), ParseError> {
        match self.0.next() {
            None => Ok(()),
            Some(_) => Err(ParseError::TooManyArgs),
        }
    }
}

fn parse_u32(str: String) -> Result<u32, ParseError> {
    match str.parse() {
        Ok(u32) => Ok(u32),
        Err(_) => Err(ParseError::InvalidInteger(str)),
    }
}

fn parse_args() -> Result<Frame, ParseError> {
    let mut args = ParseArgs::new();

    // skip the command name
    args.require_arg()?;

    let width_str = args.require_arg()?;
    let height_str = args.require_arg()?;
    args.require_no_args()?;

    Ok(Frame { width: parse_u32(width_str)?, height: parse_u32(height_str)? })
}

fn main() {
    println!("{:?}", parse_args());
}