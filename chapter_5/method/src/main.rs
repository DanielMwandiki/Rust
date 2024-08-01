#[derive(Debug)]

struct Rectangle{
    height: u32,
    width:u32,
}

impl Rectangle {
    fn area(&self) -> u32{

    self.height * self.width
    }

    fn can_hold(&self, other: &Rectangle) -> bool{
        self.height > other.height && self.width >other.width 
    }
}


fn main() {
    let rect1 = Rectangle{
        height: 50,
        width: 30,
    };

    let rect2 = Rectangle{
        height: 10,
        width: 40,
    };

    let rect3 = Rectangle{
        height: 60,
        width: 45,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    println!("The area of the rectangle is {} square pixels",
            rect1.area());
    
    println!("The area of the rectangle is {} square pixels",
            rect2.area());
    
    println!("The area of the rectangle is {} square pixels",
            rect3.area());
}

//test

struct Point(i32, i32);
fn main() {
  let p = Point(1, 2);
  impl p {
    fn x(&self) -> i32 { self.0 }
  }
  
  println!("{}", p.x());
}


//test 2

struct Point {
    x: i32,
    y: i32
  }
  impl Point {
    fn get_x(&mut self) -> &mut i32 {
      &mut self.x
    }
  }
  fn main() {
    let mut p = Point { x: 1, y: 2 };
    let x = p.get_x();
    *x += 1;
    println!("{} {}", *x, p.y);
  }