use std::{io};

fn main() {
    println!("输入斐波数列第 n 项");

    let mut input_str = String::new();
        
    io::stdin()
        .read_line(&mut input_str)
        .expect("please input!!!");

    let input_str: i32 = input_str.trim().parse().expect("请输入正确的整数");

    // println!("递归：斐波数列第 {} 项是 {}", input_str, fibonacci(input_str));
    println!("循环：斐波数列第 {} 项是 {}", input_str, fibonacci_2(input_str));
}

fn fibonacci(n: i32) -> i32 {
    if n == 1 || n == 2 {
        1
    } else {
        fibonacci(n - 1) + fibonacci(n - 2)
    }
}

fn fibonacci_2(n: i32) -> i32 {
    let mut index = n;
    if n == 1 || n == 2 {
        1
    } else {
        let mut l: i32 = 1;
        let mut r: i32 = 1;

        while index > 2 {
            let tmp = l.overflowing_add(r);
            if tmp.1 == false {
                println!(">>>>>数字溢出了");
                break;
            }
            l = r;
            r = tmp.0;
            index -= 1;
        }

        r
    }
}


