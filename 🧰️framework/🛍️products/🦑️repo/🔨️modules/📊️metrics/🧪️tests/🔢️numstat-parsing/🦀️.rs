//! 🦀️ Rust subject for the numstat parsing case. The subject half is gated behind the `sut`
//! feature so the oracle role never links, or even compiles, the implementation under test.

#[cfg(feature = "sut")]
mod subject {
    use semio_framework_repo_metrics as metrics;
    use semio_repo_test_host::{digest, parse_json, Context, Json, Outcome};
    use std::collections::BTreeSet;

    //#region 🔖️Helpers
    /// 🎞️ The recorded transcript this case replays.
    fn transcript(ctx: &Context) -> Result<metrics::GitTranscript, String> {
        metrics::GitTranscript::from_json(&String::from_utf8_lossy(&ctx.fixture_bytes("shared://🎞️git-transcript.json")?))
    }

    /// 🗣️ The default weight set every scenario parses under.
    fn weights() -> BTreeSet<String> {
        metrics::make_numstat_lang_set(&metrics::DEFAULT_CODE_LANGUAGES.iter().map(|value| (*value).to_string()).collect::<Vec<String>>())
    }

    /// 🧩️ Parses one recorded ref into commit records.
    fn commits(ctx: &Context, git_ref: &str) -> Result<Vec<metrics::CommitDelta>, String> {
        let recorded = transcript(ctx)?;
        Ok(metrics::parse_numstat_log(recorded.logs.get(git_ref).map(String::as_str).unwrap_or(""), &weights(), None))
    }
    //#endregion 🔖️Helpers

    //#region 🔖️Scenarios
    pub fn recorded_transcript_matches_real_git(ctx: &Context) -> Result<Outcome, String> {
        let recorded = transcript(ctx)?;
        let no_merges = recorded.logs.get("").cloned().unwrap_or_default();
        let with_merges = recorded.logs.get("🔀️with-merges").cloned().unwrap_or_default();
        let shas: Vec<String> = metrics::parse_numstat_log(&no_merges, &weights(), None).into_iter().map(|commit| commit.sha).collect();
        Ok(Outcome::projection(Json::Object(vec![
            ("noMergesDigest".to_string(), Json::String(digest(no_merges.as_bytes()))),
            ("withMergesDigest".to_string(), Json::String(digest(with_merges.as_bytes()))),
            ("commitCount".to_string(), Json::Number(shas.len() as f64)),
            ("shas".to_string(), Json::Array(shas.into_iter().map(Json::String).collect())),
            ("trackedAtHead".to_string(), Json::Array(recorded.tracked.get("").cloned().unwrap_or_default().into_iter().map(Json::String).collect())),
        ])))
    }

    pub fn parses_recorded_numstat_stream(ctx: &Context) -> Result<Outcome, String> {
        Ok(Outcome::projection(parse_json(&metrics::to_json_string(&commits(ctx, "")?)?)?))
    }

    pub fn resolves_renames_and_quoted_paths(ctx: &Context) -> Result<Outcome, String> {
        let files: Vec<metrics::FileDelta> = commits(ctx, "")?.into_iter().flat_map(|commit| commit.files).collect();
        Ok(Outcome::projection(parse_json(&metrics::to_json_string(&files)?)?))
    }

    pub fn merge_commit_carries_no_file_rows(ctx: &Context) -> Result<Outcome, String> {
        Ok(Outcome::projection(parse_json(&metrics::to_json_string(&commits(ctx, "🔀️with-merges")?)?)?))
    }
    //#endregion 🔖️Scenarios
}

//#region 🔖️Registration
/// 🧭️ Registration entry point the generated host calls.
pub fn adapter() -> semio_repo_test_host::Adapter {
    let adapter = semio_repo_test_host::Adapter::new("rust");
    #[cfg(feature = "sut")]
    let adapter = adapter
        .subject("recorded-transcript-matches-real-git", subject::recorded_transcript_matches_real_git)
        .subject("parses-recorded-numstat-stream", subject::parses_recorded_numstat_stream)
        .subject("resolves-renames-and-quoted-paths", subject::resolves_renames_and_quoted_paths)
        .subject("merge-commit-carries-no-file-rows", subject::merge_commit_carries_no_file_rows);
    adapter
}
//#endregion 🔖️Registration
