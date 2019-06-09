extern crate webbrowser;
extern crate rand;
use rand::Rng;

#[allow(dead_code)]
fn antigravity() -> () {
    webbrowser::open("https://xkcd.com/353/").is_ok();
}

#[allow(dead_code)]
pub fn approx_pi() -> f32 {
    let mut in_a_circle : f32 = 0.0;
    let in_a_square : f32 = 1_000_000.0;

    for _ in 0..in_a_square as i32 {
        
        let x : f32 = rand::thread_rng().gen_range(0.0, 1.0);
        let y : f32 = rand::thread_rng().gen_range(0.0, 1.0);
        
        if x.powf(2.0) + y.powf(2.0) <= 1.0 {
            in_a_circle += 4.0;
        }
    }
    
    return in_a_circle / in_a_square;
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

#[allow(dead_code)]
fn change_endian(arr: &mut [u8]) {
    let l = arr.len();
    for i in 0..(arr.len() / 2) {
        arr.swap(i, l - i - 1)
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

    #[test]
    fn approx_pi(){
        let pi = super::approx_pi();
        let diff = (pi - std::f32::consts::PI).abs();

        assert!(diff < 0.02);
    }

    #[test]
    fn endian_changed() 
    {
        let mut odd : [u8; 3] = [0xFF, 0xEE, 0xDD];
        let mut even : [u8; 4] = [0xFF, 0xEE, 0xDD, 0xCC];
        super::change_endian(&mut odd[..]);
        super::change_endian(&mut even[..]);
        
        assert_eq!(&[0xDD, 0xEE, 0xFF], &odd);
        assert_eq!(&[0xCC, 0xDD, 0xEE, 0xFF], &even);
    }
}