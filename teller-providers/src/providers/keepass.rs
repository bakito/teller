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
    collections::{BTreeMap},
    env,
};

use async_trait::async_trait;
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


#[async_trait]
impl Provider for Keepass {
    fn kind(&self) -> ProviderInfo {
        ProviderInfo {
            kind: ProviderKind::Keepass,
            name: self.name.clone(),
        }
    }

    async fn get(&self, pm: &PathMap) -> Result<Vec<KV>> {
        let path = pm.path.split("/").ok_or_else(|| {
            Error::Message(
                "path must have initial mount seperated by '/', e.g. `secret/foo`".to_string(),
            )
        })?;
        let nr = self.db.root.get(path.collect()).ok_or_else(|| {
            Error::Message(
                "could not find entry`".to_string(),
            )
        })?;

        // source

        let mut data = BTreeMap::new();


        match nr {
            kp::db::NodeRef::Group(g) => {
                println!("Saw group '{0}'", g.name);
            }
            kp::db::NodeRef::Entry(e) => {
                data.insert("title", e.get_title().unwrap_or("(no title)"));
                data.insert("username", e.get_username().unwrap_or("(no user)"));
                data.insert("password", e.get_password().unwrap_or("(no password)"));
            }
        }

        Ok(data)
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
impl Keepass {}

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
