use postgres::error::Error as PGError;
use r2d2::Error as PoolError;
use std::fmt;

#[derive(Debug)]
pub enum DBError {
    NoRecord,
    PGError(PGError),
    PoolError(PoolError),
}

#[derive(Debug)]
pub enum MeetingError {
    ScheduleConflict,
}

#[derive(Debug)]
pub enum MyError {
    DBError(DBError),
    MeetingError(MeetingError),
    ValueError,
}

impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &MyError::DBError(ref err) => write!(f, "DB Error: {:?}", err),
            &MyError::MeetingError(ref err) => write!(f, "Meeting Error: {:?}", err),
            &MyError::ValueError => write!(f, "Value Error"),
        }
    }
}
