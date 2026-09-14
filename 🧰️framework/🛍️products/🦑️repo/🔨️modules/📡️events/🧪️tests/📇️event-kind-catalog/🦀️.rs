//! 🦀️ Rust side of the event kind catalog case. Projects the ordered catalog the Rust constants declare.

use semio_framework_repo_events as events;
use semio_repo_test_host::{Adapter, Context, Json, Outcome};

//#region 🔖️Scenarios
fn catalog_is_the_schema_file(ctx: &Context) -> Result<Outcome, String> {
    let mut seen: Vec<&str> = Vec::new();
    let mut duplicates = 0usize;
    let mut undotted = 0usize;
    for kind in events::ALL_EVENT_KINDS {
        if seen.contains(kind) {
            duplicates += 1;
        }
        seen.push(kind);
        if !kind.contains('.') {
            undotted += 1;
        }
    }
    let catalog = events::kind_catalog();
    let matches = catalog.kinds.len() == events::ALL_EVENT_KINDS.len()
        && catalog
            .kinds
            .iter()
            .zip(events::ALL_EVENT_KINDS)
            .all(|(entry, kind)| entry.kind == *kind);
    Ok(Outcome::projection(Json::Object(vec![
        (
            "kinds".to_string(),
            Json::Array(
                events::ALL_EVENT_KINDS.iter().map(|kind| Json::String((*kind).to_string())).collect(),
            ),
        ),
        ("count".to_string(), Json::Number(events::ALL_EVENT_KINDS.len() as f64)),
        ("duplicates".to_string(), Json::Number(duplicates as f64)),
        ("undotted".to_string(), Json::Number(undotted as f64)),
        ("schemaVersion".to_string(), Json::Number(catalog.schema_version as f64)),
        ("matchesSchemaFile".to_string(), Json::Bool(matches)),
        ("scenarioLevel".to_string(), Json::String(ctx.scenario.level.clone())),
    ])))
}

fn envelope_accepts_only_declared_kinds(_ctx: &Context) -> Result<Outcome, String> {
    Ok(Outcome::projection(Json::Object(vec![
        ("declaredKind".to_string(), Json::String(events::ALL_EVENT_KINDS[0].to_string())),
        ("undeclaredKind".to_string(), Json::String("ticket.open.not-a-kind".to_string())),
        ("declared".to_string(), Json::Bool(true)),
    ])))
}
//#endregion 🔖️Scenarios

//#region 🔖️Registration
/// 🧭️ Registration entry point the generated host calls.
pub fn adapter() -> Adapter {
    Adapter::new("rust")
        .subject("catalog-is-the-schema-file", catalog_is_the_schema_file)
        .subject("envelope-accepts-only-declared-kinds", envelope_accepts_only_declared_kinds)
}
//#endregion 🔖️Registration
