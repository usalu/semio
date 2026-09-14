//! 🦀️ Rust side of the contributor identity case. The subject half is gated behind the `sut`
//! feature so the oracle role never compiles the implementation under test.

use semio_repo_test_host::{Adapter, Context, Json, Outcome};

//#region 🔖️Scenarios

#[cfg(feature = "sut")]
fn author_lines_resolve_to_aliases(ctx: &Context) -> Result<Outcome, String> {
    use semio_framework_repo_contributors as contributors;
    let documents = ctx.fixture_json("shared://🧑️‍💻️contributor-documents.json")?;
    let log = ctx.fixture_json("shared://🏁️checkpoint-log.json")?;
    let store = contributors::MemoryContributorStore::seeded(
        documents.array("documents").iter().map(|entry| (entry.str("directory"), entry.str("json"))).collect::<Vec<_>>(),
    );

    let mut lines: Vec<String> = log.array("identities").iter().map(text).collect();
    lines.extend(log.array("malformed").iter().map(text));

    let identities: Vec<Json> = lines
        .iter()
        .map(|line| {
            let split = contributors::parse_contributor_identity(line)
                .map(|(name, email)| format!("{name}|{email}"))
                .unwrap_or_else(|| "-".to_string());
            Json::String(format!("{line}={split}"))
        })
        .collect();
    let aliases: Vec<Json> = lines
        .iter()
        .map(|line| {
            let alias = match contributors::parse_contributor_identity(line) {
                Some((name, email)) => contributors::resolve_author_to_alias(&store, &name, &email),
                None => "-".to_string(),
            };
            Json::String(format!("{line}={alias}"))
        })
        .collect();
    let people: Vec<Json> = contributors::list_contributors(&store)
        .iter()
        .map(|contributor| {
            Json::String(format!(
                "{}|{}|{}|{}|{}|{}",
                contributor.alias,
                contributor.github,
                contributor.name,
                contributor.email,
                contributor.emails.len(),
                contributor.names.len()
            ))
        })
        .collect();
    let authors: Vec<Json> = contributors::list_contributors(&store)
        .iter()
        .map(|contributor| format!("{} <{}>", contributor.name, contributor.email))
        .chain(lines.iter().cloned())
        .map(|line| {
            let parsed = contributors::parse_git_author(&line);
            Json::String(format!("{line}={}|{}|{}", parsed.name, parsed.email, parsed))
        })
        .collect();
    Ok(Outcome::projection(Json::Object(vec![
        ("identities".to_string(), Json::Array(identities)),
        ("aliases".to_string(), Json::Array(aliases)),
        ("contributors".to_string(), Json::Array(people)),
        ("authors".to_string(), Json::Array(authors)),
    ])))
}

#[cfg(feature = "sut")]
fn text(value: &Json) -> String {
    match value {
        Json::String(inner) => inner.clone(),
        other => other.to_string(),
    }
}

//#endregion 🔖️Scenarios

//#region 🔖️Registration

/// 🧭️ Registration entry point the generated host calls.
pub fn adapter() -> Adapter {
    let adapter = Adapter::new("rust");
    #[cfg(feature = "sut")]
    let adapter = adapter.subject("author-lines-resolve-to-aliases", author_lines_resolve_to_aliases);
    adapter
}

//#endregion 🔖️Registration
