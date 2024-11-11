use thiserror::Error;

#[derive(Error, Debug)]
pub enum MyError {
    #[error("I/O error: {0}")]
    IO(#[from] std::io::Error),
    #[error("Parse error: {0}")]
    Parse(#[from] std::num::ParseIntError),
    #[error("Serialize json error: {0}")]
    Serialize(#[from] serde_json::Error),
    #[error("Custom error: {0}")]
    Custom(String),
}
fn main() -> Result<(), MyError> {
    println!("size of MyError is {}", std::mem::size_of::<MyError>());

    fail_with_error()?;

    Ok(())
}

fn fail_with_error() -> Result<(), MyError> {
    Err(MyError::Custom("This is a custom error".to_string()))
}
