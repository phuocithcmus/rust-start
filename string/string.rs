fn main() {
    let _url: &str = "phuocnd.dev";

    let mut new_url = String::new();

    new_url.push_str("phuocnd.dev");

    let new_url = &new_url;

    println!("str {}", new_url);

    let _array: [&str; 4] = ["phuocn", "phuocn", "phuocn", "phuocn"];

    println!("array: {:?}", _array);
}
