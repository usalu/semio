//! 🪆️ Scenario consumer uses the declared parent and sole retained rich child.
use semio_repo_test_host::{Context, Json};
use semio_framework_value::FromValue;
use semio_s_artifact_trinity_rewriting::standards::v1::subsets::any::schema::snapshot::{decode_rewriting_snapshot_json, encode_rewriting_snapshot_json, RewritingSnapshot};
use semio_s_artifact_stdio_semio::standards::v1::subsets::graph::schema::snapshot::{decode_semio_graph_snapshot_json, encode_semio_graph_snapshot_json};
use semio_s_artifact_trinity_jack::{jack_content_for_handle, materialize_jack_snapshot};

pub fn declared_text(ctx: &Context, role: &str) -> Result<String, String> {
    let prefix = format!("{role} ");
    let mut found = ctx.scenario.steps.iter().filter_map(|(_, text)| text.strip_prefix(&prefix).filter(|uri| uri.starts_with("shared://") || uri.starts_with("asset://")));
    let uri = found.next().ok_or_else(|| format!("{} requires its declared {role}", ctx.scenario.id))?;
    if found.next().is_some() {
        return Err(format!("{} repeats its declared {role}", ctx.scenario.id));
    }
    String::from_utf8(ctx.fixture_bytes(uri)?).map_err(|error| format!("{role} is not UTF-8: {error}"))
}

pub fn declared_owner(ctx: &Context, parent_role: &str, child_role: &str) -> Result<RewritingSnapshot, String> {
    let mut parent = decode_rewriting_snapshot_json(&declared_text(ctx, parent_role)?).map_err(|error| error.into_message())?.guard_decoded();
    bind_declared_child(ctx, parent.get_mut(), child_role)?;
    Ok(parent.take())
}

pub fn bind_declared_child(ctx: &Context, parent: &mut RewritingSnapshot, child_role: &str) -> Result<(), String> {
    let dialect = &parent.working_graph.content.target.dialect;
    if dialect.artifact_kind != "s.stdio.semio" || dialect.standard != "v1" || dialect.subset != "graph" {
        return Err("scenario owner requires its exact Semio v1 graph child dialect".into());
    }
    let child = decode_semio_graph_snapshot_json(&declared_text(ctx, child_role)?).map_err(|error| error.into_message())?;
    materialize_jack_snapshot(&mut parent.working_graph.content, child);
    Ok(())
}

pub fn full_projection(parent: &RewritingSnapshot) -> Result<Json, String> {
    let child = jack_content_for_handle(&parent.working_graph.content).map_err(|error| error.into_message())?;
    Ok(Json::Object(vec![
        ("snapshot".into(), parse_json(&encode_rewriting_snapshot_json(parent).map_err(|error| error.into_message())?)?),
        ("child".into(), parse_json(&encode_semio_graph_snapshot_json(child.snapshot()).map_err(|error| error.into_message())?)?),
    ]))
}

pub fn touches_one(scenario: &str, member: &str, before: &RewritingSnapshot, after: &RewritingSnapshot) -> Result<(), String> {
    let before = full_projection(before)?;
    let after = full_projection(after)?;
    let was = before.get("snapshot").ok_or("declared parent projection is absent")?;
    let now = after.get("snapshot").ok_or("declared parent projection is absent")?;
    let moved: Vec<_> = ["workingGraph", "lhs", "rhs", "parameterBindings", "ruleLayout"].into_iter().filter(|field| was.get(field) != now.get(field) || *field == "workingGraph" && before.get("child") != after.get("child")).collect();
    if moved != [member] {
        return Err(format!("{scenario} requires only {member} to move, found {moved:?}"));
    }
    Ok(())
}

pub fn restores(before: &RewritingSnapshot, restored: &RewritingSnapshot) -> Result<(), String> {
    if full_projection(before)? != full_projection(restored)? {
        return Err("inverse must restore the complete declared parent and retained rich child".into());
    }
    Ok(())
}
