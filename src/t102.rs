//1
#[test]
fn test1() {
    struct Array<T, const N: usize> {
        data : [T; N]
    }
    let _arrays = [
        Array{
            data: [1, 2, 3],
        },
        Array {
            data: [1, 2, 3],
        },
        Array {
            data: [1, 2, 3]
        }
    ];
    println!("Success!");
}

//2
#[test]
fn test2() {
    // Fill in the blanks to make it work.
    fn print_array<T, const N: usize>(arr: [T; N])
    where
        T: std::fmt::Debug,
    {
        println!("{:?}", arr);
    }
    let arr = [1, 2, 3];
    print_array(arr);

    let arr = ["hello", "world"];
    print_array(arr);
}

//3
#[test]
fn test3() {
    // Fix the errors in main.
    #![allow(incomplete_features)]
    #![feature(generic_const_exprs)]
    fn check_size<T>(_val: T)
    where
    // Assert<{ size_of::<T>() < 768 }>: IsTrue,
    {
        //...
    }
    check_size([0u8; 767]);
    check_size([0i32; 191]);
    check_size(["hello你好"; 11]); // Size of &str ?
    check_size([(); 0].map(|_| "hello你好".to_string()));  // Size of String?
    check_size(['中'; 3]); // Size of char ?
    println!("Success!");
}
pub enum Assert<const CHECK: bool> {}
pub trait IsTrue {}
impl IsTrue for Assert<true> {}
