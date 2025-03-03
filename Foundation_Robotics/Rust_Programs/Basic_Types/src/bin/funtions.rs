
fn main() {
    // Don't modify the following two lines!
    let (x, y) = (1, 2);
    let s = sum(x, y);

    assert_eq!(s, 3);

    println!("Success!");
}

fn sum(x: i32, y: i32) -> i32{
    x + y
}

//***********************************************/

// fn main() {
//     print();
//  }
 
//  // Replace i32 with another type
//  fn print() {
//     println!("Success!");
//  }

//***********************************************/

// // Solve it in two ways
// // DON'T let `println!` work
// fn main() {
//     never_return();
//     println!("Failed!");
// }

// fn never_return() -> ! {
//     // Implement this function, don't modify the fn signatures
//     // panic!()
// }

// //***********************************************/

// // Diverging functions
// // Diverging functions never return to the caller, so they may be used in places where a value of any type is expected.

// fn main() {
//     println!("Success!");
// }

// fn get_option(tp: u8) -> Option<i32> {
//     match tp {
//         1 => {
//             // TODO
//         }
//         _ => {
//             // TODO
//         }
//     };
    
//     // Rather than returning a None, we use a diverging function instead
//     never_return_fn()
// }

// // // IMPLEMENT this function in THREE ways
// fn never_return_fn() -> ! {
//     // todo!()
//     unimplemented!()

// }

// //***********************************************/

// fn main() {
//     // FILL in the blank
//     let b = __;

//     let _v = match b {
//         true => 1,
//         // Diverging functions can also be used in match expression to replace a value of any value
//         false => {
//             println!("Success!");
//             panic!("we have no value for `false`, but we can panic");
//         }
//     };

//     println!("Exercise Failed if printing out this line!");
// }