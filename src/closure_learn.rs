pub fn closure_learning() {
    let a = || 43;
    let b = |x| x + 1;
    println!("a is {}", a());
    println!("b is {}", b(2));

}