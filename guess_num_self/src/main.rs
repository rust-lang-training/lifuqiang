use std::{cmp::Ordering, io};
use rand::Rng;

fn main() {
    println!("Guess Number!!!");
    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("猜数字 {secret_number}");

    loop {
        let mut guess_number = String::new();
        
        io::stdin()
            .read_line(&mut guess_number)
            .expect("please input!!!");

        let guess_number: u32 = guess_number.trim().parse().expect("please type a number!!!");
        match guess_number.cmp(&secret_number) {
            Ordering::Less => println!("小于"),
            Ordering::Greater => println!("大于"),
            Ordering::Equal => {
                println!("猜对了 {guess_number}");
                // continue;
                break;
            },
        }
    }
}

#[test]
fn test() {
    assert!(12_i32 as u32 == 12u32, "测试通过");
}
