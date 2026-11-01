fn main() {
    println!("Hello, world!");

    //step 1 : 建立 a
    let a = String::from("Hello");
    //STACK                     HEAP(動態配置)
    //┌─────────────┐
    //│ a           │
    //│ ptr ────────┼──────────> "Hello"
    //│ len = 5     │
    //│ capacity=5  │
    //└─────────────┘
    {
        //step 2 :
        let b = a;
        //STACK                     HEAP
        //┌─────────────┐
        //│ a           │
        //│ 已失效       │
        //├─────────────┤
        //│ b           │
        //│ ptr ────────┼──────────> "Hello"
        //│ len = 5     │
        //│ capacity=5  │
        //└─────────────┘
        // 這邊有點像是把 heap 上的所有權轉移到 b 也就是說 b 其實沒有被釋放
        //
        //
    }
    //當我離開這個scope時 會自動執行 drop() [釋放 Heap 的 "Hello" 配置]
    //
    let s1 = String::from("hello");

    let len = calculate_len(&s1);
}

fn calculate_len(s: &String) {
    s.len();
}

// so in rust world, the author recommend me to use "snake case to name  a function"
// for example fn fuck_you(){}
//
//
//
//
// reference and borrow
