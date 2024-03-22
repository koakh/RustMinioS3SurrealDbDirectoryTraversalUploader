#![allow(dead_code)]

use chrono::{DateTime, Utc};
use surrealdb::sql::Datetime as SdbDatetime;

pub fn iso8601(st: &std::time::SystemTime) -> DateTime<Utc> {
    let dt: DateTime<Utc> = st.clone().into();
    dt
}

// https://stackoverflow.com/questions/64146345/how-do-i-convert-a-systemtime-to-iso-8601-in-rust
// formats like "2001-07-08T00:34:60.026490+09:30"
pub fn iso8601_to_string(st: &std::time::SystemTime) -> String {
    format!("{}", iso8601(st).format("%+"))
}

/// systemtime to chrono datetime
pub fn st2cdt(st: &std::time::SystemTime) -> DateTime<Utc> {
    let dt_now_utc: DateTime<Utc> = st.clone().into();
    dt_now_utc
}

/// systemtime to surrealdb datetime
pub fn st2sdt(st: &std::time::SystemTime) -> SdbDatetime {
    SdbDatetime(st2cdt(&st))
}
