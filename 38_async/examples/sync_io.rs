use std::fs;

use anyhow::Result;
use serde_yaml::Value;
fn main() -> Result<()> {
    //
    let content1 = fs::read_to_string("./Cargo.toml")?;
    //
    let content2 = fs::read_to_string("./Cargo.lock")?;

    let yaml1 = toml2yaml(&content1)?;
    let yaml2 = toml2yaml(&content2)?;

    fs::write("/tmp/Cargo.yaml", &yaml1)?;
    fs::write("/tmp/Cargo.lock.yaml", &yaml2)?;

    println!("{}", yaml1);
    println!("{}", yaml2);

    Ok(())
}

fn toml2yaml(content: &str) -> Result<String> {
    let value: Value = toml::from_str(content)?;
    Ok(serde_yaml::to_string(&value)?)
}
