use std::cmp::PartialOrd;

// When you recognize situations in your code with multiple struct or enum definitions that differ
// only in the types of the values they hold, you can avoid duplication by using generic types instead.

fn largest(list: &[i32]) -> i32 {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn largest_i32(list: &[i32]) -> i32 {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn largest_char(list: &[char]) -> char {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}
// generic type being passed into a function.
//fn largest_generic<T>(list: &[T]) -> T {
//    let mut largest = list[0];
//
//    for &item in list.iter() {
//        if item > largest {
//            largest = item;
//        }
//    }
//
//    largest
//}

fn main() {
    // code works, finds largest value in array, but is tedious and error prone
    {
        let number_list = vec![34, 50, 25, 100, 65];

        let mut largest = number_list[0];

        for number in number_list {
            if number > largest {
                largest = number;
            }
        }

        println!("The largest number is {}", largest);
    }
    // using a function instead of some code block in main
    {
        let number_list = vec![34, 50, 25, 100, 65];
        let result = largest(&number_list);
        println!("The largest number is {}", result);

        let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];

        let result = largest(&number_list);
        println!("The largest number is {}", result);
    }
    // both of these functions do the same thing, find the largest of some type.
    // this could be broken down into one function that takes a generic type as param
    {
        let number_list = vec![34, 50, 25, 100, 65];

        let result = largest_i32(&number_list);
        println!("The largest number is {}", result);

        let char_list = vec!['y', 'm', 'a', 'q'];

        let result = largest_char(&char_list);
        println!("The largest char is {}", result);
    }
    // using the generic function to find the largest
//    {
//        let number_list = vec![34, 50, 25, 100, 65];
//
//        let result = largest(&number_list);
//        println!("The largest number is {}", result);
//
//        let char_list = vec!['y', 'm', 'a', 'q'];
//
//        let result = largest(&char_list);
//        println!("The largest char is {}", result);
//    }
    // struct definition with generics
    {
        // struct where are values will be of the same generic type
        struct Point<T> {
            x: T,
            y: T,
        }
        let integer = Point { x: 5, y: 10 };
        let float = Point { x: 1.0, y: 4.0 };

        // struct where you can define the generic types for each key value
        struct PointP<T, U>{
            x: T,
            y: U,
        }
        let both_same = PointP {x: 15, y: 10};
        let both_different = PointP {x:1.0, y: 5};
    }
    // method generics
    {
        struct Point<T> {
            x: T,
            y: T,
        }

        impl<T> Point<T> {
            fn x(&self) -> &T {
                &self.x
            }
        }
        let p = Point { x: 5, y: 10 };
        println!("p.x = {}", p.x());
    }
    // another struct generic example
    {
        struct Point<T, U> {
            x: T,
            y: U,
        }

        impl<T,U> Point<T,U> {
            fn mixup<V,W>(self, other: Point<V,W>) -> Point<T,W> {
                Point {
                    x: self.x,
                    y: other.y,
                }
            }
        }
        let p1 = Point {x: 5, y: 10.4};
        let p2 = Point {x: "hello", y:'c'};

        let p3 = p1.mixup(p2);
        println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
    }
}
