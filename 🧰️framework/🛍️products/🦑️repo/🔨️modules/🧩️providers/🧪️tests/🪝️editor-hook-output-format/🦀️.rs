//! 🦀️ Rust side of the editor hook output case. The subject halves are gated behind the `sut`
//! feature the generated host turns on for the subject role only.

use semio_repo_test_host::Adapter;

//#region 🔖️Scenarios
#[cfg(feature = "sut")]
mod subject {
    use semio_framework_repo_providers::{get_editor_provider, EditorProvider, HookEvent, HookResult, ToolKind};
    use semio_repo_test_host::{parse_json, Context, Json, Outcome};

    const VECTORS: &str = "shared://🪝️editor-hook-output-format/🪝️hook-outputs.json";

    fn strings(value: &Json, key: &str) -> Vec<String> {
        value
            .array(key)
            .into_iter()
            .map(|item| match item {
                Json::String(text) => text,
                other => other.to_string(),
            })
            .collect()
    }

    /// 🪝️ Every editor formats every fixture result.
    pub fn every_editor_formats_the_same_result(ctx: &Context) -> Result<Outcome, String> {
        let vectors = ctx.fixture_json(VECTORS)?;
        let editors = strings(&vectors, "editors");
        let mut projected = Vec::new();
        for record in vectors.array("records") {
            let id = record.str("id");
            let hook_event_name = record.str("hookEventName");
            let allowed = matches!(record.get("allowed"), Some(Json::Bool(true)));
            let result = HookResult::new(allowed, record.str("message"));
            let mut per_editor = Vec::new();
            for editor in &editors {
                let provider = get_editor_provider(editor).ok_or_else(|| format!("no editor provider for {editor}"))?;
                let formatted = provider.format_hook_output(&hook_event_name, &result);
                per_editor.push((editor.clone(), parse_json(&formatted)?));
            }
            projected.push((id, Json::Object(per_editor)));
        }
        Ok(Outcome::projection(Json::Object(projected)))
    }

    /// 🔤️ Every editor names every neutral hook event.
    pub fn native_event_names_are_derived_the_same_way(ctx: &Context) -> Result<Outcome, String> {
        let vectors = ctx.fixture_json(VECTORS)?;
        let editors = strings(&vectors, "editors");
        let parents = strings(&vectors, "parents");
        let mut projected = Vec::new();
        for editor in &editors {
            let provider = get_editor_provider(editor).ok_or_else(|| format!("no editor provider for {editor}"))?;
            let mut names = Vec::new();
            for slug in strings(&vectors, "events") {
                let event = HookEvent::parse(&slug).ok_or_else(|| format!("fixture names an unknown hook event {slug}"))?;
                for parent in &parents {
                    names.push(Json::String(format!("{slug}|{parent}={}", provider.native_event_from_hook_event(event, parent))));
                }
            }
            projected.push((editor.clone(), Json::Array(names)));
        }
        Ok(Outcome::projection(Json::Object(projected)))
    }

    /// ⚠️ An unknown native event is refused rather than defaulted.
    pub fn an_unknown_native_event_is_rejected(ctx: &Context) -> Result<Outcome, String> {
        let vectors = ctx.fixture_json(VECTORS)?;
        let editors = strings(&vectors, "editors");
        let mut projected = Vec::new();
        for editor in &editors {
            let provider = get_editor_provider(editor).ok_or_else(|| format!("no editor provider for {editor}"))?;
            let mut resolved = Vec::new();
            for native in vectors.array("natives") {
                let name = native.str("event");
                let kind = ToolKind::parse(&native.str("toolKind"));
                let answer = match provider.resolve_native_event(&name, kind) {
                    Ok((event, parent)) => format!("{}|{parent}", event.as_str()),
                    Err(_) => "refused".to_string(),
                };
                resolved.push(Json::String(format!("{name}/{}={answer}", kind.as_str())));
            }
            projected.push((editor.clone(), Json::Array(resolved)));
        }
        Ok(Outcome::projection(Json::Object(projected)))
    }
}
//#endregion 🔖️Scenarios

//#region 🔖️Registration
/// 🧭️ Registration entry point the generated host calls.
pub fn adapter() -> Adapter {
    let registered = Adapter::new("rust");
    #[cfg(feature = "sut")]
    let registered = registered
        .subject("every-editor-formats-the-same-result", subject::every_editor_formats_the_same_result)
        .subject("native-event-names-are-derived-the-same-way", subject::native_event_names_are_derived_the_same_way)
        .subject("an-unknown-native-event-is-rejected", subject::an_unknown_native_event_is_rejected);
    registered
}
//#endregion 🔖️Registration
