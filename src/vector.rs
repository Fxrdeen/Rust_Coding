use std::io;
fn vector() {
    let mut v: Vec<i32> = Vec::new();
    loop {
        println!("Enter your choice to continue adding elements to the vector?");
        println!("1 --> YES");
        println!("0 --> NO");
        let mut ans = String::new();
        io::stdin().read_line(&mut ans).expect("bruh");
        let ans:i8 = match ans.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Bro You just had to enter an alphabet 💀");
                0
            },
        }; 
        if ans == 0 {
            break;
        }
        println!("Enter the integer vector element:");
        let mut el = String::new();
        io::stdin().read_line(&mut el).expect("Bruh");
        let el:i32 = match el.trim().parse() {
            Ok(num) =>num,
            Err(_) => {
                println!("Bro You dont know what a number is ? 💀");
                0
            }
        };
        v.push(el);
    }

    println!("The vector elements are:");
    for i in v{
        print!("{} ",i);
    }
}
