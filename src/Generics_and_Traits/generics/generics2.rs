
// // Implement the generic function below.
// fn sum

// fn main() {
//     assert_eq!(5, sum(2i8, 3i8));
//     assert_eq!(50, sum(20, 30));
//     assert_eq!(2.46, sum(1.23, 1.23));

//     println!("Success!");
// }



//  A function call with explicitly specified type parameters looks like: fun::<A, B, ...>().
fn sum <T: std::ops::Add<Output = T>>(a:T, b:T) -> T{
    return a + b
}

fn main() {
    assert_eq!(5, sum(2i8, 3i8));
    assert_eq!(50, sum(20, 30));
    assert_eq!(2.46, sum(1.23, 1.23));

    println!("Success!");
}
