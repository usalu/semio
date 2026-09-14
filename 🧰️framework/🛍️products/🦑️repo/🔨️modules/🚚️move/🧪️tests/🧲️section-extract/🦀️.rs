//! 🦀️ Rust side of the section extract case. The subject half is gated behind the `sut` feature so
//! the oracle role never compiles the implementation under test.

use semio_repo_test_host::{Adapter, Context, Json, Outcome};

//#region 🔖️Helpers

/// 🧲️ Extracts a section out of a one-file workspace and returns the two resulting files.
#[cfg(feature = "sut")]
fn extract(source: &str, content: &str, section: &str, target: &str) -> Result<(String, String, Vec<String>), String> {
    let workspace = semio_framework_repo_move::Workspace::new().with_file(source, content);
    let plan = semio_framework_repo_move::plan_extract(&workspace, source, section, target).map_err(|error| error.0)?;
    let mut applied = workspace;
    semio_framework_repo_move::execute(&plan, &mut applied).map_err(|error| error.0)?;
    Ok((applied.file(target).unwrap_or_default().to_string(), applied.file(source).unwrap_or_default().to_string(), plan.lines()))
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
fn the_section_leaves_with_its_imports(ctx: &Context) -> Result<Outcome, String> {
    let file = ctx.fixture_json("shared://🧲️section-extract-trees.json")?;
    let mut extracted = Vec::new();
    for vector in file.array("cases") {
        let name = vector.str("name");
        let (target, source, lines) = extract(&vector.str("sourceFile"), &vector.str("content"), &vector.str("section"), &vector.str("targetFile")).map_err(|error| format!("{name}: {error}"))?;
        let messages = Json::Array(lines.into_iter().map(Json::String).collect());
        expect(&name, "extracted file", &Json::String(vector.str("expectedTarget")), &Json::String(target.clone()))?;
        expect(&name, "remaining source", &Json::String(vector.str("expectedSource")), &Json::String(source.clone()))?;
        expect(&name, "output lines", vector.get("messages").unwrap_or(&Json::Null), &messages)?;
        extracted.push(Json::Object(vec![
            ("name".to_string(), Json::String(name)),
            ("target".to_string(), Json::String(target)),
            ("source".to_string(), Json::String(source)),
            ("messages".to_string(), messages),
        ]));
    }
    Ok(Outcome::projection(Json::Object(vec![("extracted".to_string(), Json::Array(extracted))])))
}

#[cfg(feature = "sut")]
fn extraction_removes_exactly_the_section(ctx: &Context) -> Result<Outcome, String> {
    let file = ctx.fixture_json("shared://🧲️section-extract-trees.json")?;
    let mut arithmetic = Vec::new();
    for vector in file.array("cases") {
        let name = vector.str("name");
        let content = vector.str("content");
        let source_path = vector.str("sourceFile");
        let sections = semio_framework_repo_move::parse_sections(&content, &source_path);
        let section = semio_framework_repo_move::find_section(&sections, &vector.str("section")).ok_or_else(|| format!("{name}: the recorded section is not in the file"))?;
        let span = section.end_line - section.start_line + 1;
        let (_, remaining, _) = extract(&source_path, &content, &vector.str("section"), &vector.str("targetFile")).map_err(|error| format!("{name}: {error}"))?;
        let before = content.split('\n').count() as i64;
        let after = remaining.split('\n').count() as i64;
        if before - after != span {
            return Err(format!("{name}: the source lost {} lines but the section spans {span}", before - after));
        }
        arithmetic.push(Json::String(format!("{name}: before={before} after={after} span={span}")));
    }
    Ok(Outcome::projection(Json::Object(vec![("arithmetic".to_string(), Json::Array(arithmetic))])))
}

#[cfg(feature = "sut")]
fn a_missing_section_is_refused(ctx: &Context) -> Result<Outcome, String> {
    let file = ctx.fixture_json("shared://🧲️section-extract-trees.json")?;
    let sample = file
        .array("cases")
        .into_iter()
        .find(|vector| vector.str("sourceFile") == "sample.ts")
        .ok_or_else(|| "the vector set carries no sample.ts case".to_string())?;
    let workspace = semio_framework_repo_move::Workspace::new().with_file("sample.ts", &sample.str("content")).with_file("plain.txt", "no sections here\n");
    let mut refusals = Vec::new();
    for vector in file.array("errors") {
        let name = vector.str("name");
        let message = match semio_framework_repo_move::plan_extract(&workspace, &vector.str("sourceFile"), &vector.str("section"), &vector.str("targetFile")) {
            Ok(_) => return Err(format!("{name}: the extraction was accepted but the vector records a refusal")),
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
        .subject("the-section-leaves-with-its-imports", the_section_leaves_with_its_imports)
        .subject("extraction-removes-exactly-the-section", extraction_removes_exactly_the_section)
        .subject("a-missing-section-is-refused", a_missing_section_is_refused);
    adapter
}

//#endregion 🔖️Registration
