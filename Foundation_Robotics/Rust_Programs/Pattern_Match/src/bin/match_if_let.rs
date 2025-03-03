//***********************************************/
//
// Match
//
// Fill the blanks
#[allow(unused)]
enum Direction {
    East,
    West,
    North,
    South,
}

fn main() {
    let dire = Direction::South;
    match dire {
        Direction::East => println!("East"),
        Direction::South | Direction::North => { // Matching South or North here
            println!("South or North");
        },
        Direction::West => println!("West"),
    };
}

// //***********************************************/
// //
// // Match is an expression, so we can use it in assignments.
// //
// fn main() {
//     let boolean = true;

//     // Fill the blank with a match expression:
//     //
//     // boolean = true => binary = 1
//     // boolean = false =>  binary = 0
//     let binary = match boolean {
//             true => 1,
//             false => 0,
//     };

//     assert_eq!(binary, 1);

//     println!("Success!");
// }

// //***********************************************/
// //
// // Using match to get the data an enum variant holds.
// //
// // Fill in the blanks
// enum Message {
//     Quit,
//     Move { x: i32, y: i32 },
//     Write(String),
//     ChangeColor(i32, i32, i32),
// }

// fn main() {
//     let msgs = [
//         Message::Quit,
//         Message::Move{x:1, y:3},
//         Message::ChangeColor(255,255,0)
//     ];

//     for msg in msgs {
//         show_message(msg)
//     }

//     println!("Success!");
// } 

// fn show_message(msg: Message) {
//     match msg {
//         Message::Move{x:a, y:b} => { // match  Message::Move
//             assert_eq!(a, 1);
//             assert_eq!(b, 3);
//         },
//         Message::ChangeColor(r, g, b) => {
//             assert_eq!(g, 255);
//             assert_eq!(b, 0);
//         }
//         _ => println!("no data in these variants")
//     }
// }

// //***********************************************/
// //
// // matches! looks like match, but can do something different.
// //
// fn main() {
//     let alphabets = ['a', 'E', 'Z', '0', 'x', '9' , 'Y'];

//     // Fill the blank with `matches!` to make the code work
//     for ab in alphabets {
//         assert!(matches!(ab, 'A'..='Z' | 'a'..='z' | '0'..='9' ))
//     }

//     println!("Success!");
// } 

// //***********************************************/
// //
// // 
// //
// enum MyEnum {
//     Foo,
//     Bar
// }

// fn main() {
//     let mut count = 0;

//     let v = vec![MyEnum::Foo,MyEnum::Bar,MyEnum::Foo];
//     for e in v {
//         if matches!(e, MyEnum::Foo)  { // Fix the error by changing only this line
//             count += 1;
//         }
//     }

//     assert_eq!(count, 2);

//     println!("Success!");
// }

// //***********************************************/
// //
// // For some cases, when matching enums, match is too heavy. We can use if let instead.
// //
// fn main() {
//     let o = Some(7);

//     // Remove the whole `match` block, using `if let` instead 
//     if let Some(i) = o {
//             println!("This is a really long string and `{:?}`", i);

//             println!("Success!");
//     };
// }

// //***********************************************/
// //
// // 
// //
// // Fill in the blank
// enum Foo {
//     Bar(u8)
// }

// fn main() {
//     let a = Foo::Bar(1);

//     if let Foo::Bar(i) = a {
//         println!("foobar holds the value: {}", i);

//         println!("Success!");
//     }
// }

// //***********************************************/
// //
// // 
// //
// enum Foo {
//     Bar,
//     Baz,
//     Qux(u32)
// }

// fn main() {
//     let a = Foo::Qux(10);

//     // Remove the codes below, using `match` instead 
//     match a {
//         Foo::Bar => println!("match foo::bar"),
//         Foo::Baz => println!("match foo::baz"),
//         _ => println!("match others"),
//     }
// }

// //***********************************************/
// //
// // Shadowing
// //
// // Fix the errors in-place
// fn main() {
//     let age = Some(30);
//     if let Some(age) = age { // Create a new variable with the same name as previous `age`
//        assert_eq!(age, 30);
//     } // The new variable `age` goes out of scope here
    
//     match age {
//         // Match can also introduce a new shadowed variable
//         Some(age) =>  println!("age is a new variable, it's value is {}",age),
//         _ => ()
//     }
//  }
