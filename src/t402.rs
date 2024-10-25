//1
// Make it work
//#[test]
//use std::mem::size_of_val;
fn test1() {
    let c1: char = 'a';//4 байта
    assert_eq!(size_of_val(&c1),4);

    let c2: char = '中';
    assert_eq!(size_of_val(&c2),4);

    println!("Success!");
}
//2
// Make it work
#[test]
fn test2() {
    let c1: char = '中';
    print_char(c1);
}
fn print_char(c : char) {
    println!("{}", c);
}
//3
// Make println! work
#[test]
fn test3() {
    let _f: bool = false;

    let t: bool = true;
    if t {
        println!("Success!");
    }
}
//4
// Make it work
#[test]
fn test4() {
    let f:bool = false;
    let t:bool = true && false;//false
    assert_eq!(t, f);

    println!("Success!");
}
//5
// Make it work, don't modify `implicitly_ret_unit` !
#[test]
fn test5() {
    let _v: () = ();

    let v:(i32, i32) = (2, 3);
    assert_eq!(_v, implicitly_ret_unit());

    println!("Success!");
}

fn implicitly_ret_unit() {
    println!("I will return a ()");
}

// Don't use this one
fn explicitly_ret_unit() -> () {
    println!("I will return a ()");
}
//6
// Modify `4` in assert to make it work
#[test]
//use std::mem::size_of_val;
fn test6() {
    let unit: () = ();
    assert!(size_of_val(&unit) == 0);

    println!("Success!");
}

