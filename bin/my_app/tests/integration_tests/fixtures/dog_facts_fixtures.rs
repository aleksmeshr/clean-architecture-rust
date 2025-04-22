use animal_facts_repositories::postgres::db_connection::DbConnection;
use animal_facts_repositories::postgres::schema::dog_facts;
use animal_facts_repositories::postgres::schema::dog_facts::dsl::*;
use diesel::{insert_into, Insertable, RunQueryDsl};
use serde::Deserialize;

use crate::utils::utils_file::read_from_file;

#[derive(Deserialize, Insertable, Debug)]
#[table_name = "dog_facts"]
pub struct DogFactDb {
    pub id: i32,
    pub fact: String,
}

pub fn execute_imports(conn: &DbConnection) {
    import_dog_facts_fixtures(conn);
}

fn import_dog_facts_fixtures(conn: &DbConnection) {
    let json = read_from_file::<Vec<DogFactDb>>("tests/integration_tests/fixtures/dog_facts.json")
        .unwrap();

    let conn = conn.get_pool().get().expect("couldn't get db connection from pool");
    insert_into(dog_facts).values(&json).execute(&conn).unwrap();
}
