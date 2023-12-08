use std::ops::Deref;
use surrealdb::{
    engine::local::{Db, Mem, TiKv},
    Surreal,
};

pub struct Connection {
    client: Surreal<Db>,
}

enum ConnectionType {
    Memory,
    TiKv(String),
}

pub struct SurrealdbConnectionManager {
    connection_type: ConnectionType,
}

impl SurrealdbConnectionManager {
    pub fn memory() -> Self {
        Self {
            connection_type: ConnectionType::Memory,
        }
    }

    pub fn tikv(url: String) -> Self {
        Self {
            connection_type: ConnectionType::TiKv(url),
        }
    }
}

#[async_trait::async_trait]
impl bb8::ManageConnection for SurrealdbConnectionManager {
    type Connection = Connection;
    type Error = surrealdb::Error;

    async fn connect(&self) -> Result<Self::Connection, Self::Error> {
        let db = match &self.connection_type {
            ConnectionType::Memory => {
                let client = Surreal::new::<Mem>(()).await?;
                Connection { client }
            }
            ConnectionType::TiKv(url) => {
                let client = Surreal::new::<TiKv>(url).await?;
                Connection { client }
            }
        };

        Ok(db)
    }

    async fn is_valid(&self, conn: &mut Self::Connection) -> Result<(), Self::Error> {
        conn.query("SELECT * FROM 1;").await?;
        Ok(())
    }

    fn has_broken(&self, _: &mut Self::Connection) -> bool {
        false
    }
}

impl Deref for Connection {
    type Target = Surreal<Db>;

    fn deref(&self) -> &Self::Target {
        &self.client
    }
}

// impl Deref for Connection {
//     type Target = SurrealdbConnection;

//     fn deref(&self) -> &Self::Target {
//         &self.client
//     }
// }
