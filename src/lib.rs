//! # Decimal Floating-Point Arithmetic for Rust
//!
//! Based on bindings for **Intel(R) Decimal Floating-Point Math Library v2.3**

extern crate dfp_number_sys;

mod decimal128;

pub use decimal128::*;
