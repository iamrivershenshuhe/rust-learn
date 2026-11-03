fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);
    //執行時會將s1所有權轉到韓世忠

    println!("'{}' 的長度為 {}。", s1, len);
}

fn calculate_length(s: &String) -> usize { //define functuion 這邊工作室 我指定回傳類型

    // 先說一下這邊為啥用 &String 回到上一張說到的所有權 因為這String 擁有這heap 的所有權
    s.len()

    //main()
    //  │
    //  └── s1 ─────────► String ─────► Heap: "hello"
    //      Owner              ▲
    //                         │
    //calculate_length()       │
    //  │                      │
    //  └── s ─────────────────┘
    //      Borrower
    // 這function 接受一個指向 String 的 reference 而不是搶走所有權 
 }
