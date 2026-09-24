//! 🦀️ Rust side of the ticket id scheme case. The subject halves are gated behind the `sut`
//! feature the generated host turns on for the subject role only.

use semio_repo_test_host::Adapter;

//#region 🔖️Scenarios
#[cfg(feature = "sut")]
mod subject {
    use semio_framework_repo_tickets::{ticket_slug_from_title, validate_ticket_emoji_title, TicketId, TicketLayout};
    use semio_repo_test_host::{Context, Json, Outcome};

    const VECTORS: &str = "shared://🪪️ticket-id-scheme/🪪️id-vectors.json";

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

    fn pairs(value: &Json, key: &str) -> Vec<(String, String)> {
        value
            .array(key)
            .into_iter()
            .map(|item| match item {
                Json::Array(members) => {
                    let read = |index: usize| match members.get(index) {
                        Some(Json::String(text)) => text.clone(),
                        Some(other) => other.to_string(),
                        None => String::new(),
                    };
                    (read(0), read(1))
                }
                other => (other.to_string(), String::new()),
            })
            .collect()
    }

    /// 🔁️ Both spellings name the same identity.
    pub fn both_spellings_name_the_same_identity(ctx: &Context) -> Result<Outcome, String> {
        let vectors = ctx.fixture_json(VECTORS)?;
        let mut projected: Vec<(String, Json)> = Vec::new();
        for raw in strings(&vectors, "ids") {
            let id = TicketId::parse(&raw).map_err(|error| error.message)?;
            let again = TicketId::parse(&id.rel_path()).map_err(|error| error.message)?;
            projected.push((
                raw,
                Json::Object(vec![
                    ("id".to_string(), Json::String(id.id())),
                    ("relPath".to_string(), Json::String(id.rel_path())),
                    ("uri".to_string(), Json::String(id.uri())),
                    ("stable".to_string(), Json::String(if again == id { "yes".to_string() } else { "no".to_string() })),
                    ("parent".to_string(), Json::String(id.parent_slug().unwrap_or_default())),
                ]),
            ));
        }
        Ok(Outcome::projection(Json::Object(projected)))
    }

    /// ⚠️ A malformed id is refused.
    pub fn a_malformed_id_is_refused(ctx: &Context) -> Result<Outcome, String> {
        let vectors = ctx.fixture_json(VECTORS)?;
        let projected: Vec<(String, Json)> = strings(&vectors, "invalidIds")
            .into_iter()
            .map(|raw| {
                let outcome = match TicketId::parse(&raw) {
                    Ok(id) => format!("accepted:{}", id.id()),
                    Err(error) => format!("refused:{}", error.class),
                };
                (raw, Json::String(outcome))
            })
            .collect();
        Ok(Outcome::projection(Json::Object(projected)))
    }

    /// 🔤️ A title becomes one slug, idempotently.
    pub fn a_title_becomes_one_slug(ctx: &Context) -> Result<Outcome, String> {
        let vectors = ctx.fixture_json(VECTORS)?;
        let projected: Vec<(String, Json)> = strings(&vectors, "titles")
            .into_iter()
            .map(|title| {
                let first = ticket_slug_from_title(&title).map(|slug| slug).unwrap_or_else(|error| format!("refused:{}", error.class));
                let second = ticket_slug_from_title(&first).unwrap_or_else(|error| format!("refused:{}", error.class));
                (title, Json::Object(vec![("slug".to_string(), Json::String(first.clone())), ("idempotent".to_string(), Json::String(if first == second { "yes".to_string() } else { second }))]))
            })
            .collect();
        Ok(Outcome::projection(Json::Object(projected)))
    }

    /// 🎫️ An emoji and title pair is validated together.
    pub fn an_emoji_and_title_pair_is_validated_together(ctx: &Context) -> Result<Outcome, String> {
        let vectors = ctx.fixture_json(VECTORS)?;
        let projected: Vec<(String, Json)> = pairs(&vectors, "emojiTitles")
            .into_iter()
            .map(|(emoji, title)| {
                let outcome = match validate_ticket_emoji_title(&emoji, &title) {
                    Ok(slug) => format!("ok:{slug}"),
                    Err(error) => format!("refused:{}", error.message),
                };
                (format!("{emoji}|{title}"), Json::String(outcome))
            })
            .collect();
        Ok(Outcome::projection(Json::Object(projected)))
    }

    /// 🗺️ Every owned path hangs off the folder.
    pub fn every_owned_path_hangs_off_the_folder(ctx: &Context) -> Result<Outcome, String> {
        let vectors = ctx.fixture_json(VECTORS)?;
        let layout = TicketLayout::new(vectors.str("repoMetaDir"));
        let mut projected: Vec<(String, Json)> = vec![("ticketsDir".to_string(), Json::String(layout.tickets_dir()))];
        for raw in strings(&vectors, "ids") {
            let id = TicketId::parse(&raw).map_err(|error| error.message)?;
            projected.push((
                id.id(),
                Json::Object(vec![
                    ("folder".to_string(), Json::String(layout.ticket_dir(&id))),
                    ("document".to_string(), Json::String(layout.document_path(&id))),
                    ("importantDir".to_string(), Json::String(layout.important_dir(&id))),
                    ("importantDocument".to_string(), Json::String(layout.important_path(&id))),
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
        .subject("both-spellings-name-the-same-identity", subject::both_spellings_name_the_same_identity)
        .subject("a-malformed-id-is-refused", subject::a_malformed_id_is_refused)
        .subject("a-title-becomes-one-slug", subject::a_title_becomes_one_slug)
        .subject("an-emoji-and-title-pair-is-validated-together", subject::an_emoji_and_title_pair_is_validated_together)
        .subject("every-owned-path-hangs-off-the-folder", subject::every_owned_path_hangs_off_the_folder);
    registered
}
//#endregion 🔖️Registration
