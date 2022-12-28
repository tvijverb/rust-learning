fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let largest = largest(&number_list);

    println!("The largest number is {}", largest);


    let char_list = vec!['y', 'm', 'a', 'q'];

    // wont work, cant compare char
    // let result = largest(&char_list);
    // println!("The largest char is {}", result);

    let both_integer = Point { x: 5, y: 10 };
    let both_float = Point { x: 1.0, y: 4.0 };
    let integer_and_float = Point { x: 5, y: 4.0 };
    println!("p.x = {}", integer_and_float.x());
    println!("p.y = {}", integer_and_float.y());
}

// generic
fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

struct Point<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point<T, U> {
    fn x(&self) -> &T {
        &self.x
    }
    fn y(&self) -> &U {
        &self.y
    }
}