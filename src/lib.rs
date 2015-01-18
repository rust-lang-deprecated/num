// Copyright 2014-2015 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Simple numerics.
//!
//! This crate contains arbitrary-sized integer, rational, and complex types.
//!
//! ## Example
//!
//! This example uses the BigRational type and [Newton's method][newt] to
//! approximate a square root to arbitrary precision:
//!
//! ```
//! # #![allow(unstable)]
//! extern crate num;
//!
//! use std::num::FromPrimitive;
//! use num::bigint::BigInt;
//! use num::rational::{Ratio, BigRational};
//!
//! fn approx_sqrt(number: u64, iterations: usize) -> BigRational {
//!     let start: Ratio<BigInt> = Ratio::from_integer(FromPrimitive::from_u64(number).unwrap());
//!     let mut approx = start.clone();
//!
//!     for _ in 0..iterations {
//!         approx = (&approx + (&start / &approx)) /
//!             Ratio::from_integer(FromPrimitive::from_u64(2).unwrap());
//!     }
//!
//!     approx
//! }
//!
//! fn main() {
//!     println!("{}", approx_sqrt(10, 4)); // prints 4057691201/1283082416
//! }
//! ```
//!
//! [newt]: https://en.wikipedia.org/wiki/Methods_of_computing_square_roots#Babylonian_method

#![feature(slicing_syntax)]
#![cfg_attr(test, deny(warnings))]
#![doc(html_logo_url = "http://www.rust-lang.org/logos/rust-logo-128x128-blk-v2.png",
       html_favicon_url = "http://www.rust-lang.org/favicon.ico",
       html_root_url = "http://doc.rust-lang.org/num/",
       html_playground_url = "http://play.rust-lang.org/")]

#![allow(unstable)]

extern crate "rustc-serialize" as rustc_serialize;

pub use bigint::{BigInt, BigUint};
pub use rational::{Rational, BigRational};
pub use complex::Complex;
pub use integer::Integer;
pub use iter::{range, range_inclusive, range_step, range_step_inclusive};
pub use traits::{Num, Zero, One, Signed, Unsigned, Bounded,
                 Saturating, CheckedAdd, CheckedSub, CheckedMul, CheckedDiv};

#[cfg(test)] use std::hash;

use std::ops::{Mul};

pub mod bigint;
pub mod complex;
pub mod integer;
pub mod iter;
pub mod traits;
pub mod rational;

/// Returns the additive identity, `0`.
#[inline(always)] pub fn zero<T: Zero>() -> T { Zero::zero() }

/// Returns the multiplicative identity, `1`.
#[inline(always)] pub fn one<T: One>() -> T { One::one() }

/// Computes the absolute value.
///
/// For `f32` and `f64`, `NaN` will be returned if the number is `NaN`
///
/// For signed integers, `::MIN` will be returned if the number is `::MIN`.
#[inline(always)]
pub fn abs<T: Signed>(value: T) -> T {
    value.abs()
}

/// The positive difference of two numbers.
///
/// Returns zero if `x` is less than or equal to `y`, otherwise the difference
/// between `x` and `y` is returned.
#[inline(always)]
pub fn abs_sub<T: Signed>(x: T, y: T) -> T {
    x.abs_sub(&y)
}

/// Returns the sign of the number.
///
/// For `f32` and `f64`:
///
/// * `1.0` if the number is positive, `+0.0` or `INFINITY`
/// * `-1.0` if the number is negative, `-0.0` or `NEG_INFINITY`
/// * `NaN` if the number is `NaN`
///
/// For signed integers:
///
/// * `0` if the number is zero
/// * `1` if the number is positive
/// * `-1` if the number is negative
#[inline(always)] pub fn signum<T: Signed>(value: T) -> T { value.signum() }

/// Raises a value to the power of exp, using exponentiation by squaring.
///
/// # Example
///
/// ```rust
/// use num;
///
/// assert_eq!(num::pow(2i, 4), 16);
/// ```
#[inline]
pub fn pow<T: Clone + One + Mul<T, Output = T>>(mut base: T, mut exp: usize) -> T {
    if exp == 1 { base }
    else {
        let mut acc = one::<T>();
        while exp > 0 {
            if (exp & 1) == 1 {
                acc = acc * base.clone();
            }
            base = base.clone() * base;
            exp = exp >> 1;
        }
        acc
    }
}

#[cfg(test)]
fn hash<T: hash::Hash<hash::SipHasher>>(x: &T) -> u64 {
    hash::hash::<_, hash::SipHasher>(x)
}
