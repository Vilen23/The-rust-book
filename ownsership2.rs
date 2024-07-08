fn main(){
    let s1 = String::from("hello");
    let (s2,len) = calculate_length(s1); //gives ownership and get back the string for the ownership and length
    println!("The length of '{s2}' is {len}.");
}

fn calculate_length(s:String)->(String,usize){
    let length = s.len();
    (s,length) //sends back the string
}