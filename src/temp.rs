// file temp.rs
// MIT License
// Copyright (c) 2025 Neeraj Singhal

/// Function used to convert temperature from celsius to fahrenheit
///
/// ## Example:
/// ```rust,ignore
/// let celsius = 0.0;
/// let result = celsius_to_fahrenheit(celsius);
/// assert!((result - 32.0).abs() < 1e-6);
/// ```
pub fn celsius_to_fahrenheit(temp: f64) -> f64 {
    (temp * 1.8) + 32.0
}

/// Function used to convert temperature from fahrenheit to celsius
///
/// ## Example:
/// ```rust,ignore
/// let fahrenheit = 32.0;
/// let result = fahrenheit_to_celsius(fahrenheit);
/// assert!(result.abs() < 1e-6);
/// ```
pub fn fahrenheit_to_celsius(temp: f64) -> f64 {
    (temp - 32.0) * (5.0 / 9.0)
}

#[cfg(test)]
mod tests {
    //! This module contains unit tests for above functions
    use super::*;
    use rand::Rng;

    #[test]
    fn ut_celsius_to_fahrenheit() {
        let celsius: f64 = rand::rng().random_range(0.0..1000.0);

        let result = celsius_to_fahrenheit(celsius);
        assert!((result - ((celsius * 1.8) + 32.0)).abs() < 1e-9);
    }

    #[test]
    fn ut_fahrenheit_to_celsius() {
        let fahrenheit: f64 = rand::rng().random_range(0.0..1000.0);
        let result = fahrenheit_to_celsius(fahrenheit);
        assert!((result - (fahrenheit - 32.0) * (5.0 / 9.0)).abs() < 1e-9);
    }
}
