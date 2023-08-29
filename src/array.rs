use std::io;


pub fn array(){
    let mut ele = String::new();
    io::stdin().read_line(&mut ele).expect("Bruh");
    let ele:i32 = ele.trim().parse().expect("Bruh");
    let mut v:Vec<i32> = Vec::new();
    for _ in 0..ele {
        let mut el = String::new();
        io::stdin().read_line(&mut el).expect("Bruh");
        let el:i32 = el.trim().parse().expect("Bruh");
        v.push(el);
    }
    let mut sum = 0;
    for i in v{
        sum += i; 
    }
    println!("The sum of the elements of the array is :{sum}");
}