fn main() {
    println!("Hello, world!");
    let res1 = feibo_match(4);
    println!("n: {}, res: {}", 4, res1);

    let res1 = feibo_match(0);
    println!("n: {}, res: {}", 0, res1);

    let res1 = feibo_match(1);
    println!("n: {}, res: {}", 1, res1);

    let res1 = feibo_match(17);
    println!("n: {}, res: {}", 17, res1);
}

fn feibo_match(n: i32) -> i32 {
    let res = match n {
        0 => 0,
        1 | 2 => 1,
        _ => feibo_match(n - 1) + feibo_match(n - 2),
    };

    res
}
