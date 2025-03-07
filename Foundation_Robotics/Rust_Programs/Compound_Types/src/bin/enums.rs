
//***********************************************/
//
// Enums can be created with explicit discriminator.
//
// Fix the errors
#[allow(dead_code)]
enum Number {
    Zero,
    Hi,
    Two,
}

#[allow(dead_code)]
enum Number1 {
    Zero = 0,
    One,
    Two,
}

#[allow(dead_code)]
// C-like enum
enum Number2 {
    Zero = 0,
    One = 1,
    Two = 6,
}


fn main() {
    // An enum variant can be converted to a integer by `as`
    assert_eq!(Number::Hi as u8, Number1::One as u8);
    assert_eq!(Number1::One as u8 , Number2::One as u8);
    println!("{}", Number2::Two as u8);
    println!("{}", Number::Hi as u8);
    println!("Success!");
} 

// //***********************************************/
// //
// //Each enum variant can hold its own data.
// //
// // Fill in the blank
// enum Message {
//     Quit,
//     Move { x: i32, y: i32 },
//     Write(String),
//     ChangeColor(i32, i32, i32),
// }

// fn main() {
//     let msg1 = Message::Move{x : 1, y : 2}; // Instantiating with x = 1, y = 2 
//     let msg2 = Message::Write(String::from("hello, world")); // Instantiating with "hello, world!"

//     println!("Success!");
// } 

// //***********************************************/
// //
// // We can get the data which an enum variant is holding by pattern match.
// //
// // Fill in the blank and fix the error
// enum Message {
//     Quit,
//     Move { x: i32, y: i32 },
//     Write(String),
//     ChangeColor(i32, i32, i32),
// }

// fn main() {
//     let msg = Message::Move{x: 1, y: 1};

//     if let Message::Move{x:a, y:b} = msg {
//         assert_eq!(a, b);
//     } else {
//         panic!("NEVER LET THIS RUN！");
//     }

//     println!("Success!");
// } 

// //***********************************************/
// //
// // Fill in the blank and fix the errors
// //
// #[derive(Debug)]
// enum Message {
//     Quit,
//     Move { x: i32, y: i32 },
//     Write(String),
//     ChangeColor(i32, i32, i32),
// }

// fn main() {
//     let msgs: [Message; 3] = [
//         Message::Quit,
//         Message::Move{x:1, y:3},
//         Message::ChangeColor(255,255,0)
//     ];

//     for msg in msgs {
//         show_message(msg)
//     }
// } 

// fn show_message(msg: Message) {
//     println!("{:?}", msg);
// }

// //***********************************************/
// //
// //Since there is no null in Rust, we have to use enum Option<T> to deal with the cases when the value is absent.
// //
// // Fill in the blank to make the `println` work.
// // Also add some code to prevent the `panic` from running.
// fn main() {
//     let five = Some(5);
//     let six = plus_one(five);
//     let none = plus_one(None);

//     if let Some(n) = six {
//         println!("{}", n);

//         println!("Success!");
//     } 
//     else{
//         panic!("NEVER LET THIS RUN！");
//     }
// } 

// fn plus_one(x: Option<i32>) -> Option<i32> {
//     match x {
//         None => None,
//         Some(i) => Some(i + 1),
//     }
// }

// //***********************************************/
// //
// // Implement a linked-list via enums.
// //
// use crate::List::*;

// enum List {
//     // Cons: Tuple struct that wraps an element and a pointer to the next node
//     Cons(u32, Box<List>),
//     // Nil: A node that signifies the end of the linked list
//     Nil,
// }

// // Methods can be attached to an enum
// impl List {
//     // Create an empty list
//     fn new() -> List {
//         // `Nil` has type `List`
//         Nil
//     }

//     // Consume a list, and return the same list with a new element at its front
//     fn prepend(self, elem: u32) -> __ {
//         // `Cons` also has type List
//         Cons(elem, Box::new(self))
//     }

//     // Return the length of the list
//     fn len(&self) -> u32 {
//         // `self` has to be matched, because the behavior of this method
//         // depends on the variant of `self`
//         // `self` has type `&List`, and `*self` has type `List`, matching on a
//         // concrete type `T` is preferred over a match on a reference `&T`
//         // After Rust 2018 you can use self here and tail (with no ref) below as well,
//         // rust will infer &s and ref tail. 
//         // See https://doc.rust-lang.org/edition-guide/rust-2018/ownership-and-lifetimes/default-match-bindings.html
//         match *self {
//             // Can't take ownership of the tail, because `self` is borrowed;
//             // Instead take a reference to the tail
//             Cons(_, ref tail) => 1 + tail.len(),
//             // Base Case: An empty list has zero length
//             Nil => 0
//         }
//     }

//     // Return representation of the list as a (heap allocated) string
//     fn stringify(&self) -> String {
//         match *self {
//             Cons(head, __ tail) => {
//                 // `format!` is similar to `print!`, but returns a heap
//                 // allocated string instead of printing to the console
//                 format!("{}, {}", head, tail.__())
//             },
//             Nil => {
//                 format!("Nil")
//             },
//         }
//     }
// }

// fn main() {
//     // Create an empty linked list
//     let mut list = List::new();

//     // Prepend some elements
//     list = list.prepend(1);
//     list = list.prepend(2);
//     list = list.prepend(3);

//     // Show the final state of the list
//     println!("linked list has length: {}", list.len());
//     println!("{}", list.stringify());
// }