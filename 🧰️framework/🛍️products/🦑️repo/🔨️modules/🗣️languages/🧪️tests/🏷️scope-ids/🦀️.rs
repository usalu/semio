//! 🦀️ Rust side of the scope identifier case.

use semio_framework_repo_languages::{build_scope_id, build_scopes_for_file};
use semio_repo_test_host::{Adapter, Context, Json, Outcome};

// #region 🔖️Projection

const VECTORS: &[(&str, &str, &str, &str)] = &[
    ("file", "a/b.ts", "", ""),
    ("section", "a/b.ts", "Outer.Inner", ""),
    ("definition", "a/b.ts", "Outer", "alpha"),
    ("definition", "a/b.ts", "", "alpha"),
];

const SOURCES: &[&str] = &["🟦️sample.ts", "🐹️sample.go", "📰️sample.md"];

// #endregion 🔖️Projection

// #region 🔖️Scenarios

fn scope_id_grammar(_ctx: &Context) -> Result<Outcome, String> {
    Ok(Outcome::projection(Json::Array(
        VECTORS
            .iter()
            .map(|(kind, path, section, definition)| {
                Json::Object(vec![
                    ("kind".to_string(), Json::String((*kind).to_string())),
                    ("sectionPath".to_string(), Json::String((*section).to_string())),
                    ("definition".to_string(), Json::String((*definition).to_string())),
                    ("id".to_string(), Json::String(build_scope_id(kind, path, section, definition))),
                ])
            })
            .collect(),
    )))
}

fn scopes_of_a_source_file(ctx: &Context) -> Result<Outcome, String> {
    let mut entries = Vec::new();
    for name in SOURCES {
        let bytes = ctx.fixture_bytes(&format!("shared://{name}"))?;
        let content = String::from_utf8(bytes).map_err(|e| e.to_string())?;
        let scopes = build_scopes_for_file(name, &content);
        entries.push((
            (*name).to_string(),
            Json::Array(
                scopes
                    .iter()
                    .map(|s| {
                        Json::Object(vec![
                            ("kind".to_string(), Json::String(s.kind.clone())),
                            ("id".to_string(), Json::String(s.id.clone())),
                            ("sectionPath".to_string(), Json::String(s.section_path.clone())),
                            ("definition".to_string(), Json::String(s.definition.clone())),
                            ("startLine".to_string(), Json::Number(s.start_line as f64)),
                            ("endLine".to_string(), Json::Number(s.end_line as f64)),
                        ])
                    })
                    .collect(),
            ),
        ));
    }
    Ok(Outcome::projection(Json::Object(entries)))
}

// #endregion 🔖️Scenarios

// #region 🔖️Registration

/// 🧭️ Registration entry point the generated host calls.
pub fn adapter() -> Adapter {
    Adapter::new("rust")
        .subject("scope-id-grammar", scope_id_grammar)
        .subject("scopes-of-a-source-file", scopes_of_a_source_file)
}

// #endregion 🔖️Registration
