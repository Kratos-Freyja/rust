// Copyright 2018 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![feature(label_break_value)]

// Simple continue pointing to an unlabeled break should yield in an error
fn continue_simple() {
    'b: {
        continue; //~ ERROR unlabeled `continue` inside of a labeled block
    }
}

// Labeled continue pointing to an unlabeled break should yield in an error
fn continue_labeled() {
    'b: {
        continue 'b; //~ ERROR `continue` pointing to a labeled block
    }
}

// Simple continue that would cross a labeled block should yield in an error
fn continue_crossing() {
    loop {
        'b: {
            continue; //~ ERROR unlabeled `continue` inside of a labeled block
        }
    }
}

pub fn main() {}
