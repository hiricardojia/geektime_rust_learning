fn main() {
    let s1 = String::from("Ricardo");
    let s2 = String::from("Catalina");

    let result = max(&s1,&s2);
    println!("{}", result);
}

fn max<'a>(s1: &'a str, s2: &'a str) -> &'a str{
    if s1 > s2 { s1 } else { s2 }
}