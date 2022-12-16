mod temp_converter;
mod fibonacci;
mod twelve_days_of_christmas;

use temp_converter::temp_converter;
use fibonacci::fibonacci;
use twelve_days_of_christmas::lyrics;

fn main() {
    println!("1 - TEMPERATURE CONVERSION");
    let indoors_fahrenheit: f32 = 85.0;
    let indoors_celsius: f32 = temp_converter('F', 'C', indoors_fahrenheit);

    println!("85 in C: {indoors_celsius}");

    let hk_celsius: f32 = 18.0;
    let hk_fahrenheit: f32 = temp_converter('C', 'F', hk_celsius);

    println!("18 in F: {hk_fahrenheit}");

    println!();

    println!("2 - FIBONACCI");
    for num in 0..10 {
        let res = fibonacci(num);
        println!("fib({num}): {res}")
    }

    println!();

    println!("3 - TWELVE DAYS OF CHRISTMAS");
    lyrics();
}
