

// match 匹配的学习
pub fn match_learning(){
    let some_value = Some(0u8);
    match some_value {
        Some(3) => println!("three"),
        _ => println!("Nothing")
    }
    if let Some(0) = some_value { println!("three!!!!!");}
}