// Copyright 2017 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![crate_name = "foo"]

// @has foo/fn.foo.html
// @has - '//*[@class="rust fn"]' 'pub fn foo<X, Y: ?Sized>(_: &X)'
// @has - '//*[@class="rust fn"]' 'where X: ?Sized,'
pub fn foo<X, Y: ?Sized>(_: &X) where X: ?Sized {}
