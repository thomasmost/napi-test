#![deny(clippy::all)]

#[macro_use]
extern crate napi_derive;

#[napi]
pub fn multiply(a: i32, b: i32) -> i32 {
  a * b
}
