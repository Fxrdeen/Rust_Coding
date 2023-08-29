fn main() {
    let mut arr: [i32; 5] = [0; 5];
    for i in 0..5 {
        let mut a: String = String::new();
        println!("Enter the element of the array:");
        io::stdin().read_line(&mut a).expect("Bruh");
        let a: i32 = a.trim().parse().expect("Bruh");
        arr[i] = a;
    }
    for n in arr {
        println!("sThe element is {n}");
    }
    let mut a = String::new();
    println!("Enter the integer:");
    io::stdin().read_line(&mut a).expect("Bruh");
    let a: i32 = match a.trim().parse() {
        Ok(err) => err,
        Err(_) => -1,
    }; 
    if a==-1{
        println!("Bro do you understand english, it says 'integer' 💀");
    } else{
        println!("The value of a is {a}")
    }
    

}