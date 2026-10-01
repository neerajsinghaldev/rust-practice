// file main.rs
// MIT License
// Copyright (c) 2025 Neeraj Singhal

//! A small Rust practice application.

use rand::Rng;

// Import user-defined modules
mod file;
mod math;
mod temp;

/// main function
fn main() {    println!("\nHello World!\nI'm a Rustacean!\n");

    let secret_number = rand::rng().random_range(1..=100);
    println!("The secret number = {secret_number}");

    let a = 2;
    let b = 9;
    let sum = math::add(a, b);
    println!("Result {a} + {b} = {sum}");
    let sub = math::sub(a, b);
    println!("Result {a} - {b} = {sub}");
    let mul = math::mul(a, b);
    println!("Result {a} * {b} = {mul}");
    let div = math::div(2.0, 3.0);
    println!("Result 2.0 / 3.0 = {div}");

    let average = math::average(13, 2.3, 120.0);
    assert!((average - 45.1).abs() < 1e-6);
    println!("Test passed: Average of 13, 2.3 and 120 = {average}");

    let celsius_temp = 23.0;
    let fahrenheit_temp = temp::celsius_to_fahrenheit(celsius_temp);
    assert!((fahrenheit_temp - 73.4).abs() < 1e-6);
    println!("Test passed: celsius_to_fahrenheit({celsius_temp}) = {fahrenheit_temp}");

    let fahrenheit_temp = 73.4;
    let celsius_temp = temp::fahrenheit_to_celsius(fahrenheit_temp);
    assert!((celsius_temp - 23.0).abs() < 1e-6);
    println!("Test passed: fahrenheit_to_celsius({fahrenheit_temp}) = {celsius_temp}");
}
