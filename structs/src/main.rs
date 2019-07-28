#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}

fn main() {
    // define the struct (essentially an object)
    struct User {
        username: String,
        email: String,
        sign_in_count: i32,
        active: bool
    }
    // create a new instance of the struct (in this case some user)
    let user1 = User {
        email: String::from("exampleEmail@email.com"),
        username: String::from("Json_Jon"),
        active: true,
        sign_in_count: 1,
    };

    // this struct can be mutated. The whole struct is mutable, you can't specify specific keys as mutable
    let mut user2 = User {
        email: String::from("anotherEmaail@email.com"),
        username: String::from("ChickenEllaBella"),
        active: false,
        sign_in_count: 0
    };

    // struct's allow for dot notion
    let email = user1.email;
    println!("user email is: {}", email);

    // using ..user1, the remaining keys will be filled with the values of those keys from user1
    let user3 = User {
        email: String::from("anotherEMailforAnotherUser.com"),
        username: String::from("anotherUserName!"),
        ..user1
    };

    // creating a tuple struct
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    // Ownership of Struct Data
    {
        struct NewUser {
            username: String,
            email: String,
            sign_in_count: u32,
            active: bool
        };

        let user1 = NewUser {
            email: String::from("SomeEmail@email.com"),
            username: String::from("someUserName"),
            active: true,
            sign_in_count: 1,
        };
    }
    // find the area of a rectangle , simple way
    {
        let width1 = 30;
        let height1 = 50;

        println!("The area of the rectangle is: {} square pixels", area(width1, height1))
    }
    // find area of rectangle with tuple
    {
        let rec1 = (30, 50); // create tuple with i32 dimensions
        println!("The Area of the rectangle is {} square pixels", area_struct_tuple(rec1));
    }
    // find area of rectangle with struct (object format)
    {

        let rectangle_1 = Rectangle { width: 30, height: 50 };
        println!("rectangle_1 is : {:?}", rectangle_1); // simple struct output
        println!("rectangle_1 is : {:#?}", rectangle_1); // pretty struct output
        println!("The area of the rectangle is {} square pixels (object style)", area_struct(&rectangle_1) )
    }
    // method syntax as opposed to functions
    {
        impl  Rectangle {
            fn area_method(&self) -> u32 {
                self.width * self.height
            }
        }
        let rec1 = Rectangle{width: 100, height: 50};
        println!("The Area of the rectangle is {} square pixels [method syntax]", rec1.area_method())
    }
    // methods with multiple params
    {
        impl Rectangle {
            fn can_hold(&self, other: &Rectangle) -> bool {
                self.width > other.width && self.height > other.height
            }
            fn square(size: u32) -> Rectangle { // associated function won't use &self as it isn't referencing an instance of itself, it's creating a new one
                Rectangle { width: size, height: size }
            }
        }

        let rec1 = Rectangle { width: 30, height: 50 };
        let rec2 = Rectangle { width: 10, height: 40 };
        let rec3 = Rectangle { width: 60, height: 45 };

        println!("Can rec1 hold rec2? {}", rec1.can_hold(&rec2));
        println!("Can rec1 hold rec3? {}", rec1.can_hold(&rec3));

        let square = Rectangle::square(5); // :: syntax to call associated function in a struct
    }
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}

fn area_struct_tuple(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

fn area_struct(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

//fn build_user(email: String, username: String)  -> User {
//    User {
//        email,
//        username,
//        active: true,
//        sign_in_count: 1,
//    }
//}
