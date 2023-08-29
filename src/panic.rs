use std::fs::File;
use std::io::ErrorKind;
pub fn panic(){
    let f = File::open("hello.txt").unwrap_or_else(|error|{
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error|{
                panic!("Problem in creating the file{:?}",error);
            })
        } else{
            panic!("Problem opening the file {:?}",error);
        }
       });
}
