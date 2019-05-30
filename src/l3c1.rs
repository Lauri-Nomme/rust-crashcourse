use std::env::args;

fn main() {
    for a in args() {
        println!("{:?}", a);
    }

    {
        let mut args = args();
        while let Some(arg) = args.next() {
            println!("{:?}", arg);
        }
    }

    {
        let mut args = args();
        args.next();

        for a in args {
            println!("{:?}", a);
        }
    }
}