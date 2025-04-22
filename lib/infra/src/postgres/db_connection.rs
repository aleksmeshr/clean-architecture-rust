use diesel::{pg::PgConnection, r2d2::ConnectionManager};

type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub struct DbConnection {
    pub database_host: String,
    pub db_name: String,
}

impl DbConnection {
    pub fn get_pool(&self) -> DbPool {
        let database_url = format!("{}/{}", &self.database_host, &self.db_name);
        let manager = ConnectionManager::<PgConnection>::new(&database_url);
        r2d2::Pool::new(manager).unwrap()
    }
}
