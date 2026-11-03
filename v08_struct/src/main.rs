#[derive(Debug)]

struct Rectangle {
    width: u32,
    length: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.length
    }

    fn rec_length(&self) -> u32 {
        (self.width + self.length) * 2
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        length: 40,
    };

    //let area = calculate_the_area(&rect1);
    //
    //
    let p = rect1.area();

    println!("{p}");
}

//fn calculate_the_area(rectangle: &Rectangle) -> u32 {
//    rectangle.width * rectangle.length
//aka rectangle.0*rectangle.1
//}
