use std::{collections::HashMap, fs::File, path::Path};

use anyhow::{anyhow, Ok};
use serde_yaml::{Mapping, Value};

pub struct Environment {
    mapping: Option<Mapping>,
    properties: HashMap<String, String>,
}

impl Environment {
    pub fn get_u16<T: AsRef<str>>(&self, key: T) -> Option<anyhow::Result<u16>> {
        let key = key.as_ref();
        if let Some(data) = self.properties.get(key) {
            if data.is_empty() {
                return None;
            }
            let v = data
                .parse()
                .map_err(|e| anyhow!("Unable to convert {data} to u16.key={key},msg{e}"));
            return Some(v);
        }
        None
    }

    pub fn get_i64<T: AsRef<str>>(&self, key: T) -> Option<anyhow::Result<i64>> {
        let key = key.as_ref();
        if let Some(data) = self.properties.get(key) {
            if data.is_empty() {
                return None;
            }
            let v = data
                .parse()
                .map_err(|e| anyhow!("Unable to convert {data} to i64.key={key}.error={e}"));
            return Some(v);
        }
        None
    }

    pub fn get_string<T: AsRef<str>>(&self, key: T) -> Option<String> {
        let key = key.as_ref();
        self.properties.get(key).map(|data| data.clone())
    }

    pub fn get_bool<T: AsRef<str>>(&self, key: T) -> Option<anyhow::Result<bool>> {
        let key = key.as_ref();
        self.properties
            .get(key)
            .map(|data| match data.to_lowercase().as_str() {
                "true" => Ok(true),
                "false" => Ok(false),
                v => Err(anyhow!("cannot convert {} to bool", v)),
            })
    }

    pub fn get_array<T: AsRef<str>>(&self, key: T) -> Vec<String> {
        let key = key.as_ref();
        let mut result = Vec::new();
        for i in 0.. {
            let index = format!("{}.[{}]", key, i);
            if let Some(data) = self.get_string(index.as_str()) {
                result.push(data);
            } else {
                break;
            }
        }
        result
    }
}

impl Environment {
    pub fn new<P: AsRef<Path>>(path: P) -> anyhow::Result<Environment> {
        let file = File::open(path)?;
        let source: Mapping = serde_yaml::from_reader(file)
            .map_err(|e| e.to_string())
            .unwrap();
        let mut env = Environment {
            properties: HashMap::new(),
            mapping: None,
        };
        env.refresh(source);
        Ok(env)
    }

    fn refresh(&mut self, source: Mapping) {
        self.mapping = Some(source.clone());
        self.properties = self.get_flattened_map(source.clone());
    }

    fn get_flattened_map(&self, source: Mapping) -> HashMap<String, String> {
        let mut result = HashMap::new();
        self.build_flattened_map(&mut result, &source, None);
        result
    }

    /// 将Yaml类型的数据展开平铺
    fn build_flattened_map(
        &self,
        result: &mut HashMap<String, String>,
        source: &Mapping,
        path: Option<&String>,
    ) {
        for k in source.keys() {
            let mut key = String::from(k.as_str().unwrap());
            let o = source.get(&key);
            if o.is_none() {
                return;
            }
            let value = o.unwrap();
            if path.is_some() {
                let mut path = path.cloned().unwrap();
                if key.starts_with("[") {
                    key.insert_str(0, &path)
                } else {
                    path.push_str(".");
                    key.insert_str(0, &path)
                }
            }
            if value.is_string() {
                let str = value.as_str().unwrap();
                result.insert(key, String::from(str));
            } else if value.is_mapping() {
                let mapping = value.as_mapping().unwrap();
                self.build_flattened_map(result, mapping, Some(&key))
            } else if value.is_sequence() {
                let connection = value.as_sequence();
                if connection.is_none() {
                    result.insert(key, String::from(""));
                    continue;
                }
                let arr = connection.unwrap();
                if arr.len() == 0 {
                    result.insert(key, String::from(""));
                    continue;
                }
                let mut count = 0;
                for value in arr {
                    let mut m = Mapping::new();
                    m.insert(Value::from(format!("[{}]", count)), value.clone());
                    self.build_flattened_map(result, &m, Some(&key));
                    count += 1;
                }
            } else if value.is_bool() {
                let b = value.as_bool().unwrap();
                result.insert(key, String::from(if b { "true" } else { "false" }));
            } else {
                result.insert(key, String::from(""));
            }
        }
    }
}
