fn _if_expression_1() -> i32 {
    let mut x = 10;

    if x % 2 == 0 {
        x = x / 2;
    } else {
        x = 3 * x + 1;
    }
    x
}

fn _if_expression_2() -> i32 {
    let mut x = 10;

    x = if x % 2 == 0 { x / 2 } else { 3 * x + 1 };
    x
}

fn _break_continue() {
    let v = vec![10, 20, 30];
    let mut iter = v.into_iter();

    while let Some(x) = iter.next() {
        println!("x -> {x}");

        let mut i = 0;
        while i < x {
            println!("x: {x} , i: {i}");

            i += 1;
            if i == 3 {
                break;
            }
        }
    }
}

fn _loop() {
    let mut x: i32 = 10;
    loop {
        x = if x % 2 == 0 { x / 2 } else { 3 * x + 1 }
    }
}

fn _pointer() {
    // let mut x: i32 = 10;
    // let ref_x: mut &i32 = 5;
    // *ref_x = 20;
    // println!("x: {x}");
}

fn generate_random_number() -> i32 {
    // Implementation based on https://xkcd.com/221/
    4 // Chosen by fair dice roll. Guaranteed to be random.
}

// trait
#[derive(Debug)]
enum CoinFlip {
    Head,
    Tail,
}

fn _enum() -> CoinFlip {
    let random_number = generate_random_number();
    if random_number % 2 == 0 {
        CoinFlip::Head
    } else {
        CoinFlip::Tail
    }
}

enum WebEvent {
    PageLoad,
    KeyPress(char),
    Click { x: i64, y: i64 },
}

fn inspect(event: WebEvent) {
    match event {
        WebEvent::PageLoad => println!("Page Loaded !!!"),
        WebEvent::KeyPress(c) => println!("Key Press {c}"),
        WebEvent::Click { x, y } => println!("Click {x}, {y} !!!"),
    }
}

fn main() {
    // if Expression
    // println!("X-1 -> {:?}", _if_expression_1());
    // println!("X-2 -> {:?}", _if_expression_2());

    // break, continue
    // _break_continue();

    //pointer
    // _pointer();

    // enum
    // println!("{:?}", _enum());

    // enum variant payload
    // inspect(WebEvent::PageLoad);
    // inspect(WebEvent::KeyPress('c'));
    // inspect(WebEvent::Click { x: 10, y: 20 });
}
