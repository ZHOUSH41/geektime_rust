use std::{
    collections::{hash_map::DefaultHasher, HashMap},
    hash::{Hash, Hasher},
};

#[derive(Debug, Hash, PartialEq, Eq)]
struct Student<'a> {
    name: &'a str,
    age: u8,
}

impl<'a> Student<'a> {
    fn new(name: &'a str, age: u8) -> Self {
        Student { name, age }
    }
}

fn main() {
    let mut hasher = DefaultHasher::new();
    let student = Student::new("Tom", 18);

    student.hash(&mut hasher);
    let mut map = HashMap::new();
    map.insert(student, vec!["Math", "Writing"]);
    println!("hash: 0x{:x}, map: {:?}", hasher.finish(), map);
}
