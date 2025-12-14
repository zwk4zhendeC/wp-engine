use crate::{WplRule, WplStatementType, ast::AnnFun};
use derive_getters::Getters;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[derive(Serialize, Deserialize)]
pub struct WplRuleMeta {
    pub name: String,
    pub tags: Vec<WplTag>,
}

impl From<&WplRule> for WplRuleMeta {
    fn from(value: &WplRule) -> Self {
        let mut tags = Vec::new();
        match &value.statement {
            WplStatementType::Express(x) => {
                Self::export_tags(&mut tags, &x.tags);
            }
        }
        Self {
            name: value.name.clone(),
            tags,
        }
    }
}

impl WplRuleMeta {
    fn export_tags(tags: &mut Vec<WplTag>, x: &Option<AnnFun>) {
        if let Some(tag_obj) = x {
            for (k, v) in &tag_obj.tags {
                tags.push(WplTag::new(k.clone(), v.clone()))
            }
        }
    }
}

#[derive(Default, Getters, Debug, Clone)]
pub struct WplTag {
    pub key: String,
    pub val: String,
}
impl WplTag {
    pub fn new(key: String, val: String) -> Self {
        Self { key, val }
    }
}

impl Serialize for WplTag {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(format!("{}:{}", self.key.as_str(), self.val.as_str()).as_str())
    }
}

impl<'de> Deserialize<'de> for WplTag {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let parts: Vec<&str> = s.split(':').collect();
        if parts.len() == 2 {
            Ok(WplTag {
                key: parts[0].to_string(),
                val: parts[1].to_string(),
            })
        } else {
            Err(serde::de::Error::custom("Invalid format"))
        }
    }
}
