fn main() {
    // create new string
    let mut s = String::new();

    // also a String
    let data = "initial contents";

    let s = data.to_string();

    // the method also works on a literal directly:
    let s = "initial contents".to_string();

    // 50 shades of strings
    let hello = String::from("السلام عليكم");
    let hello = String::from("Dobrý den");
    let hello = String::from("Hello");
    let hello = String::from("שָׁלוֹם");
    let hello = String::from("नमस्ते");
    let hello = String::from("こんにちは");
    let hello = String::from("안녕하세요");
    let hello = String::from("你好");
    let hello = String::from("Olá");
    let hello = String::from("Здравствуйте");
    let hello = String::from("Hola");

    // manipulating strings
    let mut s1 = String::from("foo");
    let s2 = "bar";
    // replace
    s1.push_str(s2);
    println!("s2 is {s2}");

    // append / concat
    let mut s = String::from("lo");
    s.push('l');
    println!("s is {s}");

    // append / concat with existing
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used
    println!("s3 is {s3}");

    // format string
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = format!("{s1}-{s2}-{s3}");
    println!("{s}");

    // iterating of strings
    for c in s.chars() {
        println!("{c}");
    }

    // iterating of string bytes
    for b in s.bytes() {
        println!("{b}");
    }
}
