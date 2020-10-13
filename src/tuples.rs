// 元组的学习


pub fn tuples_learning(){
    let person:(&str,&str,i8) = ("Li","Man",33);
    println!("{} is {} and is {}",person.0,person.1,person.2);
}