// file file.rs
// MIT License
// Copyright (c) 2025 Neeraj Singhal

use std::{error, result};

#[allow(dead_code)]
type TResult<T> = result::Result<T, TError>;

#[allow(dead_code)]
type TError = Box<dyn error::Error>;

/// Read in the file as String
#[allow(dead_code)]
pub fn read_file(_p: &str) -> TResult<String> {
    unimplemented!();
}

// Covert String to a Vector of Numbers
// Sum all of the Numbers together
// Write the Sum back into the file

