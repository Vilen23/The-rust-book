fn main() {
    let s = String::from("hello");

    let len = calculate_length(&s); //sending the string as the refernce

    println!("{len}");

    let mut s1 = String::from("hello");
    change_string(&mut s1); //can only have one mutable referecne at a time
    //can have multiple immutable references but if so then no mutable reference is allowed
    println!("{s1}")
}

fn calculate_length(s: &String) -> usize {
    s.len() 
}

fn change_string(s:&mut String){
    s.push_str(",world!");
}

