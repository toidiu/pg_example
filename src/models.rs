use chrono::prelude::*;
use postgres::{
    error::EXCLUSION_VIOLATION,
    rows::{Row, Rows},
    transaction::Transaction,
};
use slog::Logger;
use uuid::Uuid;

use db::TSTZRange;
use errors::{DBError, MeetingError, MyError};



#[derive(Debug, Clone)]
pub struct User {
    pub id: i64,
    pub ext_id: Uuid,
    pub first_name: String,
    pub last_name: String,
    pub username: String,
}
impl User {
    /// 'add_user' features a functional-style implementation
    pub fn add_user(first_name: String,
                    last_name: String,
                    username: String,
                    logger: &Logger,
                    tx: &Transaction)
                    -> Result<User, MyError> {
        let stmt = "
		INSERT INTO testing.users(first_name, last_name, username)
		VALUES ($1, $2, $3)
		RETURNING testing.users.id,
				  testing.users.ext_id,
				  testing.users.first_name,
				  testing.users.last_name,
				  testing.users.username;";

        tx.query(stmt, &[&first_name, &last_name, &username])
          .map_err(|err| {
              error!(logger, "Failed to add user: DB Error.";
                    "step"=>"add_user", "err"=>err.to_string());
              MyError::DBError(DBError::PGError(err))
          })
          .and_then(|rows: Rows| {
              rows.into_iter()
                  .next()
                  .map(|row: Row| {
                           // example of referencing row elements by index
                           let user = User { id: row.get(0),
                                             ext_id: row.get(1),
                                             first_name: row.get(2),
                                             last_name: row.get(3),
                                             username: row.get(4), };
                           info!(logger, "Added user: {}", user.username);
                           user
                       })
                  .ok_or_else(|| {
                      error!(logger, "Error adding user to db: No record returned.";
  							"step"=>"add_user");
                      MyError::DBError(DBError::NoRecord)
                  })
          })
    }

    pub fn get_users(logger: &Logger, tx: &Transaction)
                        -> Result<Vec<User>, MyError> {
        let stmt = "
		SELECT users.id, ext_id, first_name, last_name, username
		  FROM testing.users;";

        tx.query(stmt, &[])
          .map_err(|err| {
              error!(logger, "Failed to query for users: DB Error.";
                    "step"=>"get_users", "err"=>err.to_string());
              MyError::DBError(DBError::PGError(err))
          })
          .and_then(|rows: Rows| {
              if rows.is_empty() {
                  error!(logger, "Error querying users from db: \
                        No record returned."; "step"=>"get_users");
                  return Err(MyError::DBError(DBError::NoRecord));
              }

              let users = rows.into_iter()
                              .map(|row: Row| User { id: row.get(0),
                                                     ext_id: row.get(1),
                                                     first_name: row.get(2),
                                                     last_name: row.get(3),
                                                     username: row.get(4), })
                              .collect::<Vec<User>>();
              Ok(users)
          })
    }
}

#[derive(Debug, Clone)]
pub struct Building {
    pub id: i64,
    pub ext_id: Uuid,
    pub name: String,
}
impl Building {
    /// add_building features a procedural-style implementation
    pub fn add_building(name: String,
                        logger: &Logger,
                        tx: &Transaction)
                        -> Result<Building, MyError> {
        let stmt = "
		INSERT INTO testing.building(name)
		VALUES ($1)
		RETURNING testing.building.id,
				  testing.building.ext_id,
				  testing.building.name;";

        let result = tx.query(stmt, &[&name]).map_err(|err| {
            error!(logger, "Failed to add building: DB Error.";
					"step"=>"add_building", "err"=>err.to_string());
            MyError::DBError(DBError::PGError(err))
        })?;

        if result.is_empty() {
            error!(logger, "Error adding building to db: No record returned.";
				"step"=>"add_building");
            return Err(MyError::DBError(DBError::NoRecord));
        }

        let row: Row = result.iter().next().unwrap();

        // example of referencing row elements by name
        let bldg = Building { id: row.get("id"),
                              ext_id: row.get("ext_id"),
                              name: row.get("name"), };

        info!(logger, "Added building: {}", bldg.name);

        Ok(bldg)
    }

    pub fn get_buildings(logger: &Logger, tx: &Transaction)
                         -> Result<Vec<Building>, MyError> {
        let stmt = "
		SELECT id, ext_id, name
		  FROM testing.building;";

        tx.query(stmt, &[])
          .map_err(|err| {
              error!(logger, "Failed to query for buildings: DB Error.";
					"step"=>"get_buildings", "err"=>err.to_string());
              MyError::DBError(DBError::PGError(err))
          })
          .and_then(|rows: Rows| {
              if rows.is_empty() {
                  error!(logger, "Error querying buildings from db: \
						  No record returned."; "step"=>"get_buildings");
                  return Err(MyError::DBError(DBError::NoRecord));
              }

              let bldgs = rows.into_iter()
                              .map(|row: Row| Building { id: row.get(0),
                                                         ext_id: row.get(1),
                                                         name: row.get(2), })
                              .collect::<Vec<Building>>();
              Ok(bldgs)
          })
    }
}

#[derive(Debug, Clone)]
pub struct Room {
    pub id: i64,
    pub ext_id: Uuid,
    pub building_id: i64,
    pub code: String,
    pub floor_num: i32,
}
impl Room {
    pub fn add_room(building_id: i64,
                    code: String,
                    floor: i32,
                    logger: &Logger,
                    tx: &Transaction)
                    -> Result<Room, MyError> {
        let stmt = "
		INSERT INTO testing.room(building_id, code, floor_num)
		VALUES ($1, $2, $3)
		RETURNING testing.room.id,
				  testing.room.ext_id,
				  testing.room.building_id,
				  testing.room.code,
				  testing.room.floor_num;";

        tx.query(stmt, &[&building_id, &code, &floor])
          .map_err(|err| {
              error!(&logger, "Failed to add meeting room: DB Error.";
					"step"=>"add_room", "err"=>err.to_string());
              MyError::DBError(DBError::PGError(err))
          })
          .and_then(|rows: Rows| {
              rows.into_iter()
                  .next()
                  .map(|row: Row| {
                    let room = Room { id: row.get(0),
                                        ext_id: row.get(1),
                                        building_id: row.get(2),
                                        code: row.get(3),
                                        floor_num: row.get(4), };
                    info!(&logger, "Added meeting room: {}", room.code);
                    room })
                  .ok_or_else(|| {
                      error!(logger, "Error adding room to db: No record returned.";
  							  "step"=>"add_room");
                      MyError::DBError(DBError::NoRecord)
                  })
          })
    }

    pub fn get_rooms(logger: &Logger, tx: &Transaction)
                        -> Result<Vec<Room>, MyError> {
        let stmt = "
		SELECT id, ext_id, building_id, code, floor_num 
		  FROM testing.room;";

        tx.query(stmt, &[])
          .map_err(|err| {
              error!(logger, "Failed to query for rooms: DB Error.";
					"step"=>"get_rooms", "err"=>err.to_string());
              MyError::DBError(DBError::PGError(err))
          })
          .and_then(|rows: Rows| {
              if rows.is_empty() {
                  error!(logger, "Error querying rooms from db: \
						  No record returned."; "step"=>"get_rooms");
                  return Err(MyError::DBError(DBError::NoRecord));
              }

              let rooms = rows.into_iter()
                              .map(|row: Row| Room { id: row.get(0),
                                                     ext_id: row.get(1),
                                                     building_id: row.get(2),
                                                     code: row.get(3),
                                                     floor_num: row.get(4), })
                              .collect::<Vec<Room>>();
              Ok(rooms)
          })
    }
}

#[derive(Debug)]
pub struct Meeting {
    pub id: i64,
    pub ext_id: Uuid,
    pub organizer_id: i64,
    pub room_id: i64,
    pub title: String,
    pub time_slot: TSTZRange,
}
impl Meeting {
    pub fn schedule_meeting(username: String,
                            bldg_ext_id: Uuid,
                            room_code: String,
                            start_dt: String,
                            end_dt: String,
                            title: String,
                            logger: &Logger,
                            tx: &Transaction)
                            -> Result<Meeting, MyError> {
        let start_dt = match start_dt.parse::<DateTime<Utc>>() {
            Ok(x) => x,
            Err(_) => {
                info!(logger, "Failed to convert user-provided start_dt");
                return Err(MyError::ValueError);
            }
        };

        let end_dt = match end_dt.parse::<DateTime<Utc>>() {
            Ok(x) => x,
            Err(_) => {
                info!(logger, "Failed to convert user-provided end_dt");
                return Err(MyError::ValueError);
            }
        };
        let time_slot: TSTZRange = range!('[' start_dt, end_dt; ']');

        let stmt = "
		INSERT INTO testing.meeting(organizer_id, room_id, title, time_slot)
		SELECT u.id, rooms.id, $1, $2
		FROM (SELECT r.id
			    FROM testing.room r
				JOIN testing.building b
				  ON r.building_id = b.id
			   WHERE r.code = $3
			     AND b.ext_id = $4) rooms, testing.users u
		WHERE u.username = $5
		RETURNING  testing.meeting.id,
					testing.meeting.ext_id,
					testing.meeting.organizer_id,
					testing.meeting.room_id,
					testing.meeting.title,
					testing.meeting.time_slot;";

        tx.query(stmt,
                 &[&title, &time_slot, &room_code, &bldg_ext_id, &username])
          .map_err(|err| {
              if Some(&EXCLUSION_VIOLATION) == err.code() {
                  info!(logger, "Meeting schedule overlap.  Could not schedule.");
                  return MyError::MeetingError(MeetingError::ScheduleConflict);
              } else {
                  error!(logger, "Failed to schedule meeting: Unknown DB Error.";
						"step"=>"schedule_meeting", "err"=>err.to_string());
                  MyError::DBError(DBError::PGError(err))
              }
          })
          .and_then(|rows: Rows| {
              rows.into_iter()
                  .next()
                  .map(|row: Row| {
                           let mtg = Meeting { id: row.get(0),
                                               ext_id: row.get(1),
                                               organizer_id: row.get(2),
                                               room_id: row.get(3),
                                               title: row.get(4),
                                               time_slot: row.get(5), };
                           info!(logger, "Scheduled Meeting: {}", mtg.ext_id);
                           mtg
                       })
                  .ok_or_else(|| {
                      error!(logger, "Error scheduling meeting in db: No record returned.";
  							  "step"=>"schedule_meeting");
                      MyError::DBError(DBError::NoRecord)
                  })
          })
    }

    /// Confirm what preferred timeslots are available for scheduling.
    ///
    /// The point of this function is to show how to import datasets for use
    /// within SQL.
    pub fn check_room_availability_v1(room_cd: String,
                                      bldg_ext_id: Uuid,
                                      preferred_timeslots: Vec<(i64, String, String)>,
                                      logger: &Logger,
                                      tx: &Transaction)
                                      -> Result<Vec<i64>, MyError> {
        let (p_ids, p_timeslots): (Vec<i64>, Vec<TSTZRange>) =
			preferred_timeslots
			.into_iter()
			.map(|(id, start, end)| {
				let start_dt = start.parse::<DateTime<Utc>>().unwrap(); //unsafe
				let end_dt = end.parse::<DateTime<Utc>>().unwrap(); //unsafe
        		let time_slot: TSTZRange = range!('[' start_dt, end_dt; ']');

				(id, time_slot)
			})
			.unzip();

        let stmt = "
		SELECT pref.id 
		  FROM unnest($1::bigint[], $2::tstzrange[]) as pref(id, time_slot)
		 WHERE not exists (SELECT true
		 					 FROM testing.meeting mtg
							 JOIN testing.room r
							   ON mtg.room_id = r.id
							 JOIN testing.building b
							   ON r.building_id = b.id
							WHERE r.code = $3
							  AND b.ext_id = $4
							  AND mtg.time_slot && pref.time_slot);";

        tx.query(stmt, &[&p_ids, &p_timeslots, &room_cd, &bldg_ext_id])
          .map_err(|err| {
              error!(logger, "Failed to check room availability: DB Error.";
					"step"=>"check_room_availability", "err"=>err.to_string());
              MyError::DBError(DBError::PGError(err))
          })
          .and_then(|rows: Rows| {
            let mtgs = rows.into_iter()
                            .map(|row: Row| {
                                    let id: i64 = row.get(0);
                                    id
                                })
                            .collect::<Vec<i64>>();
            Ok(mtgs)})
    }
}
