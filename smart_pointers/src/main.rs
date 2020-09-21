mod box_type;

fn main() {
    let b = Box::new(5);
    println!("B = {}", b);
    let new_box: box_type::BoxDox = box_type::BoxDox::get_boxdox();
    println!("Box: {:?}", new_box);

}