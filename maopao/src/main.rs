use rand::Rng;

fn main() {
    println!("Hello, world!");
    let mut number = [0u32; 10];

    for i in 0..10 {
        number[i] = rand::thread_rng().gen_range(0..100);
    }

    println!("排序数据：{:?}", number);
    
    let len = number.len();
    for i in 0..len {
        for j in 0..len - i - 1 {
            if number[j] > number[j + 1] {
                number.swap(j, j+1);
            }
        }
    }

    println!("排序数据结果：{:?}", number);
}
