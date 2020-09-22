pub struct MyBox {
    pub height: i32,
    pub width: i32,
}

impl MyBox {
    pub fn get_box() -> MyBox {
        let y: i32 = 5;
        let x: i32 = 4;
        MyBox {height: y, width: x }
    }
}