use std::{cmp::Ordering, io};

use rand::prelude::*;

//use : 引入模組
//rand : 我今天引用的腕部隨機函式庫 (crate) https://github.com/rust-random/rand/releases
// prelude : 有點想慣例 他會依據最常引用的一些核心工具
// * : means "for all" : 把所有 rand prelude 中的所有項目一次全部導入
//
// 今天如果不寫的話我就要在code 前面“手動引入特真

// ex rand::
//
//

//to obtain the user inputs we have to use this fucking library called io
//

fn main() {
    println!("guesd the numbers");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("fail to read the fucking line motherfucker");

    println!("your guess is {}", guess);

    //複習一下 常數的一些定義
    //
    // 第一種情況:（可讀） immutable
    // let fuck = 1000; this means in rust coding world we define a memory sector for this valuable
    // //it contians the idea that we cannot overwrite the value of this memory. for example, we cannot
    // //usre "fuck = 12;" to modify the valuable
    // & : 引用 （可以用同一個位置的數據）
    //
    // 第二種情況：（可讀可寫） mutable
    // let mut fuck = 19999;
    // fuck = 10; "ok 因為我們一開始定義它可讀可寫"
    //
    //
    //
    //
    // terminal : cargo update : ignore the cargo.lock
    //
    //
    //
    //
    //
    // *create a random numbers*
    let secret_number = rand::rng().random_range(1..=100);

    println!("secret number is {secret_number}");

    //use match instead of compare
    //
    //
    //
    let guess: u32 = guess.trim().parse().expect("please type a num");

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("too small"),
        Ordering::Greater => println!("too big"),
        Ordering::Equal => println!("of course son of the bitch"),
    }
}
