fn main() {
    //let r = other_func();
    //println!("&r {:p}", r);

    let a = 11;
    let b = &a;
    println!("{}",b);
    println!("{:p}",&b);
    println!("{}",a);
    println!("{:p}",&a);
}

// fn other_func<'a>() -> &'a i32 {
//     let a = 42;
//     &a
// }