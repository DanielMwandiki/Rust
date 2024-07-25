
struct Rectangle{
    height:u32,
    width:u32,
}
fn main() {

    // let width1 = 30;
    // let height1 = 50;
    //let rect1 = (50, 30);
    let rect = Rectangle{
        height: 50,
        width: 30,
    };

    println!("The area of the rectangle is {} square pixels.",
            //area(rect1));
            area(&rect))
}

//fn area(width:u32, height:u32) -> u32{
   // width*height
//}

//fn area(dimension: (u32, u32)) -> u32{
    //dimension.0 * dimension.1
//}

fn area(rectangle: &Rectangle) -> u32{
    rectangle.height * rectangle.width
}


// test1

#[derive(Debug)]

struct Rectangle{
    height: u32,
    width: u32,
}

fn main() {
    let rect = Rectangle{
        height: 50,
        width:30,
    };

    let a = area(&rect);

    println!("{} * {} = {}", rect.width, rect.height, a);
}

fn area(rectangle: &Rectangle) -> u32{

    rectangle.height * rectangle.width

}