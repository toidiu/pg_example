/*
This module creates a database from scratch, beginning with a new db 
schema, 'testing', followed by seeding tables in the new schema with fake data.
*/
use chrono::prelude::*;
use postgres::transaction::Transaction;
use postgres_range::Range;
use r2d2::{Pool as PgPool, PooledConnection};
use r2d2_postgres::{PostgresConnectionManager, TlsMode};
use slog::Logger;

use errors::{DBError, MyError};
use models::{Building, Room, User};

pub type PgConnection = PooledConnection<PostgresConnectionManager>;
pub type TSTZRange = Range<DateTime<Utc>>;

pub struct Pool {
    pub inner: PgPool<PostgresConnectionManager>,
}
impl Pool {
    pub fn get_conn(&self, logger: &Logger) -> Result<PgConnection, MyError> {
        self.inner.get().map_err(|err| {
            error!(logger, "Failed to create connection";
						"step"=>"get_conn", "err"=>err.to_string());
            MyError::DBError(DBError::PoolError(err))
        })
    }

    pub fn get_tx<'t>(&self,
                      conn: &'t PgConnection,
                      logger: &Logger)
                      -> Result<Transaction<'t>, MyError> {
        conn.transaction().map_err(|err| {
            error!(logger, "Failed to create transaction";
						"step"=>"get_tx", "err"=>err.to_string());
            MyError::DBError(DBError::PGError(err))
        })
    }

    pub fn from_url(logger: &Logger, db_url: &str) -> Result<Pool, MyError> {
        //db_url = "postgres://{user_id}:{password}@{host}:{port}/{db}"

        PostgresConnectionManager::new(db_url, TlsMode::None).map_err(|err| {
            error!(logger, "Failed to create pg manager";
				"step"=>"create_pool", "err"=>err.to_string());
            MyError::DBError(DBError::PGError(err))
        })
        .and_then(|manager| {
                      PgPool::new(manager)
                    .map_err(|err| {
                        error!(logger, "Failed to create pg manager"; "err"=>err.to_string());
                        MyError::DBError(DBError::PoolError(err))
                    }).and_then(|pool| Ok(Pool { inner: pool }))
                  })
    }
}

pub fn seed_db(logger: &Logger, pool: &Pool) -> Result<(), MyError> {
    // Step 1:  create 20 users, 10 buildings, and meeting rooms

    // create 20 users
    let conn = pool.get_conn(&logger)?;
    let tx = pool.get_tx(&conn, &logger)?;

    for _ in 1..20 {
        let first_name = fake!(Name.first_name).to_lowercase();
        let last_name = fake!(Name.last_name).to_lowercase();
        let username = format!("{}_{}", &first_name, &last_name);

        User::add_user(first_name, last_name, username, logger, &tx)?;
    }

    for x in 1..10 {
        let bldg_name = format!("{} building {}", fake!(Company.industry).to_lowercase(), x);

        let b = Building::add_building(bldg_name, logger, &tx)?;

        for floor in 2..8 {
            // no meeting rooms on first floor
            // Add 3 conference rooms per-floor
            let code = format!("{}A", floor);
            let _ = Room::add_room(b.id, code, floor, logger, &tx)?;

            let code = format!("{}B", floor);
            let _ = Room::add_room(b.id, code, floor, logger, &tx)?;

            let code = format!("{}C", floor);
            let _ = Room::add_room(b.id, code, floor, logger, &tx)?;
        }
    }

    tx.commit().map_err(|err| {
        error!(logger, "Failed to commit transaction";
				"step"=>"commit_tx", "err"=>err.to_string());
        MyError::DBError(DBError::PGError(err))
    })?;

    Ok(())
}
