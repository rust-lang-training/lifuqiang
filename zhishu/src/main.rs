fn main() {
    println!("判断是不是质数");

    let checkNumber1 = 100;
    let checkNumber2 = 3;
    let checkNumber3 = 33;
    let checkNumber4 = 37;


    checkNumber(checkNumber1);
    checkNumber(checkNumber2);
    checkNumber(checkNumber3);
    checkNumber(checkNumber4);
}

fn checkNumber(number: i32) -> bool {
    let mut i = 2;
    while i * i < number {
        if number % i == 0 {
            println!(">>>{} 不是质数", number);
            return false;
        }

        i += 1;
    }

    println!(">>>{} 是质数", number);
    return true;
}