// Fix error without adding new line
fn main() {
    let _s: &str = "hello, world";

    println!("Success!");
}

//***********************************************/

// // Fix the error with at least two solutions
// fn main() {
//     let s: Box<str> = "hello, world".into();
//     greetings(&s)
// }

// fn greetings(s: &str) {
//     println!("{}",s)
// }

//***********************************************/

// // Fill the blank
// fn main() {
//     let mut s = String::from("");
//     s.push_str("hello, world");
//     s.push('!');

//     assert_eq!(s, "hello, world!");

//     println!("Success!");
// }

//***********************************************/

// // Fix all errors without adding newline
// fn main() {
//     let mut s = String::from("hello");
//     s.push(',');
//     s.push_str(" world");
//     s += "!";

//     println!("{}", s);
// }

//***********************************************/

// // Fill the blank
// fn main() {
//     let s = String::from("I like dogs");
//     // Allocate new memory and store the modified string there
//     let s1 = s.replace("dogs","cats");

//     assert_eq!(s1, "I like cats");

//     println!("Success!");
// }

//***********************************************/

// // Fix errors without removing any line
// fn main() {
//     let s1 = String::from("hello,");
//     let s2 = String::from("world!");
//     let s3 = s1.clone() + &s2.clone(); 
//     assert_eq!(s3, "hello,world!");
//     println!("{}", s1);
// }

//***********************************************/

// // Fix error with at least two solutions
// fn main() {
//     let s = "hello, world";
//     greetings(&s)
// }

// fn greetings(s: &str) {
//     println!("{}", s)
// }
// //----
// fn main() {
//     let s = String::from("hello, world");
//     greetings(&s)
// }

// fn greetings(s: &String) {
//     println!("{}", s)
// }

// //***********************************************/

// // Use two approaches to fix the error and without adding a new line
// fn main() {
//     let s = "hello, world";
//     let s1= s;

//     println!("Success!");
// }
// //---
// fn main() {
//     let s = "hello, world".to_string();
//     let s1: String = s;

//     println!("Success!");
// }

//***********************************************/
// fn main() {
//     // You can use escapes to write bytes by their hexadecimal values
//     // Fill the blank below to show "I'm writing Rust"
//     let byte_escape = "I'm writing Ru\x73\x74!";
//     println!("What are you doing\x3F (\\x3F means ?) {}", byte_escape);

//     // ...Or Unicode code points.
//     let unicode_codepoint = "\u{211D}";
//     let character_name = "DOUBLE-STRUCK CAPITAL R";

//     println!("Unicode character {} (U+211D) is called {}",
//                 unicode_codepoint, character_name );

//     let long_string = "String literals
//                         can span multiple lines.
//                         The linebreak and indentation here \
//                          can be escaped too!";
//     println!("{}", long_string);
// }

//***********************************************/

// /* Fill in the blank and fix the errors */
// fn main() {
//     let raw_str = "Escapes don't work here: \x3F \u{211D}";
//     // Modify above line to make it work
//     assert_eq!(raw_str, "Escapes don't work here: ? ℝ");

//     // If you need quotes in a raw string, add a pair of #s
//     let quotes = r#"And then I said: "There is no escape!""#;
//     println!("{}", quotes);

//     // If you need "# in your string, just use more #s in the delimiter.
//     // You can use up to 65535 #s.
//     let delimiter = r###"A string with "# in it. And even "##!"###;
//     println!("{}", delimiter);

//     // Fill the blank
//     let long_delimiter = r###"Hello, "##""###;
//     assert_eq!(long_delimiter, "Hello, \"##\"");

//     println!("Success!");
// }

// //***********************************************/
// use std::str;

// fn main() {
//     // Note that this is not actually a `&str`
//     let bytestring: &[u8; 21] = b"this is a byte string";

//     // Byte arrays don't have the `Display` trait, so printing them is a bit limited
//     println!("A byte string: {:?}", bytestring);

//     // Byte strings can have byte escapes...
//     let escaped = b"\x52\x75\x73\x74 as bytes";
//     // ...But no unicode escapes
//     // let escaped = b"\u{211D} Is not allowed";
//     println!("Some escaped bytes: {:?}", escaped);


//     // Raw byte strings work just like raw strings
//     let raw_bytestring = br"\u{211D} is not escaped here";
//     println!("{:?}", raw_bytestring);

//     // Converting a byte array to `str` can fail
//     if let Ok(my_str) = str::from_utf8(raw_bytestring) {
//         println!("And the same as text: '{}'", my_str);
//     }

//     let _quotes = br#"You can also use "fancier" formatting, \
//                     like with normal raw strings"#;

//     // Byte strings don't have to be UTF-8
//     let shift_jis = b"\x82\xe6\x82\xa8\x82\xb1\x82\xbb"; // "ようこそ" In SHIFT-JIS

//     // But then they can't always be converted to `str`
//     match str::from_utf8(shift_jis) {
//         Ok(my_str) => println!("Conversion successful: '{}'", my_str),
//         Err(e) => println!("Conversion failed: {:?}", e),
//     };
// }

//***********************************************/

// fn main() {
//     let s1 = String::from("hi,中国");
//     let h = &s1[0..1]; // Modify this line to fix the error, tips: `h` only takes 1 byte in UTF8 format
//     assert_eq!(h, "h");

//     let h1 = &s1[3..6]; // Modify this line to fix the error, tips: `中`  takes 3 bytes in UTF8 format
//     assert_eq!(h1, "中");

//     println!("Success!");
// }

//***********************************************/

// fn main() {
//     // Fill the blank to print each char in "你好，世界"
//     for c in "你好，世界".chars() {
//         println!("{}", c)
//     }
// }

// //***********************************************/

// // use utf8_slice::slice;
// fn main() {
//     let s = "The 🚀 goes to the 🌑!";

//     let rocket = s.split(pat);
//     // Will equal "🚀"
// }