#[macro_use]
extern crate cpython;

extern crate webbrowser;

use cpython::{Python, PyResult};

fn antigravity(_py: Python) -> PyResult<bool> {
    if webbrowser::open("https://xkcd.com/353/").is_ok() {
        Ok(true)
    } else {
        Ok(false)
    }
}

fn palindrome(_py: Python, sentence: String) -> PyResult<bool> {
    let chars:Vec<char> = sentence.replace(" ", "").to_lowercase().chars().collect();
    let size:usize = chars.len();

    for index in 0..(size / 2) {
        if chars[index] != chars[size - 1 - index] {
            return Ok(false);
        }
    }
    Ok(true)
}

py_module_initializer!(libpyextend, initlibpyextend, PyInit_libpyextend, |py, m | {
    m.add(py, "palindrome", py_fn!(py, palindrome(sentence: String)))?;
    m.add(py, "antigravity", py_fn!(py, antigravity()))?;
    m.add(py, "__doc__", "Python extensions written in Rust, using cpython bindings.")?;
    Ok(())
});