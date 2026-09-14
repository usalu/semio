//! 🦀️ Rust side of the file integrate case. The subject half is gated behind the `sut` feature so
//! the oracle role never compiles the implementation under test.

use semio_repo_test_host::{Adapter, Context, Json, Outcome};

//#region 🔖️Helpers

/// 🧬️ Integrates one file into a section of another and returns the new target content.
#[cfg(feature = "sut")]
fn integrate(vector: &Json) -> Result<(String, Vec<String>), String> {
    let source = vector.str("sourceFile");
    let target = vector.str("targetFile");
    let workspace = semio_framework_repo_move::Workspace::new().with_file(&source, &vector.str("sourceContent")).with_file(&target, &vector.str("targetContent"));
    let plan = semio_framework_repo_move::plan_integrate(&workspace, &source, &vector.str("targetSection"), &target, &vector.str("parentSection")).map_err(|error| error.0)?;
    let mut applied = workspace;
    semio_framework_repo_move::execute(&plan, &mut applied).map_err(|error| error.0)?;
    Ok((applied.file(&target).unwrap_or_default().to_string(), plan.lines()))
}

/// 🏷️ Every section name of a file, parents before children, in document order.
#[cfg(feature = "sut")]
fn section_names(sections: &[semio_framework_repo_move::Section], out: &mut Vec<String>) {
    for section in sections {
        out.push(section.name.clone());
        section_names(&section.children, out);
    }
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
fn the_source_lands_inside_the_markers(ctx: &Context) -> Result<Outcome, String> {
    let file = ctx.fixture_json("shared://📥️file-integrate-trees.json")?;
    let mut integrated = Vec::new();
    for vector in file.array("cases") {
        let name = vector.str("name");
        let (content, lines) = integrate(&vector).map_err(|error| format!("{name}: {error}"))?;
        let messages = Json::Array(lines.into_iter().map(Json::String).collect());
        expect(&name, "integrated file", &Json::String(vector.str("expected")), &Json::String(content.clone()))?;
        expect(&name, "output lines", vector.get("messages").unwrap_or(&Json::Null), &messages)?;
        integrated.push(Json::Object(vec![("name".to_string(), Json::String(name)), ("content".to_string(), Json::String(content)), ("messages".to_string(), messages)]));
    }
    Ok(Outcome::projection(Json::Object(vec![("integrated".to_string(), Json::Array(integrated))])))
}

#[cfg(feature = "sut")]
fn the_new_section_is_parseable(ctx: &Context) -> Result<Outcome, String> {
    let file = ctx.fixture_json("shared://📥️file-integrate-trees.json")?;
    let mut read_back = Vec::new();
    for vector in file.array("cases") {
        let name = vector.str("name");
        let target = vector.str("targetFile");
        let (content, _) = integrate(&vector).map_err(|error| format!("{name}: {error}"))?;
        let mut names = Vec::new();
        section_names(&semio_framework_repo_move::parse_sections(&content, &target), &mut names);
        let wanted = vector.str("targetSection");
        if !names.contains(&wanted) {
            return Err(format!("{name}: the section {wanted} written by the integration is not found again, only {}", names.join(", ")));
        }
        read_back.push(Json::Object(vec![("name".to_string(), Json::String(name)), ("sections".to_string(), Json::Array(names.into_iter().map(Json::String).collect()))]));
    }
    Ok(Outcome::projection(Json::Object(vec![("readBack".to_string(), Json::Array(read_back))])))
}

#[cfg(feature = "sut")]
fn an_unknown_parent_section_is_refused(ctx: &Context) -> Result<Outcome, String> {
    let file = ctx.fixture_json("shared://📥️file-integrate-trees.json")?;
    let sample = file
        .array("cases")
        .into_iter()
        .find(|vector| vector.str("targetFile") == "sample.ts")
        .ok_or_else(|| "the vector set carries no sample.ts case".to_string())?;
    let workspace = semio_framework_repo_move::Workspace::new()
        .with_file("gamma.ts", &sample.str("sourceContent"))
        .with_file("sample.ts", &sample.str("targetContent"))
        .with_file("plain.txt", "no sections here\n");
    let mut refusals = Vec::new();
    for vector in file.array("errors") {
        let name = vector.str("name");
        let message = match semio_framework_repo_move::plan_integrate(&workspace, &vector.str("sourceFile"), &vector.str("targetSection"), &vector.str("targetFile"), &vector.str("parentSection")) {
            Ok(_) => return Err(format!("{name}: the integration was accepted but the vector records a refusal")),
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
        .subject("the-source-lands-inside-the-markers", the_source_lands_inside_the_markers)
        .subject("the-new-section-is-parseable", the_new_section_is_parseable)
        .subject("an-unknown-parent-section-is-refused", an_unknown_parent_section_is_refused);
    adapter
}

//#endregion 🔖️Registration
