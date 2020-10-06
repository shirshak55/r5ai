use crate::error::Errors;
use crate::error::Errors::InvalidData;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Request {
    pub query_string: Map,
    pub post_body: Map,
}

impl Request {
    pub fn new(query_string: Map, post_body: Map) -> Self {
        Self {
            query_string,
            post_body,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Map {
    inner: HashMap<String, serde_json::Value>,
}

impl From<HashMap<String, serde_json::Value>> for Map {
    fn from(hmap: HashMap<String, serde_json::Value>) -> Map {
        Self { inner: hmap }
    }
}

impl Map {
    pub fn get_action_name(&self) -> Result<&str, Errors> {
        let vv = match self.inner.get("action") {
            Some(serde_json::Value::String(ss)) => ss,
            _ => return Err(InvalidData),
        };
        Ok(vv)
    }

    pub fn get_string(&self, key: &str) -> Result<&str, Errors> {
        let query = self.inner.get(key).map(|v| v.as_str());

        let ret = query.ok_or(InvalidData).map(|v| v.ok_or(InvalidData))??;

        Ok(ret)
    }

    pub fn get_vec_string(&self, key: &str) -> Result<Vec<&str>, Errors> {
        let query = self.inner.get(key).map(|v| v.as_array());

        let result = query.ok_or(InvalidData).map(|v| v.ok_or(InvalidData))??;

        let ret = result
            .iter()
            .map(|v| v.as_str().ok_or(InvalidData))
            .collect::<Result<Vec<_>, Errors>>()?;
        Ok(ret)
    }

    pub fn get_bool(&self, key: &str) -> Result<bool, Errors> {
        let query = self.inner.get(key).map(|v| v.as_bool());

        let ret = query.ok_or(InvalidData).map(|v| v.ok_or(InvalidData))??;

        Ok(ret)
    }
}
