use std::{ io };

fn main() {
    // F= C*1.8 +32
    // C= （F - 32）/ 1.8
    println!("华氏度和摄氏度转换");

    loop {
        println!(">>>>>华氏度转摄氏度 1<<<<");
        println!(">>>>>摄氏度转华氏度 2<<<<");
        println!(">>>>>退出: 0<<<<");

        let mut choice = String::new();
        io::stdin()
            .read_line(&mut choice)
            .expect("please input!!!");

        let mut temperature_number = String::new();

        match choice.trim().parse() {
            Ok(1) => {
                println!("华氏度转摄氏度");
                io::stdin()
                    .read_line(&mut temperature_number)
                    .expect("please input!!!");

                c_to_f(&temperature_number);
            },
            Ok(2) => {
                println!("摄氏度转华氏度");
                io::stdin()
                    .read_line(&mut temperature_number)
                    .expect("please input!!!");

                f_to_c(&temperature_number);
            },
            Ok(0) => {
                println!("退出");
                break;
            },
            _ => {
                println!("退出");
                break;
            }
        }
    }

}

fn c_to_f(c: &str) {
    let temperature = c.trim().parse::<f64>().unwrap();
    let temperature_c = (temperature * 1.8) + 32.0;

    println!(">>>>输入温度 {}, 转换后摄氏度 {:.2}", temperature, temperature_c);
}

fn f_to_c(f: &str) {
    let temperature = f.trim().parse::<f64>().unwrap();
    let temperature_c = (temperature - 32.0) / 1.8;

    println!(">>>>输入温度 {}, 转换后摄氏度 {:.2}", temperature, temperature_c);
}
