mod db;
pub mod schema {
    pub mod query;
}

pub use db::Database;
pub use schema::*;
