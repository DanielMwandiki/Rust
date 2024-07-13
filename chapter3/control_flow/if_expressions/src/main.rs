
//if statement

fn main() {
    //let number = 3;
    let number = 7; 

    if number < 5 {
        println!("Conditions is true");
    }
    else {
        println!("Condtion is false");
    }
}

// Else if

fn main (){
    let number = 6;

    if number %4 == 0{
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3 or 2");
    }
}





//test 1
fn main() {
    let cond = true;

    let x;
    if cond {
        x = 1;
    } else {
        x = 2;
    }
    println!("{x}")
}

//test 2 Using if in a let Statement

fn main() {
    let x = 1; // must be bool
    let y = if x { 0 } else { 1 }; 
    println!("{y}");
  }