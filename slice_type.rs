fn main(){
    let s = String::from("String is new");

    let ans:usize = first_word(&s);
    println!("{ans}");
}

fn first_word(s:&String)->usize {
    let array = s.as_bytes();

    for(i,&item) in array.iter().enumerate() {
        if item == b' '{
            return i;
        }
    }

    s.len()
}