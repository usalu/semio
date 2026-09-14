//! 🦀️ Rust side of the ticket document codec case, run against real committed documents.

use semio_repo_test_host::Adapter;

//#region 🔖️Scenarios
#[cfg(feature = "sut")]
mod subject {
    use semio_framework_repo_tickets::{decode_ticket_document, encode_ticket_document};
    use semio_repo_test_host::{Context, Json, Outcome};

    const VECTORS: &str = "local://📄️documents.json";

    fn documents(vectors: &Json) -> Vec<(String, String)> {
        vectors
            .array("documents")
            .into_iter()
            .map(|entry| {
                let source = entry.str("source");
                let text = entry.str("text");
                (source, text)
            })
            .collect()
    }

    fn member_names(text: &str) -> Vec<String> {
        match semio_repo_test_host::parse_json(text) {
            Ok(Json::Object(members)) => members.into_iter().map(|(name, _)| name).collect(),
            _ => Vec::new(),
        }
    }

    /// 📖️ Every committed document decodes to the same ticket.
    pub fn real_documents_decode_to_the_same_ticket(ctx: &Context) -> Result<Outcome, String> {
        let vectors = ctx.fixture_json(VECTORS)?;
        let mut projected: Vec<(String, Json)> = Vec::new();
        for (source, text) in documents(&vectors) {
            let ticket = decode_ticket_document(&text).map_err(|error| error.message)?;
            projected.push((
                source,
                Json::Object(vec![
                    ("title".to_string(), Json::String(ticket.title)),
                    ("emoji".to_string(), Json::String(ticket.emoji)),
                    ("status".to_string(), Json::String(ticket.status.as_str().to_string())),
                    ("description".to_string(), Json::String(ticket.description)),
                    ("summary".to_string(), Json::String(ticket.summary)),
                    ("issue".to_string(), Json::String(ticket.management.map(|management| management.issue).unwrap_or_default())),
                    ("goal".to_string(), Json::String(ticket.goal)),
                    ("parent".to_string(), Json::String(ticket.parent)),
                    ("plan".to_string(), Json::String(ticket.plan.map(|plan| format!("{}|{}|{}|{}", plan.client, plan.id, plan.source, plan.local)).unwrap_or_default())),
                    ("sessions".to_string(), Json::Array(ticket.sessions.into_iter().map(Json::String).collect())),
                ]),
            ));
        }
        Ok(Outcome::projection(Json::Object(projected)))
    }

    /// 🖨️ Re-encoding reproduces Go's bytes.
    pub fn encoding_is_go_marshal_indent(ctx: &Context) -> Result<Outcome, String> {
        let vectors = ctx.fixture_json(VECTORS)?;
        let mut projected: Vec<(String, Json)> = Vec::new();
        for (source, text) in documents(&vectors) {
            let ticket = decode_ticket_document(&text).map_err(|error| error.message)?;
            projected.push((source, Json::String(encode_ticket_document(&ticket))));
        }
        Ok(Outcome::projection(Json::Object(projected)))
    }

    /// 🔁️ Encoding is idempotent and unknown members are gone.
    pub fn unknown_members_are_dropped(ctx: &Context) -> Result<Outcome, String> {
        let vectors = ctx.fixture_json(VECTORS)?;
        let mut projected: Vec<(String, Json)> = Vec::new();
        for (source, text) in documents(&vectors) {
            let first = encode_ticket_document(&decode_ticket_document(&text).map_err(|error| error.message)?);
            let second = encode_ticket_document(&decode_ticket_document(&first).map_err(|error| error.message)?);
            let before = member_names(&text);
            let after = member_names(&first);
            let lost: Vec<Json> = before.into_iter().filter(|name| !after.contains(name)).map(Json::String).collect();
            projected.push((source, Json::Object(vec![("stable".to_string(), Json::String(if first == second { "yes".to_string() } else { "no".to_string() })), ("lost".to_string(), Json::Array(lost))])));
        }
        Ok(Outcome::projection(Json::Object(projected)))
    }

    /// ⚠️ A document without a status is refused.
    pub fn a_document_without_a_status_is_refused(ctx: &Context) -> Result<Outcome, String> {
        let vectors = ctx.fixture_json(VECTORS)?;
        let projected: Vec<(String, Json)> = vectors
            .array("refused")
            .into_iter()
            .map(|entry| {
                let text = match entry {
                    Json::String(value) => value,
                    other => other.to_string(),
                };
                let outcome = match decode_ticket_document(&text) {
                    Ok(_) => "accepted".to_string(),
                    Err(error) => format!("refused:{}", error.class),
                };
                (text, Json::String(outcome))
            })
            .collect();
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
        .subject("real-documents-decode-to-the-same-ticket", subject::real_documents_decode_to_the_same_ticket)
        .subject("encoding-is-go-marshal-indent", subject::encoding_is_go_marshal_indent)
        .subject("unknown-members-are-dropped", subject::unknown_members_are_dropped)
        .subject("a-document-without-a-status-is-refused", subject::a_document_without_a_status_is_refused);
    registered
}
//#endregion 🔖️Registration
