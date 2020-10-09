// 定义一个用户结构体
struct User {
    email: String,
    phone: String,
    is_active: bool,
    user_name: String,
    sign_in_account: u64,
}

// 定义一个颜色元组结构体
struct Color(i32, i32, i32);

// 定义一个点元组结构体
struct Point(i32, i32, i32);

// 定义一个长方形或者正方形的结构体
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
// 我们还可以定义一个在结构式内部的函数
// 这样当我们想要计算面积的时候 我们实例出来的对象就可以调用这个方法了 非常的方便
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    // 一个长方形能够包含住另一个长方形
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.height > other.height && self.width > other.width
    }
}

pub fn print_user_info() {
    // 实例一个 User 结构体对象
    let mut user1 = User {
        email: String::from("liwenyang@qq.com"),
        phone: String::from("14553344321"),
        is_active: false,
        user_name: "li_wen_yang".to_string(),
        sign_in_account: 21245,
    };
    // 实例第二个 User 结构体对象
    let user2 = User {
        // 这里的..user1 就表示我们使用的值和user1的一模一样
        ..user1
    };
    user1.phone = "11111111".to_string();
    println!("user1 information is {:?}", user1.phone);
    println!("user2 information is {:?}", user2.phone);

    // 下面计算长方形的面积
    let rectangle1 = Rectangle {
        width: 30,
        height: 15,
    };
    let rectangle2 = Rectangle {
        width: 10,
        height: 14,
    };
    println!("the area of rectangle1 is {}", area(&rectangle1));
    // 通过使用注解 #[derive(Debug)] 来打印出结构体
    println!("the Rectangle is {:#?}", rectangle1);
    // 我们使用自定义结构体函数来计算出长方形的面积并打印他
    println!(
        "我们自定义的结构体函数计算出来的面积是: {}",
        rectangle1.area()
    );
    // 一个长方形能否包含另一个长方形
    println!(
        "rectangle1 能包含 rectangle2 吗？: {}",
        rectangle1.can_hold(&rectangle2)
    );
}
// 计算面积
fn area(rectangle: &Rectangle) -> u32 {
    rectangle.height * rectangle.width
}
