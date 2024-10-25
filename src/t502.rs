//1
#[test]
fn test1() {
    let x: i32 = 5;
    // Fill the blank
    let p: &i32 = &x;

    println!("the memory address of x is {:p}", p); // One possible output: 0x16fa3ac84
}
//2
#[test]
fn test2() {
    let x: i32 = 5;
    let y: &i32= &x;

    // Modify this line only
    assert_eq!(5, *y);

    println!("Success!");
}
//3
// Fix error
#[test]
fn test3() {
    let mut s: String = String::from("hello, ");

    borrow_object(&s);

    println!("Success!");
}

fn borrow_object(s: &String) {}

//4
// Fix error
#[test]
fn test4() {
    let mut s: String = String::from("hello, ");

    push_str(&mut s);

    println!("Success!");
}
fn push_str(s: &mut String) {
    s.push_str("world")
}

//5
#[test]
fn test5() {
    let mut s: String = String::from("hello, ");

    // Fill the blank to make it work
    let p: &mut String = &mut s;

    p.push_str("world");

    println!("Success!");
}

//6
#[test]
fn test6() {
    let c: char = '中';

    let r1: &char = &c;
    // Fill the blank，dont change other code
    let ref r2 = c;

    assert_eq!(*r1, *r2);

    // Check the equality of the two address strings
    assert_eq!(get_addr(r1),get_addr(r2));

    println!("Success!");
}

// Get memory address string
fn get_addr(r: &char) -> String {
    format!("{:p}", r)
}

//7
// Remove something to make it work
// Don't remove a whole line !
#[test]
fn test7() {
    let mut s: String = String::from("hello");

    let r1 = &s;
    let r2 = &s;

    println!("{}, {}", r1, r2);

    println!("s: {}", s);

    println!("Success!");
}

//8
#[test]
fn test8() {
    // Fix error by modifying this line
    let  mut s: String = String::from("hello, ");

    borrow_object(&mut s);

    println!("Success!");
}

fn borrow_object1(s: &mut String) {}

//9
// This code has no errors!\
#[test]
fn test9() {
    let mut s: String = String::from("hello, ");

    borrow_object(&s);

    s.push_str("world");

    println!("Success!");
}

fn borrow_object2(s: &String) {}

//10
// Comment one line to make it work
#[test]
fn test10() {
    let mut s: String = String::from("hello, ");

    let r1: &mut String = &mut s;
    r1.push_str("world");

    let r2: &mut String = &mut s;
    r2.push_str("!");

    println!("{}",r2);
}

//11
#[test]
fn test11() {
    let mut s: String = String::from("hello, ");

    //let r1 = &mut s;
    //let r2 = &mut s;

    // Add one line below to make a compiler error: cannot borrow `s` as mutable more than once at a time
    // You can't use r1 and r2 at the same time
    //println!("{}, {}", r1, r2);
}