#![allow(unused_imports)]

use std::io::*; // should warn

// Should NOT warn: regular (non-glob) import
use std::collections::HashMap;

// Should NOT warn: prelude imports are allowed
use std::prelude::rust_2021::*;

fn main() {}
