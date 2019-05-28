fn main() {
    let val: String = String::from("Hello, World!");
    printer(&val);
    printer(&val);
}

fn printer(val: &String) {
    println!("The value is: {}", val);
}