use crate::common::{IdLaporan, Laporan, LaporanStatus, NomorLaporan};
use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

pub(crate) type ResultInsertLaporan = (IdLaporan, NomorLaporan);

pub(crate) fn establish_connection() -> PgConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}

pub(crate) fn insert_laporan(pg_connection: &PgConnection, laporan: Laporan) -> ResultInsertLaporan {
    (IdLaporan::new_v4(), "some_nomor".into())
}
