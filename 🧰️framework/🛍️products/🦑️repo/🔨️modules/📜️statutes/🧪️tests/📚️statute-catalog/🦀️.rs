//! 🦀️ Rust side of the statute catalog case. The subject half is gated behind the `sut` feature so
//! the oracle role never compiles the implementation under test.

use semio_repo_test_host::{Adapter, Context, Json, Outcome};

//#region 🔖️Rendering

/// 🪜️ Renders one territory and its nested territories as a flat, depth-first path list.
#[cfg(feature = "sut")]
fn render_territory(prefix: &str, territory: &semio_framework_repo_statutes::Territory, out: &mut Vec<Json>) {
    let path = if prefix.is_empty() { territory.name.clone() } else { format!("{prefix}/{}", territory.name) };
    let kinds = territory.kinds.iter().map(|kind| kind.0.clone()).collect::<Vec<_>>().join(",");
    out.push(Json::String(format!("{path}|{}|{kinds}", territory.scopes.join(";"))));
    for child in &territory.groups {
        render_territory(&path, child, out);
    }
}

/// 🏛️ The policy that first claims each statute through its territory tree.
#[cfg(feature = "sut")]
fn policy_owners() -> std::collections::BTreeMap<String, String> {
    let mut owner = std::collections::BTreeMap::new();
    for policy in semio_framework_repo_statutes::policies() {
        for kind in semio_framework_repo_statutes::policy_kinds(policy) {
            owner.entry(kind.0).or_insert_with(|| policy.id.clone());
        }
    }
    owner
}

//#endregion 🔖️Rendering

//#region 🔖️Scenarios

#[cfg(feature = "sut")]
fn the_catalog_is_one_table(_ctx: &Context) -> Result<Outcome, String> {
    use semio_framework_repo_statutes as statutes;
    let mut ordered: Vec<&statutes::StatuteMeta> = statutes::statutes().iter().collect();
    ordered.sort_by(|left, right| left.kind.0.cmp(&right.kind.0));
    let owner = policy_owners();
    let statute_rows: Vec<Json> = ordered
        .iter()
        .map(|meta| {
            let policy_id = owner.get(&meta.kind.0).cloned().unwrap_or_default();
            Json::String(format!("{}|{policy_id}|{}|{}|{}|{}", meta.kind.0, meta.priority, meta.autofixable, meta.reason, meta.solution))
        })
        .collect();
    let mut policy_rows: Vec<Json> = Vec::new();
    for policy in statutes::policies() {
        let scopes = policy.scopes.clone().unwrap_or_default().join(";");
        policy_rows.push(Json::String(format!("{}|{}|{}|{}", policy.id, policy.name, policy.description.clone().unwrap_or_default(), scopes)));
        let mut territories = Vec::new();
        for group in policy.groups.iter().flatten() {
            render_territory(&policy.id, group, &mut territories);
        }
        policy_rows.extend(territories);
    }
    Ok(Outcome::projection(Json::Object(vec![
        ("schemaVersion".to_string(), Json::String(statutes::catalog().schema_version.to_string())),
        ("statutes".to_string(), Json::Array(statute_rows)),
        ("policies".to_string(), Json::Array(policy_rows)),
    ])))
}

#[cfg(feature = "sut")]
fn the_catalog_is_consistent(_ctx: &Context) -> Result<Outcome, String> {
    use semio_framework_repo_statutes as statutes;
    let declared: Vec<String> = statutes::statutes().iter().map(|meta| meta.kind.0.clone()).collect();
    let mut unknown: Vec<Json> = Vec::new();
    let mut claimed_twice: Vec<Json> = Vec::new();
    let mut owner: std::collections::BTreeMap<String, String> = std::collections::BTreeMap::new();
    for policy in statutes::policies() {
        for kind in statutes::policy_kinds(policy) {
            if !declared.contains(&kind.0) {
                unknown.push(Json::String(format!("{}:{}", policy.id, kind.0)));
            }
            match owner.get(&kind.0) {
                Some(previous) if previous != &policy.id => claimed_twice.push(Json::String(format!("{}:{previous}+{}", kind.0, policy.id))),
                _ => {
                    owner.insert(kind.0.clone(), policy.id.clone());
                }
            }
        }
    }
    let mut missing_metadata: Vec<Json> = declared
        .iter()
        .filter(|kind| statutes::statute_info(&statutes::Statute::from(kind.as_str())).reason == "Unknown breach")
        .map(|kind| Json::String(kind.clone()))
        .collect();
    let mut unclaimed: Vec<Json> = declared.iter().filter(|kind| !owner.contains_key(*kind)).map(|kind| Json::String(kind.clone())).collect();
    let key = |value: &Json| match value {
        Json::String(inner) => inner.clone(),
        other => other.to_string(),
    };
    unknown.sort_by_key(key);
    claimed_twice.sort_by_key(key);
    missing_metadata.sort_by_key(key);
    unclaimed.sort_by_key(key);
    Ok(Outcome::projection(Json::Object(vec![
        ("declared".to_string(), Json::String(declared.len().to_string())),
        ("unknown".to_string(), Json::Array(unknown)),
        ("claimedTwice".to_string(), Json::Array(claimed_twice)),
        ("missingMetadata".to_string(), Json::Array(missing_metadata)),
        ("unclaimed".to_string(), Json::Array(unclaimed)),
    ])))
}

//#endregion 🔖️Scenarios

//#region 🔖️Registration

/// 🧭️ Registration entry point the generated host calls.
pub fn adapter() -> Adapter {
    let adapter = Adapter::new("rust");
    #[cfg(feature = "sut")]
    let adapter = adapter.subject("the-catalog-is-one-table", the_catalog_is_one_table).subject("the-catalog-is-consistent", the_catalog_is_consistent);
    adapter
}

//#endregion 🔖️Registration
