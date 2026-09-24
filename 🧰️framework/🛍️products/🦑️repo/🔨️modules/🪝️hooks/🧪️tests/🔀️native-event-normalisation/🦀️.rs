//! 🦀️ Rust side of the native event normalisation case. The subject halves are gated behind the
//! `sut` feature the generated host turns on for the subject role only.

use semio_repo_test_host::Adapter;

//#region 🔖️Scenarios
#[cfg(feature = "sut")]
mod subject {
    use semio_framework_repo_hooks::{classify_command_kind, classify_tool, extract_command_and_cwd, extract_effort, extract_hook_event_name, extract_llm, extract_message_id, extract_session_id, extract_tool_name, extract_transcript, resolve_hook_event, resolve_parent_session_id};
    use semio_framework_repo_hooks::serde_json::{self, Value};
    use semio_repo_test_host::{Context, Json, Outcome};

    const VECTORS: &str = "shared://🔀️native-event-normalisation/🔀️native-events.json";

    /// 📥️ Reads the fixture with the crate's own JSON reader so the payloads reach the subject in
    /// exactly the shape an IDE would send.
    fn vectors(ctx: &Context) -> Result<Value, String> {
        let bytes = ctx.fixture_bytes(VECTORS)?;
        serde_json::from_slice(&bytes).map_err(|error| error.to_string())
    }

    fn text(value: &Value, key: &str) -> String {
        value.get(key).and_then(Value::as_str).unwrap_or_default().to_string()
    }

    fn rows(value: &Value, key: &str) -> Vec<Value> {
        value.get(key).and_then(Value::as_array).cloned().unwrap_or_default()
    }

    /// 🔀️ Every native event resolves to its pinned neutral event.
    pub fn every_native_event_resolves_to_its_pinned_neutral_event(ctx: &Context) -> Result<Outcome, String> {
        let vectors = vectors(ctx)?;
        let mut projected = Vec::new();
        for vector in rows(&vectors, "vectors") {
            let id = text(&vector, "id");
            let client = text(&vector, "client");
            let native = text(&vector, "nativeEvent");
            let tool = text(&vector, "toolName");
            let payload = vector.get("payload").cloned();
            let (event, parent) = resolve_hook_event(&native, &client, &tool, payload.as_ref()).map_err(|error| format!("{id}: {}", error.message))?;
            let expected_event = text(&vector, "expectedEvent");
            let expected_parent = text(&vector, "expectedParent");
            projected.push((
                id,
                Json::Object(vec![
                    ("event".to_string(), Json::String(event.as_str().to_string())),
                    ("parent".to_string(), Json::String(parent.clone())),
                    ("matchesSpecification".to_string(), Json::Bool(event.as_str() == expected_event && parent == expected_parent)),
                ]),
            ));
        }
        Ok(Outcome::projection(Json::Object(projected)))
    }

    /// 🏛️ The tool name, the command and the pair are classified the same way everywhere.
    pub fn the_command_outranks_the_tool_name_when_it_is_more_specific(ctx: &Context) -> Result<Outcome, String> {
        let vectors = vectors(ctx)?;
        let mut projected = Vec::new();
        for vector in rows(&vectors, "vectors") {
            let id = text(&vector, "id");
            let tool = text(&vector, "toolName");
            let payload = vector.get("payload").cloned();
            let (command, _) = extract_command_and_cwd(payload.as_ref());
            projected.push((
                id,
                Json::Object(vec![
                    ("byToolName".to_string(), Json::String(classify_tool(&tool).as_str().to_string())),
                    ("command".to_string(), Json::String(command.clone())),
                    ("byCommand".to_string(), Json::String(if command.is_empty() { String::new() } else { classify_command_kind(&command).as_str().to_string() })),
                ]),
            ));
        }
        Ok(Outcome::projection(Json::Object(projected)))
    }

    /// ⚠️ A native event a client does not know is refused.
    pub fn an_unrecognised_native_event_is_refused(ctx: &Context) -> Result<Outcome, String> {
        let vectors = vectors(ctx)?;
        let mut projected = Vec::new();
        for vector in rows(&vectors, "refusals") {
            let id = text(&vector, "id");
            let client = text(&vector, "client");
            let native = text(&vector, "nativeEvent");
            let tool = text(&vector, "toolName");
            let answer = match resolve_hook_event(&native, &client, &tool, None) {
                Ok((event, parent)) => Json::Object(vec![("refused".to_string(), Json::Bool(false)), ("event".to_string(), Json::String(event.as_str().to_string())), ("parent".to_string(), Json::String(parent))]),
                Err(error) => Json::Object(vec![("refused".to_string(), Json::Bool(true)), ("message".to_string(), Json::String(error.message))]),
            };
            projected.push((id, answer));
        }
        Ok(Outcome::projection(Json::Object(projected)))
    }

    /// 🪆️ Neutral facts are found in every payload shape.
    pub fn neutral_facts_are_read_out_of_every_payload_shape(ctx: &Context) -> Result<Outcome, String> {
        let vectors = vectors(ctx)?;
        let mut projected = Vec::new();
        for vector in rows(&vectors, "payloadFacts") {
            let id = text(&vector, "id");
            let payload = vector.get("payload").cloned();
            let input = payload.as_ref();
            let (command, cwd) = extract_command_and_cwd(input);
            projected.push((
                id,
                Json::Object(vec![
                    ("session".to_string(), Json::String(extract_session_id(input))),
                    ("transcript".to_string(), Json::String(extract_transcript(input))),
                    ("toolName".to_string(), Json::String(extract_tool_name(input))),
                    ("command".to_string(), Json::String(command)),
                    ("cwd".to_string(), Json::String(cwd)),
                    ("parent".to_string(), Json::String(resolve_parent_session_id("", input))),
                    ("llm".to_string(), Json::String(extract_llm(input))),
                    ("effort".to_string(), Json::String(extract_effort(input))),
                    ("messageId".to_string(), Json::String(extract_message_id(input))),
                    ("hookEventName".to_string(), Json::String(extract_hook_event_name(input))),
                ]),
            ));
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
        .subject("every-native-event-resolves-to-its-pinned-neutral-event", subject::every_native_event_resolves_to_its_pinned_neutral_event)
        .subject("the-command-outranks-the-tool-name-when-it-is-more-specific", subject::the_command_outranks_the_tool_name_when_it_is_more_specific)
        .subject("an-unrecognised-native-event-is-refused", subject::an_unrecognised_native_event_is_refused)
        .subject("neutral-facts-are-read-out-of-every-payload-shape", subject::neutral_facts_are_read_out_of_every_payload_shape);
    registered
}
//#endregion 🔖️Registration
