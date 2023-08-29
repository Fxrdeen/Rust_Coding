use std::io;
pub fn check(){
    let mut a = String::new();
    println!("Enter the integer");
    io::stdin().read_line(&mut a).expect("Bruh");
    let a:i32 = match a.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Bro its says, enter an integer 💀👻");
            0
        }
    };
    check_even(a);
}
fn check_even(a:i32){
    if a==0{
        return
    }
    let bruh = a%2==0;
    match bruh {
        true => {
            println!("The number is divisible by 2")
        }
        false => {
            println!("THe number is an odd number so not divisible by 2");
        }
    }
    
}
