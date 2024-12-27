use std::array;

fn main() {
    // TODO: Create an array called `a` with at least 100 elements in it.
    let a: [i32; 110] = array::from_fn(|_| 0);
    println!("{:?}",a);
    // let a = [0; 110];
    if a.len() >= 100 {
        println!("Wow, that's a big array!");
    } else {
        println!("Meh, I eat arrays like that for breakfast.");
        panic!("Array not big enough, more elements needed");
    }

    Yun::hello();
    Bo::hello();
}
struct Yun;

impl Yun {
    fn hello() {
        println!("Hello, Yun!")
    }
}

struct Bo;

impl Bo {
    fn hello() {
        println!("Hello, Bo!")
    }
}
