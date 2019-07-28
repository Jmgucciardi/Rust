enum IpAddrKind {
    V4,
    V6
}

struct IpAddr {
    kind: IpAddrKind,
    address: String
}
// eum allows us to define types like this. v4 will look for four values , all of int of size u8, v6 just a string value
enum IpAddrEnum {
    V4(u8, u8, u8, u8),
    V6(String),
}
// this Option enum comes standard in rust and stands in place of null. Rust has no null value as it causes more trouble than its worth
// your code will have access to Some and None because it's already in scope in a rust project
//enum Option<T> {
//    Some(T),
//    None,
//}

fn main() {
    // create an instance of each of the enum values
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    {
        let home = IpAddr {
            kind: IpAddrKind::V4,
            address: String::from("127.0.0.1")
        };

        route(four);

        let loopback = IpAddr {
            kind: IpAddrKind::V6,
            address: String::from("::1")
        };
    }
    // concise way to evaluate enumerated values
    {
        let home = IpAddrEnum::V4(127, 0, 0, 1);
        let loopback = IpAddrEnum::V6(String::from("::1"));
    }

    {
        // create message enum
        enum Message {
            Quit,
            Move { x: i32, y: i32 },
            Write(String),
            ChangeColor(i32, i32, i32),
        }
        // we can define methods on enums just as we can with a struct
        impl Message {
            fn call(&self) {
                // define the body of the method here

            }
        }
        let m = Message::Write(String::from("Hello there"));
        m.call();
    }

    {
        let some_number = Some(5);
        let some_string = Some("A String");


    }

    // using the match control flow operator (important to know!)
    {
        #[derive(Debug)]
        enum UsState {
            Alabama,
            Alaska
        }

        enum Coin {
            Penny,
            Nickel,
            Dime,
            Quarter(UsState) // bind the enum type UsState to the quarter value of enum type Coin
        }
        // behaves similar to a switch statement. but with match, you aren't just looking for a bool, you can look for a data type,
        // like here ,we look for type Coin and a variant of that type
        // also counts the number of coins that isn't a quarter
        fn value_in_cents(coin: Coin) -> u8 {
            let mut count = 0;
            match coin {
                Coin::Penny => {
                    println!("Lucky Penny");
                    count += 1;
                    1 // this is the return value
                },
                Coin::Nickel => {
                    count += 1;
                    5
                },
                Coin::Dime => {
                    count += 1;
                    10
                },
                Coin::Quarter(state) => {
                    println!("State quarter from {:?}!", state); // we can access the value of the quarters state, easily.
                    25
                },
            }
        }
        // will do the same thing as the match above, but with an if let statement
        fn value_in_cents_if_let(coin_state: Coin) -> u8 {
            let mut count = 0;
            if let Coin::Quarter(state) = coin_state { // if (Coin.Quarter.State === coin_state) print the state, else, increase counter
                println!("State quarter is from {:?}!", state);
                25
            } else {
                count += 1;
                count
            }
        }

        value_in_cents(Coin::Quarter(UsState::Alaska)); // send in the quarter with Alaska as the state
        value_in_cents_if_let(Coin::Quarter(UsState::Alabama)); // send Alabama to the if let function
    }
    // matching with Option<T>
    {
        fn plus_one(x: Option<i32>) -> Option<i32> {
            match x {
                None=> None,
                Some(i) => Some(i + 1),
            }
        }

        let five = Some(5);
        let six = plus_one(five);
        let none = plus_one(None);
    }
    //concise Control flow with if let
    {
        let some_u8_value = Some(0u8);
        match some_u8_value {
            Some(3) => println!("there"),
            _ => (),
        }
        // the following code behaves the same as above
        if let Some(3) = some_u8_value {
            println!("Three!");
        }
    }
}

fn route(ip_kind: IpAddrKind) {

}
