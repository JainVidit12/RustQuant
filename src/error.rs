// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// RustQuant: A Rust library for quantitative finance tools.
// Copyright (C) 2023 https://github.com/avhz
// Dual licensed under Apache 2.0 and MIT.
// See:
//      - LICENSE-APACHE.md
//      - LICENSE-MIT.md
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

use std::{error::Error, fmt};

/// Error type for RustQuant.
#[derive(Debug)]
pub struct RustQuantError {
    /// Error message.
    message: String,
}

impl RustQuantError {
    /// Create a new RustQuant error.
    pub fn new(message: &str) -> RustQuantError {
        RustQuantError {
            message: String::from(message),
        }
    }
}

impl Error for RustQuantError {}

impl std::fmt::Display for RustQuantError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}
