pub fn print_test() {
    // :b 将2以二进制形式打印出来 10
    println!("{} of {:b} people know binary , the other half don't ", 1, 2);
    // format 函数在进行字符串格式化的同时会返回这个字符串
    let a = format!("hello world");
    println!("a is {}", a);

    // 获取v的第2个元素的可变引用，并通过解引用修改该元素的值。
    let v = &mut [1, 2, 3, 4, 5];
    {
        let third = v.get_mut(2).unwrap();
        *third += 50;
        println!("v={:?}", v); // v=[1, 2, 53, 4, 5]
    }
    let number = 22;
    match number {
        0 => println!("Origin"),
        1...4 => println!("All"),
        n @ 22 => println!("the number is {}", n),
        _ => println!("Common"),

    }
}