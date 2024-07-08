fn main() {
    let mut counter: i32 = 0;

    'counting_up: loop {
        println!("count = {counter}");
        let mut remaining: i32 = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if counter == 2 {
                //Break the outside loop named counting_up
                break 'counting_up;
            }
            remaining -= 1;
        }
        counter += 1;
    }
    println!("End count = {counter}");

    reverse();
}

fn reverse(){
    let arr:[i32;5] = [1,2,3,4,5];
    for number in (0..5){
        println!("{}",arr[number]);
    }
}