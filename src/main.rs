use ecosystem::MyError;

fn main() -> Result<(), MyError> {
    println!("size of MyError is {}", std::mem::size_of::<MyError>());

    fail_with_error()?;

    Ok(())
}

fn fail_with_error() -> Result<(), MyError> {
    Err(MyError::Custom("This is a custom error".to_string()))
}
