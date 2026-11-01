use std::io;

fn main() {
    //println!("Hello, world!");

    //let mut x: u8 = 255;
    //x = x + 1;
    //這邊會直接 panic 因為我們剛剛觸發了 integer overflow
    //簡單來說 data type 分成 signed(負數 0 正數) 跟 unsigned(0 正數)
    //1. signed
    // for example i8 means 8 bit for 負數 0 正數 所以範圍是 -2^7~0~2^7-1
    //2. unsigned
    // for example u8 means 8 bit for 0 正數 所以範圍是 0 ~ 2^8-1
    //
    // 那剛剛這邊為啥會觸發 panic 主要是因為 我今天 x = x+ 1 導致我變成 256 而256 需要9bit 這時候就overflow 了
    //
    //let tup: (i32, f64, u8) = (500, 6.4, 1);
    //let (x, y, z) = tup;

    //println!("{x} {y} {z}");

    //let guess: u32 /*unsigned integer*/ = "24".parse().expect("not a number");

    //let five_hundred = x.0;

    //let six_point_four = x.1;

    //let one = x.2;

    //  print!("{five_hundred} {six_point_four} {one}")
    //
    //
    let a = [1, 2, 3, 4, 5];

    println!("please enter an array index");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read the line");

    let index: usize = index
        .trim()
        .parse()
        .expect("index entered was not a fucking number");

    let element = a[index];

    println!("the value of the element index at index {index} is : {element}");
}
