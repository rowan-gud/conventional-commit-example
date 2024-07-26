/// Returns the sine of the angle. The input angle is interpreted as radians.
///
/// # Examples
///
/// ```
/// use conventional_commit_example::sin;
///
/// let x = std::f64::consts::PI / 2.0;
/// let err: f64 = 1e-6;
///
/// let actual = sin(x);
/// let expected = 1.0;
///
/// assert!((actual - expected).abs() < err);
/// ```
pub fn sin(x: f64) -> f64 {
    x.sin()
}

/// Returns the cosine of the angle. The input angle is interpreted as radians.
///
/// # Examples
///
/// ```
/// use conventional_commit_example::cos;
///
/// let x = std::f64::consts::PI;
/// let err: f64 = 1e-6;
///
/// let actual = cos(x);
/// let expected = -1.0;
///
/// assert!((actual - expected).abs() < err);
/// ```
pub fn cos(x: f64) -> f64 {
    x.cos()
}

/// Returns the tangent of the angle. The input angle is interpreted as radians.
///
/// # Examples
///
/// ```
/// use conventional_commit_example::tan;
///
/// let x = std::f64::consts::PI / 4.0;
/// let err: f64 = 1e-6;
///
/// let actual = tan(x);
/// let expected = 1.0;
///
/// assert!((actual - expected).abs() < err);
/// ```
///
pub fn tan(x: f64) -> f64 {
    sin(x) / cos(x)
}
