use std::{io};

fn main() {
    println!("一个 [u32; 5] 的数组，计算其中元素的和");
    let arr = [0u32; 5];
    let input_str = input_str();

    let mut index = 0;
    
    for n  in input_str {
        let n_number: u32 = n.parse().except("请输入正确的整数");
        arr[index] = n_number;
        index += 1;

        if(index == 5) {
            break;
        }
    }

    add_five_number(&arr[..]);
}

fn add_five_number(arr: &[u32]) {
    let mut sum = 0;
    for i in arr {
        sum += i;
    }

    println!("5个数之和为 {}", sum);
}

fn input_str() -> () {
    let mut input_str = String::new();
        
    io::stdin()
        .read_line(&mut input_str)
        .expect("please input!!!");

    input_str.trim().split(" ");
}
