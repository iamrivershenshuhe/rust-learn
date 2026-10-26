use std::{cmp::Ordering, io}; //prelude
//io 庫在 std

use rand::Rng; //trait

fn main() {
    println!("猜一個數"); /*宏*/

    //let foo = 1;
    //let bar = foo; //這邏輯在Rust世界裡面就是無法修改的那個比如說我等一下直接強行把副去把它改成二的話沒辦法錯誤無法為沒辦法在Define他一次，那我們這邊可能要用另外一種思路在那個這個變亮前面加一個M U T

    //Rand library crate
    // cargo
    //
    let secret = rand::thread_rng().gen_range(1, 101); //隨機數生成器

    println!("{}", secret);

    loop {
        let mut guess = String::new();

        io::stdin().read_line(&mut guess /*你的概念理解就是上面不是我們Devin一個Mother ball get嗎？然後那中間這個mate它指向著前面有一個Ref：rain他就是指向guess這個地方。*/).expect("無法讀取");
        //可以注意一下，就是我剛才理解我們剛剛的那個reference就是在rest世界裡面也是不能被改動了還是要加上mut
        //
        // io::Result Ok(成功並戴返回值), Err(操做失敗+失敗原因)
        //
        //
        //shadow
        let guess: u32 = guess.trim().parse().expect("無法讀取");

        match guess.cmp(&secret) {
            Ordering::Less => println!("too small"), //arm

            Ordering::Greater => println!("too big"),

            Ordering::Equal => {
                println!("correct {}", guess);
                break;
            }
        }
    }
}

//這邊這一題要討論的是差數遊戲，然後描述一下這題題目，他說這題題目的是我要生成一個一到100之間的隨機數，然後相信事情呢是這時候我們會提示使用者輸入一個猜測然後拆完之後呢？我們的cold會提示他這東西是太大還是太小然後如果猜對話我們就中。
//
// //然後這邊先說我的第一個idea就是一個很簡單典型的Excel st然後Eve為什麼差不？因為他輸入他的輸出是說你去判斷這個東西有沒有差猜的正確，所以我判第一個判斷是大方向是E-flat的，所以兩種情況第一步呢？使用者輸入使用者輸入第隨機數，然後第二步的是玩家輸入然後一個Excel判斷if他說兩種情況衣服輸入大於我們的那個紙那則回答說大於這個print說這個帶這個紙L S L S小於p小於，然後這個L S那我們就直接pre猜中了這邊邏輯沒問題，但是但是卡在這個地方是我不知道怎麼寫一個輸出輸入的程序我覺得我們可以先開啟看一下他怎麼教輸入
