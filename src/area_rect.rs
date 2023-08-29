use std::io;
pub fn get_area_rec(){
    let mut width = String::new();
    let mut height = String::new();
    println!("Enter the width:");
    io::stdin().read_line(&mut width).expect("Bruh Momentos Occured");
    let width :u32 = match width.trim().parse() {
      Ok(num) => num,
      Err(_) => 0,  
    };
    println!("Enter the height");
    io::stdin().read_line(&mut height).expect("Bruh Momentos Occured");
    let height :u32 = match height.trim().parse() {
      Ok(num) => num,
      Err(_) => 0,  
    };
    if width == 0 || height == 0{
        println!("Bro Are you dumb, dont know that height/width should be some positive number 💀");
    } else{
        let res = area(width, height);
        println!("The area of the rectangle is {res} sq units");
    }
}
pub fn area(width: u32, height:u32) -> u32{
    width*height
}