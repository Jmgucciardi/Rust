#[derive(Debug)]
pub struct BoxDox {
    pub height: i32,
    pub width: i32
}

impl BoxDox {
    pub fn get_boxdox() -> BoxDox {
        let x: i32 = 5;
        let y: i32 = 10;

        BoxDox { height: y, width: x}
    }
}