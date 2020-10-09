use std::io::stdin;

pub fn fibonacci() {
    let mut i32_user_input: i32;
    println!("请输入想要生成的斐波那契数列的阶数:");
    // 1、 处理用户输入
    loop {
        let mut user_input = String::new();
        stdin().read_line(&mut user_input).expect("Failed");

        match user_input.trim().parse() {
            Ok(num) => {
                i32_user_input = num;
                println!("您输入的数字是:{}", user_input);
                break;
            }
            Err(_) => {
                println!("请输入一个整数:");
                continue;
            }
        }
    }
    let mut v: Vec<usize> = Vec::new();
    println!("Vector 能存的最大值为: {}", usize::pow(2, 31));
    for i in 0..(i32_user_input + 1) {
        println!("i is {}", i as usize);
        if i == 0 {
            v.push(i as usize);
        } else if i == 1 || i == 2 {
            v.push(1)
        } else {
            // let value_n_1 :i32 = i - 1 ;
            // let value_n_2 :i32 = i - 2 ;
            v.push(v[(i - 1) as usize] + v[(i - 2) as usize]);
            // println!("--------> {}", v[i as usize]);
        }
    }
    // 斐波那契数列为
    println!("斐波那契数列为:----> {:?}", v);
    // 第一种方法 调用斐波那契数列函数
    // println!("斐波那契数列第{}项的值为：{}", i32_user_input, get_fibonacci(i32_user_input));
    // 第二种方法 直接去上面的数组的最后一个元素
    println!(
        "斐波那契数列第{}项的值为：{}",
        i32_user_input, v[i32_user_input as usize]
    );
}

// 递归求斐波那契数列第n项的值
pub fn get_fibonacci(number: i32) -> i32 {
    return if number == 0 {
        0
    } else if number == 1 {
        1
    } else {
        get_fibonacci(number - 1) + get_fibonacci(number - 2)
    };
}
