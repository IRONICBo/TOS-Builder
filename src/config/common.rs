use serde::{Serialize, Deserialize};

/// Common struct
#[derive(Serialize, Deserialize, Debug)]
pub struct StringValue {
    pub key: String,
    pub value: String,
    pub comment: String,
}

impl StringValue {
    pub fn new(key:String, value: String, comment: String) -> Self {
        Self {
            key,
            value,
            comment,
        }
    }

    pub fn to_vec(&self) -> Vec<String> {
        vec![self.key.clone(), self.value.clone(), self.comment.clone()]
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BoolValue {
    pub key: String,
    pub value: bool,
    pub comment: String,
}

impl BoolValue {
    pub fn new(key: String, value: bool, comment: String) -> Self {
        Self {
            key,
            value,
            comment,
        }
    }

    pub fn to_vec(&self) -> Vec<String> {
        vec![self.key.clone(), self.value.to_string().clone(), self.comment.clone()]
    }
}
