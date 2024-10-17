//1
// Fix error without adding new line
#[test]
fn test1() {
    let s: &str = "hello, world";

    println!("Success!");
}

//2
// Fix the error with at least two solutions
#[test]
fn test2() {
    let s: Box<str> = "hello, world".into();
    greetings(&s)
}

fn greetings(s: &str) {
    println!("{}",s)
}

//3
// Fill the blank
#[test]
fn test3() {
    let mut s: String = String::from("");
    s.push_str("hello, world");
    s.push('!');

    assert_eq!(s, "hello, world!");

    println!("{}", s);

    println!("Success!");
}

//4
// Fix all errors without adding newline
#[test]
fn test4() {
    let mut s = String::from("hello");
    s.push(',');
    s.push_str(" world");
    s += "!";

    println!("{}", s);
}

//5
// Fill the blank
#[test]
fn test5() {
    let s: String = String::from("I like dogs");
    // Allocate new memory and store the modified string there
    let s1 = s.replace("dogs", "cats");

    assert_eq!(s1, "I like cats");

    println!("Success!");
}

//6
// Fix errors without removing any line
#[test]
fn test6() {
    let s1: String = String::from("hello,");
    let s2: String = String::from("world!");
    let s3: String = s1 + s2.as_str(); //Strinf -> &str
    assert_eq!(s3, "hello,world!");
    println!("{}", s3);
}
