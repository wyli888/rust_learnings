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


pub fn print_user_info() {
    let mut user1 = User {
        email: String::from("liwenyang@qq.com"),
        phone: String::from("14553344321"),
        is_active: false,
        user_name: "li_wen_yang".to_string(),
        sign_in_account: 21245,
    };

    let user2 = User {
        ..user1
    };
    user1.phone = "11111111".to_string();

    println!("user1 information is {:?}", user1.phone);
    println!("user2 information is {:?}", user2.phone);
    // 下面计算长方形的面积

    let rectangle = Rectangle{
        width: 30,
        height: 15
    };
    println!("the area of rectangle is {}", area(&rectangle));
    // 通过使用注解 #[derive(Debug)] 来打印出结构体
    println!("the Rectangle is {:#?}", rectangle);
}
// 计算面积
fn area(rectangle :&Rectangle) -> u32{
   rectangle.height * rectangle.width
}
