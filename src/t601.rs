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

//7
// Fix error with at least two solutions
#[test]
fn test7() {
    let s: &str = "hello, world";
    greetings(&s.to_owned()) //&str -> String
}
fn greetings1(s: String) {
    println!("{}", s)
}

//8
// Use two approaches to fix the error and without adding a new line
#[test]
fn test8() {
    let s: String = "hello, world".to_string();
    let s1: &str = &s.as_str(); //&String -> &str

    println!("Success!");
}

//9
#[test]
fn test9() {
    // You can use escapes to write bytes by their hexadecimal values
    // Fill the blank below to show "I'm writing Rust"
    let byte_escape = "I'm writing Ru\x73\x74!";
    println!("What are you doing\x3F (\\x3F means ?) {}", byte_escape);

    // ...Or Unicode code points.
    let unicode_codepoint = "\u{211D}";
    let character_name = "\"DOUBLE-STRUCK CAPITAL R\"";

    println!("Unicode character {} (U+211D) is called {}",
             unicode_codepoint, character_name );

    let long_string = "String literals
                        can span multiple lines.
                        The linebreak and indentation here \
                         can be escaped too!";
    println!("{}", long_string);
}

//10
#[test]
fn test10() {
    let raw_str = "Escapes don't work here: ? ℝ";
    assert_eq!(raw_str, "Escapes don't work here: ? ℝ");

    // If you need quotes in a raw string, add a pair of #s
    let quotes = r#"And then I said: "There is no escape!""#;
    println!("{}", quotes);

    // If you need # in your string, just use more #s in the delimiter.
    // You can use up to 65535 #s.
    let delimiter = r###"A string with "# in it. And even "##!"###;
    println!("{}", delimiter);

    // Fill the blank
    let long_delimiter = r####"Hello, "##""####;
    assert_eq!(long_delimiter, "Hello, \"##\"");

    println!("Success!");
}

//11
#[test]
fn test11() {
    let s1: String = String::from("hi,中国");
    let h: &str = &s1[0..1]; // Modify this line to fix the error, tips: `h` only takes 1 byte in UTF8 format
    assert_eq!(h, "h");

    let h1 = &s1[3..6]; // Modify this line to fix the error, tips: `中`  takes 3 bytes in UTF8 format
    assert_eq!(h1, "中");

    println!("{}", h1);

    println!("Success!");
}

//12
#[test]
fn test12() {
    // Fill the blank to print each char in "你好，世界"
    for c in "你好，世界".chars() {
        println!("{}", c)
    }
}

