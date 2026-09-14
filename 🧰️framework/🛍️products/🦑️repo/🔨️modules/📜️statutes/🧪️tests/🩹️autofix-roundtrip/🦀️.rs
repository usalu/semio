//! 🦀️ Rust side of the autofix round trip case. The subject half is gated behind the `sut` feature
//! so the oracle role never compiles the implementation under test.

use semio_repo_test_host::{Adapter, Context, Json, Outcome};

//#region 🔖️Scenarios

#[cfg(feature = "sut")]
fn the_fixable_source_becomes_the_expected_source(ctx: &Context) -> Result<Outcome, String> {
    use semio_framework_repo_statutes as statutes;
    let fixable_uri = "shared://📁️some/📁️folder/🧪️file-fixable/🟦️.tsx";
    let expected_uri = "shared://📁️some/📁️folder/🧪️file-fixable-expected/🟦️.tsx";
    let read = |uri: &str| -> Result<String, String> {
        let path = ctx.fixture(uri)?;
        std::fs::read_to_string(&path).map_err(|error| format!("cannot read {uri}: {error}"))
    };
    let source = statutes::SourceFile { path: fixable_uri.trim_start_matches("shared://").to_string(), content: read(fixable_uri)? };
    let expected = read(expected_uri)?;
    let before = statutes::analyze(&statutes::SourceSet::new(vec![source.clone()]));
    let first = statutes::autofix(&source);
    let second = statutes::autofix(&statutes::SourceFile { path: source.path.clone(), content: first.content.clone() });
    let after = statutes::analyze(&statutes::SourceSet::new(vec![statutes::SourceFile { path: source.path.clone(), content: first.content.clone() }]));
    if first.content != expected {
        return Err("the repaired text is not the expected text".to_string());
    }
    if second.content != first.content || !second.fixed.is_empty() {
        return Err("the repair is not idempotent".to_string());
    }
    Ok(Outcome::projection(Json::Object(vec![
        ("before".to_string(), Json::Array(before.iter().map(|b| Json::String(format!("{}|{}|{}", b.kind.0, b.scope, b.line))).collect())),
        ("fixed".to_string(), Json::Array(first.fixed.iter().map(|kind| Json::String(kind.0.clone())).collect())),
        ("matchesExpected".to_string(), Json::String((first.content == expected).to_string())),
        ("idempotent".to_string(), Json::String((second.content == first.content).to_string())),
        ("after".to_string(), Json::Array(after.iter().map(|b| Json::String(format!("{}|{}|{}", b.kind.0, b.scope, b.line))).collect())),
    ])))
}

//#endregion 🔖️Scenarios

//#region 🔖️Registration

/// 🧭️ Registration entry point the generated host calls.
pub fn adapter() -> Adapter {
    let adapter = Adapter::new("rust");
    #[cfg(feature = "sut")]
    let adapter = adapter.subject("the-fixable-source-becomes-the-expected-source", the_fixable_source_becomes_the_expected_source);
    adapter
}

//#endregion 🔖️Registration
