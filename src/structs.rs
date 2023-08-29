struct Rect{
    width:u32,
    height:u32,
}
impl Rect {
    fn area(&self) -> u32{
        self.width * self.height
    }
    fn check_w(&self) -> bool {
        self.width > 0
    }
    fn check_h(&self) -> bool {
        self.height > 0
    }
    fn can_hold(&self, other:Rect) -> bool {
        self.width > other.width && self.height > other.height
    }
}
pub fn stuff(){
    let rec1 = Rect{
        width:12,
        height:12,
    };
    let rec2 = Rect{
        width:13,
        height:13,
    };
    if rec1.check_h() && rec1.check_w() {
        let res = rec1.area();
        println!("The area of the rectangle is {res} sq units");
    } else{
        println!("Bro why are you finding area of a line, something is 0 💀");
    }
    if rec1.can_hold(rec2) {
        println!("It can hold it the other rectangle");
    } else{
        println!("It cant lmao");
    }
    
    
}