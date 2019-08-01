use std::collections::HashMap;

fn average(numbers: &[i32]) -> f32 {
    numbers.iter().sum::<i32>() as f32 / numbers.len() as f32
}

fn median(numbers: &mut[i32]) -> i32 {
    numbers.sort();
    let mid = numbers.len() / 2;
    numbers[mid]
}

fn mode(numbers: &[i32]) -> i32 {
    let mut number_map = HashMap::new();

    for &value in numbers {
        *number_map.entry(value).or_insert(0) += 1;
    }

    number_map.into_iter()
        .max_by_key(|&(_, count)| count)
        .map(|(val, _)| val)
        .expect("Can not compute the mode of zero numbers!")
}

fn main() {
    let mut numbers = vec![42, 1, 36, 76, 378, 43, 1, 43, 54, 2, 3, 43];

    println!("Average: {}", average(&numbers));
    println!("Median: {}", median(&mut numbers));
    println!("Median: {}", mode(&numbers));
}
