// src/dal.rs

use crate::db::{DbPool, DbConn};
use crate::traits::{Create, List, Update, Delete, Retrieve};
use diesel::result::QueryResult;

pub struct DataAccessLayer {
    pool: DbPool,
}

impl DataAccessLayer {
    pub fn new(pool: DbPool) -> Self {
        DataAccessLayer { pool }
    }

    pub fn create<T, U>(&self, item: U) -> QueryResult<T>
    where
        T: Create<U>,
    {
        let mut conn = self.get_conn();
        T::create(&mut conn, item)
    }
    
    pub fn update<T, U>(&self, id: i32, item: U) -> QueryResult<T>
    where
        T: Update<U>,
    {
        let mut conn = self.get_conn();
        T::update(&mut conn, id, item)
    }

    pub fn list<T>(&self) -> QueryResult<Vec<T>>
    where
        T: List<T>,
    {
        let mut conn = self.get_conn();
        T::list(&mut conn)
    }


    pub fn delete<T>(&self, id: i32) -> QueryResult<usize>
    where
        T: Delete,
    {
        let mut conn = self.get_conn();
        T::delete(&mut conn, id)
    }

    pub fn retrieve<T>(&self, id: i32) -> QueryResult<T>
    where
        T: Retrieve<T>,
    {
        let mut conn = self.get_conn();
        T::retrieve(&mut conn, id)
    }
    
    
    fn get_conn(&self) -> DbConn {
        self.pool.get().expect("Failed to get db connection from pool")
    }
}
