//! 🦀️ Rust side of the malformed region case.

use semio_framework_repo_languages::{parse_sections, Section};
use semio_repo_test_host::{Adapter, Context, Json, Outcome};

// #region 🔖️Projection

fn flat(section: &Section) -> Json {
    Json::Object(vec![
        ("name".to_string(), Json::String(section.name.clone())),
        ("emoji".to_string(), Json::String(section.emoji.clone())),
        ("startLine".to_string(), Json::Number(section.start_line as f64)),
        ("endLine".to_string(), Json::Number(section.end_line as f64)),
        ("children".to_string(), Json::Array(section.children.iter().map(flat).collect())),
    ])
}

fn parse(ctx: &Context, name: &str) -> Result<Json, String> {
    let bytes = ctx.fixture_bytes(&format!("local://{name}"))?;
    let content = String::from_utf8(bytes).map_err(|e| e.to_string())?;
    let sections = parse_sections(&content, name);
    Ok(Json::Object(vec![
        ("lineCount".to_string(), Json::Number(content.split('\n').count() as f64)),
        ("sections".to_string(), Json::Array(sections.iter().map(flat).collect())),
    ]))
}

// #endregion 🔖️Projection

// #region 🔖️Scenarios

fn unclosed_region_runs_to_the_end(ctx: &Context) -> Result<Outcome, String> {
    Ok(Outcome::projection(parse(ctx, "❌️unclosed.ts")?))
}

fn stray_endregion_is_ignored(ctx: &Context) -> Result<Outcome, String> {
    Ok(Outcome::projection(parse(ctx, "❌️stray-end.ts")?))
}

fn unclaimed_extension_has_no_sections(ctx: &Context) -> Result<Outcome, String> {
    Ok(Outcome::projection(parse(ctx, "❌️unclaimed.unknown")?))
}

// #endregion 🔖️Scenarios

// #region 🔖️Registration

/// 🧭️ Registration entry point the generated host calls.
pub fn adapter() -> Adapter {
    Adapter::new("rust")
        .subject("unclosed-region-runs-to-the-end", unclosed_region_runs_to_the_end)
        .subject("stray-endregion-is-ignored", stray_endregion_is_ignored)
        .subject("unclaimed-extension-has-no-sections", unclaimed_extension_has_no_sections)
}

// #endregion 🔖️Registration
