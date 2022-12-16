fn main() {
    // String::from is a mutable string; "hello" is a 
    // string literal, included with the executable
    let s1 = String::from("hello"); 
    let s2 = s1;

    // 1 - Invalidated reference - this is called a move
    // this is necessary to prevent a double free when going out of scope
    // (double free-ing of the pointer, length and capacity in the stack, not heap)
    // println!("s1: {s1}");
    println!("s2: {s2}");

    // 2 - Deep copying using clone
    let s3 = String::from("Deep cloning");
    let s4 = s3.clone();

    println!("s3: {s3}, s4: {s4}");

    // 3 - Data types of a known size at compile time copy by value (cheap to copy)
    // these implement the Copy trait - boolean, ints, floats, char, tuples (of the previous ones)
    let x = 3;
    let y = x;

    println!("x: {x}, y: {y}");

    // 4 - ownership rules apply to passing variables into functions as well
    let str = String::from("func");

    take_ownership(str);

    let int_value = 3;

    makes_copy(int_value);

    // println!("str is invalidated: {str}"); // value borrowed after move

    println!("int_value: {int_value}");

    // 5 - a function that gives ownership of the variable where the pointer points to
    let returned = gives_ownership();

    println!("returned: {returned}");

    let sample_str = String::from("sample");
    let borrowed_and_returned = takes_and_gives_back(sample_str);

    println!("borrowed and returned: {borrowed_and_returned}");

    // 6 - returning multiple values
    let main_str = String::from("main_str");
    let (str, len) = calculate_length(main_str);

    println!("The length of {str} is {len}");

    // 7 - without taking ownership of value, use a reference, which is called borrowing
    // not dropped because *s is not owned by the function - only s the reference is
    let sample_str_ref = String::from("Sample str ref");
    let str_len_ref = calculate_length_ref(&sample_str_ref);

    println!("{sample_str_ref} length is {str_len_ref}");

    // 8 - references are immutable by default, even if the dereferenced value is mutable
    // let sample_str_b = String::from("hey");
    // change_ref(&sample_str_b);

    // 9 - we can make the reference mutable by marking it as such explicitly
    let mut sample_str_mutable = String::from("sample str mutable");
    change_mutable(&mut sample_str_mutable);

    println!("sample_str_mutable: {sample_str_mutable}");

    // 10 - only one mutable reference is allowed
    // this is extremely important to PREVENT RACES!!
    // recall races happen when 
    // 1. Two or more pointers access the same data at the same time.
    // 2. At least one of the pointers is being used to write to the data.
    // 3. There’s no mechanism being used to synchronize access to the data.
    let mut single_mut_int = String::from("heyyyyy");

    let r1 = &mut single_mut_int;
    // let r2 = &mut single_mut_int;

    println!("{r1}");
    // println!("{r1}, {r2}");

    // 11 - multiple mutable references are technically allowed, but
    // only in different scopes
    // REVISIT THIS!!
    let mut s3 = String::from("multiple mut refs");

    let _r_s3 = &mut s3;

    // {
    //     let r2_s3 = &mut s3;

    //     println!("{r2_s3}");
    // }

    // println!("{_r_s3}"); // this is not allowed because mutation happens between its mutable definition and usage - i.e. the scopes overlap

    let r3_s3 = &mut s3;

    println!("{r3_s3}"); // this IS allowed because it only uses the mutable ref right after - the execution is sequential

    // 12 - no simultaneous mutable and immutable references
    let mut _s4 = String::from("s4");

    let r1_s4 = &_s4;
    let r2_s4 = &_s4;
    // let r3_s4 = &mut _s4;

    println!("{r1_s4}");
    println!("{r2_s4}");
    // println!("{r3_s4}"); // I think this isn't ok because it'll affect the previous immutable references

    // 13 - dangling references are disallowed
    // see dangle and no_dangle
    no_dangle();
}

fn take_ownership(str: String) {
    println!("str: {str}");
}

fn makes_copy(num: i32) {
    println!("num: {num}");
}

fn gives_ownership() -> String {
    let some_string = String::from("gives ownership");

    some_string
}

fn takes_and_gives_back(str: String) -> String {
    str
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();

    (s, length)
}

fn calculate_length_ref(s: &String) -> usize {
    s.len()
}

// fn change_ref(some_str: &String) {
//     some_str.push_str("Hi!")
// }

fn change_mutable(some_str: &mut String) {
    some_str.push_str("heyyy")
}

// fn dangle() -> &String {
//     let s = String::from("dangle");

//     &s
// }

fn no_dangle() -> String {
    let s = String::from("no dangle");

    s
}