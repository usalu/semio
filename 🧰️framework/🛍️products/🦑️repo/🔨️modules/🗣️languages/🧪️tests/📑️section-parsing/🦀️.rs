//! 🦀️ Rust side of the section parsing case. Reads every fixture through the crate's own parsers so
//! the projection is produced by production code, never restated in the adapter.

use semio_framework_repo_languages::{parse_json_sections, parse_markdown_sections, parse_sections, Section};
use semio_repo_test_host::{Adapter, Context, Json, Outcome};

// #region 🔖️Projection

const MARKER_FIXTURES: &[&str] = &[
    "🟦️sample.ts",
    "🐹️sample.go",
    "🐍️sample.py",
    "🔷️sample.cs",
    "🦀️sample.rs",
    "💎️sample.rb",
    "🐚️sample.sh",
    "📢️sample.toml",
    "🤸️sample.yaml",
    "🕌️sample.sql",
    "🎙️sample.graphql",
];

fn ranged(section: &Section) -> Json {
    Json::Object(vec![
        ("name".to_string(), Json::String(section.name.clone())),
        ("emoji".to_string(), Json::String(section.emoji.clone())),
        ("startLine".to_string(), Json::Number(section.start_line as f64)),
        ("endLine".to_string(), Json::Number(section.end_line as f64)),
        ("startIndex".to_string(), Json::Number(section.start_index as f64)),
        ("endIndex".to_string(), Json::Number(section.end_index as f64)),
        ("children".to_string(), Json::Array(section.children.iter().map(ranged).collect())),
    ])
}

fn started(section: &Section) -> Json {
    Json::Object(vec![
        ("name".to_string(), Json::String(section.name.clone())),
        ("startLine".to_string(), Json::Number(section.start_line as f64)),
        ("startIndex".to_string(), Json::Number(section.start_index as f64)),
        ("children".to_string(), Json::Array(section.children.iter().map(started).collect())),
    ])
}

fn read(ctx: &Context, name: &str) -> Result<String, String> {
    let bytes = ctx.fixture_bytes(&format!("shared://{name}"))?;
    String::from_utf8(bytes).map_err(|e| e.to_string())
}

// #endregion 🔖️Projection

// #region 🔖️Scenarios

fn marker_regions_across_languages(ctx: &Context) -> Result<Outcome, String> {
    let mut entries = Vec::new();
    for name in MARKER_FIXTURES {
        let content = read(ctx, name)?;
        let sections = parse_sections(&content, name);
        entries.push((name.to_string(), Json::Array(sections.iter().map(ranged).collect())));
    }
    Ok(Outcome::projection(Json::Object(entries)))
}

fn markdown_heading_ranges(ctx: &Context) -> Result<Outcome, String> {
    let content = read(ctx, "📰️sample.md")?;
    let sections = parse_markdown_sections(&content);
    Ok(Outcome::projection(Json::Array(sections.iter().map(ranged).collect())))
}

fn json_object_key_tree(ctx: &Context) -> Result<Outcome, String> {
    let content = read(ctx, "🔣️sample.json")?;
    let sections = parse_json_sections(&content);
    Ok(Outcome::projection(Json::Array(sections.iter().map(started).collect())))
}

// #endregion 🔖️Scenarios

// #region 🔖️Registration

/// 🧭️ Registration entry point the generated host calls.
pub fn adapter() -> Adapter {
    Adapter::new("rust")
        .subject("marker-regions-across-languages", marker_regions_across_languages)
        .subject("markdown-heading-ranges", markdown_heading_ranges)
        .subject("json-object-key-tree", json_object_key_tree)
}

// #endregion 🔖️Registration
