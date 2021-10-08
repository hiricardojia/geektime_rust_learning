struct Dog;
struct Cat;

trait Animal {
    fn name(&self) -> &'static str;
}

impl Animal for Dog {
    fn name(&self) -> &'static str {
        "Dog!"
    }
}

impl Animal for Cat {
    fn name(&self) -> &'static str {
        "Cat!"
    }
}
// fn name<T: Animal>(animal: T) -> &'static str {}简写成下面这种形式
fn name(animal: impl Animal) -> &'static str {
    animal.name()
}

fn main() {
    let cat = Cat;
    let dog = Dog;
    println!("cat: {}", name(cat));
    println!("cat: {}", name(dog));
}