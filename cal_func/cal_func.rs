fn main() {
    let a = 2;
    let b = 5;
    let sum = a + b;

    println!("Sum is {}", sum);

    println!("Power of 1001, p: {}", power(1001));

    println!("Power of 1000, p: {}", power(1000));
}

fn power(n: i32) -> i32 {
    if n > 1000 {
        println!("n is too big");
        return -1;
    }

    return n * n;
}
