//1
// Remove something to make it work
#[test]
fn main1() {
    let x: i32 = 5;
    let mut y;
    y = x;
    let z: i32 = 10; // Type of z ?
    println!("Success!");
}
//2
#[test]
// Fill the blank
fn main2() {
    let v: u16 = 38_u8 as u16;

    println!("Success!");
}
//3
#[test]
// Modify `assert_eq!` to make it work
fn main3() {
    let x: u32 = 5;
    assert_eq!("u32".to_string(), type_of(&x));

    println!("Success!");
}
// Get the type of given variable, return a string representation of the type  , e.g "i8", "u8", "i32", "u32"
fn type_of<T>(_: &T) -> String {
    format!("{}", std::any::type_name::<T>())
}
//4
#[test]
// Fill the blanks to make it work
fn main4() {
    assert_eq!(i8::MAX, 127);
    assert_eq!(u8::MAX, 255);
    println!("Success!");
}
//5
#[test]
fn main5() {
    let v1 = 251_u8.wrapping_add(8); // обгортаємо результат при переповненні
    let v2 = i8::checked_add(120, 8).unwrap_or(-1); // значення за замовчуванням у разі переповнення
    println!("{},{}", v1, v2);
}
//6
#[test]
// Modify `assert!` to make it work
fn main6() {
    let v = 1_024 + 0xff + 0o77 + 0b1111_1111;
    assert!(v == 1597);
    println!("Success!");
}
//7


//8