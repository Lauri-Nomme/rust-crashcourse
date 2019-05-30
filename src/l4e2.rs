struct TheAnswer {
    i: u32,
}

impl TheAnswer {
    fn new() -> TheAnswer {
        TheAnswer {
            i: 1
        }
    }
}

impl Iterator for TheAnswer {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.i > 10 {
            None
        } else {
            let res = Some(self.i);
            self.i += 1;
            res
        }
    }
}

fn main() {
    // only take 10 to avoid looping forever
    for i in TheAnswer::new() {
        println!("The answer to life, the universe, and everything is {}", i);
    }
    println!("All done!");
}