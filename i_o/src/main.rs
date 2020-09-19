mod lifestyle;

fn main(){
    println!("hello world");
    lifestyle::my_name();
    lifestyle::clayton_granola();

    let name = String::from("Clayton");

    let my_new_person = lifestyle::new_person(name, lifestyle::Diet::Vegan);

    println!("{:?}", my_new_person);
}