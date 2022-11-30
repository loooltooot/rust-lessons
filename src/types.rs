pub fn types() {
    // numbers can be signed or unsigned
    let _signed_number: i8 = 13;
    let _unsigned_number: u8 = 26;

    // two types of strings
    let simple_string: &str = "I'm simple string";
    let _complex_string: String = String::from("Pls empower me");

    let _im_char: char = 'p';

    println!("Len count bytes, so you should use .chars().count() chain instead of .len()");
    println!("Len of string: {:?}", simple_string.len());
    println!("Count of chars of string: {:?}", simple_string.chars().count());
}
