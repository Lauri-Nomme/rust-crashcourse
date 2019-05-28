fn my_drop<T>(_x : T) {

}

fn main() {
    let x = 123123;
    my_drop(x);
    my_drop(x);
}