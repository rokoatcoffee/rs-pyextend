#[macro_use]
extern crate cpython;

use cpython::{Python, PyResult};

fn palindrom(_py: Python, sentence: String) -> PyResult<bool> {
    let chars:Vec<char> = sentence.replace(" ", "").to_lowercase().chars().collect();
    let size:usize = chars.len();

    for index in 0..(size / 2) {
        if chars[index] != chars[size - 1 - index] {
            return Ok(false);
        }
    }
    Ok(true)
}

py_module_initializer!(libpyextenstion, initlibpyextenstion, PyInit_libpyextenstion, |py, m | {
    m.add(py, "libpyextenstion", py_fn!(py, palindrom(sentence: String)))?;
    m.add(py, "__doc__", "Function to check if a string is a palindrom. Returns a boolean.")?;
    Ok(())
});