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

fn permutation(_py: Python, word: String) -> PyResult<Vec<String>> {
    let r: usize = word.len();
    let mut all: Vec<String> = Vec::new();
    let mut chars: Vec<char> = word.chars().collect();

    permute(&mut all, &mut chars, 0 , r);
    
    Ok(all)
}

fn permute(all: &mut  Vec<String>, c: &mut Vec<char>, l: usize, r: usize){ 
   if l == r {
       all.push(c.clone().iter().collect());
   }
   else
   { 
       for i in l..r {
           c.swap(l, i);
           permute(all, c, l + 1, r);
           c.swap(l, i);
       }
   } 
}

py_module_initializer!(libpyextend, initlibpyextend, PyInit_libpyextend, |py, m | {
    m.add(py, "palindrome", py_fn!(py, palindrome(sentence: String)))?;
    m.add(py, "antigravity", py_fn!(py, antigravity()))?;
    m.add(py, "permutation", py_fn!(py, permutation(word: String)))?;
    m.add(py, "__doc__", "Python extensions written in Rust, using cpython bindings.")?;
    Ok(())
});