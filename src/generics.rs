
struct Point<T> {
    x:T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T{
        &self.x
    }
}

pub fn x(){
    let p1 = Point {x:5, y:6};
    let p2 = Point{x:5.0, y:6.0};
}