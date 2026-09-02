//! 🧱️ `s.block.block2d` exhaustive mutation case — Rust SUBJECT adapter. Ticket
//! 26/08/23/END-TO-END-TESTING-REFACTOR.
//!
//! This case is a CROSS-LANGUAGE DIFFERENTIAL. The reference is `🐍️component.py` beside this file —
//! a second implementation of the node-kind document and all twenty-six typed mutations, written in
//! Python from this subset's committed schemas, grammar and specification vectors. This adapter
//! registers the SUBJECT half only.
//!
//! **What changed here, and why it had to.** Before this conversion this adapter did not link the
//! plugin crate at all: it read the committed vectors and asserted laws OVER THEM, so the subject
//! phase never ran this subset's own implementation on anything. The subset shipped no test bridge
//! to run it through — every other converted subset (`🗺️gismap`, `🏗️fem`, `🏔️gisterrain`) ships one.
//! `block2d_mutation_report_json` was added to
//! `../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️.rs` in the same shape, and
//! this adapter now drives it.
//!
//! **What is NOT asserted here, and why.** The `.dsl.semio` carrier law. This subset's
//! `store::ArtifactDsl` impl is handwritten `async` (`📸️snapshot/🦀️.rs`), and the generated
//! test host is synchronous, so `parse_dsl`/`print_dsl` are unreachable from a case adapter without
//! an async bridge this conversion did not add. `identity-round-trip` therefore exercises the JSON
//! codec on the real derived document and says so; the carrier gap is reported, not papered over.

use semio_repo_test_host::{parse_json, Adapter, Json};

//#region 🔖️Kinds
/// 🏷️ Mirrors `Block2dMutation::KINDS`
/// (`../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️.rs`) — duplicated, not
/// imported, because the oracle-only build must not link the subject crate.
const KINDS: &[&str] = &[
    "rename-node-kind",
    "change-node-kind-label",
    "change-node-kind-variant",
    "change-node-kind-description",
    "change-node-kind-icon",
    "change-node-kind-unit",
    "update-presentation",
    "create-handle-kind",
    "delete-handle-kind",
    "rename-handle-kind",
    "change-handle-kind-label",
    "change-handle-kind-color",
    "change-handle-kind-default-wire-kind",
    "create-handle",
    "delete-handle",
    "move-handle",
    "change-handle-handle-kind",
    "add-compatibility-rule",
    "remove-compatibility-rule",
    "add-attribute",
    "remove-attribute",
    "add-author",
    "remove-author",
    "move-camera2d",
    "scale-camera2d",
    "change-meta-description",
];


/// 🧫️ The real derived node kind: the committed *Hexagonal Cut Concrete Forest Left* example, with
/// the members its carrier does not hold taken from committed specification vectors. Derived once,
/// provenance recorded in the feature description.
#[cfg(feature = "sut")]
const DERIVED_ASSET: &str = "local://🧱️hexagonal-cut-concrete-forest-left.snapshot.json";

/// 🗂️ The ten members `Block2dSnapshot` declares — the cross-language projection.
const MEMBERS: &[&str] = &["schema", "nodeKind", "presentation", "handleKinds", "handles", "compatibility", "attributes", "authors", "camera2d", "meta"];
//#endregion 🔖️Kinds

//#region 🔖️Subject
#[cfg(feature = "sut")]
mod subject {
    use super::{DERIVED_ASSET, MEMBERS};
    use semio_repo_test_host::{parse_json, Context, Json, Outcome};
    use semio_s_plugin_block::artifacts::block2d::standards::v1::subsets::any::schema::mutations::block2d_mutation_report_json;

    //#region 🔖️Plan
    /// 📋️ One member of the bridge's report, named in the error when it is absent — never defaulted.
    fn member<'a>(report: &'a Json, key: &str) -> Result<&'a Json, String> {
        report.get(key).ok_or_else(|| format!("the report carries no {key:?} member"))
    }

    /// 📋️ An array member of the report, rejecting a present-but-wrong-shaped value.
    fn members(report: &Json, key: &str) -> Result<Vec<Json>, String> {
        match member(report, key)? {
            Json::Array(items) => Ok(items.clone()),
            other => Err(format!("the report's {key:?} member is {}, not an array", other.to_string())),
        }
    }

    /// 🧫️ The one declared fixture URI of this scenario's steps containing `needle`.
    fn uri_in(ctx: &Context, needle: &str) -> Result<String, String> {
        ctx.scenario
            .steps
            .iter()
            .flat_map(|(_, step)| step.split_whitespace())
            .find(|token| (token.starts_with("asset://") || token.starts_with("local://") || token.starts_with("shared://")) && token.contains(needle))
            .map(|token| token.to_string())
            .ok_or_else(|| format!("scenario {} declares no fixture URI containing {needle:?}", ctx.scenario.id))
    }

    /// 🧫️ The declared fixture's bytes as UTF-8 text.
    fn fixture_text(ctx: &Context, needle: &str) -> Result<String, String> {
        let uri = uri_in(ctx, needle)?;
        String::from_utf8(ctx.fixture_bytes(&uri)?).map_err(|error| format!("the declared fixture {uri} is not UTF-8: {error}"))
    }

    fn report_of(scenario: &str, base: &str, mutation: &str, after: &str) -> Result<Json, String> {
        parse_json(&block2d_mutation_report_json(base, mutation, after).map_err(|error| format!("{scenario}: the input did not reach this subset's own codec: {error}"))?)
    }

    /// 🔀️ The one member a verb writes. Spelled out rather than pattern-matched on suffixes: three
    /// of the twenty-six kinds share a suffix with a kind that writes a different member
    /// (`change-handle-handle-kind` writes `handles`, not `handleKinds`), and a suffix rule that got
    /// one of them wrong would quietly weaken the very check this function exists to make.
    fn written_member(kind: &str) -> &'static str {
        match kind {
            "rename-node-kind" | "change-node-kind-label" | "change-node-kind-variant" | "change-node-kind-description" | "change-node-kind-icon" | "change-node-kind-unit" => "nodeKind",
            "update-presentation" => "presentation",
            "create-handle-kind" | "delete-handle-kind" | "rename-handle-kind" | "change-handle-kind-label" | "change-handle-kind-color" | "change-handle-kind-default-wire-kind" => "handleKinds",
            "create-handle" | "delete-handle" | "move-handle" | "change-handle-handle-kind" => "handles",
            "add-compatibility-rule" | "remove-compatibility-rule" => "compatibility",
            "add-attribute" | "remove-attribute" => "attributes",
            "add-author" | "remove-author" => "authors",
            "move-camera2d" | "scale-camera2d" => "camera2d",
            _ => "meta",
        }
    }

    /// 🔀️ Each verb writes exactly ONE of the ten members. That is the check an after-snapshot
    /// comparison cannot make on its own: an implementation that re-derived a sibling table on every
    /// edit — renumbering handles, re-sorting handle kinds — would still land on the right value for
    /// the member it meant to write.
    fn touches_one(scenario: &str, kind: &str, before: &Json, after: &Json) -> Result<(), String> {
        let written = written_member(kind);
        let moved: Vec<String> = MEMBERS.iter().filter(|name| before.get(name) != after.get(name)).map(|name| (*name).to_string()).collect();
        if moved != vec![written.to_string()] {
            return Err(format!("{scenario}: this verb writes {written} and nothing else, but {moved:?} moved"));
        }
        Ok(())
    }

    fn faults(report: &Json, key: &str) -> Result<Vec<String>, String> {
        Ok(members(report, key)?.iter().filter(|message| { let level = message.str("level"); level == "error" || level == "fatal" }).map(|message| message.str("code")).collect())
    }
    //#endregion 🔖️Plan

    //#region 🔖️Handlers
    /// 🎯️ Applies one kind to the REAL derived node kind with the parameters the feature states.
    pub fn mutate(kind: &'static str) -> impl Fn(&Context) -> Result<Outcome, String> {
        move |ctx: &Context| {
            let base = fixture_text(ctx, "hexagonal-cut-concrete-forest-left")?;
            let report = report_of(&format!("mutate-{kind}"), &base, ctx.doc_string()?, &base)?;
            let raised = faults(&report, "messages")?;
            if !raised.is_empty() {
                return Err(format!("mutate-{kind}: the feature's parameters were rejected with {raised:?}"));
            }
            let applied = member(&report, "snapshot")?;
            let start = member(&report, "base")?;
            if applied == start {
                return Err(format!("mutate-{kind}: the forward mutation left the document untouched, so nothing was proved"));
            }
            touches_one(&format!("mutate-{kind}"), kind, start, applied)?;
            Ok(Outcome::with_raw(applied.to_string().into_bytes(), applied.clone()))
        }
    }

    /// ↩️ Applies one kind to the REAL derived document and then EVERY step of its OWN computed
    /// inverse. The projection carries BOTH documents, so all twenty-six rows do not project the same
    /// value.
    pub fn inverse(kind: &'static str) -> impl Fn(&Context) -> Result<Outcome, String> {
        move |ctx: &Context| {
            let base = fixture_text(ctx, "hexagonal-cut-concrete-forest-left")?;
            let report = report_of(&format!("inverse-{kind}"), &base, ctx.doc_string()?, &base)?;
            let raised = faults(&report, "inverseMessages")?;
            if !raised.is_empty() {
                return Err(format!("inverse-{kind}: an inverse step was rejected with {raised:?}, so the document never got the chance to return"));
            }
            let applied = member(&report, "snapshot")?;
            let start = member(&report, "base")?;
            if applied == start {
                return Err(format!("inverse-{kind}: the forward mutation left the document untouched, so restoring it proves nothing"));
            }
            let restored = member(&report, "inverseSnapshot")?;
            if restored != start {
                return Err(format!("inverse law violated: applying {kind:?} to the real document and then its own inverse did not restore it"));
            }
            let projection = Json::Object(vec![("mutated".to_string(), applied.clone()), ("restored".to_string(), restored.clone())]);
            Ok(Outcome::with_raw(projection.to_string().into_bytes(), projection))
        }
    }

    /// 📐️ Replays one committed handcrafted specification vector, read through the plan's declared
    /// fixtures — the same three files the Python reference reads.
    pub fn spec_vector(kind: &'static str) -> impl Fn(&Context) -> Result<Outcome, String> {
        move |ctx: &Context| {
            let before = fixture_text(ctx, "⬅️before")?;
            let mutation = fixture_text(ctx, "🦠️mutation")?;
            let after = fixture_text(ctx, "➡️after")?;
            let report = report_of(&format!("spec-vector-{kind}"), &before, &mutation, &after)?;
            let applied = member(&report, "snapshot")?;
            if applied != member(&report, "expectedSnapshot")? {
                return Err(format!("spec-vector-{kind}: the applied document is not the committed after-snapshot"));
            }
            let start = member(&report, "base")?;
            if applied == start {
                return Err(format!("spec-vector-{kind}: the committed vector left the document untouched, so nothing was proved"));
            }
            touches_one(&format!("spec-vector-{kind}"), kind, start, applied)?;
            if member(&report, "inverseSnapshot")? != start {
                return Err(format!("spec-vector-{kind}: applying the vector and then its own inverse did not restore the committed before-snapshot"));
            }
            Ok(Outcome::with_raw(applied.to_string().into_bytes(), applied.clone()))
        }
    }

    /// 🔁️ The real derived node kind through this subset's own JSON codec, reached through the
    /// bridge's `base` member with a payload that names the meta description the document ALREADY
    /// holds, so nothing is applied. The `.dsl.semio` carrier law is NOT asserted here: this subset's
    /// `store::ArtifactDsl` impl is handwritten `async` and the generated test host is synchronous,
    /// so its `parse_dsl`/`print_dsl` are unreachable from a case adapter. That gap is stated in this
    /// file's module docstring and in the feature, not papered over.
    pub fn round_trip(ctx: &Context) -> Result<Outcome, String> {
        let derived = fixture_text(ctx, "hexagonal-cut-concrete-forest-left")?;
        let report = report_of("identity-round-trip", &derived, ctx.doc_string()?, &derived)?;
        let base = member(&report, "base")?;
        if member(&report, "snapshot")? != base {
            return Err("identity-round-trip: the doc string must name the value the derived document already holds, but applying it moved the document".to_string());
        }
        Ok(Outcome::with_raw(base.to_string().into_bytes(), base.clone()))
    }
    //#endregion 🔖️Handlers
}
//#endregion 🔖️Subject

//#region 🔖️Registration
/// 🧭️ Registration entry point the generated host calls, by FULL expanded scenario id. SUBJECT only:
/// the reference for every scenario here is the Python implementation beside this file.
pub fn adapter() -> Adapter {
    let built = Adapter::new("rust");
    #[cfg(feature = "sut")]
    {
        let mut built = built;
        for kind in KINDS {
            built = built.subject(&format!("mutate-{kind}"), subject::mutate(kind));
            built = built.subject(&format!("inverse-{kind}"), subject::inverse(kind));
            built = built.subject(&format!("spec-vector-{kind}"), subject::spec_vector(kind));
        }
        return built.subject("identity-round-trip", subject::round_trip);
    }
    #[cfg(not(feature = "sut"))]
    {
        let _ = (KINDS, MEMBERS, parse_json as fn(&str) -> Result<Json, String>);
        built
    }
}
//#endregion 🔖️Registration
