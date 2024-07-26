/// Adds two numbers
///
/// # Examples
///
/// ```
/// use conventional_commit_example::add;
///
/// let result = add(2, 3);
/// assert_eq!(result, 5);
/// ```
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

/// Subtracts two numbers
///
/// # Examples
///
/// ```
/// use conventional_commit_example::subtract;
///
/// let result = subtract(3, 2);
/// assert_eq!(result, 1);
/// ```
pub fn subtract(a: i32, b: i32) -> i32 {
    a - b
}

/// Multiplies two numbers
///
/// # Examples
///
/// ```
/// use conventional_commit_example::multiply;
///
/// let result = multiply(2, 3);
/// assert_eq!(result, 6);
/// ```
pub fn multiply(a: i32, b: i32) -> i32 {
    a * b
}

enum MathERROR {
    DivisionByZero,
}
/// Divides two numbers
///
/// # Examples
///
/// ```
/// use conventional_commit_example::divide;
///
/// let result = divide(6, 3);
/// assert_eq!(result, 2);
/// ```
pub fn divide(a: i32, b: i32) -> Result<i32, MathERROR> {
    if b == 0 {
        return Err(MathERROR::DivisionByZero);
    }

    Ok(a / b)
}
