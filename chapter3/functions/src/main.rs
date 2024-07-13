// Basic funncion

fn main() {
    println!("Hello World!");

    another_function();
}

fn another_function() {
    println!("Another Function")
}


// Parameters

fn main() {
    another_function(5);
}

fn another_function(x: i32) { //you must declare the type of each parameter
    println!("The value of x is {x}");
}


// Statements and  Expressions

fn f(x: i32) -> i32 { x + 1 }

fn main() {
  println!("{}", f({
    let y = 1;
    y + 1
  }));
}

// Functions with Return Values

fn five() -> i32 {
    5
}

fn main() {
    let x = five();

    println!("The value of x is: {x}");
}


// test
fn f(x: i8) {
    println!("{x}")
}

fn main() {
    f(0);
}
*/

//test2

fn f(x: i32) -> i32 { x + 1 }
fn main() {
  println!("{}", f({
    let y = 1;
    y + 1
  }));
}