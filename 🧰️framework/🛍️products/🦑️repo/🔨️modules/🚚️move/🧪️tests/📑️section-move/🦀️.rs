//! 🦀️ Rust side of the section move case. The subject half is gated behind the `sut` feature so the
//! oracle role never compiles the implementation under test.

use semio_repo_test_host::{Adapter, Context, Json, Outcome};

//#region 🔖️Helpers

/// 🎬️ Applies a section rename to a one-file workspace and returns the new content.
#[cfg(feature = "sut")]
fn rename(file: &str, content: &str, old: &str, new: &str) -> Result<(String, Vec<String>), String> {
    let workspace = semio_framework_repo_move::Workspace::new().with_file(file, content);
    let plan = semio_framework_repo_move::plan_section_move(&workspace, file, old, new).map_err(|error| error.0)?;
    let mut applied = workspace;
    semio_framework_repo_move::execute(&plan, &mut applied).map_err(|error| error.0)?;
    Ok((applied.file(file).unwrap_or_default().to_string(), plan.lines()))
}

/// ⚖️ Fails the scenario when the produced value is not the recorded one.
#[cfg(feature = "sut")]
fn expect(name: &str, field: &str, expected: &Json, produced: &Json) -> Result<(), String> {
    if expected == produced {
        return Ok(());
    }
    Err(format!("{name}: {field} differs from the recorded expectation\nrecorded: {}\nproduced: {}", expected.to_string(), produced.to_string()))
}

//#endregion 🔖️Helpers

//#region 🔖️Scenarios

#[cfg(feature = "sut")]
fn markers_follow_the_new_name(ctx: &Context) -> Result<Outcome, String> {
    let file = ctx.fixture_json("shared://📑️section-move-trees.json")?;
    let mut renamed = Vec::new();
    for vector in file.array("cases") {
        let name = vector.str("name");
        let (content, lines) = rename(&vector.str("file"), &vector.str("content"), &vector.str("oldPath"), &vector.str("newPath")).map_err(|error| format!("{name}: {error}"))?;
        let messages = Json::Array(lines.into_iter().map(Json::String).collect());
        expect(&name, "renamed file", &Json::String(vector.str("expected")), &Json::String(content.clone()))?;
        expect(&name, "output lines", vector.get("messages").unwrap_or(&Json::Null), &messages)?;
        renamed.push(Json::Object(vec![("name".to_string(), Json::String(name)), ("content".to_string(), Json::String(content)), ("messages".to_string(), messages)]));
    }
    Ok(Outcome::projection(Json::Object(vec![("renamed".to_string(), Json::Array(renamed))])))
}

#[cfg(feature = "sut")]
fn renaming_back_restores_the_file(ctx: &Context) -> Result<Outcome, String> {
    let file = ctx.fixture_json("shared://📑️section-move-trees.json")?;
    let mut round_trips = Vec::new();
    for vector in file.array("cases") {
        let name = vector.str("name");
        let path = vector.str("file");
        let original = vector.str("content");
        let old = vector.str("oldPath");
        let new = vector.str("newPath");
        let (forward, _) = rename(&path, &original, &old, &new).map_err(|error| format!("{name}: {error}"))?;
        let (restored, _) = rename(&path, &forward, &new, &old).map_err(|error| format!("{name}: {error}"))?;
        expect(&name, "restored file", &Json::String(original), &Json::String(restored.clone()))?;
        round_trips.push(Json::Object(vec![("name".to_string(), Json::String(name)), ("restored".to_string(), Json::String(restored))]));
    }
    Ok(Outcome::projection(Json::Object(vec![("roundTrips".to_string(), Json::Array(round_trips))])))
}

#[cfg(feature = "sut")]
fn a_missing_file_is_refused(ctx: &Context) -> Result<Outcome, String> {
    let file = ctx.fixture_json("shared://📑️section-move-trees.json")?;
    let workspace = semio_framework_repo_move::Workspace::new().with_file("present.ts", "// #region 🔖️Alpha\n// #endregion 🔖️Alpha\n");
    let mut refusals = Vec::new();
    for vector in file.array("errors") {
        let name = vector.str("name");
        let message = match semio_framework_repo_move::plan_section_move(&workspace, &vector.str("file"), &vector.str("oldPath"), &vector.str("newPath")) {
            Ok(_) => return Err(format!("{name}: the rename was accepted but the vector records a refusal")),
            Err(error) => error.0,
        };
        expect(&name, "refusal message", &Json::String(vector.str("expectedError")), &Json::String(message.clone()))?;
        refusals.push(Json::Object(vec![("name".to_string(), Json::String(name)), ("message".to_string(), Json::String(message))]));
    }
    Ok(Outcome::projection(Json::Object(vec![("refusals".to_string(), Json::Array(refusals))])))
}

//#endregion 🔖️Scenarios

//#region 🔖️Registration

/// 🧭️ Registration entry point the generated host calls.
pub fn adapter() -> Adapter {
    let adapter = Adapter::new("rust");
    #[cfg(feature = "sut")]
    let adapter = adapter
        .subject("markers-follow-the-new-name", markers_follow_the_new_name)
        .subject("renaming-back-restores-the-file", renaming_back_restores_the_file)
        .subject("a-missing-file-is-refused", a_missing_file_is_refused);
    adapter
}

//#endregion 🔖️Registration
