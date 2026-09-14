//! 🦀️ Rust side of the session kind derivation case. The subject half is gated behind the `sut`
//! feature so the oracle role never compiles the implementation under test.

use semio_repo_test_host::{Adapter, Context, Json, Outcome};

//#region 🔖️Vectors

#[cfg(feature = "sut")]
fn source(ctx: &Context) -> Result<semio_framework_repo_contributors::MemorySessionSource, String> {
    use semio_framework_repo_contributors as contributors;
    let file = ctx.fixture_json("shared://📡️session-vectors.json")?;
    let entries = file
        .array("sessions")
        .iter()
        .map(|entry| {
            let key = contributors::SessionKey {
                year: number(entry, "year"),
                month: number(entry, "month"),
                day: number(entry, "day"),
                uuid: entry.str("uuid"),
            };
            let files = match entry.get("files") {
                Some(Json::Object(members)) => members
                    .iter()
                    .map(|(name, value)| {
                        let content = match value {
                            Json::String(text) => text.clone(),
                            other => other.to_string(),
                        };
                        (name.clone(), content)
                    })
                    .collect(),
                _ => Default::default(),
            };
            let age_seconds = match entry.get("ageSeconds") {
                Some(Json::Number(value)) => Some(*value as i64),
                _ => None,
            };
            (key, contributors::MemorySession { files, age_seconds })
        })
        .collect::<Vec<_>>();
    Ok(contributors::MemorySessionSource::seeded(entries))
}

#[cfg(feature = "sut")]
fn number(entry: &Json, key: &str) -> i64 {
    match entry.get(key) {
        Some(Json::Number(value)) => *value as i64,
        _ => 0,
    }
}

//#endregion 🔖️Vectors

//#region 🔖️Scenarios

#[cfg(feature = "sut")]
fn every_branch_of_the_derivation_agrees(ctx: &Context) -> Result<Outcome, String> {
    use semio_framework_repo_contributors as contributors;
    let source = source(ctx)?;
    let sessions = contributors::list_sessions(&source);
    let records: Vec<Json> = sessions
        .iter()
        .map(|session| {
            Json::String(format!(
                "{}|{}|{}|{}|{}|{:02}-{:02}-{:02}",
                session.uuid, session.kind, session.client, session.started_at, session.checkpoint, session.year, session.month, session.day
            ))
        })
        .collect();
    let identifiers: Vec<Json> = sessions.iter().map(|session| Json::String(format!("{}={}|{}", session.uuid, session.id(), session.uri()))).collect();
    Ok(Outcome::projection(Json::Object(vec![
        ("records".to_string(), Json::Array(records)),
        ("identifiers".to_string(), Json::Array(identifiers)),
        ("found".to_string(), Json::Number(sessions.len() as f64)),
    ])))
}

#[cfg(feature = "sut")]
fn the_kind_emoji_comes_from_the_shared_vocabulary(ctx: &Context) -> Result<Outcome, String> {
    use semio_framework_repo_contributors as contributors;
    let source = source(ctx)?;
    let mut emojis: Vec<Json> = contributors::list_sessions(&source)
        .iter()
        .map(|session| {
            let kind = match session.kind.as_str() {
                "running" => Some(contributors::SessionKind::Running),
                "completed" => Some(contributors::SessionKind::Completed),
                _ => Some(contributors::SessionKind::Interrupted),
            };
            Json::String(format!("{}={}", session.kind, contributors::session_kind_emoji(kind)))
        })
        .collect();
    emojis.push(Json::String(format!("none={}", contributors::session_kind_emoji(None))));
    Ok(Outcome::projection(Json::Object(vec![("emojis".to_string(), Json::Array(emojis))])))
}

//#endregion 🔖️Scenarios

//#region 🔖️Registration

/// 🧭️ Registration entry point the generated host calls.
pub fn adapter() -> Adapter {
    let adapter = Adapter::new("rust");
    #[cfg(feature = "sut")]
    let adapter = adapter
        .subject("every-branch-of-the-derivation-agrees", every_branch_of_the_derivation_agrees)
        .subject("the-kind-emoji-comes-from-the-shared-vocabulary", the_kind_emoji_comes_from_the_shared_vocabulary);
    adapter
}

//#endregion 🔖️Registration
