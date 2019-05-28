fn main() {
    let val: String = String::from("Hello, World!");
    printer(val.clone());
    printer(val.clone());
}

fn printer(val: String) {
    println!("The value is: {}", val);
}