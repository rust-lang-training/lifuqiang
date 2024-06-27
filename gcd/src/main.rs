fn gcd(a: u32, b: u32) -> u32 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn main() {
    let a: u32 = 42;
    let b: u32 = 28;
    let result = gcd(a, b);
    println!("The greatest common divisor of {} and {} is {}", a, b, result);
}