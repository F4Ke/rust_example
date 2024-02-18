// src/traits.rs

use diesel::prelude::*;

pub trait Create<T> {
    fn create(conn: &mut PgConnection, item: T) -> QueryResult<Self>
    where
        Self: Sized;
}

pub trait List<T> {
    fn list(conn: &mut PgConnection) -> QueryResult<Vec<T>>;
}

pub trait Update<T> {
    fn update(conn: &mut PgConnection, id: i32, item: T) -> QueryResult<Self>
    where
        Self: Sized;
}
pub trait Delete {
    fn delete(conn: &mut PgConnection, id: i32) -> QueryResult<usize>;
}

pub trait Retrieve<T> {
    fn retrieve(conn: &mut PgConnection, id: i32) -> QueryResult<T>;
}