use std::io::stdin;

pub fn change_temperature() {
    let mut user_input: i32 = 22;
    // let mut user_input_temperature = String::new();

    loop {
        println!("请输入想要转换的摄氏度 xxx°C:");
        let mut user_input_temperature = String::new();

        stdin().read_line(&mut user_input_temperature).expect("readline unexpect quit!");
        // println!("你输入的值为{} °C:",user_input_temperature.trim());
        match user_input_temperature.trim().parse() {
            Ok(input) => {
                println!("你输入的是 {} °C", input);
                user_input = input;
                break;
            }
            Err(_) => continue,
        };

    }

    // 摄氏度转华氏度公式°F = (°C * 1.8) + 32
    let hua_shi_du :f64 = user_input as f64 * 1.8 + 32 as f64;
    println!("转换结果是:{}°F",hua_shi_du);
}