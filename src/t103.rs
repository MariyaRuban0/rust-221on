//1
#[test]
fn test1() {
    // Fill in the two impl blocks to make the code work.
    // DON'T modify the code in main.
    trait Hello {
        fn say_hi(&self) -> String {
            String::from("hi")
        }
        fn say_something(&self) -> String;
    }
    struct Student {}
    impl Hello for Student {
        fn say_something(&self) -> String {
            String::from("I'm a good student")
        }
    }
    struct Teacher {}
    impl Hello for Teacher {
        fn say_hi(&self) -> String {
            String::from("Hi, I'm your new teacher")
        }
        fn say_something(&self) -> String {
            String::from("I'm not a bad teacher")
        }
    }
    let s = Student {};
    assert_eq!(s.say_hi(), "hi");
    assert_eq!(s.say_something(), "I'm a good student");
    let t = Teacher {};
    assert_eq!(t.say_hi(), "Hi, I'm your new teacher");
    assert_eq!(t.say_something(), "I'm not a bad teacher");
    println!("Success!");
}

//2
#[test]
fn test2() {
    // Centimeters, a tuple struct that can be compared
    #[derive(PartialEq, PartialOrd)]
    struct Centimeters(f64);
    // Inches, a tuple struct that can be printed
    #[derive(Debug)]
    struct Inches(i32);
    impl Inches {
        fn to_centimeters(&self) -> Centimeters {
            let &Inches(inches) = self;
            Centimeters(inches as f64 * 2.54)
        }
    }
    // ADD some attributes to make the code work!
    // DON'T modify other code!
    #[derive(Debug, PartialEq, PartialOrd)]
    struct Seconds(i32);
    let _one_second = Seconds(1);
    println!("One second looks like: {:?}", _one_second);
    let _this_is_true = _one_second == _one_second;
    let _this_is_false = _one_second > _one_second;
    let foot = Inches(12);
    println!("One foot equals {:?}", foot);
    let meter = Centimeters(100.0);
    let cmp =
        if foot.to_centimeters() < meter {
            "smaller"
        } else {
            "bigger"
        };
    println!("One foot is {} than one meter.", cmp);
}

//3
#[test]
fn test3() {
    // Implement fn multiply to make the code work.
    // As mentioned above, + needs T to implement std::ops::Add Trait.
    // So, what about *?  You can find the answer here: https://doc.rust-lang.org/core/ops/
    fn multiply<T>(a: T, b: T) -> T
    where
        T: std::ops::Mul<Output = T>,
    {
        a * b
    }
    assert_eq!(6, multiply(2u8, 3u8));
    assert_eq!(5.0, multiply(1.0, 5.0));
    println!("Success!");
}

//4
#[test]
fn test1014() {
    // Fix the errors, DON'T modify the code in main.
    use std::ops;
    struct Foo;
    struct Bar;
    #[derive(Debug, PartialEq)]
    struct FooBar;
    #[derive(Debug, PartialEq)]
    struct BarFoo;
    // The std::ops::Add trait is used to specify the functionality of +.
    // Here, we make Add<Bar> - the trait for addition with a RHS of type Bar.
    // The following block implements the operation: Foo + Bar = FooBar
    impl ops::Add<Bar> for Foo {
        type Output = FooBar;
        fn add(self, _rhs: Bar) -> FooBar {
            FooBar
        }
    }
    impl ops::Sub<Bar> for Foo {
        type Output = BarFoo;
        fn sub(self, _rhs: Bar) -> BarFoo {
            BarFoo
        }
    }
    // DON'T modify the code below.
    // You need to derive some trait for FooBar to make it comparable.
    assert_eq!(Foo + Bar, FooBar);
    assert_eq!(Foo - Bar, BarFoo);
    println!("Success!");
}

//5
#[test]
fn test5() {
    // Implement fn summary to make the code work.
    // Fix the errors without removing any code line
    trait Summary {
        fn summarize(&self) -> String;
    }
    #[derive(Debug)]
    struct Post {
        title: String,
        author: String,
        content: String,
    }
    impl Summary for Post {
        fn summarize(&self) -> String {
            format!("The author of post {} is {}", self.title, self.author)
        }
    }
    #[derive(Debug)]
    struct Weibo {
        username: String,
        content: String,
    }
    impl Summary for Weibo {
        fn summarize(&self) -> String {
            format!("{} published a weibo {}", self.username, self.content)
        }
    }
    let post = Post {
        title: "Popular Rust".to_string(),
        author: "Sunface".to_string(),
        content: "Rust is awesome!".to_string(),
    };
    let weibo = Weibo {
        username: "sunface".to_string(),
        content: "Weibo seems to be worse than Tweet".to_string(),
    };
    summary(&post);
    summary(&weibo);
    println!("{:?}", post);
    println!("{:?}", weibo);
    // Implement fn summary below.
    fn summary(t: &impl Summary) {
        println!("{}", t.summarize());
    }
}


//6
#[test]
fn test6() {
    struct Sheep {}
    struct Cow {}
    trait Animal {
        fn noise(&self) -> String;
    }
    impl Animal for Sheep {
        fn noise(&self) -> String {
            "baaaaah!".to_string()
        }
    }
    impl Animal for Cow {
        fn noise(&self) -> String {
            "moooooo!".to_string()
        }
    }
    // Returns some struct that implements Animal, but we don't know which one at compile time.
    // FIX the errors here, you can make a fake random, or you can use trait object.
    fn random_animal(random_number: f64) -> Box<dyn Animal> {
        if random_number < 0.5 {
            Box::new(Sheep {})
        } else {
            Box::new(Cow {})
        }
    }
    let random_number = 0.234;
    let animal = random_animal(random_number);
    println!("You've randomly chosen an animal, and it says {}", animal.noise());
}

//7
#[test]
fn test7() {
    assert_eq!(sum(1, 2), 3);
}
// Implement fn sum with trait bound in two ways.
fn sum<T: std::ops::Add<Output = T>>(x: T, y: T) -> T {
    x + y
}

//8
// FIX the errors.
struct Pair<T> {
    x: T,
    y: T,
}
impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self {
            x,
            y,
        }
    }
}
impl<T: std::fmt::Debug + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {:?}", self.x);
        } else {
            println!("The largest member is y = {:?}", self.y);
        }
    }
}
#[derive(Debug, PartialEq, PartialOrd)]
struct Unit(i32);
#[test]
fn test8() {
    let pair = Pair{
        x: Unit(1),
        y: Unit(3)
    };
    pair.cmp_display();
}



//9
#[test]
fn test9() {
    // Fill in the blanks to make it work
    fn example1() {
        // T: Trait is the commonly used way.
        // T: Fn(u32) -> u32 specifies that we can only pass a closure to T.
        struct Cacher<T: Fn(u32) -> u32> {
            calculation: T,
            value: Option<u32>,
        }
        impl<T: Fn(u32) -> u32> Cacher<T> {
            fn new(calculation: T) -> Cacher<T> {
                Cacher {
                    calculation,
                    value: None,
                }
            }
            fn value(&mut self, arg: u32) -> u32 {
                match self.value {
                    Some(v) => v,
                    None => {
                        let v = (self.calculation)(arg);
                        self.value = Some(v);
                        v
                    }
                }
            }
        }
        let mut cacher = Cacher::new(|x| x+1);
        assert_eq!(cacher.value(10), 11);
        assert_eq!(cacher.value(15), 11);
    }
    fn example2() {
        // We can also use where to construct T
        struct Cacher<T>
        where T: Fn(u32) -> u32,
        {
            calculation: T,
            value: Option<u32>,
        }
        impl<T> Cacher<T>
        where T: Fn(u32) -> u32,
        {
            fn new(calculation: T) -> Cacher<T> {
                Cacher {
                    calculation,
                    value: None,
                }
            }
            fn value(&mut self, arg: u32) -> u32 {
                match self.value {
                    Some(v) => v,
                    None => {
                        let v = (self.calculation)(arg);
                        self.value = Some(v);
                        v
                    }
                }
            }
        }
        let mut cacher = Cacher::new(|x| x+1);
        assert_eq!(cacher.value(20), 21);
        assert_eq!(cacher.value(25), 21);
    }
    example1();
    example2();
    println!("Success!");
}
