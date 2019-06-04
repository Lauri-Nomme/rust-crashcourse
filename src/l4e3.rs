struct Fibonacci {
    i0: u32,
    i1: u32,
}

impl Fibonacci {
    fn new() -> Fibonacci {
        Fibonacci {
            i0: 0,
            i1: 1,
        }
    }
}

impl Iterator for Fibonacci {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        let temp = self.i0;
        self.i0 = self.i1;
        self.i1 += temp;
        Some(temp)
    }
}

fn main() {
    for i in Fibonacci::new().take(20) {
        println!("{}", i);
    }
    println!("All done!");
}