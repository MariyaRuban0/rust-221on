//1
#[test]
fn test1() {
    // FILL in the blank.
    trait Bird {
        fn quack(&self) -> String;
    }
    struct Duck;
    impl Duck {
        fn swim(&self) {
            println!("Look, the duck is swimming")
        }
    }
    struct Swan;
    impl Swan {
        fn fly(&self) {
            println!("Look, the duck.. oh sorry, the swan is flying")
        }
    }
    impl Bird for Duck {
        fn quack(&self) -> String{
            "duck duck".to_string()
        }
    }
    impl Bird for Swan {
        fn quack(&self) -> String{
            "swan swan".to_string()
        }
    }
    let duck = Duck;
    duck.swim();
    let bird = hatch_a_bird(2);
    // This bird has forgotten how to swim, so below line will cause an error.
    // bird.swim();
    // But it can quak.
    assert_eq!(bird.quack(), "duck duck");
    let bird = hatch_a_bird(1);
    // This bird has forgotten how to fly, so below line will cause an error.
    // bird.fly();
    // But it can quak too.
    assert_eq!(bird.quack(), "swan swan");
    println!("Success!");
    // IMPLEMENT this function.
    fn hatch_a_bird(species: u8) -> Box<dyn Bird> {
        match species {
            1 => Box::new(Swan),
            2 => Box::new(Duck),
            _ => panic!(),
        }
    }
}

//2
#[test]
fn test2() {
    // FILL in the blank to make the code work.
    trait Bird {
        fn quack(&self);
    }
    struct Duck;
    impl Duck {
        fn fly(&self) {
            println!("Look, the duck is flying")
        }
    }
    struct Swan;
    impl Swan {
        fn fly(&self) {
            println!("Look, the duck.. oh sorry, the swan is flying")
        }
    }
    impl Bird for Duck {
        fn quack(&self) {
            println!("{}", "duck duck");
        }
    }
    impl Bird for Swan {
        fn quack(&self) {
            println!("{}", "swan swan");
        }
    }
    let birds: [&dyn Bird; 2] = [&Duck, &Swan];
    for bird in birds {
        bird.quack();
        // When duck and swan turn into Birds, they all forgot how to fly, only remember how to quack.
        // So, the code below will cause an error.
        // bird.fly();
    }
}


//3
#[test]
fn test3() {
    // FILL in the blanks.
    trait Draw {
        fn draw(&self) -> String;
    }
    impl Draw for u8 {
        fn draw(&self) -> String {
            format!("u8: {}", *self)
        }
    }
    impl Draw for f64 {
        fn draw(&self) -> String {
            format!("f64: {}", *self)
        }
    }
    let x = 1.1f64;
    let y = 8u8;
    // Draw x.
    draw_with_box(Box::new(x));
    // Draw y.
    draw_with_ref(&y);
    println!("Success!");
    fn draw_with_box(x: Box<dyn Draw>) {
        x.draw();
    }
    fn draw_with_ref(x: &dyn Draw) {
        x.draw();
    }
}


//4
#[test]
fn test4() {
    trait Foo {
        fn method(&self) -> String;
    }
    impl Foo for u8 {
        fn method(&self) -> String { format!("u8: {}", *self) }
    }
    impl Foo for String {
        fn method(&self) -> String { format!("string: {}", *self) }
    }

    // IMPLEMENT below with generics.
    fn static_dispatch<T: Foo>(a: T) {
        // or: fn static_dispatch(a: impl Foo)
        a.method();
    }
    // Implement below with trait objects.
    fn dynamic_dispatch(a: &dyn Foo) {
        a.method();
    }
    let x = 5u8;
    let y = "Hello".to_string();
    static_dispatch(x);
    dynamic_dispatch(&y);
    println!("Success!");
}


//5
#[test]
fn test5() {
    // Use at least two approaches to make it work.
    // DON'T add/remove any code line.
    trait MyTrait {
        fn f(&self) -> Self;
    }
    impl MyTrait for u32 {
        fn f(&self) -> Self { 42 }
    }
    impl MyTrait for String {
        fn f(&self) -> Self { self.clone() }
    }
    fn my_function<T: MyTrait>(x: T) -> T {
        x.f()
    }
    my_function(13_u32);
    my_function(String::from("abc"));
    println!("Success!");
}
