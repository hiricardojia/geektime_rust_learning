fn main() {
    let mut v: Vec<&i32> = Vec::new();
    let a = 42;
    v.push(&a);
    println!("a: {:p}", &a);
    println!("data: {:p}", &v[0]);
}