#[derive(Debug)]
struct Rectangle {
    height: usize,
    width: usize,
}
impl Rectangle {
    fn area(&self)->usize{ // borrows the self 
       self.width * self.height
    }
    fn can_hold(&self,rec:&Rectangle)->bool{
        self.width > rec.width && self.height > rec.height
    }
    fn square(size:usize)->Self{
        Self {
            width:size,
            height:size
        }
    }
}
fn main() {
    let scale = 2;
    let rectangle1 = Rectangle {
        height: dbg!(30*scale),
        width: 40,
    };

    println!("The area of the rectangle is {}", rectangle1.area()); //{:#?}=--> for the debugging
    dbg!(&rectangle1); //--> The dbg returns the ownsership so we dont the dbg to take ownership so
    //we using reference
    
    
    let rec2 = Rectangle{
        height:30,
        width:50
    };
    let rec3 = Rectangle{
        height:10,
        width:40
    };

    println!("Can rec2 hold rec3? {}",rec2.can_hold(&rec3));

    let sq = Rectangle::square(3);
}

fn area(rec: &Rectangle) -> usize {
    rec.width * rec.height
}
