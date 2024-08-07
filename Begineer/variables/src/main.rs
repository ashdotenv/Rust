fn main() {
    // creation
    let a: i32 = 5;
    // Mutability
    let mut b = 5;
    b = 10;
    // Shadowing
    let c = 10;
    let c = 20;
    print!("{}", c);
    // Scope
    let x = 50;
    {
        print!("{x}");
    }
    print!("{x}");
}
