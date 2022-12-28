use std::collections::HashMap;


fn main() {
    // hashmap is equivalent to python dictionary
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    println!("{:#?}", scores);


    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    // .get() returns Option<&i32>
    // with .copied it creates a new variable with Option<i32>
    // .unwrap_or(0) parses Option and replaces None result with 0
    // now score variable will always hold an integer
    let score = scores.get(&team_name).copied().unwrap_or(0);

    // loop over keys and values in hashmap
    // no order guarantees
    for (key, value) in &scores {
        println!("{key}: {value}");
    }

    // ownership transfer
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");
    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // field_name and field_value are invalid at this point, try using them and
    // see what compiler error you get!

    // overwrite key in hashmap
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);
    println!("{:?}", scores);

    // keep existing key using or_insert()
    scores.insert(String::from("Blue"), 10);
    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);
    println!("{:?}", scores);

    // hashmap get item or create if it doesn't exist
    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        // retrieve &i32 or create index and return 0 (as &i32)
        let count = map.entry(word).or_insert(0);
        // dereference and increment by 1
        *count += 1;
    }

    println!("{:?}", map);
}
