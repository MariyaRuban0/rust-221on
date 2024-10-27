//1
// Fix the error
struct Person {
    name: String,
    age: u8,
    hobby: String
}
#[test]
fn test1() {
    let age: u8 = 30;
    let p = Person {
        name: String::from("sunface"),
        age,
        hobby: String::from("coding"),
    };

    println!("Success!");
}

//2
/*#[test]
struct Unit;
trait SomeTrait {
    // ...Some behaviors defined here.
}
// We don't care about what fields  are  in the Unit, but we care about its behaviors.
// So we use a struct with no fields and implement some behaviors for it
impl SomeTrait for Unit {  }
fn main() {
    let u = Unit;
    do_something_with_unit(u);

    println!("Success!");
}

// Fill the blank to make the code work
fn do_something_with_unit(u: __) {   } */

//3
// Fix the error and fill the blanks
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);
#[test]
fn test3() {
    let v: Point = Point(0, 127, 255);
    check_color(v);

    println!("Success!");
}

fn check_color(p: Point) {
    let Point(x, _, z) = p;
    assert_eq!(x, 0);
    assert_eq!(p.1, 127);
    assert_eq!(z, 255);
}

//4
// Fill the blank and fix the error without adding/removing new line
struct Person2 {
    name: String,
    age: u8,
}
#[test]
fn test4() {
    let age: u8 = 18;
    let mut p: Person2 = Person2 {
        name: String::from("sunface"),
        age,
    };

    // How can you believe sunface is only 18?
    p.age = 30;

    // Fill the blank
    p.name = String::from("sunfei");

    println!("Success!");
}

//5
// Fill the blank
struct Person3 {
    name: String,
    age: u8,
}
#[test]
fn test5() {
    println!("Success!");
}

fn build_person(name: String, age: u8) -> Person3 {
    Person3 {
        age,
        name,
    }
}

//6
// Fill the blank to make the code work
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}
#[test]
fn test6() {
    let u1: User = User {
        email: String::from("someone@example.com"),
        username: String::from("sunface"),
        active: true,
        sign_in_count: 1,
    };

    let u2: User = set_email(u1);

    println!("Success!");
}

fn set_email(u: User) -> User {
    User {
        email: String::from("contact@im.dev"),
        ..u
    }
}

//7
// Fill the blanks to make the code work
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
#[test]
fn test7() {
    let scale: u32 = 2;
    let rect1: Rectangle = Rectangle {
        width: dbg!(30 * scale), // Print debug info to stderr and assign the value of  `30 * scale` to `width`
        height: 50,
    };

    dbg!(&rect1); // Print debug info to stderr

    println!("{:?}", rect1); // Print debug info to stdout
}

//8
// Fix errors to make it work
#[derive(Debug)]
struct File {
    name: String,
    data: String,
}
#[test]
fn test8() {
    let f: File = File {
        name: String::from("readme.md"),
        data: "Rust By Practice".to_string()
    };

    let _name: String = f.name;

    // ONLY modify this line
    println!("{}, {}",_name, f.data);
}