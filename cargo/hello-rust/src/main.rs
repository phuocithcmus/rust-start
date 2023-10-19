fn hello_world() {
    let mut x: i8 = 6;
    print!("{x}");

    while x != 1 {
        if x % 2 == 0 {
            x = x / 2;
        } else {
            x = 3 * x + 1;
        }
        print!(" -> {x}");
    }
    println!();
}

fn compound_types() {
    // arrays
    let mut arr: [i32; 10] = [2; 10];
    println!("{:?}", arr);

    println!("change arr[5] = 0 ");
    arr[5] = 0;
    println!("{:?}", arr);
    // tuples
    let mut tuples: (i32, bool) = (6, true);
    println!("t.1 -> {}", tuples.0);
    println!("t.2 -> {}", tuples.1);
}

// fn dangling() {
//     let ref_x: &i32;
//     {
//         let x: i32 = 10;
//         ref_x = &x;
//     }

//     println!("ref_x: {ref_x}");
// }

fn slices() {
    let mut a: [i32; 6] = [10, 20, 30, 40, 50, 60];
    println!("a: {a:?}");

    let s: &[i32] = &a[2..4];

    // s[3] = println!("s: {s:?}");
}

fn string_and_str() {
    let s1: &str = "nd";
    println!("s1 -> {s1}");

    let mut s2: String = String::from("phuoc");
    s2.push_str(s1);

    println!("s2 -> {s2}");
}

fn is_divisible(n: u32, divisor: u32) -> bool {
    if divisor == 0 {
        return false;
    }

    n % divisor == 0
}

fn fizzbuzz(n: u32) -> String {
    let fizz = if is_divisible(n, 3) { "fizz" } else { "" };
    let buzz = if is_divisible(n, 5) { "buzz" } else { "" };

    if fizz.is_empty() && buzz.is_empty() {
        return format!("{n}");
    }
    format!("{fizz}{buzz}")
}

fn print_fizzbuzz_to(n: u32) {
    for i in 1..=n {
        println!("{}", fizzbuzz(i));
    }
}

struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn inc_width(&mut self, delta: u32) {
        self.width += delta;
    }

    fn new(width: u32, height: u32) -> Rectangle {
        Rectangle { width, height }
    }
}

fn print_area() {
    let mut rec: Rectangle = Rectangle::new(5, 5);

    println!("old area -> {}", rec.area());

    rec.inc_width(6);

    println!("new area -> {}", rec.area());
}

// IF LET EXPRESSION
fn _if_let(value: &str) {
    let arg = std::env::args().next();

    if let Some(value) = arg {
        println!("Program name: {value}");
    } else {
        println!("Missing name?");
    }
}

fn main() {
    // hello world
    // hello_world()

    // compound_types
    // compound_types()

    //dang lings
    // dangling();

    // slice
    // slices();

    //string and &str
    // string_and_str();

    // fizzbuzz
    // print_fizzbuzz_to(20);

    // class
    // print_area();

    // _if_let
    _if_let("a");
}
