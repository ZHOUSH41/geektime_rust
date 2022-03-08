struct Cat;
struct Dog;

trait Animal {
    fn name(&self) -> &'static str;
    fn name1() -> &'static str;
}

impl Animal for Cat {
    fn name(&self) -> &'static str {
        "Cat"
    }

    fn name1() -> &'static str {
        "Cat"
    }
}

impl Animal for Dog {
    fn name(&self) -> &'static str {
        "Dog"
    }

    fn name1() -> &'static str {
        "Dog"
    }
}

fn name<T: Animal>(animal: T) -> &'static str {
    animal.name()
}
fn main() {
    let cat = Cat;
    println!("cat: {}", name(cat));
    println!("cat: {}", Cat::name1());
}
