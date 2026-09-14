//! 🦀️ Rust subject for the time bucketing case. Gated behind the `sut` feature like every subject.

#[cfg(feature = "sut")]
mod subject {
    use semio_framework_repo_metrics as metrics;
    use semio_repo_test_host::{parse_json, Context, Json, Outcome};

    //#region 🔖️Helpers
    /// 🕰️ The six granularities, in the order the feature states them.
    const GRANULARITIES: [(&str, metrics::TimeBucket); 6] = [
        ("commit", metrics::TimeBucket::Commit),
        ("hour", metrics::TimeBucket::Hour),
        ("day", metrics::TimeBucket::Day),
        ("week", metrics::TimeBucket::Week),
        ("month", metrics::TimeBucket::Month),
        ("year", metrics::TimeBucket::Year),
    ];

    /// ⏱️ The instants the feature states: pre-epoch, epoch, a leap day, a century boundary.
    const INSTANTS: [i64; 7] = [-2_208_988_800, 0, 951_782_400, 1_234_567_890, 1_767_225_600, 1_769_904_000, 4_102_444_800];

    /// 🧩️ The recorded commit stream every scenario groups.
    fn commits(ctx: &Context) -> Result<Vec<metrics::CommitDelta>, String> {
        let recorded = metrics::GitTranscript::from_json(&String::from_utf8_lossy(&ctx.fixture_bytes("shared://🎞️git-transcript.json")?))?;
        let weights = metrics::make_numstat_lang_set(&metrics::DEFAULT_CODE_LANGUAGES.iter().map(|value| (*value).to_string()).collect::<Vec<String>>());
        Ok(metrics::parse_numstat_log(recorded.logs.get("").map(String::as_str).unwrap_or(""), &weights, None))
    }
    //#endregion 🔖️Helpers

    //#region 🔖️Scenarios
    pub fn groups_commits_by_granularity(ctx: &Context) -> Result<Outcome, String> {
        let stream = commits(ctx)?;
        let mut rows: Vec<(String, Json)> = Vec::new();
        for (name, bucket) in GRANULARITIES {
            rows.push((name.to_string(), parse_json(&metrics::to_json_string(&metrics::bucket_commits(&stream, bucket))?)?));
        }
        Ok(Outcome::projection(Json::Object(rows)))
    }

    pub fn formats_utc_timestamps(_ctx: &Context) -> Result<Outcome, String> {
        let rows: Vec<(String, Json)> = INSTANTS
            .iter()
            .map(|unix| {
                let mut keys: Vec<(String, Json)> = vec![("rfc3339".to_string(), Json::String(metrics::format_rfc3339_utc(*unix)))];
                for (name, bucket) in GRANULARITIES {
                    keys.push((name.to_string(), Json::String(metrics::bucket_key(*unix, bucket))));
                    keys.push((format!("{name}Start"), Json::Number(metrics::bucket_start(*unix, bucket) as f64)));
                }
                (unix.to_string(), Json::Object(keys))
            })
            .collect();
        Ok(Outcome::projection(Json::Object(rows)))
    }

    pub fn civil_date_round_trips(_ctx: &Context) -> Result<Outcome, String> {
        let mut checked = 0_u32;
        let mut mismatches: Vec<Json> = Vec::new();
        let mut day = -30_000_i64;
        while day <= 30_000 {
            let (year, month, of_month) = metrics::civil_from_unix(day * 86_400);
            if metrics::unix_days_from_civil(year, month, of_month) != day {
                mismatches.push(Json::Number(day as f64));
            }
            checked += 1;
            day += 97;
        }
        Ok(Outcome::projection(Json::Object(vec![("checked".to_string(), Json::Number(checked as f64)), ("mismatches".to_string(), Json::Array(mismatches))])))
    }
    //#endregion 🔖️Scenarios
}

//#region 🔖️Registration
/// 🧭️ Registration entry point the generated host calls.
pub fn adapter() -> semio_repo_test_host::Adapter {
    let adapter = semio_repo_test_host::Adapter::new("rust");
    #[cfg(feature = "sut")]
    let adapter = adapter
        .subject("groups-commits-by-granularity", subject::groups_commits_by_granularity)
        .subject("formats-utc-timestamps", subject::formats_utc_timestamps)
        .subject("civil-date-round-trips", subject::civil_date_round_trips);
    adapter
}
//#endregion 🔖️Registration
