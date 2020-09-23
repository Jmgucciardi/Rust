enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(Color), 
}

enum Color {
    Rgb(i32, i32, i32),
    Hsv(i32, i32, i32),
}

struct Point {
    x: i32,
    y: i32
}

fn main() {
    // let msg = Message::ChangeColor(0, 160, 255);

    // match msg {
    //     Message::Quit => {
    //         println!("The quit variant has no data to destructure");
    //     }
    //     Message::Move {x , y} => {
    //         println!("Move in the x direction {} and in the y direction {}", x ,y);
    //     }
    //     Message::Write(text) => println!("text message {}", text),
    //     Message::ChangeColor(r, g, b) => {
    //         println!("Change the color to red {}, green {}, and blue {}", r ,g ,b);
    //     }
    // }

    let msg = Message::ChangeColor(Color::Hsv(0, 160, 255));

    match msg {
        Message::ChangeColor(Color::Rgb(r,g,b)) => println!(
            "Change the color to red {}, green {}, blue {}",
            r, g, b
        ),
        Message::ChangeColor(Color::Hsv(h, s, v)) => println!(
            "Change the color to hue {}, saturation {}, and the value {}",
            h, s, v
        ),
        _ => (),
    }

    let ((feet, inches), Point {x, y}) = ((3, 10), Point {x: 3, y: -10});

    println!("feet {}, inches {}, point x {}, point y {}", feet, inches, x, y);
}
