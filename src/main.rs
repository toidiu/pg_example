extern crate postgres;
extern crate postgres_range;
extern crate pg_example;


use pg_example::{
	db, 
	errors::MyError,
	log::create_logger
};

fn main() -> Result<(), MyError> {
    let logger = create_logger();

    // Note:  the testing database requires pg admin privileges to install
    // a btree_gist extension (used for tstzrange constraint)
    let db_url = "postgres://test_user:testing@127.0.0.1:5432/testing_db";
    let pool = db::Pool::from_url(&logger, db_url)?;

    db::seed_db(&logger, &pool)?;

    Ok(())
}
