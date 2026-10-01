// file math.rs
// MIT License
// Copyright (c) 2025 Neeraj Singhal

/// Function used to add two numbers
///
/// ## Example:
/// ```rust,ignore
/// let result = add(10, 2);
/// assert_eq!(12, result);
/// ```
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

/// Function used to subtract two numbers
///
/// ## Example:
/// ```rust,ignore
/// let result = sub(10, 2);
/// assert_eq!(8, result);
/// ```
pub fn sub(a: i32, b: i32) -> i32 {
    a - b
}

/// Function used to multiply two numbers
///
/// ## Example:
/// ```rust,ignore
/// let result = mul(10, 2);
/// assert_eq!(20, result);
/// ```
pub fn mul(a: i32, b: i32) -> i32 {
    a * b
}

/// Function used to divide two numbers
///
/// ## Example:
/// ```rust,ignore
/// let result = div(10.0, 2.0);
/// assert!((result - 5.0).abs() < 1e-6);
/// ```
pub fn div(a: f32, b: f32) -> f32 {
    a / b
}

/// Function used to calculate average of three numbers
///
/// ## Example:
/// ```rust,ignore
/// let result = average(10, 2.0, 3.0);
/// assert!((result - 5.0).abs() < 1e-6);
/// ```
pub fn average(a: i32, b: f64, c: f64) -> f64 {
    (f64::from(a) + b + c) / 3.0
}

#[cfg(test)]
mod tests {
    //! This module contains unit tests for above functions
    use super::*;

    #[test]
    fn ut_add() {
        assert_eq!(4, add(2, 2));
    }

    #[test]
    fn ut_sub() {
        assert_eq!(0, sub(2, 2));
    }

    #[test]
    fn ut_mul() {
        assert_eq!(4, mul(2, 2));
    }

    #[test]
    fn ut_div() {
        assert!((div(10.0, 2.0) - 5.0).abs() < 1e-6);
    }

    #[test]
    fn ut_average() {
        assert!((average(2, 2.0, 2.0) - 2.0).abs() < 1e-6);
    }
}
