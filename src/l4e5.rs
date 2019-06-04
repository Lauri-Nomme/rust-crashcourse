fn main() {
    let res = (1..=10).fold(0, |acc, i| acc + i);
    println!("{:?}", res);
}