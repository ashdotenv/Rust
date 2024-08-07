fn main() {
    //  if else
    let a = 1;
    if a > 6 {
        print!("Hello World")
    } else {
        print!("Hello Bro")
    }
    let b = if a > 2 { 3 } else { 7 };
    print!("{}", b);
    //Loops
    // Loop Loop
    // Labelled Loop
    'def: loop {
        loop {
            print!("Hello World!");
            break 'def;
        }
    }
    let x = loop {
        break 5;
    };
    println!("{}", x);
    // while Loop
    let mut z = 0;
    while z < 5 {
        print!("z is less than 5 ");
        z += 1;
    }
    // For Loops
    let m = [1, 2, 3, 4, 5];
    for each in m {
        print!("{}", each);
    }
}
