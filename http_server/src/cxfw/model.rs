use serde::{Serialize, Deserialize};
use rusqlite;

#[derive(Serialize, Deserialize)]
pub struct Users {
    id: Option<i64>,
    name: Option<String>,
    data: Option<String>
}

impl Users {
    pub fn new() -> Self {
        Users {
            id: None,
            name: None,
            data: None
        }
    }

    pub fn from(row: &rusqlite::Row) -> Self {
        Users {
            id: match row.get(0) {
                Ok(id) => Some(id),
                Err(_) => None,
            },
            name: match row.get(1) {
                Ok(name) => Some(name),
                Err(_) => None,
            },
            data: match row.get(2) {
                Ok(data) => data,
                Err(_) => None,
            }
        }
    }

    pub fn id(&self) -> i64 {
        self.id.unwrap_or(0)
    }

    pub fn name_ref(&self) -> &str {
        match &self.name {
            Some(name) => name,
            None => ""
        }
    }

    pub fn data_ref(&self) -> &str {
        match &self.data {
            Some(data) => data,
            None => ""
        }
    }
}
