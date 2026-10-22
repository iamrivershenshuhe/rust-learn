fn test_1(){

    let mut /*可修改*/ count = 0; //這邊呢我們要做的事情是建立一個可以修改的變數

    for i in 1..=5{  //迴圈從1~5

        count += 1;
        print!("i={}, count={}", i, count);
    }
}



fn test_2(){

    let mut count = 0;

    for i in 1..=10{

        if i%2==0{
            count+=i;
        }
    }

    print!("count={}", count);
}


//array and loop
fn array_1(){

    let numbers=[1,2,4,3,7];

    for number in numbers {

        print!("{} ", number);
    }
}

fn ifnd_max(){

    /*
        OK，我们先想一下這一題的思路是什麼。
        OK，他說要找最大值嘛，
        所以我們先 define問題。輸入是什麼？輸入是這個 array，然後輸出是這個 array裡面最大的 max嘛。然後 max呢，可能第一種方法是你先 define一個 max變數在外面，然後先 define max等於一個極小的數字，或是直接 define 0。然後呢去 for loop迴圈這個 array。然後 array跑完之後呢，它就是每個都會去遞迴嘛。如果今天你今天遞迴到這個值大於你 define的那個 max的話，它就會 swap掉。OK，所以我們就照這個思路來寫
    */
    let numbers = [-4, -7, -2, -9, -6];

    let mut max = i32::MIN;

    //直接拿array的第一個職當成max 
    //let mut max = numbers[0];

    //嘗試用rust中最小整數常數 i32::MIN

    for number in numbers {

        if number>max{
            
            max = number;
        }


    }


    print!("max is {}", max);



}


fn fas(){

    let numbers: [i32; 0] = [];


    if numbers.is_empty() == false { //邏輯上跟 !numbers.is_empty() 一樣
        print!("Array is  not empty");
    }else{

        print!("Array is  empty");
    }
}


fn finding_max(numbers: &[i32]/*Slice 可以讓函式讀取一段連續資料，而不必將陣列長度固定寫在函式參數中*/)->i32/*回傳值*/{
//f 尝试猜解一下这个function，我们给你把它理解成输入呢，我们要做的事情是它会给一个 array嘛，输入一个 array，然后最后 return是一个最大值

    let mut max = numbers[0];

    for &n in numbers{

        if n > max{

            max = n;
        }


    }

    return max;

    //另外一種寫法是直截 max 不加分號

}


fn main(){

    let numbers = [-4, -7, -2, -9, -6, 6];

    println!("max is {}",finding_max(&numbers)); //這邊記得


}