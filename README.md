# rs-pyextend

A simple implementation of CPython bindings in Rust, for >= Python3.6.
It was written in [Rust](https://www.rust-lang.org/ "Rust"), using [cpython](https://crates.io/crates/cpython "Rust crate") crate.


## Build

Project is written using nightly Rust (rustc 1.32.0-nightly (ca79ecd69 2018-11-11))

To build the library use release for the highest level of optimisation.

`cargo build --release`

The output is a dynamic library (shared object) in the `target/release`.

## Usage

The build library can be copied to a wanted destination where it can be imported.
It consists of a single function inside a module.

`libptextend.palindrome(sentence: String) -> bool`

The result visible in python is a boolean value.

Python usage is similar to Rust definition.

```python

import libpyextend as pyext

print(pyext.palindrome("aa bc    cbAa"))

```

## Test

For testing, measuring, and comparison of the Python itertools and Rust implementation, `pytest` is used, along with `pytest-benchmark`. Both easily installed using pip.

The `pyextend.py` file is used for tests. Both Python and Rust need to return a boolean.

The input string is changed, deleting spaces and changing it to lowercase.

To see the benchmarks, use `pytest pyextend.py`.

The Rust library should be in the same location as the python script.

## References

[Speed up your Python using Rust - RHD Blog](https://developers.redhat.com/blog/2017/11/16/speed-python-using-rust/?fbclid=IwAR2lHj26f-mRIYzfhT7_fLFsTQ1W-Fb6lXWQyi35VgfYru7_XzD71-XxMEc "Speed up your Python using Rust")

[dgrunwald/rust-cpython](https://github.com/dgrunwald/rust-cpython "cpython github")

[Extending Python with Rust (Samuel Cormier-Iijima)](https://www.youtube.com/watch?v=-ylbuEzkG4M&fbclid=IwAR0y6xQUCkTOubEL1AuLYQ_Ue1b_bcXZljgdW2_Zgs4veDrNZ3mGcyy8B8M "youtube")