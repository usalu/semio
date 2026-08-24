//! 🦀️ Assembly-scene exhaustive mutation case — Rust adapter. Ticket 26/08/23/END-TO-END-TESTING-
//! REFACTOR. Recorded no-oracle decision `jack-1-assembly-scene-mutation-semantics`
//! (`../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️oracle/🔣️component.json`): `trinity.graph` is a
//! semio-NATIVE assembly scene with no third-party reader or writer, so `oracle` here reads the
//! committed, independently handcrafted per-kind specification fixtures
//! (`../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/<slug>/🧪️tests/<fixture>/`)
//! literally — no recomputation, no reimplementation of mutation semantics. `subject` drives this
//! repository's own `apply_trinity_graph_mutation_reporting` over the full eight-kind
//! `TrinityGraphMutation` vocabulary.
//!
//! **The two facts everything here turns on.** `JackSnapshot` persists NO pieces and NO connections
//! — only a manifest, a camera, a root node id and one composed `s.stdio.semio.graph` child handle
//! whose `childId` is a content digest — so the persisted projection moves if and only if the scene
//! moved, and a committed `➡️after` for an APPLIED mutation cannot be hand-authored. And all eight
//! committed vectors leave the document byte-identical for FOUR different reasons: `target-missing`
//! at Error, `duplicate-id` at Fatal, `invariant` at Fatal for an edge naming an absent endpoint,
//! and `no-op` at Warning for the four kinds that degrade rather than refuse. Each `mutate-<kind>`
//! handler therefore runs both halves: the committed vector for its exact `(code, severity)` pair
//! with the handle required to stay put, and the feature's own real-effect payload against the real
//! committed Nakagin tower with the handle required to move.
//!
//! **Where the assertion lives.** A recorded no-oracle case runs NO oracle role, so every law this
//! case claims is asserted INSIDE the subject handler. A handler that merely returned `Ok` would
//! report a pass having checked nothing at all.
//!
//! **Why the shared `⚖️law` module is not used here.** `✏️s/🔌️plugins/🗄️stdio/🧪️oracle/⚖️law` is
//! reachable only where the stdio oracle crate is linked into the generated host, which happens for
//! a case whose owner sits under `✏️s/🔌️plugins/🗄️stdio`. This case's owner does not, and declaring
//! stdio's contribution directory as a host package for the jack artifact would make one plugin's
//! test tree a build dependency of another's. The laws are stated inline, in the same words and with
//! the same strictness.
//!
//! **How the fixture reaches typed values.** The generated test host links only
//! `semio-repo-test-host` and, behind `sut`, this plugin's own crate — no `serde`, no `serde_json`,
//! and this crate's `protocol`/`store` extern-crate aliases are private (`📦️glue.rs`). The subset's
//! own production code exports the bridges instead: `decode_jack_snapshot_json`/
//! `encode_jack_snapshot_json`/`parse_jack_dsl`/`print_jack_dsl`/`jack_scene_summary`
//! (`…/🧬️schema/📸️snapshot/🦀️component.rs`) and `decode_trinity_graph_mutation_json`/
//! `apply_trinity_graph_mutation_reporting`/`inverse_trinity_graph_mutation_steps`
//! (`…/🧬️schema/🧬️mutations/🦀️component.rs`).

use semio_repo_test_host::{parse_json, Adapter, Context, Json, Outcome};

//#region 🔖️Kinds
/// 🏷️ Mirrors `TrinityGraphMutation::KINDS` (`../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/
/// 🧬️mutations/🦀️component.rs`) — duplicated, not imported, because the oracle-only build must not
/// link the subject crate. The contract's mutation-coverage gate keeps this list honest against the
/// catalog; `kinds_match_the_enum_and_the_catalog` in that production file keeps it honest against
/// the enum.
const KINDS: &[&str] = &["create-node", "delete-node", "create-edge", "delete-edge", "rename-node", "move-node", "change-data-property", "remove-data-property"];

/// 🗣️ The real committed artifact — the Nakagin Capsule Tower: a service core, five stacked capsule
/// pieces, three unattached jacks, six connections.
#[cfg(feature = "sut")]
const DSL_ASSET: &str = "asset://🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🖼️assets/🗣️example.dsl.semio";
//#endregion 🔖️Kinds

//#region 🔖️Fixtures
/// 🧫️ The committed `(before, mutation, after, outcome)` specification vector TEXT for one kind,
/// read literally via `include_str!` — this IS the independently handcrafted vector the no-oracle
/// decision rests on, never recomputed.
fn fixture_text(kind: &str) -> (&'static str, &'static str, &'static str, &'static str) {
    match kind {
        "create-node" => (
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌱️create-node/🧪️tests/rejects-a-node-id-the-scene-already-holds/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌱️create-node/🧪️tests/rejects-a-node-id-the-scene-already-holds/🦠️mutation/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌱️create-node/🧪️tests/rejects-a-node-id-the-scene-already-holds/📸️snapshot/➡️after/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌱️create-node/🧪️tests/rejects-a-node-id-the-scene-already-holds/🎯️outcome/🔣️component.json"),
        ),
        "delete-node" => (
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗑️delete-node/🧪️tests/rejects-deleting-a-node-the-scene-never-had/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗑️delete-node/🧪️tests/rejects-deleting-a-node-the-scene-never-had/🦠️mutation/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗑️delete-node/🧪️tests/rejects-deleting-a-node-the-scene-never-had/📸️snapshot/➡️after/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗑️delete-node/🧪️tests/rejects-deleting-a-node-the-scene-never-had/🎯️outcome/🔣️component.json"),
        ),
        "create-edge" => (
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔗️create-edge/🧪️tests/rejects-an-edge-whose-endpoints-are-absent/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔗️create-edge/🧪️tests/rejects-an-edge-whose-endpoints-are-absent/🦠️mutation/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔗️create-edge/🧪️tests/rejects-an-edge-whose-endpoints-are-absent/📸️snapshot/➡️after/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔗️create-edge/🧪️tests/rejects-an-edge-whose-endpoints-are-absent/🎯️outcome/🔣️component.json"),
        ),
        "delete-edge" => (
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✂️delete-edge/🧪️tests/rejects-cutting-an-edge-the-scene-never-had/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✂️delete-edge/🧪️tests/rejects-cutting-an-edge-the-scene-never-had/🦠️mutation/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✂️delete-edge/🧪️tests/rejects-cutting-an-edge-the-scene-never-had/📸️snapshot/➡️after/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✂️delete-edge/🧪️tests/rejects-cutting-an-edge-the-scene-never-had/🎯️outcome/🔣️component.json"),
        ),
        "rename-node" => (
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✏️rename-node/🧪️tests/keeps-the-name-a-node-already-carries/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✏️rename-node/🧪️tests/keeps-the-name-a-node-already-carries/🦠️mutation/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✏️rename-node/🧪️tests/keeps-the-name-a-node-already-carries/📸️snapshot/➡️after/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✏️rename-node/🧪️tests/keeps-the-name-a-node-already-carries/🎯️outcome/🔣️component.json"),
        ),
        "move-node" => (
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📍️move-node/🧪️tests/keeps-a-node-at-the-point-it-already-occupies/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📍️move-node/🧪️tests/keeps-a-node-at-the-point-it-already-occupies/🦠️mutation/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📍️move-node/🧪️tests/keeps-a-node-at-the-point-it-already-occupies/📸️snapshot/➡️after/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📍️move-node/🧪️tests/keeps-a-node-at-the-point-it-already-occupies/🎯️outcome/🔣️component.json"),
        ),
        "change-data-property" => (
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔧️change-data-property/🧪️tests/keeps-a-node-property-at-the-value-it-already-holds/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔧️change-data-property/🧪️tests/keeps-a-node-property-at-the-value-it-already-holds/🦠️mutation/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔧️change-data-property/🧪️tests/keeps-a-node-property-at-the-value-it-already-holds/📸️snapshot/➡️after/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔧️change-data-property/🧪️tests/keeps-a-node-property-at-the-value-it-already-holds/🎯️outcome/🔣️component.json"),
        ),
        "remove-data-property" => (
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧹️remove-data-property/🧪️tests/keeps-an-edge-without-the-property-it-never-had/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧹️remove-data-property/🧪️tests/keeps-an-edge-without-the-property-it-never-had/🦠️mutation/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧹️remove-data-property/🧪️tests/keeps-an-edge-without-the-property-it-never-had/📸️snapshot/➡️after/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧹️remove-data-property/🧪️tests/keeps-an-edge-without-the-property-it-never-had/🎯️outcome/🔣️component.json"),
        ),
        other => panic!("mutate-jack-1: no specification vector registered for kind {other:?}"),
    }
}

/// 🔎️ Parses one embedded fixture file into the framework's own dependency-free `Json`.
fn canonical(text: &str) -> Json {
    parse_json(text).unwrap_or_else(|error| panic!("committed fixture JSON must parse: {error}"))
}
//#endregion 🔖️Fixtures

//#region 🔖️Oracle
/// 🔮️ The forward reference answer: the committed AFTER snapshot, read literally. For every kind in
/// this vocabulary that is byte-identical to the BEFORE snapshot — four refusals and four degenerate
/// applications — which is exactly why the `(code, severity)` pair rather than the document is what
/// the subject handler holds each vector to.
fn mutate_oracle_for(kind: &'static str) -> impl Fn(&Context) -> Result<Outcome, String> {
    move |_ctx: &Context| {
        let (_before, _mutation, after, _outcome) = fixture_text(kind);
        Ok(Outcome::with_raw(after.as_bytes().to_vec(), canonical(after)))
    }
}

/// 🔮️ The inverse reference answer: the committed BEFORE snapshot.
fn inverse_oracle_for(kind: &'static str) -> impl Fn(&Context) -> Result<Outcome, String> {
    move |_ctx: &Context| {
        let (before, _mutation, _after, _outcome) = fixture_text(kind);
        Ok(Outcome::with_raw(before.as_bytes().to_vec(), canonical(before)))
    }
}
//#endregion 🔖️Oracle

//#region 🔖️Subject
#[cfg(feature = "sut")]
mod subject {
    use semio_repo_test_host::{parse_json, Context, Json, Outcome};
    use semio_s_plugin_trinity::artifacts::jack::standards::v1::subsets::any::schema::mutations::{apply_trinity_graph_mutation_reporting, decode_trinity_graph_mutation_json, inverse_trinity_graph_mutation_steps, TrinityGraphMutation};
    use semio_s_plugin_trinity::artifacts::jack::standards::v1::subsets::any::schema::snapshot::{decode_jack_snapshot_json, encode_jack_snapshot_json, jack_scene_summary, parse_jack_dsl, print_jack_dsl, JackSnapshot};

    //#region 🔖️FixtureDecode
    fn snapshot_of(text: &str, label: &str, kind: &str) -> Result<JackSnapshot, String> {
        decode_jack_snapshot_json(text).map_err(|error| format!("mutate-jack-1: the committed {label}-snapshot for {kind:?} must decode: {error}"))
    }

    fn mutation_of(text: &str, label: &str, kind: &str) -> Result<TrinityGraphMutation, String> {
        decode_trinity_graph_mutation_json(text).map_err(|error| format!("mutate-jack-1: the {label} payload for {kind:?} must decode: {error}"))
    }

    fn projection(snapshot: &JackSnapshot) -> Result<Json, String> {
        parse_json(&encode_jack_snapshot_json(snapshot))
    }

    fn disagreement(what: &str, got: &JackSnapshot, expected: &JackSnapshot) -> String {
        format!("{what}\n     got: {} {}\nexpected: {} {}", encode_jack_snapshot_json(got), jack_scene_summary(got), encode_jack_snapshot_json(expected), jack_scene_summary(expected))
    }

    /// 🗣️ The real committed tower, with its composed child resolved by the parse itself.
    fn tower(ctx: &Context) -> Result<JackSnapshot, String> {
        let text = String::from_utf8(ctx.fixture_bytes(super::DSL_ASSET)?).map_err(|error| format!("mutate-jack-1: the committed tower artifact is not UTF-8: {error}"))?;
        let parsed = parse_jack_dsl(&text)?;
        let summary = jack_scene_summary(&parsed);
        if !summary.contains("jack_orphan") || !summary.contains("e-jack-prune") {
            return Err(format!("mutate-jack-1: the committed tower must resolve its composed child to the nine-piece scene these payloads address, got {summary}"));
        }
        Ok(parsed)
    }

    fn params_text(row: &Json) -> String {
        row.get("params").map(Json::to_string).unwrap_or_default()
    }
    //#endregion 🔖️FixtureDecode

    //#region 🔖️Laws
    /// 🎯️ The committed vector's own claim, in full: the declared diagnostic code, its declared
    /// SEVERITY, and — the part only this artifact can state — that the content-addressed child
    /// handle was not re-minted. All eight vectors leave the document byte-identical, so without the
    /// severity a refusal and a degenerate application are indistinguishable, and without the handle
    /// check a refusal that quietly rebuilt the child would look clean.
    fn vector_reports(kind: &str, code: &str, level: &str, declared: &Json, raised: &[(String, String)], before: &JackSnapshot, after: &JackSnapshot, expected: &JackSnapshot) -> Result<(), String> {
        let declared_code = if declared.str("code").is_empty() { declared.array("messages").first().map(|message| message.str("code")).unwrap_or_default() } else { declared.str("code") };
        if declared_code != code {
            return Err(format!("mutate-{kind}: the feature's Examples row names code {code:?} but the committed outcome declares {declared_code:?} — the two declarations of the same vector have drifted"));
        }
        let Some((raised_code, raised_level)) = raised.first() else {
            return Err(format!("mutate-{kind}: the committed vector must report {code:?} at {level}, but the implementation raised nothing at all"));
        };
        if raised_code != code || raised_level != level {
            return Err(format!("mutate-{kind}: the committed vector must report {code:?} at {level}, but the implementation raised {raised_code:?} at {raised_level} — all eight vectors here leave the document byte-identical, so this pair is the only thing that tells a refusal from a degenerate application"));
        }
        if after != expected {
            return Err(disagreement(&format!("mutate-{kind}: the vector must leave the document at the committed after-snapshot"), after, expected));
        }
        if after.content != before.content {
            return Err(format!("mutate-{kind}: the vector re-minted the composed child handle ({} -> {}) — the scene may be unchanged, but the document is no longer the same document", before.content.child_id, after.content.child_id));
        }
        Ok(())
    }

    /// 👁️ The observability law, in the exact form this artifact can state it: an APPLIED mutation
    /// must move the content-addressed child handle, which is a digest of the child and therefore
    /// moves if and only if the scene moved.
    fn application_is_observable(kind: &str, raised: &[(String, String)], base: &JackSnapshot, mutated: &JackSnapshot) -> Result<(), String> {
        if !raised.is_empty() {
            return Err(format!("mutate-{kind}: the real-effect payload was meant to APPLY to the committed tower, but the implementation raised {raised:?}"));
        }
        if mutated.content.child_id == base.content.child_id {
            return Err(format!("mutate-{kind}: applying this kind to the tower left the content-addressed child handle at {} — the mutation never reached the scene, so the scenario would report a pass for a mutation it never observed ({})", base.content.child_id, jack_scene_summary(mutated)));
        }
        Ok(())
    }
    //#endregion 🔖️Laws

    //#region 🔖️Handlers
    /// 🎯️ Both halves in one scenario: the committed vector for its exact `(code, severity)` pair,
    /// then the feature's own real-effect payload against the real committed tower for its effect.
    pub fn mutate(kind: &'static str) -> impl Fn(&Context) -> Result<Outcome, String> {
        move |ctx: &Context| {
            let row = ctx.doc_json()?;
            let (before, vector, after, outcome) = super::fixture_text(kind);
            let base = snapshot_of(before, "before", kind)?;
            let expected = snapshot_of(after, "after", kind)?;
            let vector = mutation_of(vector, "committed vector", kind)?;
            let mut replayed = base.clone();
            let raised = apply_trinity_graph_mutation_reporting(&mut replayed, &vector);
            vector_reports(kind, &row.str("code"), &row.str("level"), &parse_json(outcome)?, &raised, &base, &replayed, &expected)?;

            let tower = tower(ctx)?;
            let payload = mutation_of(&params_text(&row), "feature real-effect", kind)?;
            let mut applied = tower.clone();
            let applied_messages = apply_trinity_graph_mutation_reporting(&mut applied, &payload);
            application_is_observable(kind, &applied_messages, &tower, &applied)?;
            let projection = projection(&applied)?;
            Ok(Outcome::with_raw(projection.to_string().into_bytes(), projection))
        }
    }

    /// ↩️ The metamorphic inverse law over the real committed tower: applying the kind and then its
    /// OWN computed inverse must restore the document exactly, content handle included. Because that
    /// handle is a digest of the child, restoring it is the strongest available statement that the
    /// whole scene came back — piece order, port ids and connection endpoints included, not merely
    /// membership.
    pub fn inverse(kind: &'static str) -> impl Fn(&Context) -> Result<Outcome, String> {
        move |ctx: &Context| {
            let row = ctx.doc_json()?;
            let base = tower(ctx)?;
            let payload = mutation_of(&params_text(&row), "feature real-effect", kind)?;
            let mut current = base.clone();
            let raised = apply_trinity_graph_mutation_reporting(&mut current, &payload);
            if !raised.is_empty() {
                return Err(format!("inverse-{kind}: the forward mutation was rejected: {raised:?}"));
            }
            if current.content.child_id == base.content.child_id {
                return Err(format!("inverse-{kind}: the forward mutation left the content handle untouched, so restoring it proves nothing ({})", jack_scene_summary(&current)));
            }
            for step in inverse_trinity_graph_mutation_steps(&payload, &base) {
                let undone = apply_trinity_graph_mutation_reporting(&mut current, &step);
                if undone.iter().any(|(code, _)| code != "mutation.no-op") {
                    return Err(format!("inverse-{kind}: an inverse step was rejected: {undone:?}"));
                }
            }
            if current != base {
                return Err(disagreement(&format!("inverse law violated: applying {kind:?} and then its own inverse did not restore the original"), &current, &base));
            }
            let projection = projection(&current)?;
            Ok(Outcome::with_raw(projection.to_string().into_bytes(), projection))
        }
    }

    /// 🔁️ The real committed tower through its own DSL carrier. `.jack.dsl.semio` is a fixed-layout
    /// record grammar whose field values are hex-encoded, with no writer freedom at all, and the
    /// committed example is this codec's own output, committed as such. The wave's usual "output
    /// must not equal input" tripwire therefore does not apply and would be the wrong law here; the
    /// byte-exact law is asserted instead, together with a scene check a parser returning an
    /// unresolved child cannot satisfy.
    pub fn round_trip(ctx: &Context) -> Result<Outcome, String> {
        let text = String::from_utf8(ctx.fixture_bytes(super::DSL_ASSET)?).map_err(|error| format!("identity-round-trip: the committed tower artifact is not UTF-8: {error}"))?;
        let parsed = parse_jack_dsl(&text)?;
        let summary = jack_scene_summary(&parsed);
        for piece in ["jack_orphan", "jack_prune", "jack_spare", "ci_t_f8_b_c0"] {
            if !summary.contains(piece) {
                return Err(format!("identity-round-trip: the committed tower carries the piece {piece:?}, but the parse resolved {summary}"));
            }
        }
        if parsed.root_node_id.as_deref() != Some("7dc5b737-3b6b-4068-b315-b7bacc91c2e1") {
            return Err(format!("identity-round-trip: the committed tower is rooted at its service core, but the parse read root {:?}", parsed.root_node_id));
        }
        let printed = print_jack_dsl(&parsed);
        let reparsed = parse_jack_dsl(&printed)?;
        if reparsed != parsed {
            return Err(disagreement("identity-round-trip: printing the document back to DSL and reparsing it lost content", &reparsed, &parsed));
        }
        if printed != text {
            let at = printed.as_bytes().iter().zip(text.as_bytes().iter()).position(|(one, other)| one != other);
            return Err(format!(
                "exact-bytes law violated: `.jack.dsl.semio` is a fixed-layout hex-encoded record grammar and the committed example is this codec's own output, so the re-printed text was required to reproduce it — {} byte(s) out against {} byte(s) in{}",
                printed.len(),
                text.len(),
                match at {
                    Some(offset) => format!(" (first at byte {offset})"),
                    None => String::new(),
                }
            ));
        }
        let projection = projection(&parsed)?;
        Ok(Outcome::with_raw(projection.to_string().into_bytes(), projection))
    }
    //#endregion 🔖️Handlers
}
//#endregion 🔖️Subject

//#region 🔖️Registration
/// 🧭️ Registration entry point the generated host calls. Registration is by FULL expanded scenario
/// id, so the loop mirrors the feature's `Examples` tables exactly. `identity-round-trip` is
/// deliberately subject-only: the reference answer for every other scenario is a committed JSON
/// snapshot the oracle role can read literally, but the real tower is committed as `.dsl.semio` text
/// ONLY and turning that into a resolved document needs this subset's own codec, which the
/// oracle-only build must not link.
pub fn adapter() -> Adapter {
    let mut built = Adapter::new("rust");
    for kind in KINDS {
        built = built.oracle(&format!("mutate-{kind}"), mutate_oracle_for(kind)).oracle(&format!("inverse-{kind}"), inverse_oracle_for(kind));
        #[cfg(feature = "sut")]
        {
            built = built.subject(&format!("mutate-{kind}"), subject::mutate(kind)).subject(&format!("inverse-{kind}"), subject::inverse(kind));
        }
    }
    #[cfg(feature = "sut")]
    {
        built = built.subject("identity-round-trip", subject::round_trip);
    }
    built
}
//#endregion 🔖️Registration
