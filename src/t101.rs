//1
#[test]
fn test1() {
    // Fill in the blanks to make it work
    struct A;          // Concrete type A.
    struct S(A);       // Concrete type S.
    struct SGen<T>(T); // Generic type SGen.
    fn reg_fn(_s: S) {}
    fn gen_spec_t(_s: SGen<A>) {}
    fn gen_spec_i32(_s: SGen<i32>) {}
    fn generic<T>(_s: SGen<T>) {}
    // Using the non-generic functions
    reg_fn(S(A)); // Concrete type.
    gen_spec_t(SGen(A)); // Implicitly specified type parameter A.
    gen_spec_i32(SGen(9)); // Implicitly specified type parameter i32.
    // Explicitly specified type parameter char to generic().
    generic::<char>(SGen('L'));
    // Implicitly specified type parameter char to generic().
    generic(SGen('W'));
    println!("Success!");
}

//2
#[test]
fn test2() {
    // Implement the generic function below.
    fn sum<T>(a: T, b: T) -> T
    where
        T: std::ops::Add<Output = T>,
    {
        a + b
    }
    assert_eq!(5, sum(2i8, 3i8));
    assert_eq!(50, sum(20, 30));
    assert_eq!(2.46, sum(1.23, 1.23));
    println!("Success!");
}

//3
#[test]
fn test3() {
    // Implement struct Point to make it work.
    struct Point<T> {
        x: T,
        y: T,
    }
    let _integer = Point { x: 5, y: 10 };
    let _float = Point { x: 1.0, y: 4.0 };
    println!("Success!");
}

//4
#[test]
fn test4() {
    // Modify this struct to make the code work
    struct Point<T> {
        x: T,
        y: String,
    }
    // DON'T modify this code.
    let _p = Point{x: 5, y : "hello".to_string()};
    println!("Success!");
}

//5
#[test]
fn test5() {
    // Add generic for Val to make the code work, DON'T modify the code in main.
    struct Val<T> {
        val: T,
    }
    impl<T> Val<T> {
        fn value(&self) -> &T {
            &self.val
        }
    }
    let x = Val{ val: 3.0 };
    let y = Val{ val: "hello".to_string()};
    println!("{}, {}", x.value(), y.value());
}


//6
#[test]
fn test6() {
    struct Point<T, U> {
        x: T,
        y: U,
    }
    impl<T, U> Point<T, U> {
        // Implement mixup to make it work, DON'T modify other code.
        fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
            Point {
                x: self.x,
                y: other.y,
            }
        }
    }
    let p1 = Point { x: 5, y: 10 };
    let p2 = Point { x: "Hello", y: '中'};
    let p3 = p1.mixup(p2);
    assert_eq!(p3.x, 5);
    assert_eq!(p3.y, '中');
    println!("Success!");
}


//7
#[test]
fn test7() {
    // Fix the errors to make the code work.
    struct Point<T> {
        x: T,
        y: T,
    }
    impl Point<f64> {
        fn distance_from_origin(&self) -> f64 {
            (self.x.powi(2) + self.y.powi(2)).sqrt()
        }
    }
    let p: Point<f64> = Point{x: 5.0, y: 10.0};
    println!("{}",p.distance_from_origin());
}
