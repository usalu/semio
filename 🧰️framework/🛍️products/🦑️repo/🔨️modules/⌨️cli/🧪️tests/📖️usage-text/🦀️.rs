//! 🦀️ Rust subject for the usage-text case. Gated behind the `sut` feature like every subject, so
//! the oracle role never links the crate under test.

#[cfg(feature = "sut")]
mod subject {
    use semio_framework_repo_cli::repo_cli;
    use semio_repo_test_host::{Context, Json, Outcome};

    //#region 🔖️Helpers
    /// 📥️ The committed goldens every scenario reads.
    fn goldens(ctx: &Context) -> Result<Json, String> {
        ctx.fixture_json("local://📖️usage-goldens.json")
    }

    /// 📜️ A `/`-joined command path as its segments.
    fn segments(path: &str) -> Vec<String> {
        path.split('/').filter(|segment| !segment.is_empty()).map(str::to_string).collect()
    }
    //#endregion 🔖️Helpers

    //#region 🔖️Scenarios
    /// 📖️ Every golden command produces its stated usage text byte for byte.
    pub fn representative_commands_state_their_text(ctx: &Context) -> Result<Outcome, String> {
        let document = goldens(ctx)?;
        let mut rows: Vec<Json> = Vec::new();
        for golden in document.array("goldens") {
            let path: Vec<String> = golden
                .array("path")
                .into_iter()
                .map(|item| match item {
                    Json::String(text) => text,
                    other => other.to_string(),
                })
                .collect();
            let actual = repo_cli::usage(&path).ok_or_else(|| format!("{path:?}: no such command"))?;
            let wanted = golden.str("text");
            if actual != wanted {
                return Err(format!("{path:?}: expected {wanted:?} got {actual:?}"));
            }
            rows.push(Json::Object(vec![("path".to_string(), Json::String(path.join("/"))), ("text".to_string(), Json::String(actual))]));
        }
        Ok(Outcome::projection(Json::Object(vec![("goldens".to_string(), Json::Array(rows))])))
    }

    /// 📐️ Every command in the tree obeys the stated structure.
    pub fn every_command_obeys_the_usage_law(ctx: &Context) -> Result<Outcome, String> {
        let document = goldens(ctx)?;
        let structure = document.get("structure").cloned().ok_or("the goldens state no structure")?;
        let usage_marker = structure.str("usageMarker");
        let commands_marker = structure.str("commandsMarker");
        let child_indent = structure.str("childIndent");
        let mut rows: Vec<Json> = Vec::new();
        for path in repo_cli::command_paths() {
            let text = repo_cli::usage(&segments(&path)).ok_or_else(|| format!("{path}: no such command"))?;
            if !text.contains(&usage_marker) {
                return Err(format!("{path}: the usage marker is missing"));
            }
            if text.starts_with(&usage_marker) {
                return Err(format!("{path}: the summary is missing"));
            }
            if let Some(index) = text.find(&commands_marker) {
                let body = &text[index + commands_marker.len()..];
                for line in body.lines().filter(|line| !line.is_empty()) {
                    if !line.starts_with(&child_indent) {
                        return Err(format!("{path}: the child line {line:?} is not indented"));
                    }
                }
            }
            rows.push(Json::Object(vec![("path".to_string(), Json::String(path)), ("lines".to_string(), Json::Number(text.lines().count() as f64))]));
        }
        if !repo_cli::usage(&Vec::new()).ok_or("no root command")?.starts_with(&structure.str("rootPrefix")) {
            return Err("the root usage text does not open with its stated prefix".to_string());
        }
        Ok(Outcome::projection(Json::Object(vec![("commands".to_string(), Json::Array(rows))])))
    }

    /// 🗺️ The root registers exactly the stated verbs, in order.
    pub fn the_root_registers_its_stated_verbs(ctx: &Context) -> Result<Outcome, String> {
        let document = goldens(ctx)?;
        let wanted: Vec<String> = document
            .array("rootVerbs")
            .into_iter()
            .map(|item| match item {
                Json::String(text) => text,
                other => other.to_string(),
            })
            .collect();
        let actual: Vec<String> = repo_cli::command_paths().into_iter().filter(|path| !path.is_empty() && !path.contains('/')).collect();
        if actual != wanted {
            return Err(format!("expected {wanted:?} got {actual:?}"));
        }
        Ok(Outcome::projection(Json::Object(vec![("verbs".to_string(), Json::Array(actual.into_iter().map(Json::String).collect()))])))
    }
    //#endregion 🔖️Scenarios
}

//#region 🔖️Registration
/// 🧭️ Registration entry point the generated host calls.
pub fn adapter() -> semio_repo_test_host::Adapter {
    let adapter = semio_repo_test_host::Adapter::new("rust");
    #[cfg(feature = "sut")]
    let adapter = adapter
        .subject("representative-commands-state-their-text", subject::representative_commands_state_their_text)
        .subject("every-command-obeys-the-usage-law", subject::every_command_obeys_the_usage_law)
        .subject("the-root-registers-its-stated-verbs", subject::the_root_registers_its_stated_verbs);
    adapter
}
//#endregion 🔖️Registration
