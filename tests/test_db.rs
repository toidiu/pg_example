use postgres::{transaction::Transaction, Connection, TlsMode};
use rand::{thread_rng, Rng};
use slog::Logger;

use pg_example::{
    errors::{DBError, MeetingError, MyError},
    log::create_logger,
    models::{Building, Meeting, Room, User},
};

pub fn get_conn() -> Result<Connection, MyError> {
    let db_url = "postgres://test_user:testing@127.0.0.1:5432/testing_db";
    Connection::connect(db_url, TlsMode::None).map_err(|err| {
                                                           MyError::DBError(DBError::PGError(err))
                                                       })
}

pub fn get_test_data(logger: &Logger, tx: &Transaction) -> Result<(User, Building, Room), MyError> {
    let users: Vec<User> = User::get_users(&logger, &tx)?;
    let buildings: Vec<Building> = Building::get_buildings(&logger, &tx)?;
    let rooms: Vec<Room> = Room::get_rooms(&logger, &tx)?;
    // println!("\nUsers {:#?}", users);
    // println!("\nBuildings {:#?}", buildings);
    // println!("\nRooms {:#?}", rooms);

    // Randomly choose an element from each collection
    let mut rng = thread_rng();
    let user = rng.choose(&users).unwrap().clone();
    let building = rng.choose(&buildings).unwrap().clone();
    let room = rng.choose(&rooms).unwrap().clone();

    // println!("{:#?}", user);
    // println!("{:#?}", building);
    // println!("{:#?}", room);

    Ok((user, building, room))
}

#[test]
fn test_mtg_schedule() -> Result<(), MyError> {
    // This test successfully schedules an appointment and then immediately
    // tries to schedule the appointment again, returning a scheduling conflict.
    //
    // The intent of this test is to show different approaches rather than
    // design efficiency.  For instance, I realize that I could have queried
    // for a single randomly chosen user, building, etc but I wanted to explore
    // the rust-postgres api.

    let logger = create_logger();
    let conn = get_conn()?;
    let tx = conn.transaction()
                 .map_err(|err| MyError::DBError(DBError::PGError(err)))?;

    let (user, building, room) = get_test_data(&logger, &tx)?;

    let result = Meeting::schedule_meeting(user.username.clone(),
                                           building.ext_id,
                                           room.code.clone(),
                                           "2018-09-25T17:00:00Z".to_string(),
                                           "2018-09-25T19:00:00Z".to_string(),
                                           "Meeting #1".to_string(),
                                           &logger,
                                           &tx);
    assert_eq!(true, result.is_ok());

    let result = Meeting::schedule_meeting(user.username.clone(),
                                           building.ext_id,
                                           room.code.clone(),
                                           "2018-09-25T17:00:00Z".to_string(),
                                           "2018-09-25T19:00:00Z".to_string(),
                                           "Meeting #2".to_string(),
                                           &logger,
                                           &tx);
    assert_matches!(result,
                    Err(MyError::MeetingError(MeetingError::ScheduleConflict)));

    Ok(())
}

#[test]
fn test_mtg_room_availability() -> Result<(), MyError> {
    let logger = create_logger();
    let conn = get_conn()?;
    let tx = conn.transaction()
                 .map_err(|err| MyError::DBError(DBError::PGError(err)))?;

    let (user, building, room) = get_test_data(&logger, &tx)?;

    let result = Meeting::schedule_meeting(user.username.clone(),
                                           building.ext_id,
                                           room.code.clone(),
                                           "2018-09-25T09:00:00Z".to_string(),
                                           "2018-09-25T10:00:00Z".to_string(),
                                           "Meeting #1".to_string(),
                                           &logger,
                                           &tx);
    assert_eq!(true, result.is_ok());

    let result = Meeting::schedule_meeting(user.username.clone(),
                                           building.ext_id,
                                           room.code.clone(),
                                           "2018-09-25T10:00:05Z".to_string(),
                                           "2018-09-25T11:00:00Z".to_string(),
                                           "Meeting #2".to_string(),
                                           &logger,
                                           &tx);
    assert_eq!(true, result.is_ok());

    let result = Meeting::schedule_meeting(user.username.clone(),
                                           building.ext_id,
                                           room.code.clone(),
                                           "2018-09-25T14:00:00Z".to_string(),
                                           "2018-09-25T16:00:00Z".to_string(),
                                           "Meeting #3".to_string(),
                                           &logger,
                                           &tx);
    assert_eq!(true, result.is_ok());

    let preferred_timeslots: Vec<(i64, String, String)> =
        vec![(26, "2018-09-25T09:00:00Z".to_string(), "2018-09-25T11:00:00Z".to_string()),
             (27, "2018-09-25T11:00:01Z".to_string(), "2018-09-25T13:00:00Z".to_string())];

    let result: Vec<i64> = Meeting::check_room_availability_v1(room.code,
                                                               building.ext_id,
                                                               preferred_timeslots,
                                                               &logger,
                                                               &tx)?;
    assert_eq!(true, result.len() == 1);
    assert_eq!(true, result[0] == 27);

    Ok(())
}
