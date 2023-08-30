use std::io;

fn main() {
    let mut buf = String::new();

    println!("Your domain: ");

    io::stdin().read_line(&mut buf).ok().expect("error");

    println!("hello {}", buf)
}
