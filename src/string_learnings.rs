// 字符串的学习


pub fn string_test() {
    //  &str 这种类型不可以改变 而且不具有这个字符串的拥有权
    let test_string = "你好 我是rust";
    println!("字节如下：{:?}", test_string.as_bytes());
    println!("test {}", test_string);

    // 如果 想要拥有这个字符串 可以进行以下代码
    let mut string1 = String::from("Hello ");
    println!("未进行改变之前:{}", string1);

    // String 的push方法只能在String的末尾添加一个字符
    string1.push('Y');
    println!("添加了Y之后 :{}",string1);
    string1.push('\u{1f4a9}');
    println!("添加了表情之后 :{}",string1);
    // 如果想要 添加字符串的话 可以使用 push_str()
    string1.push_str("World");
    println!("使用push_str()方法添加字符串了之后 :{}",string1);

    // Capacity in bytes
    println!("Capacity is {}",string1.capacity());
    // 是否为空
    println!("string1 为空吗{}",string1.is_empty());
    // 是否包含
    println!("string1 包含 Y 吗{}",string1.contains("Y"));
    // 替换
    println!("string1 中将Y 替换为X ：{}", string1.replace("Y","X"));

    // 通过空格进行分割 并循环打印
    for world in string1.split_whitespace(){
        println!("通过空格进行分割单词{}",world);
    }
}
