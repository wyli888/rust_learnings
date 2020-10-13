/**
  基本数据类型
  Integers: u8 i8, u16 i16, u32 i32, u64 i64, u128 i128
  Float: f32 f64
  Boolean: bool
  Characters: char
  Tuple
  Array
**/

pub fn integer_learning_run() {
    // 整型默认是 i32
    let x = 5;
    // 浮点型默认是 f64
    let y = 3.3;

    let z :i64 = 34343434;

    // 查看整型i32的最大值
    println!("最大值是:{}", std::i32::MAX);
    // 查看整型i64的最大值
    println!("最大值是:{}", std::i64::MAX);
    // 定义布尔值
    let is_active :bool = true;
    // 从表达式中获取 布尔值
    let is_greater:bool = 10 > 5;
    // char 字符 只能放入一个字符 也可以放入emoji表情  unicode值
    // let a1 = 'ab';  报错
    let a1 = 'a';
    let b1 = '\u{1F929}';
    println!("{:?}",(x,y,z,is_active,is_greater,a1,b1));

}