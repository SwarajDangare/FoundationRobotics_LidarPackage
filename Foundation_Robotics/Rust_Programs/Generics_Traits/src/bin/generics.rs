#[allow(unused)]
// //***********************************************/
// //
// // Functions
// //
// // Fill in the blanks to make it work
// struct A;          // Concrete type `A`.
// struct S(A);       // Concrete type `S`.
// struct SGen<T>(T); // Generic type `SGen`.

// fn reg_fn(_s: S) {}

// fn gen_spec_t(_s: SGen<A>) {}

// fn gen_spec_i32(_s: SGen<i32>) {}

// fn generic<T>(_s: SGen<T>) {}

// fn main() {
//     // Using the non-generic functions
    
//     reg_fn(S(A));          // Concrete type.
//     gen_spec_t(SGen(A));   // Implicitly specified type parameter `A`.
//     gen_spec_i32(SGen(7)); // Implicitly specified type parameter `i32`.

//     // Explicitly specified type parameter `char` to `generic()`.
//     generic::<char>(SGen('A'));

//     // Implicitly specified type parameter `char` to `generic()`.
//     generic(SGen(57));

//     println!("Success!");
// }

// //***********************************************/
// //
// // A function call with explicitly specified type parameters looks like: fun::<A, B, ...>().
// //
// // Implement the generic function below.
// fn sum<T: std::ops::Add<Output = T>>(s1: T, s2: T) -> T{
//     s1 + s2

// }

// fn main() {
//     assert_eq!(5, sum(2i8, 3i8));
//     assert_eq!(50, sum(20, 30));
//     assert_eq!(2.46, sum(1.23, 1.23));

//     println!("Success!");
// }

// //***********************************************/
// //
// // Struct and impl
// //
// // Implement struct Point to make it work.
// struct Point<T>{
//     x: T,
//     y: T
// }

// fn main() {
//     let integer = Point { x: 5, y: 10 };
//     let float = Point { x: 1.0, y: 4.0 };

//     println!("Success!");
// }

// //***********************************************/
// //
// // 
// //
// // Modify this struct to make the code work
// struct Point<T,R> {
//     x: T,
//     y: R,
// }

// fn main() {
//     // DON'T modify this code.
//     let p = Point{x: 5, y : "hello".to_string()};

//     println!("Success!");
// }

// //***********************************************/
// //
// // 
// //
// // Add generic for Val to make the code work, DON'T modify the code in `main`.
// struct Val<T> {
//     val: T,
// }

// impl<T> Val<T> {
//     fn value(&self) -> &T {
//         &self.val
//     }
// }


// fn main() {
//     let x = Val{ val: 3.0 };
//     let y = Val{ val: "hello".to_string()};
//     println!("{}, {}", x.value(), y.value());
// }

//***********************************************/
//
// 
//
struct Point<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point<T, U> {
    // Implement mixup to make it work, DON'T modify other code.
    fn mixup<S,R>(self, p: Point<S,R>) -> Point<T,R>{
        let a: Point<T,R>= Point{x: self.x, y: p.y}; 
        a
    }
}

fn main() {
    let p1 = Point { x: 5, y: 10 };
    let p2 = Point { x: "Hello", y: '中'};

    let p3: Point<i32, char> = p1.mixup(p2);

    assert_eq!(p3.x, 5);
    assert_eq!(p3.y, '中');

    println!("Success!");
}

