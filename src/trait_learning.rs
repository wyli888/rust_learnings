
// 定义一个trait
// 类似接口
pub trait GetInformation{
    fn get_name(&self) -> &String;
    fn get_age(&self) -> u32;
}

pub struct Student {
    pub name: String,
    pub age: u32,
}
// 实现一个trait
impl GetInformation for Student{
    fn get_name(&self) -> &String {
        &self.name
    }

    fn get_age(&self) -> u32 {
        self.age
    }
}

pub fn trait_learning(){
    let student = Student{ name: "xiaoming".to_string(), age: 30 };
    println!("{} is {}", student.get_name(), student.get_age())
}