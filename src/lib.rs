extern crate webbrowser;

#[allow(dead_code)]
fn antigravity() -> () {
    webbrowser::open("https://xkcd.com/353/").is_ok();
}

#[allow(dead_code)]
fn palindrom(sentence: &String) -> bool {
    let chars:Vec<char> = sentence.replace(" ", "").to_lowercase().chars().collect();
    let size:usize = chars.len();

    for index in 0..(size / 2) {
        if chars[index] != chars[size - 1 - index] {
            return false;
        }
    }
    true
}

#[allow(dead_code)]
fn permutate(word: &String) -> Vec<String> {
    let r: usize = word.len();
    let mut all: Vec<String> = Vec::new();
    let mut chars: Vec<char> = word.chars().collect();

    permute(&mut all, &mut chars, 0 , r);
    
    all
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

#[cfg(test)]
mod tests {
    #[test]
    fn is_palindrom() {
        assert_eq!(super::palindrom(&String::from("aabbaa")), true);
        assert_eq!(super::palindrom(&String::from("aabcaa")), false);
    }

    #[test]
    fn permutations() {
        let test_str = "ABCD";
        assert_eq!(super::permutate(&test_str.to_string()).len(), (1..test_str.len()+1).fold(1, |p, n| p*n));
    }

    #[test]
    fn antigravity(){
        super::antigravity();
    }
}