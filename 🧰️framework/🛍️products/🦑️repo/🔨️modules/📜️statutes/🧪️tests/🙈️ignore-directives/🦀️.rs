//! 🦀️ Rust side of the ignore directive case. The subject half is gated behind the `sut` feature so
//! the oracle role never compiles the implementation under test.

use semio_repo_test_host::{Adapter, Context, Json, Outcome};

//#region 🔖️Vectors

/// 🔢️ A numeric member of an object, truncated to a whole line number.
#[cfg(feature = "sut")]
fn number(value: &Json, key: &str) -> i64 {
    match value.get(key) {
        Some(Json::Number(inner)) => *inner as i64,
        _ => 0,
    }
}

/// 🙈️ Renders the parsed directives of one document as `line=prefix,prefix` rows.
#[cfg(feature = "sut")]
fn render_directives(content: &str) -> Vec<Json> {
    semio_framework_repo_statutes::parse_ignore_directives(content)
        .iter()
        .map(|(line, patterns)| Json::String(format!("{line}={}", patterns.join(","))))
        .collect()
}

//#endregion 🔖️Vectors

//#region 🔖️Scenarios

#[cfg(feature = "sut")]
fn a_directive_suppresses_its_prefixes(ctx: &Context) -> Result<Outcome, String> {
    use semio_framework_repo_statutes as statutes;
    let file = ctx.fixture_json("local://🔣️vectors.json")?;
    let mut directives: Vec<Json> = Vec::new();
    let mut decisions: Vec<Json> = Vec::new();
    for document in file.array("documents") {
        let name = document.str("name");
        let content = document.str("content");
        for row in render_directives(&content) {
            directives.push(Json::String(format!("{name}:{}", match &row {
                Json::String(value) => value.clone(),
                other => other.to_string(),
            })));
        }
        let parsed = statutes::parse_ignore_directives(&content);
        for probe in document.array("probes") {
            let line = number(&probe, "line");
            let kind = statutes::Statute::from(probe.str("kind").as_str());
            decisions.push(Json::String(format!("{name}:{line}:{}={}", kind.0, statutes::is_ignored(&parsed, line, &kind))));
        }
    }
    Ok(Outcome::projection(Json::Object(vec![("directives".to_string(), Json::Array(directives)), ("decisions".to_string(), Json::Array(decisions))])))
}

#[cfg(feature = "sut")]
fn a_directive_only_reaches_forward(ctx: &Context) -> Result<Outcome, String> {
    use semio_framework_repo_statutes as statutes;
    let file = ctx.fixture_json("local://🔣️vectors.json")?;
    let document = file.array("documents").into_iter().find(|d| d.str("name") == "single-prefix").ok_or_else(|| "the single-prefix document is missing".to_string())?;
    let parsed = statutes::parse_ignore_directives(&document.str("content"));
    let kind = statutes::Statute::from("code/section/empty");
    let window: Vec<Json> = [1i64, 2, 3, 102, 103].iter().map(|line| Json::String(format!("{line}={}", statutes::is_ignored(&parsed, *line, &kind)))).collect();
    if statutes::is_ignored(&parsed, 2, &kind) {
        return Err("a directive suppressed its own line".to_string());
    }
    if statutes::is_ignored(&parsed, 103, &kind) {
        return Err("a directive reached beyond its hundred line window".to_string());
    }
    if !statutes::is_ignored(&parsed, 102, &kind) {
        return Err("a directive did not reach the last line of its window".to_string());
    }
    Ok(Outcome::projection(Json::Object(vec![("window".to_string(), Json::Array(window))])))
}

//#endregion 🔖️Scenarios

//#region 🔖️Registration

/// 🧭️ Registration entry point the generated host calls.
pub fn adapter() -> Adapter {
    let adapter = Adapter::new("rust");
    #[cfg(feature = "sut")]
    let adapter = adapter
        .subject("a-directive-suppresses-its-prefixes", a_directive_suppresses_its_prefixes)
        .subject("a-directive-only-reaches-forward", a_directive_only_reaches_forward);
    adapter
}

//#endregion 🔖️Registration
