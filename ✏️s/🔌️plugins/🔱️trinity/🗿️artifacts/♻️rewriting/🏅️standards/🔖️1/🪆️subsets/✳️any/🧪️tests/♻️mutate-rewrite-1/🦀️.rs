//! ♻️ Full typed Rewriting scenario subject pairs every declared parent with its explicit retained child input.
use semio_repo_test_host::Adapter;
#[cfg(feature = "sut")]
#[path = "🪆️owner/🦀️.rs"]
mod owner;
const KINDS: &[&str] = &["edit-before-fixture", "edit-lhs", "edit-rhs", "change-parameter-binding", "remove-parameter-binding", "change-rule-layout-point", "remove-rule-layout-point", "drag-rule-nodes", "set-rule-layout-points"];

#[cfg(feature = "sut")]
mod subject {
    use super::owner;
    use semio_framework_value::FromValue;
    use semio_repo_test_host::{Context, Json, Outcome};
    use semio_s_artifact_trinity_rewriting::standards::v1::subsets::any::schema::mutations::text::{apply_rewriting_mutation_reporting, decode_rewriting_mutation_json, inverse_rewriting_mutation_steps};
    use semio_s_artifact_trinity_rewriting::standards::v1::subsets::any::schema::mutations::RewriteRuleMutation;
    use semio_s_artifact_trinity_rewriting::standards::v1::subsets::any::schema::snapshot::{decode_rewriting_snapshot_json, encode_rewriting_snapshot_json, parse_rewriting_dsl, print_rewriting_dsl, RewritingSnapshot};

    fn written(kind: &str) -> &'static str {
        match kind {
            "edit-before-fixture" => "workingGraph",
            "edit-lhs" => "lhs",
            "edit-rhs" => "rhs",
            "change-parameter-binding" | "remove-parameter-binding" => "parameterBindings",
            _ => "ruleLayout",
        }
    }

    fn payload(text: &str, kind: &str) -> Result<RewriteRuleMutation, String> {
        let mutation = decode_rewriting_mutation_json(text).map_err(|error| error.into_message())?.guard_decoded();
        let declared = semio_repo_test_host::parse_json(text)?;
        let expected = match kind {
            "edit-before-fixture" => "editBeforeFixture", "edit-lhs" => "editLhs", "edit-rhs" => "editRhs",
            "change-parameter-binding" => "changeParameterBinding", "remove-parameter-binding" => "removeParameterBinding",
            "change-rule-layout-point" => "changeRuleLayoutPoint", "remove-rule-layout-point" => "removeRuleLayoutPoint",
            "drag-rule-nodes" => "dragRuleNodes", "set-rule-layout-points" => "setRuleLayoutPoints",
            _ => return Err("undeclared mutation kind".into()),
        };
        if declared.get("mutation") != Some(&Json::String(expected.into())) {
            return Err("payload discriminator differs from the exact scenario kind".into());
        }
        Ok(mutation.take())
    }

    fn bind_replacement(ctx: &Context, mutation: &mut RewriteRuleMutation, role: &str) -> Result<(), String> {
        if let RewriteRuleMutation::EditBeforeFixture(edit) = mutation {
            let child = semio_s_artifact_stdio_semio::standards::v1::subsets::graph::schema::snapshot::decode_semio_graph_snapshot_json(&owner::declared_text(ctx, role)?).map_err(|error| error.into_message())?;
            semio_s_artifact_trinity_jack::materialize_jack_snapshot(&mut edit.new_working_graph.content, child);
        }
        Ok(())
    }

    fn apply(snapshot: &mut RewritingSnapshot, mutation: &RewriteRuleMutation) -> Result<(), String> {
        let raised = apply_rewriting_mutation_reporting(snapshot, mutation);
        if !raised.is_empty() { return Err(format!("declared clean mutation raised {raised:?}")); }
        Ok(())
    }

    fn undo(snapshot: &mut RewritingSnapshot, mutation: &RewriteRuleMutation, before: &RewritingSnapshot) -> Result<(), String> {
        let mut inverses = inverse_rewriting_mutation_steps(mutation, before).map_err(|error| error.into_message())?.guard_decoded();
        inverses.get_mut().reverse();
        while let Some(inverse) = inverses.get_mut().pop() {
            let inverse = inverse.guard_decoded();
            apply(snapshot, inverse.get())?;
        }
        owner::restores(before, snapshot)
    }

    fn outcome(value: Json) -> Outcome { Outcome::with_raw(value.to_string().into_bytes(), value) }

    pub fn mutate(kind: &'static str, inverse: bool) -> impl Fn(&Context) -> Result<Outcome, String> {
        move |ctx| {
            let before = owner::declared_owner(ctx, "the real derived rule", "the real derived child")?.guard_decoded();
            let mut mutation = payload(ctx.doc_string()?, kind)?.guard_decoded();
            if kind == "edit-before-fixture" { bind_replacement(ctx, mutation.get_mut(), "the replacement working child")?; }
            let mut current = before.get().clone().guard_decoded();
            apply(current.get_mut(), mutation.get())?;
            if owner::full_projection(before.get())? == owner::full_projection(current.get())? { return Err("forward operation must move its declared member".into()); }
            owner::touches_one(&ctx.scenario.id, written(kind), before.get(), current.get())?;
            let mutated = owner::full_projection(current.get())?;
            if inverse {
                undo(current.get_mut(), mutation.get(), before.get())?;
                Ok(outcome(Json::Object(vec![("mutated".into(), mutated), ("restored".into(), owner::full_projection(current.get())?)])))
            } else { Ok(outcome(mutated)) }
        }
    }

    pub fn spec_vector(kind: &'static str) -> impl Fn(&Context) -> Result<Outcome, String> {
        move |ctx| {
            let before = owner::declared_owner(ctx, "the committed before-rule", "the committed before-child")?.guard_decoded();
            let mut mutation = payload(&owner::declared_text(ctx, "the committed mutation")?, kind)?.guard_decoded();
            if kind == "edit-before-fixture" { bind_replacement(ctx, mutation.get_mut(), "the committed after-child")?; }
            let mut current = before.get().clone().guard_decoded();
            apply(current.get_mut(), mutation.get())?;
            let expected = owner::declared_owner(ctx, "the committed after-rule", "the committed after-child")?.guard_decoded();
            if owner::full_projection(current.get())? != owner::full_projection(expected.get())? { return Err("computed complete parent and child differ from the committed after owner".into()); }
            if owner::full_projection(before.get())? == owner::full_projection(current.get())? { return Err("committed forward operation must move its declared member".into()); }
            owner::touches_one(&ctx.scenario.id, written(kind), before.get(), current.get())?;
            let projected = owner::full_projection(current.get())?;
            undo(current.get_mut(), mutation.get(), before.get())?;
            Ok(outcome(projected))
        }
    }

    fn child_facts(parent: &RewritingSnapshot) -> Result<(usize, usize, usize), String> {
        let value = owner::full_projection(parent)?;
        let child = value.get("child").ok_or("missing full child projection")?;
        let Some(Json::Array(nodes)) = child.get("nodes") else { return Err("child requires its actual nodes".into()); };
        let Some(Json::Array(edges)) = child.get("edges") else { return Err("child requires its actual edges".into()); };
        let mut ports = 0;
        for node in nodes {
            let Some(Json::Array(values)) = node.get("ports") else { return Err("node requires its actual ports".into()); };
            ports += values.len();
        }
        Ok((nodes.len(), edges.len(), ports))
    }

    pub fn round_trip(ctx: &Context) -> Result<Outcome, String> {
        let text = owner::declared_text(ctx, "the artifact's own committed carrier")?;
        let parsed = parse_rewriting_dsl(&text)?.guard_decoded();
        let printed = print_rewriting_dsl(parsed.get());
        let reparsed = parse_rewriting_dsl(&printed)?.guard_decoded();
        if printed != text { return Err("committed DSL must reproduce every literal byte".into()); }
        if encode_rewriting_snapshot_json(parsed.get()).map_err(|error| error.into_message())? != encode_rewriting_snapshot_json(reparsed.get()).map_err(|error| error.into_message())? {
            return Err("declared typed DSL round trip must retain every parent field".into());
        }
        let ground = owner::declared_owner(ctx, "the two-node ground-floor rule this case used to rest on", "the two-node ground-floor child")?.guard_decoded();
        let tower = owner::declared_owner(ctx, "the real derived rule", "the real derived child")?.guard_decoded();
        for (parent, expected) in [(ground.get(), (2, 1, 2)), (tower.get(), (180, 179, 364))] {
            if child_facts(parent)? != expected { return Err("complete committed Nakagin child cardinalities differ".into()); }
            let mut reread = decode_rewriting_snapshot_json(&encode_rewriting_snapshot_json(parent).map_err(|error| error.into_message())?).map_err(|error| error.into_message())?.guard_decoded();
            let child = semio_s_artifact_trinity_jack::jack_content_for_handle(&parent.working_graph.content).map_err(|error| error.into_message())?;
            semio_s_artifact_trinity_jack::materialize_jack_snapshot(&mut reread.get_mut().working_graph.content, child.snapshot().clone());
            owner::restores(parent, reread.get())?;
        }
        if ground.get().working_graph.root_node_id != tower.get().working_graph.root_node_id { return Err("both declared Nakagin roots must remain identical".into()); }
        Ok(outcome(Json::Object(vec![("groundFloor".into(), owner::full_projection(ground.get())?), ("capsuleTower".into(), owner::full_projection(tower.get())?)])))
    }
}

pub fn adapter() -> Adapter {
    let built = Adapter::new("rust");
    #[cfg(feature = "sut")]
    {
        let mut built = built;
        for kind in KINDS {
            built = built.subject(&format!("mutate-{kind}"), subject::mutate(kind, false));
            built = built.subject(&format!("inverse-{kind}"), subject::mutate(kind, true));
            built = built.subject(&format!("spec-vector-{kind}"), subject::spec_vector(kind));
        }
        built.subject("identity-round-trip", subject::round_trip)
    }
    #[cfg(not(feature = "sut"))]
    { let _ = KINDS; built }
}
