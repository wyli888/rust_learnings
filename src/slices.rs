pub fn slice_learning() {
    let words = String::from("hello my name is LiWenYang");
    let index = first_word(&words);
    println!("第一个单词的索引是----> {}", index);
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();
    println!("bytes is {:?}", bytes);
    for (i, &item) in bytes.iter().enumerate() {
        // println!("index is {}", i);
        // println!("item is {}", &item);
        if item == b' ' {
            println!("遇到空格了--->{}", item);
            return i;
        }
    }
    bytes.len()
}
