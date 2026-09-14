//! 🦀️ Rust side of the file header case.

use semio_framework_repo_languages::{format_header, language_by_name, parse_header, parse_sections_with, table, Header};
use semio_repo_test_host::{Adapter, Context, Json, Outcome};

// #region 🔖️Projection

fn fields() -> Header {
    Header {
        file_id: "💻️test/file".to_string(),
        file_uri: "repo://file/💻️test".to_string(),
        summary: "A test file".to_string(),
        contributors: "2025 Test User <test@test.com>".to_string(),
        license: "AGPL license text here".to_string(),
        requirements: "Some requirements".to_string(),
    }
}

// #endregion 🔖️Projection

// #region 🔖️Scenarios

fn header_per_language(_ctx: &Context) -> Result<Outcome, String> {
    let header = fields();
    let mut entries = Vec::new();
    for name in &table().registry {
        let lang = language_by_name(name).ok_or_else(|| format!("{name} is not registered"))?;
        entries.push((name.clone(), Json::String(format_header(lang, &header))));
    }
    Ok(Outcome::projection(Json::Object(entries)))
}

fn header_region_is_parseable(_ctx: &Context) -> Result<Outcome, String> {
    let header = fields();
    let mut entries = Vec::new();
    for name in &table().registry {
        let lang = language_by_name(name).ok_or_else(|| format!("{name} is not registered"))?;
        if !lang.supports_headers {
            continue;
        }
        let text = format_header(lang, &header);
        let sections = parse_sections_with(lang, &text);
        let parsed = parse_header(lang, &text).ok_or_else(|| format!("{name} header did not parse"))?;
        entries.push((
            name.clone(),
            Json::Object(vec![
                ("sectionCount".to_string(), Json::Number(sections.len() as f64)),
                ("sectionName".to_string(), Json::String(sections.first().map(|s| s.name.clone()).unwrap_or_default())),
                ("sectionStartLine".to_string(), Json::Number(sections.first().map_or(0, |s| s.start_line) as f64)),
                ("sectionEndLine".to_string(), Json::Number(sections.first().map_or(0, |s| s.end_line) as f64)),
                ("fileId".to_string(), Json::String(parsed.file_id.clone())),
                ("fileUri".to_string(), Json::String(parsed.file_uri.clone())),
                ("contributors".to_string(), Json::String(parsed.contributors.clone())),
                ("license".to_string(), Json::String(parsed.license.clone())),
            ]),
        ));
    }
    Ok(Outcome::projection(Json::Object(entries)))
}

// #endregion 🔖️Scenarios

// #region 🔖️Registration

/// 🧭️ Registration entry point the generated host calls.
pub fn adapter() -> Adapter {
    Adapter::new("rust")
        .subject("header-per-language", header_per_language)
        .subject("header-region-is-parseable", header_region_is_parseable)
}

// #endregion 🔖️Registration
