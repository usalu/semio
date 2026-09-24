//! 🦀️ Rust side of the golden-tree analysis case. The subject half is gated behind the `sut`
//! feature so the oracle role never compiles the implementation under test.

use semio_repo_test_host::{Adapter, Context, Json, Outcome};

//#region 🔖️Tree

/// 🌳️ The declared fixture URIs of the scenario, read as repository-relative sources.
#[cfg(feature = "sut")]
fn tree(ctx: &Context, uris: &[&str]) -> Result<Vec<semio_framework_repo_statutes::SourceFile>, String> {
    let mut files = Vec::new();
    for uri in uris {
        let path = ctx.fixture(uri)?;
        let content = std::fs::read_to_string(&path).map_err(|error| format!("cannot read {uri}: {error}"))?;
        files.push(semio_framework_repo_statutes::SourceFile { path: uri.trim_start_matches("shared://").to_string(), content });
    }
    files.sort_by(|left, right| left.path.cmp(&right.path));
    Ok(files)
}

/// 📜️ The declared fixture URIs of one scenario, in the order the data table lists them.
#[cfg(feature = "sut")]
fn declared(ctx: &Context) -> Result<Vec<String>, String> {
    Ok(ctx.data_table()?.iter().filter_map(|row| row.first()).map(|cell| cell.trim().to_string()).filter(|cell| cell.starts_with("shared://")).collect())
}

/// 🔶️ Renders one breach as the tuple both implementations must agree on.
#[cfg(feature = "sut")]
fn render(breach: &semio_framework_repo_statutes::Breach) -> String {
    format!(
        "{}|{}|{}|{}|{}|{}|{}",
        breach.id,
        breach.kind.0,
        breach.scope,
        breach.line,
        breach.column,
        semio_framework_repo_statutes::is_autofixable(&breach.kind),
        breach.summary
    )
}

//#endregion 🔖️Tree

//#region 🔖️Scenarios

#[cfg(feature = "sut")]
fn the_golden_tree_breaches(ctx: &Context) -> Result<Outcome, String> {
    use semio_framework_repo_statutes as statutes;
    let uris = declared(ctx)?;
    let files = tree(ctx, &uris.iter().map(String::as_str).collect::<Vec<_>>())?;
    let sources = statutes::SourceSet::new(files);
    let rendered: Vec<Json> = statutes::analyze(&sources).iter().map(|breach| Json::String(render(breach))).collect();
    let golden = ctx.fixture_json("shared://🔍️analyze-breaches/🔣️breaches.json")?;
    let expected = golden.array("breachs");
    if expected != rendered {
        return Err(format!("analysis drifted from the reviewed golden: {} breaches produced, {} expected", rendered.len(), expected.len()));
    }
    Ok(Outcome::projection(Json::Object(vec![("breachs".to_string(), Json::Array(rendered))])))
}

#[cfg(feature = "sut")]
fn the_clean_sources_are_clean(ctx: &Context) -> Result<Outcome, String> {
    use semio_framework_repo_statutes as statutes;
    let uris = declared(ctx)?;
    let files = tree(ctx, &uris.iter().map(String::as_str).collect::<Vec<_>>())?;
    let sources = statutes::SourceSet::new(files);
    let rendered: Vec<Json> = statutes::analyze(&sources).iter().map(|breach| Json::String(render(breach))).collect();
    if !rendered.is_empty() {
        return Err(format!("a repaired source raised {} breaches", rendered.len()));
    }
    Ok(Outcome::projection(Json::Object(vec![("breachs".to_string(), Json::Array(rendered))])))
}

//#endregion 🔖️Scenarios

//#region 🔖️Registration

/// 🧭️ Registration entry point the generated host calls.
pub fn adapter() -> Adapter {
    let adapter = Adapter::new("rust");
    #[cfg(feature = "sut")]
    let adapter = adapter.subject("the-golden-tree-breaches", the_golden_tree_breaches).subject("the-clean-sources-are-clean", the_clean_sources_are_clean);
    adapter
}

//#endregion 🔖️Registration
