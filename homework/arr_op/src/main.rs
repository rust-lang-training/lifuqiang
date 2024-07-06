fn main() {
    let numbers = [1, 5, 3, 9, 2];

    println!("数组最大值: {}", max_value(&numbers));
    println!("数组最小值: {}", min_value(&numbers));

    let max_ref = max_value_ref(&numbers);
    let min_ref = min_value_ref(&numbers);
    println!("数组引用最大值: {}", max_ref);
    println!("数组引用最小值: {}", min_ref);

    println!("熟组引用平均值: {}", average_value(&numbers));

    let mut double_numbers = numbers.to_vec();
    double_elements(&mut double_numbers);
    println!("数组二倍: {:?}", double_numbers);
}

fn max_value(arr: &[i32]) -> i32 {
    *arr.iter().max().unwrap()
}

fn min_value(arr: &[i32]) -> i32 {
    *arr.iter().min().unwrap()
}

fn max_value_ref(arr: &[i32]) -> &i32 {
    arr.iter().max().unwrap()
}

fn min_value_ref(arr: &[i32]) -> &i32 {
    arr.iter().min().unwrap()
}

fn average_value(arr: &[i32]) -> f64 {
    arr.iter().sum::<i32>() as f64 / arr.len() as f64
}

fn double_elements(arr: &mut [i32]) {
    for elem in arr.iter_mut() {
        *elem *= 2;
    }
}

