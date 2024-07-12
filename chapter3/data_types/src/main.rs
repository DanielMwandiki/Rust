// Integers

fn main() {
    let x :u8 = "42" // 8bit
    let x :i16 = "21" // 16 bit
    let x :u32 = "43" // 32 bit
    let x :i64 = "32" //64 bit
    let x :u128 = "72" // 128 bit
    let x :usize = "98" // depend on the architecture of the computer 
}

// Floating point type

fn main() {
    let x = 2.0; // f64

    let y: f32 = 3.0; // f32
}

// Characters

fn main() {
    let a = 'a';
    let p: char = 'p'; // with explicit type annotation
    let crab = '🦀';

    println!("Oh look, {} {}! :{}", a, crab, p);
}

// Boolean

fn main() {
    let t = true;

    let f: bool = false; // with explicit type annotation
}

//Tuples

fn main() {
    let tup = (500, 6.4, 1);

    let (x, y, z)= tup;

    println!("The vvalue of y is: {y}");

}

// Arrays

fn main() {
    let a = [1, 2, 3, 4, 5];
}

// Tuple

fn main() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);
}