//! 🦀️ Rust side of the definition parsing case.

use semio_framework_repo_languages::{language_by_name, parse_definition_ranges};
use semio_repo_test_host::{Adapter, Context, Json, Outcome};

// #region 🔖️Projection

fn declarations(ctx: &Context, uri: &str) -> Result<Json, String> {
    let bytes = ctx.fixture_bytes(uri)?;
    let content = String::from_utf8(bytes).map_err(|e| e.to_string())?;
    let lines: Vec<&str> = content.split('\n').collect();
    let lang = language_by_name("typescript").ok_or("typescript is not registered")?;
    Ok(Json::Array(
        parse_definition_ranges(lang, &lines)
            .iter()
            .map(|r| {
                Json::Object(vec![
                    ("name".to_string(), Json::String(r.name.clone())),
                    ("startLine".to_string(), Json::Number(r.start as f64)),
                    ("kind".to_string(), Json::String(r.kind.clone())),
                ])
            })
            .collect(),
    ))
}

// #endregion 🔖️Projection

// #region 🔖️Scenarios

fn typescript_top_level_declarations(ctx: &Context) -> Result<Outcome, String> {
    Ok(Outcome::projection(declarations(ctx, "shared://🟦️sample.ts")?))
}

fn callable_const_is_a_function(ctx: &Context) -> Result<Outcome, String> {
    Ok(Outcome::projection(declarations(ctx, "local://🔤️callables.ts")?))
}

// #endregion 🔖️Scenarios

// #region 🔖️Registration

/// 🧭️ Registration entry point the generated host calls.
pub fn adapter() -> Adapter {
    Adapter::new("rust")
        .subject("typescript-top-level-declarations", typescript_top_level_declarations)
        .subject("callable-const-is-a-function", callable_const_is_a_function)
}

// #endregion 🔖️Registration
