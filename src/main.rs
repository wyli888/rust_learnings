mod change_temperature;
mod enum_learning;
mod fibonacci;
mod ownership;
mod slices;
mod string_learnings;
mod struct_learning;
mod print_learning;
mod types;
mod tuples;
//mod philosopher_meal;

use crate::enum_learning::enum_learning_test;
use crate::ownership::scope;
use crate::slices::slice_learning;
use crate::string_learnings::string_test;
use crate::struct_learning::print_user_info;
use rand::Rng;
use std::cmp::Ordering;
use std::io;
use crate::print_learning::print_test;
use crate::types::integer_learning_run;
use crate::tuples::tuples_learning;
use std::thread;
use std::sync::{Mutex,Arc};

fn main() {
    // 猜数字游戏
    // guessing_game();
    // 哲学家进餐问题
    philosopher_meal()
    // 摄氏度转换为华氏度
    // change_temperature::change_temperature();
    // 斐波那契数列
    // fibonacci::fibonacci();
    // ownership 的作用域问题
    // scope();
    // slices 学习
    // slice_learning();
    // 结构体学习
    // print_user_info();
    // 枚举类型学习
    // enum_learning_test();
    // 字符串学习
    //string_test();
    // 打印函数的学习
    // print_test();
    // 数据类型学习
    // integer_learning_run()
    // 元组学习
    // tuples_learning()
}

// 参数字游戏
fn guessing_game() {
    println!("Guessing the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);
    // println!("The secret number is {}", secret_number)

    loop {
        println!("Please guessing a number");

        let mut guessing_number = String::new();
        io::stdin()
            .read_line(&mut guessing_number)
            .expect("Failed to readline");

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

// 哲学家进餐问题
/// 首先来了解一下什么是哲学家进餐问题  一个典型的并发问题
/// 在古代，一个富有的慈善家捐赠了一所学院来安排五个著名的哲学家。
/// 每个哲学家都有一个房间，他可以在其中从事他自己专业的思考活动；
/// 学校里有一个公共的餐厅，这里配备有一个圆形的桌子，桌子周边有五个椅子，
/// 每个椅子用坐在上面的哲学家的名字来标记。他们以逆时针的方向来围着桌子坐。
/// 每个哲学家的左侧放了一个金叉，桌子中间有一大碗不断添满的意大利面。哲学家将花费大部分时间去思考；
/// 但是当他饿了，他就会去餐厅，坐在自己的椅子上，拿起在自己左边的叉子来吃意大利面。但是意大利面条比较难吃到，
/// 需要第二个叉子才能把面条送到嘴里。因此，哲学家也要拿起他右边的叉子。当哲学家吃完面条后，他会把两个叉子都放下，
/// 离开自己的椅子，并继续思考。当然了，一个叉子一次只能有一个哲学家来使用。如果其他哲学家想要使用你的叉子，那么他仅仅需要等到这个叉子没人用了即可。
///
///
/// 这个经典问题展示了一些不同的并发性的元素。原因在于，其实际实现起来比较复杂：一个简单的实现可能导致死锁。举一个例子，让我们先想一个简单的算法来解决这个问题：
//
// 1、一个哲学家拿起了自己左边的叉子。
// 2、然后他又拿起了他右边的叉子。
// 3、吃意大利面。
// 4、放下叉子。
// 现在，让我们来想象一下这一系列的事件：
//
// 哲学家 1 开始此算法，拿起他左边的叉子。
// 哲学家 2 开始此算法，拿起他左边的叉子。
// 哲学家 3 开始此算法，拿起他左边的叉子。
// 哲学家 4 开始此算法，拿起他左边的叉子。
// 哲学家 5 开始此算法，拿起他左边的叉子。
// …？所有的餐叉都被拿起来了，没人能吃面了!

fn philosopher_meal() {
    struct Philosopher {
        name: String,
        left: usize,
        right: usize
    }
    struct Table {
        forks: Vec<Mutex<()>>,
    }

    impl Philosopher {
        fn new(name: &str, left :usize, right :usize) -> Philosopher {
            Philosopher {
                name: name.to_string(),
                left: left,
                right: right
            }
        }
        fn eat(&self, table :&Table) {
            let _left = table.forks[self.left].lock().unwrap();
            let _right = table.forks[self.right].lock().unwrap();

            println!("{} 正在吃", self.name);
            std::thread::sleep(std::time::Duration::from_millis(5000));
            println!("{} 已经吃完了", self.name);
        }
    }

    let table = Arc::new(Table{
        forks: vec![
            Mutex::new(()),
            Mutex::new(()),
            Mutex::new(()),
            Mutex::new(()),
            Mutex::new(()),
        ]
    });

    let philosophers = vec![
        Philosopher::new("Liwenyang",0,1),
        Philosopher::new("Zhouyongjian", 1,2),
        Philosopher::new("Duxiao", 2,3),
        Philosopher::new("Lvxingjie" ,3,4),
        Philosopher::new("Chenlonghui", 0, 4),
    ];

    // for p in philosophers {
    //     p.eat();
    // }

    let handles :Vec<_> = philosophers.into_iter().map(|p|{
        let table = table.clone();
        thread::spawn(move ||{
           p.eat(&table);
        })
    }).collect();

    for h in handles{
        h.join().unwrap();
    }

}
