fn main() {
    let mut s = String::from("hello");

    change(&mut s);

    println!("{s}");
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}



fn main() {
    let mut s = String::from("hello");
    for &item in s.as_bytes().iter() {
      if item == b'l' {
        s.push_str(" world");
      }
    }
    println!("{s}");
  }
  

fn main() {
    let mut v = vec![1, 2, 3];
    let n = v[0];
    v.push(4);
    println!("{n}");
}

fn main() {
    let v = vec![1, 2, 3];
    let v_ref: &Vec<i32> = &v;
    let v2 = v_ref.clone();
    drop(v2);
    drop(v);
    }