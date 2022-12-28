fn main() {
    // vec with type hint
    let v: Vec<i32> = Vec::new();

    // vec macro with default i32
    let v = vec![1, 2, 3];

    // update
    let mut v = Vec::new();
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);

    let v = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v[2];
    println!("The third element is {third}");

    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element."),
    }

    // doesn't compile
    let mut v = vec![1, 2, 3, 4, 5];
    let first = &v[0];
    v.push(6);
    // "first" is invalid since the vector is modified
    // println!("The first element is: {first}");

    // mutate in for loop
    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
        println!("The element is {i}");
    }

    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    // enum types into a vector
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
}
