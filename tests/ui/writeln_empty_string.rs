// Copyright 2014-2018 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.


#![feature(tool_lints)]

#![allow(unused_must_use)]
#![warn(clippy::writeln_empty_string)]
use std::io::Write;

fn main() {
    let mut v = Vec::new();

    // These should fail
    writeln!(&mut v, "");

    let mut suggestion = Vec::new();
    writeln!(&mut suggestion, "");

    // These should be fine
    writeln!(&mut v);
    writeln!(&mut v, " ");
    write!(&mut v, "");

}