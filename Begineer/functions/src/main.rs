fn main() {
    let a = my_function();
    print!("{:?}", a);
    let b = my_function1();
    print!("{:?}", b);
}
fn my_function() {} //Returns Unit Type
fn my_function1() -> String {
    return "hello".to_string();
}
//Returns String
