// 1 завдання
#[test]
fn test1() {
    let arr: [u8; 3] = [1, 2, 3];


    let v = Vec::from(arr);
    is_vec(&v);


    let v = vec![1, 2, 3];
    is_vec(&v);


    // vec!(..) and vec![..] are same macros, so
    let v = vec!(1, 2, 3);
    is_vec(&v);


    // in code below, v is Vec<[u8; 3]> , not Vec<u8>
    // USE `for` to rewrite the below code
    let mut v1 = Vec::new();
    for i in &v {
        v1.push(*i)
    }
    is_vec(&v1);


    assert_eq!(format!("{:?}",v), format!("{:?}",v1));


    println!("Success!")
}


fn is_vec(v: &Vec<u8>) {}




//2 завдання
// FILL in the blank
#[test]
fn test2() {
    let mut v1: Vec<i32> = Vec::from([1, 2, 4]);
    v1.pop(); //[1,2]
    v1.push(3); //[1,2,3]


    let mut v2: Vec<i32>  = Vec::new();
    v2.extend(&v1);


    assert_eq!(v1, v2);


    println!("Success!");
}


//3
// FILL in the blanks
#[test]
fn test3() {
    // Array -> Vec
    // impl From<[T; N]> for Vec
    let arr: [i32; 3] = [1, 2, 3];
    let v1: Vec<i32> = Vec::from(arr);
    let v2: Vec<i32> = arr.into();


    assert_eq!(v1, v2);




    // String -> Vec
    // impl From<String> for Vec
    let s: String = "hello".to_string(); //Vec<u8>
    let v1: Vec<u8> = s.into_bytes();


    let s: String = "hello".to_string();
    let v2: Vec<u8> = s.into_bytes();
    assert_eq!(v1, v2);


    // impl<'_> From<&'_ str> for Vec
    let s: &str = "hello";
    let v3: Vec<u8> = Vec::from(s);
    assert_eq!(v2, v3);


    // Iterators can be collected into vectors
    let v4: Vec<i32> = [0; 10].into_iter().collect();
    assert_eq!(v4, vec![0; 10]); //[0,0,0, ...,0]


    println!("Success!");
}


//4
// FIX the error and IMPLEMENT the code
#[test]
fn test4() {
    let mut v = Vec::from([1, 2, 3]);


    for i in 0..5 {
        println!("{:?}", v.get(i)) //Option<i32>
    }


    for i in 0..5 {
        match v.get(i) {
            Some(e) => v[i] = e + 1,
            None => v.push(i + 2)
        }
    }


    assert_eq!(v, vec![2, 3, 4, 5, 6]);


    println!("Success!");
}


//5
// FIX the errors
#[test]
fn test5() {
    let mut v: Vec<i32> = vec![1, 2, 3];


    let slice1: &[i32] = &v[..];
    // Out of bounds will cause a panic
    // You must use `v.len` here
    let slice2: &[i32] = &v[0..v.len()];


    assert_eq!(slice1, slice2);


    // Slices are read only
    // Note: slice and &Vec are different
    let vec_ref: &mut Vec<i32> = &mut v;
    (*vec_ref).push(4);
    let slice3: &[i32] = &v[0..];


    assert_eq!(slice3, &[1, 2, 3, 4]);


    println!("Success!");
}


//6
// FIX the errors
#[test]
fn test6() {
    let mut vec = Vec::with_capacity(10);


    // The vector contains no items, even though it has capacity for more
    assert_eq!(vec.len(), 0);
    assert_eq!(vec.capacity(), 10);


    // These are all done without reallocating...
    for i in 0..10 {
        vec.push(i);
    }
    assert_eq!(vec.len(), 10);
    assert_eq!(vec.capacity(), 10);


    // ...but this may make the vector reallocate
    vec.push(11);
    assert_eq!(vec.len(), 11);
    assert!(vec.capacity() >= 11);




    // Fill in an appropriate value to make the `for` done without reallocating
    let mut vec = Vec::with_capacity(100);
    for i in 0..100 {
        vec.push(i);
    }


    assert_eq!(vec.len(), 100);
    assert_eq!(vec.capacity(), 100);


    println!("Success!");
}


//7
#[derive(Debug, PartialEq)]
enum IpAddr {
    V4(String),
    V6(String),
}
#[test]
fn test7() {
    // FILL in the blank
    let v: Vec<IpAddr> = vec![
        IpAddr::V4("127.0.0.1".to_string()),
        IpAddr::V6("::1".to_string()),
    ];


    // Comparing two enums need to derive the PartialEq trait
    assert_eq!(v[0], IpAddr::V4("127.0.0.1".to_string()));
    assert_eq!(v[1], IpAddr::V6("::1".to_string()));


    println!("Success!")
}


//8
trait IpAddr1{
    fn display(&self);
}


struct V4(String);
impl IpAddr1 for V4 {
    fn display(&self) {
        println!("ipv4: {:?}",self.0)
    }
}
struct V6(String);
impl IpAddr1 for V6 {
    fn display(&self) {
        println!("ipv6: {:?}",self.0)
    }
}
#[test]
fn test8() {
    let v: Vec<Box<dyn IpAddr1>> = vec![
        Box::new(V4("127.0.0.1".to_string())),
        Box::new(V6("::1".to_string())),
    ];


    for ip in v {
        ip.display();
    }
}
