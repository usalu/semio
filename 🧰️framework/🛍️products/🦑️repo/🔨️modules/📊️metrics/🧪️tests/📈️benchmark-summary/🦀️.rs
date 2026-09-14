//! 🦀️ Rust subject for the benchmark summary case. Gated behind the `sut` feature like every subject.

#[cfg(feature = "sut")]
mod subject {
    use semio_framework_repo_metrics as metrics;
    use semio_repo_test_host::{parse_json, Context, Json, Outcome};

    //#region 🔖️Helpers
    /// ⏱️ The recorded stdout of every ecosystem, parsed in the report's column order.
    fn timings(ctx: &Context) -> Result<Vec<metrics::BenchmarkResult>, String> {
        let outputs = parse_json(&String::from_utf8_lossy(&ctx.fixture_bytes("shared://⏱️benchmark-output.json")?))?;
        Ok(metrics::BENCHMARK_LANGUAGES.iter().flat_map(|language| metrics::parse_benchmark_output(language, &outputs.str(language))).collect())
    }
    //#endregion 🔖️Helpers

    //#region 🔖️Scenarios
    pub fn parses_benchmark_stdout(ctx: &Context) -> Result<Outcome, String> {
        Ok(Outcome::projection(parse_json(&metrics::to_json_string(&timings(ctx)?)?)?))
    }

    pub fn summarizes_and_renders_csv(ctx: &Context) -> Result<Outcome, String> {
        let results = timings(ctx)?;
        Ok(Outcome::projection(Json::Object(vec![
            ("summary".to_string(), parse_json(&metrics::to_json_string(&metrics::summarize_benchmarks(&results))?)?),
            ("csv".to_string(), Json::String(metrics::benchmark_csv(&results))),
        ])))
    }
    //#endregion 🔖️Scenarios
}

//#region 🔖️Registration
/// 🧭️ Registration entry point the generated host calls.
pub fn adapter() -> semio_repo_test_host::Adapter {
    let adapter = semio_repo_test_host::Adapter::new("rust");
    #[cfg(feature = "sut")]
    let adapter = adapter.subject("parses-benchmark-stdout", subject::parses_benchmark_stdout).subject("summarizes-and-renders-csv", subject::summarizes_and_renders_csv);
    adapter
}
//#endregion 🔖️Registration
