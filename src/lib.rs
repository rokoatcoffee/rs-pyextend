extern crate webbrowser;

fn antigravity() -> () {
    webbrowser::open("https://xkcd.com/353/").is_ok();
}

fn palindrom(sentence: String) -> bool {
    let chars:Vec<char> = sentence.replace(" ", "").to_lowercase().chars().collect();
    let size:usize = chars.len();

    for index in 0..(size / 2) {
        if chars[index] != chars[size - 1 - index] {
            return false;
        }
    }
    true
}


#[cfg(test)]
mod tests {
    #[test]
    fn is_palindrom() {
        assert_eq!(super::palindrom(String::from("aabbaa")), true);
        assert_eq!(super::palindrom(String::from("aabcaa")), false);
    }

    #[test]
    fn antigravity(){
        super::antigravity();
    }
}