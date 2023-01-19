fn main() {
    println!("Chapter 4 - Understanding Ownership");
    what_is_ownership();
    references_and_borrowing();
    slice_types();
}

fn slice_types() {
    let s1 = String::from("Hello, world!");
    let fw1 = first_words_idx(&s1);
    let slice1 = first_word_slice(&s1);
    let arr = [1, 2, 3, 4, 5];
    let slice2 = &arr[1..3];
    assert_eq!(slice2, &[2, 3]);
    //s1.clear(); // Trying to err - shows up as mutable borrow
    println!("Using usize, first word ends at {fw1}");
    println!("Using slices, first word is \"{slice1}\""); // Shows up as immutale borrow
}

/// function that takes a string of words separated by spaces and returns the first word it finds in that string
fn first_words_idx(s: &String) -> usize {
    // Convert string to byte array
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    s.len()
}

fn first_word_slice(s: &str) -> &str { // using &str instead of &String so it works on both
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i]; // same as &s[0..i]
        }
    }
    &s[..] // same as &s[0..s.len()]
}

fn references_and_borrowing() {
    let s1 = String::from("Hello!");
    let s1_len = cal_len_borrow(&s1);
    let mut s2 = s1.clone();
    {
        let r2 = &mut s2; // Err-Multiple mutable borrows unless parentheses
        edit_borrow(r2);
        println!("In separate context - {r2}");
    }
    //let r2 = &s2;
    let r1 = &mut s2; 
    edit_borrow(r1);
    //println!("{r2}");
    println!("Kept after borrowing - {s1} - {s1_len}\nEdited after borrowing - {s2}");
    let ref_to_nothing_question = no_dangle();
    println!("No longer dangling - {ref_to_nothing_question}");
    println!("-----------------------------------------------------");
}

fn what_is_ownership() { 
    let x = 1;
    let mut y = x;
    y = y + 1;
    let mut s = String::from("hello"); // from() allocates memory on the heap to create a mutable String 
    s.push_str(", world!");
    let s1 = s; // s moved to s1
    //println!("{}\n{}",s,s1); // will throw compile time error, as s is no longer valid
    let s2 = s1.clone();
    let s3 = s2.clone();
    takes_ownership(s3);
    //println!("s3 - {}",s3); // will throw error as takes_ownership invalidates s3
    let s4 = gives_ownership();
    println!("s4 - {}",s4);
    let s2 = takes_and_gives_back(s2);
    let (s2, len) = cal_len_tuple(s2);
    makes_copy(y);
    println!("s{x} - {s1}\ns{y} - {s2} - {len}");
    println!("-----------------------------------------------------");
}

fn takes_ownership(example_string: String) {
    println!("Taken ownership of string - {}",example_string);
}

fn gives_ownership() -> String {
    let example_string = String::from("testing testing");
    example_string
}

fn takes_and_gives_back(mut example_string: String) -> String {
    example_string.push_str(" Test!");
    example_string
}

fn makes_copy(example_i32: i32) {
    println!("Made a copy of i32 - {}",example_i32);
}

fn cal_len_tuple(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
}

fn cal_len_borrow(s: &String) -> usize {
    s.len()
}

fn edit_borrow(s: &mut String) {
    s.push_str(" test");
}

fn no_dangle() -> String {
    let s = String::from("hello");
    s
}

