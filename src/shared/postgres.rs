use diesel::pg::PgConnection;
use diesel::prelude::*;

pub fn establish_connection(database_url: String) -> PgConnection {
    PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}
