fn main() {
    let s1 = String::from("hello");
    takes_ownsership(s1);
    let mut s2 = s1.clone(); //drop trait has been called on s1
    s2.push_str(",world!");
    println!("{s2}");
}

fn takes_ownsership(s:String){ //takes the ownsership of s1
    println!("{s}");
}
