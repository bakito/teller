//! `keepass` Provider
//!
//!
//! ## Example configuration
//!
//! ```yaml
//! providers:
//!  keepassv1:
//!    kind: keepass
//!    # options: ...
//! ```
//! ## Options
//!
//! See [`KeepassOptions`]
//!
//!
#![allow(clippy::borrowed_box)]
use std::fs::File;
use std::io::prelude::*;
use std::{
    collections::{BTreeMap, HashMap},
    env,
    path::Path,
};

use async_trait::async_trait;
use fs_err as fs;
use serde_derive::{Deserialize, Serialize};
use super::ProviderKind;
use crate::config::ProviderInfo;
use crate::{
    config::{PathMap, KV},
    Error, Provider, Result,
};
use keepass::{
    self,
};

use keepass as kp;

#[derive(PartialEq)]
enum Mode {
    Get,
    Put,
    Del,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct KeepassOptions {
    /// Keepass database file path
    pub db_path: Option<String>,
    /// Keepass password
    pub password: Option<String>,
}

pub struct Keepass {
    pub name: String,
    pub db: kp::Database,
}
impl Keepass {
    /// Create a new provider
    ///
    /// # Errors
    ///
    /// This function will return an error if cannot create a provider
    pub fn new(name: &str, opts: Option<KeepassOptions>) -> Result<Self> {
        let mut db_path: String;
        let mut password: String;
        if let Some(opts) = opts {
            if let Some(path) = opts.db_path {
                db_path = path;
            }

            if let Some(pw) = opts.password {
                password = pw;
            }
        } else {
            db_path = env::var("KEEPASS_DB_PATH")?;
            password = env::var("KEEPASS_PASSWORD")?;
        };
        let mut file = File::open(&db_path)?;
        let key = kp::DatabaseKey::new().with_password(&password);
        let db = kp::Database::open(&mut file, key).map_err(Box::from)?;

        Ok(Self {
            db,
            name: name.to_string(),
        })
    }
}

fn load(path: &Path, mode: &Mode) -> Result<BTreeMap<String, String>> {
    let content = fs::File::open(path)?;
    let mut env = BTreeMap::new();

    if mode == &Mode::Get {
        let metadata = content.metadata().map_err(|e| Error::GetError {
            path: format!("{path:?}"),
            msg: format!("could not get file metadata. err: {e:?}"),
        })?;

        if metadata.len() == 0 {
            return Err(Error::NotFound {
                path: format!("{path:?}"),
                msg: "file is empty".to_string(),
            });
        }
    }

    Ok(env)
}
// poor man's serialization, loses original comments and formatting
fn save(path: &Path, data: &BTreeMap<String, String>) -> Result<String> {
    let mut out = String::new();
    for (k, v) in data {
        let maybe_json: serde_json::Result<HashMap<String, serde_json::Value>> =
            serde_json::from_str(v);

        let json_value = if maybe_json.is_ok() {
            serde_json::to_string(&v).map(Some).unwrap_or_default()
        } else {
            None
        };

        let value = json_value.unwrap_or_else(|| v.to_string());
        if value.chars().any(char::is_whitespace) && !value.starts_with(['"', '\'']) {
            out.push_str(&format!("{k}=\"{value}\"\n"));
        } else {
            out.push_str(&format!("{k}={value}\n"));
        }
    }

    fs::write(path, &out)?;
    Ok(out)
}

#[async_trait]
impl Provider for Keepass {
    fn kind(&self) -> ProviderInfo {
        ProviderInfo {
            kind: ProviderKind::Keepass,
            name: self.name.clone(),
        }
    }

    async fn get(&self, pm: &PathMap) -> Result<Vec<KV>> {
        let data = load(Path::new(&pm.path), &Mode::Get)?;
        Ok(KV::from_data(&data, pm, &self.kind()))
    }

    async fn put(&self, pm: &PathMap, kvs: &[KV]) -> Result<()> {
        // Create file if not exists + add the option to set is as false
        self.load_modify_save(
            pm,
            |data| {
                for kv in kvs {
                    data.insert(kv.key.to_string(), kv.value.to_string());
                }
            },
            &Mode::Put,
        )?;
        Ok(())
    }

    async fn del(&self, pm: &PathMap) -> Result<()> {
        self.load_modify_save(
            pm,
            |data| {
                if pm.keys.is_empty() {
                    data.clear();
                } else {
                    for k in pm.keys.keys() {
                        if data.contains_key(k) {
                            data.remove(k);
                        }
                    }
                }
            },
            &Mode::Del,
        )?;
        Ok(())
    }
}
impl Keepass {

}

#[cfg(test)]
mod tests {
    use tokio::test;

    use super::*;
    use crate::providers::test_utils;

    #[test]
    async fn sanity_test() {
        let opts = serde_json::json!({
            "create_on_put": true,
        });

        let p: Box<dyn Provider + Send + Sync> = Box::new(
            super::Keepass::new("keepass", Some(serde_json::from_value(opts).unwrap())).unwrap(),
        ) as Box<dyn Provider + Send + Sync>;

        test_utils::ProviderTest::new(p)
            .with_root_prefix("tmp/keepass/")
            .run()
            .await;
    }
}
