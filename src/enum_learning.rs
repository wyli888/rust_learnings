enum IpAddrKind {
    V4,
    V6,
}

enum Coin {
    Dollar,
    Penny,
    Nickel,
    Dime,
    Quarter,
}
pub fn enum_learning_test() {
    let dollar = Coin::Dollar;
    let coin_value = value_in_cents(dollar);
    println!("dollar is value {}", coin_value);
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Dollar => 6,
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}
