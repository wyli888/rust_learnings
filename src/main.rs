mod change_temperature;
mod fibonacci;

use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    // 猜数字游戏
    // guessing_game();
    // 摄氏度转换为华氏度
    // change_temperature::change_temperature();
    // 斐波那契数列
    fibonacci::fibonacci();
}


// 参数字游戏
fn guessing_game() {
    println!("Guessing the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);
    // println!("The secret number is {}", secret_number)

    loop {
        println!("Please guessing a number");

        let mut guessing_number = String::new();
        io::stdin().read_line(&mut guessing_number).expect("Failed to readline");

        let guessing_number: i32 = match guessing_number.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("Your guessing is {}", guessing_number);

        match guessing_number.cmp(&secret_number) {
            Ordering::Less => println!("IS TO SMALL"),
            Ordering::Equal => {
                println!("WELL DONE");
                break;
            }
            Ordering::Greater => println!("TOO LARGER"),
        }
    }
}
