pub fn scope() {
    let s1 = gives_ownership(); // gives_ownership 将返回值
    println!("s1--->{}", s1);
    // 移给 s1
    let s5 = gives_ownership();
    println!("s5--->{}", s5);

    let s2 = String::from("hello"); // s2 进入作用域

    let s3 = takes_and_gives_back(s2); // s2 被移动到


    // 所有权
    let money :i32 = 33;
    fn borrow_it(qty: &i32) {
        println!("Your money total is ${}", qty)
    }
    borrow_it(&money); // borrowed ownership
    // the ownership of money has been returned to this higher scope
    fn consume_it(qty: i32) {
        println!("My money total is ${}", qty)
    }
    consume_it(money);
    // money memory is freed as the ownership has been
    // taken into the methods scope and that scope has ended.
    println!("No money: ${}", money);

}

fn gives_ownership() -> String {
    // gives_ownership 将返回值移动给
    // 调用它的函数

    let some_string = String::from("hello"); // some_string 进入作用域.

    some_string // 返回 some_string 并移出给调用的函数
}

// takes_and_gives_back 将传入字符串并返回该值
fn takes_and_gives_back(a_string: String) -> String {
    // a_string 进入作用域

    a_string // 返回 a_string 并移出给调用的函数
}
