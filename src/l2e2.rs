#[derive(Debug)]
struct Foobar(i32);

impl Drop for Foobar {
    fn drop(&mut self) {
        println!("Dropping a Foobar: {:?}", self);
    }
}

impl Foobar {
    fn use_it(&self) {
        println!("I consumed a Foobar: {:?}", self);
    }
}

fn main() {
    let x = Foobar(1);
    println!("Before uses_foobar");
    x.use_it();
    x.use_it();
    println!("After uses_foobar");
}