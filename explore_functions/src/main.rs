fn main() {
    let y = {
        let x = 3;

        x + 1
    };
    let z = five();

    println!("The value of y: {y}, z: {z}");
}

fn five() -> i32 {
    // this is equivalent to return 5;
    5

    // but 5; would throw a compile error when called
    // 5;
}