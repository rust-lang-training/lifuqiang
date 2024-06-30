use std::rc::Rc;
use std::cell::RefCell;


fn main() {
    // 案例一
    // let s = String::from("hello");
    // takes_ownership(s);

    // let x = 5;
    // makes_copy(x);

    // 案例二
    // let names = [
    //     String::from("Bob"),
    //     String::from("Alice"),
    //     String::from("John")
    // ];

    // for i in 0..3 {
    //     let name = &names[i];
    //     println!("{}", name);
    // }

    // println!("names[0] = {}", names[0]);
    // println!("names[0] = {}", names[0]);


    // 案例三
    // let s  = String::from("Hello World");
    // let bytes = s.into_bytes();
    // println!("{:?}", bytes);
    // println!("{} {:?}", s, bytes);

    // 案例四
    // let x = 10;
    // let y = &x;
    // let z = &10;
    // assert!(*y == 10);
    // assert!(*z == 10);
    // println!(x);

    // 案例五
    // let mut x = 10;
    // let y = &mut x;
    // *y = 20;
    // println!(y);

    // 案例六
    // let s1 = String::from("hello");
    // let len = calculate_length(&&s1);
    // println!("The length of '{}' is {}.", s1, len);

    // 案例7
    // let mut s = String::from("hello");
    // let s1 = &s;
    // let s2 = &s;
    // let s3 = &mut s;

    // println!("{} {} {}", s1, s2, s3);

    // 案例8
    // let s1 = String::from("hello");
    // let s2 = String::from("world");
    // longest(&s1, &s2);

    // 案例9
    // let mut s = String::from("Hello World");
    // let rs = &s;

    // s.push_str(" I'm rust");
    // println!("The string is: {}", rs);

    // 案例10
    // let mut s = String::from("Hello World");
    // let rs = &s;

    // // s.push_str(" I'm rust");
    // s = String::from("Hello World 2");
    // println!(">>>> {}", s);
    // println!("The string is: {}", rs);

    // 案例11
    // let s1 = String::from("Hello worid");
    // let rs1 = &s1;
    // let s2 = s1;
    // println!("String is {}", rs1);

    // 案例12
    // let bytes = gen_string();
    // let bytes2 = bytes.as_bytes();
    // println!(">>>> {:?} ", bytes2);

    // 案例 13 RC
    // let s: Rc<String> = Rc::new(String::from("sssss"));
    // println!("ref count: {}", Rc::strong_count(&s));

    // {
    //     let t: Rc<String> = s.clone();
    //     println!("ref count: {}", Rc::strong_count(&t));
    // }
    
    // println!("ref count: {}", Rc::strong_count(&s));
    // let u: Rc<String> = s.clone();
    // println!("ref count: {}", Rc::strong_count(&s));
    // println!("ref count: {}", Rc::strong_count(&u));

    // 案例 14 Box<T>
    // let b = Box::new(5);
    // println!("b = {}", b); 

    // 案例 15 Cell
    // let s = RefCell::new(String::from("woooww"));
    // append_string(&s);
    // println!(">>>>s is {}", s.borrow());

    // 案例 16 struct 
    // let user1 = User {
    //     name: String::from("name1"),
    //     age: 28,
    //     phone: String::from("18519945555"),
    //     nick_name: "nick name".to_string(),
    // };

    // let User {
    //     name,
    //     age,
    //     phone,
    //     nick_name
    // } = user1;
    
    // println!(">>> name: {}, age: {}, phone: {}, nick_name: {}", name, age, phone, nick_name);

    // let mut user1 =  User  {
    //     name: String::from("name1"),
    //     age: 28,
    //     phone: String::from("18519945555"),
    //     nick_name: "nick name".to_string(),
    // };

    // let User {
    //     name,
    //     age,
    //     phone,
    //     nick_name
    // } = user1;

    // user1.age = 31;
    // println!(">>> name: {}, age: {}, phone: {}, nick_name: {}", name, age, phone, nick_name);

    // 案例 17 元组式结构体
    // let p = Point(1.0, 2.0);
    // println!("point: {}, {}", p.0, p.1);

    // let Point(x, y) = p;
    // println!("point :{} {}", x, y);

    // 案例18
    // let mut rectRange1 = Rectrange {
    //     width: 15.0,
    //     height: 20.0,
    // };

    // let a: Rectrange = Rectrange::square(2.0);
    // rectRange1.scale(1.0, 3.0);
    // let res = rectRange1.perimeter();
    // println!("{} ", res);

    // 案例19
    // enum Message {
    //     Quit,
    //     Move { x: i32, y: i32 },
    //     Write(String),
    //     ChangeColor(i32, i32, i32)
    // }

    // let q = Message::Quit;
    // let m: Message = Message::Move { x: 100, y: 100 };
    // let w = Message::Write(String::from("hello world"));
    // let c = Message::ChangeColor(123, 456, 789);

    // println!("{:?} {:?} {:?} {:?}", q, m, w, c);


    // 案例20 模式匹配
    println!("结果: {}", descibe_point(1, 2));
    println!("结果: {}", descibe_point(0, 0));
    println!("结果: {}", descibe_point(1, 0));
    println!("结果: {}", descibe_point(0, 1));
}

fn descibe_point(x: i32, y: i32) -> &'static str {
    use std::cmp::Ordering::*;
    match (x.cmp(&0), y.cmp(&0)) {
        (Equal, Equal) => "at the x axias",
        (_, Equal) => "on the x axis",
        (Equal, _) => "on the y axis",
        (Greater, Greater) => "在第一象限",
        (Less, Greater) => "在第二象限",
        _ => "somewhere else"
    }
}

struct Rectrange {
    width: f32,
    height: f32,
}

impl Rectrange {
    fn square(size: f32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }

    fn new(width: f32, height: f32) -> Self {
        Self {
            width,
            height
        }
    }

    fn perimeter(&self) -> f32 {
        (self.width + self.height) * 2.0f32
    }

    fn area(&self) -> f32 {
        self.width * self.height
    }

    fn scale(&mut self, width_scale: f32, height_scale: f32) {
        self.width = width_scale * self.width;
        self.height = height_scale * self.height;
    }
}

#[derive(Debug)]
struct Point(f32, f32);

struct User {
    name: String,
    phone: String,
    age: u32,
    nick_name: String,
}

fn append_string(s: &RefCell<String>) {
    let rs = s.borrow();
    let mut s1 = s.borrow_mut();
    (*s1).push_str(" Rust");

    println!(">>>{}", rs);
}

fn gen_string() -> String {
    String::from("hello world")
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// fn longest2(x: &str, y: &str) -> &str {
//     if x.len() > y.len() {
//         x
//     } else {
//         y
//     }
// }

fn calculate_length(s: &String) -> usize {
    let s2 = &s;
    s2.len()
}

fn takes_ownership(s: String) {
    println!("{}", s);
}

fn makes_copy(someInteger: i32) {
    println!("{}", someInteger);
}
