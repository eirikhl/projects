use std::error::Error as StdError;
use std::io::stdin;
use std::result::Result as StdResult;

type Result<T> = StdResult<T, Box<StdError>>;

fn main() -> Result<()> {
    println!("Hello, there!  What is your name?");
    
    let buffer = &mut String::new();
    stdin().read_line(buffer)?; // <- API requires buffer param as of Rust 1.0; returns `Result` of bytes read
    let res = match buffer.trim_end() {
        "" => "aight faq u den".to_owned(),
        name => format!("Hello, {}.  Nice to meet you!", name),
    };
    println!("{}", res);

    Ok(())
}
