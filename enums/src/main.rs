// // basic enum
// #[derive(Debug)]
// enum IpAddrKind {
//     V4,
//     V6,
// }

// // use enum in struct
// #[derive(Debug)]
// struct IpAddr {
//     kind: IpAddrKind,
//     address: String,
// }

// fn main() {
//     // enum in
//     let four = IpAddrKind::V4;
//     let six = IpAddrKind::V6;
//     println!("{:?}", six);

//     // create struct
//     let new_ip = IpAddr {
//         kind: IpAddrKind::V4,
//         address: String::from("127.0.0.1"),
//     };
//     println!("{:#?}", new_ip);
// }

// better enum type, define type within enum directly
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

// more complex enum type
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

// standard library Option and Some
// enum Option<T> {
//     None,
//     Some(T),
// }




fn main() {
    let home = IpAddr::V4(127, 0, 0, 1);

    let loopback = IpAddr::V6(String::from("::1"));

    let some_number = Some(5);
    let some_char = Some('e');

    let absent_number: Option<i32> = None;
}