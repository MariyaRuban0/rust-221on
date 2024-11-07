//1 завдання
// FILL in the blanks and FIX errors
// 1. Don't use `to_string()`
// 2. Don't add/remove any code line
#[test]
fn test1() {
    let mut s: String = String::from ("hello, ");
    s.push_str("world");
    s.push('!');


    move_ownership(s.clone());


    assert_eq!(s, "hello, world!");


    println!("Success!");
}


fn move_ownership(s: String) {
    println!("ownership of \"{}\" is moved here!", s)
}


//2 задание
#[test]
// FILL in the blanks
fn test2() {
    let mut s: String = String::from("hello, world");


    let slice1: &str = s.as_str(); // In two ways
    assert_eq!(slice1, "hello, world");


    let slice2: &str = &s[..5];
    assert_eq!(slice2, "hello");


    let slice3: &mut String = &mut s;
    slice3.push('!');
    assert_eq!(slice3, "hello, world!");


    println!("Success!");
}


//3 задание
// Question: how many heap allocations are happening here?
// Your answer:
#[test]
fn test3() {
    // Create a String type based on `&str`
    // The type of string literals is `&str`
    let s: String = String::from("hello, world!"); //1


    // Create a slice point to String `s`
    let slice: &str = &s; //hello, world!


    // Create a String type based on the recently created slice
    let s: String = slice.to_string(); //2


    assert_eq!(s, "hello, world!");


    println!("Success!");
}


//4 задание
// FILL in the blank and FIX errors
#[test]
fn test4() {
    let s: String = String::from("hello, 世界");
    let slice1: &str = &s[..1]; //tips: `h` only takes 1 byte in UTF8 format
    assert_eq!(slice1, "h");


    let slice2: &str = &s[7..10]; // Tips: `中`  takes 3 bytes in UTF8 format
    assert_eq!(slice2, "世");


    // Iterate through all chars in s
    for (i, c) in s.chars().enumerate() {
        if i == 7 {
            assert_eq!(c, '世')
        }
    }


    println!("Success!");
}


//5 задание
// FILL in the blanks
#[test]
fn test5() {
    let mut s: String = String::new(); //Vec<u8> - > "hello"
    s.push_str("hello");


    // Some bytes, in a vector
    let v: Vec<u8> = vec![104, 101, 108, 108, 111];


    // Turn a byte's vector into a String
    let s1: String = String::from_utf8(v).unwrap();




    assert_eq!(s, s1); //"hello"


    println!("Success!");
}


//6 задание
// Modify the code below to print out:
// 25
// 25
// 25
// Here, there’s no need to allocate more memory inside the loop.
#[test]
fn test6() {
    let mut s: String = String::with_capacity(25);


    println!("{}", s.capacity());


    for _ in 0..2 {
        s.push_str("hello"); //"hello"
        println!("{}", s.capacity());
    }
    println!("{}", s);


    println!("Success!");
}


//7
use std::mem;
#[test]
fn test7() {
    let story = String::from("Rust By Practice");


    // Prevent automatically dropping the String's data
    let mut story = mem::ManuallyDrop::new(story);


    let ptr = story.as_mut_ptr();
    let len = story.len();
    let capacity = story.capacity();


    // story has nineteen bytes
    assert_eq!(16, len);


    // We can re-build a String out of ptr, len, and capacity. This is all
    // unsafe because we are responsible for making sure the components are
    // valid:
    let s = unsafe { String::from_raw_parts(ptr, len, capacity) };


    assert_eq!(*story, s);


    println!("Success!")
}
