use crate::schema::books;
use diesel::prelude::*;
use serde::{Serialize, Deserialize};
use crate::traits::{Create, List, Update, Delete, Retrieve};

#[derive(Queryable, Serialize, Deserialize)]
pub struct Book {
    pub id: i32,
    pub title: String,
    pub summary: String,
    pub key: String,
}

#[derive(Insertable, Serialize, Deserialize, Clone)]
#[diesel(table_name = books)]
pub struct NewBook {
  pub title: String,
  pub summary: String,
  pub key:  String,
}

// TODO: see to use timelines  
// pub struct NewBook<'a> {
//     pub title: &'a str,
//     pub summary: &'a str,
//     pub key: &'a str,
// }

#[derive(AsChangeset, Deserialize)]
#[diesel(table_name = books)]
pub struct UpdateBook {
    pub title: Option<String>,
    pub summary: Option<String>,
    pub key: Option<String>,
}

// Implementations 

impl Create<NewBook> for Book {
  fn create(conn: &mut PgConnection, new_book: NewBook) -> QueryResult<Self> {
    diesel::insert_into(books::table)
        .values(&new_book)
        .get_result(conn)
  }
}

impl List<Book> for Book {
  fn list(conn: &mut PgConnection) -> QueryResult<Vec<Self>> {
      books::table.load::<Book>(conn)
  }
}

impl Update<UpdateBook> for Book {
  fn update(conn: &mut PgConnection, id: i32, update_book: UpdateBook) -> QueryResult<Self> {
      diesel::update(books::table.find(id))
          .set(update_book)
          .get_result(conn)
  }
}

impl Delete for Book {
  fn delete(conn: &mut PgConnection, id: i32) -> QueryResult<usize> {
      diesel::delete(books::table.find(id)).execute(conn)
  }
}

impl Retrieve<Book> for Book {
  fn retrieve(conn: &mut PgConnection, id: i32) -> QueryResult<Book> {
      books::table.find(id).get_result::<Book>(conn)
  }
}