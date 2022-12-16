fn main() {
    // 1 - Automatic type conversion doesn't exist, Rust requires explicit conversion
    let number = 7;

    // if number {
    //     println!("Can't compile")
    // }

    // 2 - A basic if statement
    if number < 100 {
        println!("Smaller than 100");
    }

    // 3 - The Rust version of a ternary
    let good_num = if true { 5 } else { 6 };

    println!("good_num: {good_num}");

    // 4 - Rust's equivalent of the for...of loop
    let half_months = ["Jan", "Feb", "Mar", "Apr", "May", "Jun"];
    for month in half_months {
        println!("month: {month}")
    }

    // 5 - Rust can also do range :)
    for num in (1..4).rev() {
        println!("{num}")
    }
}
