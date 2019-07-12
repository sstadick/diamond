# Diamond

A Rust library to emulate the diamond operator in Perl / fileinput in Python.

This is very much under development and started when I was looking at https://gist.github.com/ayosec/2ee0993247e003b42c5c

## Prior Art

- fileinput, pythyon: https://github.com/python/cpython/blob/3.7/Lib/fileinput.py
- diamond operator, perl: https://perlmaven.com/the-diamond-operator
- fileinput, rust: https://crates.io/crates/fileinput lacks reading from
  stdin
- grabinput, rust: https://crates.io/crates/grabinput not performant,
  not safe

## Motivation

This would be a super useful operator to have for day to day usage and
small scripts as they come up.
