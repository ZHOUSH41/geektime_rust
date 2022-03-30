/// 对存储的抽象，我们不关心数据存在哪儿，但需要定义外界如何和存储打交道
pub trait Storage {
    /// 从一个 HashTable 里获取一个 key 的 value
    fn get(&self, table: &str, key: &str) -> Result<Option<String>, KvError>;
    /// 从一个 HashTable 里设置一个 key 的 value，返回旧的 value
    fn set(&mut self, key: &str, value: &str);
}
