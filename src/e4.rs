fn main() {
    for i in 1..101 {
        if 0 == i % 15 {
            println!("fizzbuzz")
        } else if 0 == i % 3 {
            println!("fizz")
        } else if 0 == i % 5 {
            println!("buzz")
        } else {
            println!("{}", i)
        }
    }
}
