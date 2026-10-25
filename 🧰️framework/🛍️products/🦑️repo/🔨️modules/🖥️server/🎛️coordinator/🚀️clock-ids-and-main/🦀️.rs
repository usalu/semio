//! 🚀️ Clock, ids and main of the coordinator: the unix clock, the id generation, the RFC 3339 time and the blocking entry point. See [`super`].

use super::*;

//#region 🚀️Main
pub(super) fn unix_seconds() -> u64 {
    SystemTime::now().duration_since(UNIX_EPOCH).map(|value| value.as_secs()).unwrap_or_default()
}

/// 🆔️ A monotonic-prefixed unique identifier for one in-process event.
pub fn new_id() -> String {
    static COUNTER: AtomicU64 = AtomicU64::new(0);
    let nanos = SystemTime::now().duration_since(UNIX_EPOCH).map(|value| value.as_nanos()).unwrap_or_default();
    format!("{nanos}-{}", COUNTER.fetch_add(1, Ordering::Relaxed))
}

/// 🕰️ The current instant as RFC 3339 in UTC, the shape Go's `time.Time` marshals to.
pub fn now_rfc3339() -> String {
    let total = SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or_default();
    let seconds = total.as_secs() as i64;
    let (days, rest) = (seconds.div_euclid(86_400), seconds.rem_euclid(86_400));
    let (year, month, day) = civil_from_days(days);
    format!(
        "{year:04}-{month:02}-{day:02}T{:02}:{:02}:{:02}.{:09}Z",
        rest / 3600,
        (rest % 3600) / 60,
        rest % 60,
        total.subsec_nanos()
    )
}

fn civil_from_days(days: i64) -> (i64, i64, i64) {
    let shifted = days + 719_468;
    let era = shifted.div_euclid(146_097);
    let day_of_era = shifted.rem_euclid(146_097);
    let year_of_era = (day_of_era - day_of_era / 1460 + day_of_era / 36_524 - day_of_era / 146_096) / 365;
    let year = year_of_era + era * 400;
    let day_of_year = day_of_era - (365 * year_of_era + year_of_era / 4 - year_of_era / 100);
    let shifted_month = (5 * day_of_year + 2) / 153;
    let day = day_of_year - (153 * shifted_month + 2) / 5 + 1;
    let month = if shifted_month < 10 { shifted_month + 3 } else { shifted_month - 9 };
    (if month <= 2 { year + 1 } else { year }, month, day)
}

/// ▶️ Starts the coordinator from the ambient environment and blocks forever.
pub fn main_blocking() -> Result<(), StoreError> {
    let config = Config::from_env();
    let service = start(config)?;
    println!("repo coordinator listening on {}", service.address);
    loop {
        std::thread::sleep(Duration::from_secs(3600));
    }
}
//#endregion 🚀️Main
