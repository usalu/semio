//! 🦀️ Rust subject for the LOC aggregation case. The subject half is gated behind the `sut`
//! feature so the oracle role never links the implementation under test.

#[cfg(feature = "sut")]
mod subject {
    use semio_framework_repo_metrics as metrics;
    use semio_framework_repo_metrics::GitLogSource;
    use semio_repo_test_host::{parse_json, Context, Json, Outcome};

    //#region 🔖️Helpers
    /// 🎞️ The recorded transcript every scenario reads.
    fn transcript(ctx: &Context) -> Result<metrics::GitTranscript, String> {
        metrics::GitTranscript::from_json(&String::from_utf8_lossy(&ctx.fixture_bytes("shared://🎞️git-transcript.json")?))
    }

    /// 📊️ The snapshot report the last two scenarios both start from.
    fn report(recorded: &metrics::GitTranscript) -> Result<metrics::LocReport, String> {
        metrics::build_loc_report(recorded, &metrics::LocOptions::default(), None, &metrics::default_contributor_alias)
    }
    //#endregion 🔖️Helpers

    //#region 🔖️Scenarios
    pub fn counts_unified_loc_per_tracked_file(ctx: &Context) -> Result<Outcome, String> {
        let recorded = transcript(ctx)?;
        let mut rows: Vec<(String, Json)> = Vec::new();
        for path in recorded.tracked.get("").cloned().unwrap_or_default() {
            let Ok(body) = recorded.tracked_bytes("", &path) else { continue };
            rows.push((path.clone(), Json::Number(metrics::count_unified_loc_for_file(&path, &body) as f64)));
        }
        rows.sort_by(|left, right| left.0.cmp(&right.0));
        Ok(Outcome::projection(Json::Object(rows)))
    }

    pub fn composes_the_snapshot_table(ctx: &Context) -> Result<Outcome, String> {
        Ok(Outcome::projection(parse_json(&metrics::to_json_string(&report(&transcript(ctx)?)?)?)?))
    }

    pub fn history_stamps_delta_against_previous_row(ctx: &Context) -> Result<Outcome, String> {
        let recorded = transcript(ctx)?;
        let options = metrics::LocOptions { history: true, branch: "⛳️wip".to_string(), ..metrics::LocOptions::default() };
        let built = metrics::build_loc_report(&recorded, &options, None, &metrics::default_contributor_alias)?;
        Ok(Outcome::projection(parse_json(&metrics::to_json_string(&built.history.unwrap_or_default())?)?))
    }

    pub fn renders_the_markdown_snapshot_table(ctx: &Context) -> Result<Outcome, String> {
        let built = report(&transcript(ctx)?)?;
        let table = metrics::markdown_table("Snapshot", &built.snapshot, metrics::use_full_tree_table(&built.snapshot), false);
        Ok(Outcome::projection(Json::Object(vec![
            ("markdown".to_string(), Json::String(table)),
            ("order".to_string(), Json::Array(metrics::sorted_row_keys(&built.snapshot).into_iter().map(Json::String).collect())),
        ])))
    }
    //#endregion 🔖️Scenarios
}

//#region 🔖️Registration
/// 🧭️ Registration entry point the generated host calls.
pub fn adapter() -> semio_repo_test_host::Adapter {
    let adapter = semio_repo_test_host::Adapter::new("rust");
    #[cfg(feature = "sut")]
    let adapter = adapter
        .subject("counts-unified-loc-per-tracked-file", subject::counts_unified_loc_per_tracked_file)
        .subject("composes-the-snapshot-table", subject::composes_the_snapshot_table)
        .subject("history-stamps-delta-against-previous-row", subject::history_stamps_delta_against_previous_row)
        .subject("renders-the-markdown-snapshot-table", subject::renders_the_markdown_snapshot_table);
    adapter
}
//#endregion 🔖️Registration
