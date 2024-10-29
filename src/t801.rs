//1
#[test]
fn test1() {
    // Fill the blanks
    enum Direction {
        East,
        West,
        North,
        South,
    }
    let dire = Direction::South;
    match dire {
        Direction::East => println!("East"),
        Direction::North | Direction::South => { // Matching South or North here
            println!("South or North");
        },
        _ => println!("West"),
    };
}

//2
#[test]
fn test2() {
    let boolean = true;
    // Fill the blank with a match expression:
    //
    // boolean = true => binary = 1
    // boolean = false =>  binary = 0
    let binary = match boolean {
        true => 1,
        false => 0,
    };
    assert_eq!(binary, 1);
    println!("Success!");
}

//3
#[test]
fn test3() {
    // Fill in the blanks
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }
    let msgs = [
        Message::Quit,
        Message::Move{x:1, y:3},
        Message::ChangeColor(255,255,0)
    ];
    for msg in msgs {
        show_message(msg)
    }
    fn show_message(msg: Message) {
        match msg {
            Message::Move{x: a, y: b} => { // match  Message::Move
                assert_eq!(a, 1);
                assert_eq!(b, 3);
            },
            Message::ChangeColor(_r, g, b) => {
                assert_eq!(g, 255);
                assert_eq!(b, 0);
            }
            __ => println!("no data in these variants")
        }
    }
    println!("Success!");
}

//4
#[test]
fn test4() {
    let alphabets = ['a', 'E', 'Z', '0', 'x', '9' , 'Y'];
    // Fill the blank with matches! to make the code work
    for ab in alphabets {
        assert!(matches!(ab, 'A'..='Z' | 'a'..='z' | '0'..='9'));
    }
    println!("Success!");
}

//5
#[test]
fn test5() {
    enum MyEnum {
        Foo,
        Bar
    }
    let mut count = 0;
    let v = vec![MyEnum::Foo,MyEnum::Bar,MyEnum::Foo];
    for e in v {
        if matches!(e, MyEnum::Foo) { // Fix the error by changing only this line
            count += 1;
        }
    }
    assert_eq!(count, 2);
    println!("Success!");
}

//6
#[test]
fn test6() {
    let o = Some(7);
    // Remove the whole match block, using if let instead
    if let Some(i) = o {
        println!("This is a really long string and {:?}", i);
        println!("Success!");
    }
}

//7
#[test]
fn test7() {
    // Fill in the blank
    enum Foo {
        Bar(u8)
    }
    let a = Foo::Bar(1);
    if let Foo::Bar(i) = a {
        println!("foobar holds the value: {}", i);
        println!("Success!");
    }
}

//8
#[test]
fn test8() {
    enum Foo {
        Bar,
        Baz,
        Qux(u32)
    }
    let a = Foo::Qux(10);
    // Remove the codes below, using match instead
    match a {
        Foo::Bar => println!("match foo::bar"),
        Foo::Baz => println!("match foo::baz"),
        _ => println!("match others"),
    }
}

//9
#[test]
// Fix the errors in-place
fn test9() {
    let age = Some(30);
    if let Some(age) = age { // Create a new variable with the same name as previous age
        assert_eq!(age, 30);
    } // The new variable age goes out of scope here
    match age {
        // Match can also introduce a new shadowed variable
        Some(age) =>  println!("age is a new variable, it's value is {}",age),
        _ => ()
    }
}
