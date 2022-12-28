fn main() {
    let mut string1 = String::from("abcd");
    let mut string2 = "xyzxyz";

    // call longest with str (slices)
    // result will contain reference to either string1 or string2
    // as long as both are still valid
    let result = longest(string1.as_str(), string2);

    // x and y from longest function are still valid, so result is valid
    println!("The longest string is {}", result);
    
    // result still points to string1, so this won't compile
    // string1 = String::from("NO WAY");
    string2 = "nom?";
    // result reference still valid? why?
    println!("The longest string is {}", result);
}

// this tells the compiler that the returned reference
// will live as long as x and y are still valid
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}