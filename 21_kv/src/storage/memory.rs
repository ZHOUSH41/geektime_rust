use crate::{KvError, Kvpair, Storage, Value};
use dashmap::{mapref::one::Ref, DashMap};

/// 使用 DashMap 构建的 MemTable，实现了 Storage trait
#[derive(Clone, Debug, Default)]
pub struct MemTable {
    tables: DashMap<String, DashMap<String, Value>>,
}

impl MemTable {
    /// 创建一个缺省 MemTable
    pub fn new() -> Self {
        Self::default()
    }

    /// 如果名为 name 的 hash table 不存在，则创建，否则返回
    pub fn get_or_create_table(&self, name: &str) -> Ref<DashMap<String, Value>> {
        match self.tables.get(name) {
            Some(table) => table,
            None => {
                let entry = self.tables.entry(name.into()).or_default();
                entry.downgrade()
            }
        }
    }
}

impl Storage for MemTable {
    fn get(&self, table: &str, key: &str) -> Result<Value, KvError> {
        let table = self.get_or_create_table(table);
        match table.get(key) {
            Some(value) => Ok(value.clone()),
            None => Err(KvError::NotFound(table.to_string(), key.to_string())),
        }
    }

    fn set(&mut self, table: &str, key: &str, value: Value) -> Result<(), KvError> {
        let table = self.get_or_create_table(table);
        table.insert(key.to_string(), value);
        Ok(())
    }

    fn contains(&mut self, table: &str, key: &str) -> Result<(), KvError> {
        let table = self.get_or_create_table(table);
        table.remove(key);
        Ok(())
    }

    fn del(&mut self, table: &str, key: &str) -> Result<(), KvError> {
        let table = self.get_or_create_table(table);
        table.remove(key);
        Ok(())
    }

    fn get_all(&self, table: &str) -> Result<Vec<Kvpair>, KvError> {
        let table = self.get_or_create_table(table);
        let mut pairs = Vec::new();
        for (key, value) in table.iter() {
            pairs.push(Kvpair::new(key.to_string(), value.clone()));
        }
        Ok(pairs)
    }

    fn get_iter(&self, table: &str) -> Result<Box<dyn Iterator<Item = Kvpair>>, KvError> {
        todo!()
    }
}
