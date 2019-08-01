use std::collections::HashMap;

fn main() {
    // given a list of integers, use a vector and return the mean, median (when sorted, the value in the middle position),
    // and mode
    {
        let v = vec![1,1,1,2,1,3,4,4,5,5,5];
        let length = v.len();
        let mut median = 0;
        let mut data = HashMap::new();
        for score in &v {
            let count = data.entry(score).or_insert(0);
            *count += 1;
        }
        for ( key, value ) in &data {
            println!("{}: {}", key, value);
            median += *key * value
        }


        println!("length: {}", &length);
        println!("median: {}", median)
    }
}
