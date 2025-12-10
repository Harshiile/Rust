fn main() {
    // Scanner Types
    let _i = 434; // Default i32, it ranges from i8,i16,...,i128

    let _f = 34.4; // Default i64

    let _b: bool = true; // Default false

    let _c = 'c';

    // Compound Types

    // ---------------------------------------------------
    // 1. Arrays - Collection of variables with same types, fixed length
    // ---------------------------------------------------
    let arr = [10, 20, 30];
    let [arr0, arr1, _] = arr; // Destructuring
    println!("Array : {} - {} - {}", arr0, arr1, arr[2]);
    println!(
        "Sum of {} & {} = {} | Using sum function",
        arr0,
        arr1,
        sum(arr[1], arr[0])
    );

    let _bytes_array = [0; 8]; // Length 8 , all elements set to 0

    // ---------------------------------------------------
    // 2. Tuples - Array of fixed length with diffn types
    // ---------------------------------------------------
    let tup = ("Hello World", 34, true);
    let (tup0, tup1, tup2) = tup; // Destructuring
    println!("Tuples : {} - {} - {}", tup0, tup1, tup2)
}

fn sum(x: i32, y: i32) -> i32 {
    let sum = x + y;
    sum // or return sum;
}
