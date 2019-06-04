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

struct Doubler<I> {
    iter: I
}

impl<I> Doubler<I> {
    fn new(iter: I) -> Doubler<I> {
        Doubler {
            iter
        }
    }
}

impl<I, X> Iterator for Doubler<I>
    where
        I: Iterator<Item=X>,
        X: std::ops::Add<Output=X>,
        X: Copy, {
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
            .map(|val| -> Self::Item { val + val })
    }
}

fn main() {
    for i in Doubler::new(Fibonacci::new()).take(20) {
        println!("{}", i);
    }
    println!("All done!");
}