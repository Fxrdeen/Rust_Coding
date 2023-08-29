struct Bro {
    name:String,
    age: i32,
    gender: String,
}
impl info for Bro {
    fn info(&self) {
        println!("The name is {} and the gender is {}",self.name, self.gender);
    }
}

struct Sis{
    name:String,
    age:i32,
    gender: String,
}
impl info for Sis {
    fn info(&self) {
        println!("The name is {} and the gender is {}",self.name, self.gender);
    }
}
pub trait info {
    fn info(&self);
}


fn tra(){
    let bro = Bro{
        name:String::from("fxr"),
        age:19,
        gender:String::from("Male"),
    };
    let sis = Sis{
        name:String::from("sis"),
        age:19,
        gender:String::from("Female"),
    };   
    bro.info();
    sis.info();
}

