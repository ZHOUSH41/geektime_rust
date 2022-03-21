use std::fmt;

#[derive(Debug, Clone, Default)]
struct Developer {
    name: String,
    age: u8,
    lang: Language,
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
enum Language {
    Rust,
    TypeScript,
    Elixir,
    Haskell,
}
impl Default for Language {
    fn default() -> Language {
        Language::Rust
    }
}

impl Developer {
    pub fn new(v: &str) -> Self {
        // 用 ..Default::default() 为剩余字段使用缺省值
        Self {
            name: v.to_owned(),
            ..Default::default()
        }
    }
}

impl fmt::Display for Developer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}({} years old): {:?} developer",
            self.name, self.age, self.lang
        )
    }
}

fn main() {
    // 使用 T::default()
    let dev1 = Developer::default();
    // 使用 Default::default()，但此时类型无法通过上下文推断，需要提供类型
    let dev2: Developer = Default::default();
    // 使用 T::new
    let dev3 = Developer::new("Tyr");
    println!("dev1: {}\ndev2: {}\ndev3: {:?}", dev1, dev2, dev3);

    use std::sync::{Arc, Mutex};
    let shared = Arc::new(Mutex::new(1));
    let mut g = shared.lock().unwrap();
    *g += 1;
}
