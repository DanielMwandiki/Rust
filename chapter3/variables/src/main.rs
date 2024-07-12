//variables example
fn main() {
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");
}

// Constant example
const TWO: u32 = 1 + 1;

fn main() {
  println!("{TWO}");


//Shadowing Example
fn main() {
    let  x: u32 = 1;

    {
      let  x = x *2;
      println!("The value of x is: {x}")

    }

    println!("The value of x is: {x}");
}
  
