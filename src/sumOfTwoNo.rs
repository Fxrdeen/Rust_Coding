use std::io;

pub fn sum() {
    let mut a: String = String::new();
    let mut b: String = String::new();
    println!("Enter the first number:");
    io::stdin().read_line(&mut a).expect("Its not a number");
    println!("Enter the second number:");
    io::stdin().read_line(&mut b).expect("Its not a number");
    let a: u32 = a.trim().parse().expect("They aint numbers");
    let b: u32 = b.trim().parse().expect("They aint numbers");
    let c: u32 = a + b;
    println!("The sum of both the numbers is: {c}");
}
