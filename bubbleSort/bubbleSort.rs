// create prototype for function bubble_sort
// pass it an array which will be mutatable, size: int 32
fn bubble_sort(data: &mut [i32]) {
    let mut temp;
    let length = data.len();

    for _ in 0..length {
        for j in 0..length -1 {
            if data[j] > data[j + 1] {
                temp = data[j + 1];
                data[j + 1] = data[j];
                data[j] = temp;
            }
        }
    }
    println!("Post Sort: {:?}", data);
}

fn main() {
    // create a mutatable array, size int 32, length 11
    let mut data: [i32; 11] = [35,235,325,352,535,35,523,2,4,24,215];
    // call bubble_sort here and pass it the mutatable array
    println!("Pre Sort: {:?}", data);
    bubble_sort(&mut data);
}