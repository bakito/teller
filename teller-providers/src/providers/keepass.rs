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
        let mut db_path: String = String::new();
        let mut password: String = String::new();
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
        let path: &[&str] = &pm.path.split('/').collect::<Vec<&str>>();

        let nr = self.db.root.get(path).ok_or_else(|| {
            Error::Message(
                "could not find entry`".to_string(),
            )
        })?;

        let mut data: BTreeMap<String, String> = BTreeMap::new();

        match nr {
            kp::db::NodeRef::Group(g) => {
                println!("Saw group '{0}'", g.name);
            }
            kp::db::NodeRef::Entry(e) => {
                data.insert("title".to_string(), e.get_title().unwrap_or("(no title)").to_string());
                data.insert("username".to_string(), e.get_username().unwrap_or("(no user)").to_string());
                data.insert("password".to_string(), e.get_password().unwrap_or("(no password)").to_string());
            }
        }
        Ok(KV::from_data(&data, pm, &self.kind()))
    }

    async fn put(&self, _: &PathMap, _: &[KV]) -> Result<()> {
        Ok(())
    }

    async fn del(&self, _: &PathMap) -> Result<()> {
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
            Keepass::new("keepass", Some(serde_json::from_value(opts).unwrap())).unwrap(),
        ) as Box<dyn Provider + Send + Sync>;

        test_utils::ProviderTest::new(p)
            .with_root_prefix("tmp/keepass/")
            .run()
            .await;
    }
}
