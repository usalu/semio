//! 🦀️ Rust side of the runner-detection case: the manifest order and the JavaScript runner choice.

use semio_framework_repo_test_runner as subject;
use semio_repo_test_host::{Adapter, Context, Json, Outcome};

//#region 🔖️Support
fn vectors(ctx: &Context) -> Result<subject::DetectionVectors, String> {
    subject::parse_detection_vectors(&ctx.fixture_bytes("shared://🧭️detection-vectors.json")?)
}

fn strings(values: &[String]) -> Json {
    Json::Array(values.iter().map(|value| Json::String(value.clone())).collect())
}

fn js_argv(fixture: &subject::DetectionVectors, bundle_root: &str, filter: &str) -> Vec<String> {
    let (_, args) = subject::detect_js_test_runner(&fixture.snapshot, &fixture.snapshot.absolute(bundle_root), filter);
    args
}
//#endregion 🔖️Support

//#region 🔖️Scenarios
fn manifest_selects_the_language(ctx: &Context) -> Result<Outcome, String> {
    let fixture = vectors(ctx)?;
    let mut rows = Vec::new();
    let mut wrong = Vec::new();
    for vector in &fixture.vectors {
        let language = subject::detect_bundle_language(&fixture.snapshot, &vector.bundle_root);
        if language != vector.expected_language {
            wrong.push(format!("{}: detected {language:?}, vector declares {:?}", vector.id, vector.expected_language));
        }
        rows.push(Json::Object(vec![
            ("id".to_string(), Json::String(vector.id.clone())),
            ("language".to_string(), Json::String(language)),
        ]));
    }
    if !wrong.is_empty() {
        return Err(wrong.join("; "));
    }
    Ok(Outcome::projection(Json::Object(vec![("detected".to_string(), Json::Array(rows))])))
}

fn javascript_manifest_selects_the_runner(ctx: &Context) -> Result<Outcome, String> {
    let fixture = vectors(ctx)?;
    let mut rows = Vec::new();
    let mut wrong = Vec::new();
    for vector in &fixture.vectors {
        let Some(expected) = vector.expected_argv.as_ref() else { continue };
        let (runner, args) = subject::detect_js_test_runner(&fixture.snapshot, &fixture.snapshot.absolute(&vector.bundle_root), &vector.test_filter);
        if args != *expected {
            wrong.push(format!("{}: detected {args:?}, vector declares {expected:?}", vector.id));
        }
        rows.push(Json::Object(vec![
            ("id".to_string(), Json::String(vector.id.clone())),
            ("program".to_string(), Json::String(runner.program().to_string())),
            ("args".to_string(), strings(&args)),
        ]));
    }
    if !wrong.is_empty() {
        return Err(wrong.join("; "));
    }
    Ok(Outcome::projection(Json::Object(vec![("runners".to_string(), Json::Array(rows))])))
}

fn detection_is_idempotent_and_filter_only_appends(ctx: &Context) -> Result<Outcome, String> {
    let fixture = vectors(ctx)?;
    let mut stable = true;
    let mut appends = true;
    for vector in &fixture.vectors {
        let first = subject::detect_bundle_language(&fixture.snapshot, &vector.bundle_root);
        let second = subject::detect_bundle_language(&fixture.snapshot, &vector.bundle_root);
        stable = stable && first == second;
        let bare = js_argv(&fixture, &vector.bundle_root, "");
        let filtered = js_argv(&fixture, &vector.bundle_root, "renders");
        appends = appends && filtered.len() >= bare.len() && filtered[..bare.len()] == bare[..];
    }
    if !stable || !appends {
        return Err(format!("detection stability {stable}, filter-appends law {appends}"));
    }
    Ok(Outcome::projection(Json::Object(vec![
        ("detectionIsStable".to_string(), Json::Bool(stable)),
        ("filterOnlyAppends".to_string(), Json::Bool(appends)),
        ("vectors".to_string(), Json::Number(fixture.vectors.len() as f64)),
    ])))
}
//#endregion 🔖️Scenarios

//#region 🔖️Registration
/// 🧭️ Registration entry point the generated host calls.
pub fn adapter() -> Adapter {
    Adapter::new("rust")
        .subject("manifest-selects-the-language", manifest_selects_the_language)
        .subject("javascript-manifest-selects-the-runner", javascript_manifest_selects_the_runner)
        .subject("detection-is-idempotent-and-filter-only-appends", detection_is_idempotent_and_filter_only_appends)
}
//#endregion 🔖️Registration
