//! ⌛️ Time: UTC time bucketing.

use super::{CommitDelta, LineMetrics};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

//#region 🕰️Time

/// 🕰️ The granularity a commit stream is grouped at.
#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum TimeBucket {
    /// 🔀️ No grouping: one bucket per commit, keyed by its RFC 3339 UTC timestamp.
    Commit,
    Hour,
    Day,
    /// 📆️ ISO-8601 weeks, Monday-started, keyed `YYYY-Www`.
    Week,
    Month,
    Year,
}

/// 🌙️ `(year, month, day)` of a Unix second in UTC, by the proleptic Gregorian civil algorithm.
pub fn civil_from_unix(unix: i64) -> (i64, u32, u32) {
    let days = unix.div_euclid(86_400);
    let z = days + 719_468;
    let era = z.div_euclid(146_097);
    let day_of_era = z.rem_euclid(146_097);
    let year_of_era = (day_of_era - day_of_era / 1460 + day_of_era / 36_524 - day_of_era / 146_096) / 365;
    let year = year_of_era + era * 400;
    let day_of_year = day_of_era - (365 * year_of_era + year_of_era / 4 - year_of_era / 100);
    let month_prime = (5 * day_of_year + 2) / 153;
    let day = (day_of_year - (153 * month_prime + 2) / 5 + 1) as u32;
    let month = if month_prime < 10 { month_prime + 3 } else { month_prime - 9 } as u32;
    (if month <= 2 { year + 1 } else { year }, month, day)
}

/// 🗓️ Days since the Unix epoch for a civil UTC date, the inverse of [`civil_from_unix`].
pub fn unix_days_from_civil(year: i64, month: u32, day: u32) -> i64 {
    let year = if month <= 2 { year - 1 } else { year };
    let era = year.div_euclid(400);
    let year_of_era = year.rem_euclid(400);
    let month_prime = if month > 2 { month - 3 } else { month + 9 } as i64;
    let day_of_year = (153 * month_prime + 2) / 5 + day as i64 - 1;
    let day_of_era = year_of_era * 365 + year_of_era / 4 - year_of_era / 100 + day_of_year;
    era * 146_097 + day_of_era - 719_468
}

/// 📅️ ISO weekday of a Unix second, Monday = 1 … Sunday = 7.
pub fn iso_weekday(unix: i64) -> i64 {
    (unix.div_euclid(86_400) + 3).rem_euclid(7) + 1
}

/// ⏱️ RFC 3339 UTC rendering of a Unix second, exactly what the history rows carry.
pub fn format_rfc3339_utc(unix: i64) -> String {
    let (year, month, day) = civil_from_unix(unix);
    let seconds_of_day = unix.rem_euclid(86_400);
    format!("{year:04}-{month:02}-{day:02}T{:02}:{:02}:{:02}Z", seconds_of_day / 3600, (seconds_of_day % 3600) / 60, seconds_of_day % 60)
}

/// 🪣️ The bucket key of a Unix second at one granularity.
pub fn bucket_key(unix: i64, bucket: TimeBucket) -> String {
    let (year, month, day) = civil_from_unix(unix);
    match bucket {
        TimeBucket::Commit => format_rfc3339_utc(unix),
        TimeBucket::Hour => format!("{year:04}-{month:02}-{day:02}T{:02}Z", unix.rem_euclid(86_400) / 3600),
        TimeBucket::Day => format!("{year:04}-{month:02}-{day:02}"),
        TimeBucket::Week => {
            let weekday = iso_weekday(unix);
            let thursday = unix.div_euclid(86_400) - (weekday - 1) + 3;
            let (iso_year, _, _) = civil_from_unix(thursday * 86_400);
            let january_first = unix_days_from_civil(iso_year, 1, 1);
            format!("{iso_year:04}-W{:02}", (thursday - january_first) / 7 + 1)
        }
        TimeBucket::Month => format!("{year:04}-{month:02}"),
        TimeBucket::Year => format!("{year:04}"),
    }
}

/// ▶️ The first Unix second of the bucket a timestamp falls in.
pub fn bucket_start(unix: i64, bucket: TimeBucket) -> i64 {
    let (year, month, _) = civil_from_unix(unix);
    match bucket {
        TimeBucket::Commit => unix,
        TimeBucket::Hour => unix - unix.rem_euclid(3600),
        TimeBucket::Day => unix.div_euclid(86_400) * 86_400,
        TimeBucket::Week => {
            let weekday = iso_weekday(unix);
            (unix.div_euclid(86_400) - (weekday - 1)) * 86_400
        }
        TimeBucket::Month => unix_days_from_civil(year, month, 1) * 86_400,
        TimeBucket::Year => unix_days_from_civil(year, 1, 1) * 86_400,
    }
}

/// 📦️ One time bucket: its key, its first second, the commits in it and their summed deltas.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TimeBucketGroup {
    pub key: String,
    pub start_unix: i64,
    pub commits: Vec<String>,
    pub delta: BTreeMap<String, LineMetrics>,
}

/// 🐙️ Groups a commit stream into ordered time buckets, summing every weighted bucket delta.
pub fn bucket_commits(commits: &[CommitDelta], bucket: TimeBucket) -> Vec<TimeBucketGroup> {
    let mut order: Vec<String> = Vec::new();
    let mut groups: BTreeMap<String, TimeBucketGroup> = BTreeMap::new();
    for commit in commits {
        let key = bucket_key(commit.when_unix, bucket);
        let group = groups.entry(key.clone()).or_insert_with(|| {
            order.push(key.clone());
            TimeBucketGroup { key: key.clone(), start_unix: bucket_start(commit.when_unix, bucket), commits: Vec::new(), delta: BTreeMap::new() }
        });
        group.commits.push(commit.sha.clone());
        for (name, pair) in &commit.delta {
            let total = group.delta.entry(name.clone()).or_default();
            total.added += pair.added;
            total.removed += pair.removed;
        }
    }
    order.into_iter().filter_map(|key| groups.remove(&key)).collect()
}

//#endregion 🕰️Time
