fn main() {
    let mut s = String::from("String is new");
    let ans: &str= first_word(&s)
    s.clear(); //mutable borrow cannot occur as immutable borrow is used after this
    println!("{ans}");
}

fn first_word(s: &str) ->&str { 
    let array = s.as_bytes();

    for (i, &item) in array.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}
