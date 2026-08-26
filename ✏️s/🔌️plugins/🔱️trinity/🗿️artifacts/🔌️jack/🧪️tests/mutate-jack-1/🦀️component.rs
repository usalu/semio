//! 🔌 `s.trinity.jack` exhaustive mutation case — Rust SUBJECT adapter. Ticket
//! 26/08/23/END-TO-END-TESTING-REFACTOR.
//!
//! This case is a CROSS-LANGUAGE DIFFERENTIAL. The reference is `🐍️component.py` beside this file —
//! a second implementation of the scene, of its `.dsl.semio` carrier and of all eight typed
//! mutations, written in Python from this subset's committed snapshot schema, mutation grammar and
//! specification vectors. This adapter registers the SUBJECT half only: keeping oracle registrations
//! here would put this repository's answer on both sides of the comparison.
//!
//! **What the two roles each hold.** The cross-language projection is the six members BOTH committed
//! serializations of a jack scene carry — `schema`, `name`, `camera`, `nodes`, `edges` and
//! `rootNodeId`. The composed `content` child is outside it because its `childId` is a digest no
//! second implementation can reproduce, and `manifest`/`manifestId` are outside it because the
//! carrier writes one and the specification vectors write the other. `content` is still asserted
//! HERE, in role, and in the sharpest form this artifact allows: an applied mutation must MOVE the
//! handle (it is a digest of the child, so it moves if and only if the scene did) and an undo must
//! bring it back. Every check this case already made is still here.

use semio_repo_test_host::Adapter;

//#region 🔖️Kinds
/// 🏷️ Mirrors `TrinityGraphMutation::KINDS` (`../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/
/// 🧬️mutations/🦀️component.rs`) — duplicated, not imported, because the oracle-only build must not
/// link the subject crate. The contract's mutation-coverage gate keeps this list honest against the
/// catalog; `kinds_match_the_enum_and_the_catalog` in that production file keeps it honest against
/// the enum.
const KINDS: &[&str] = &["create-node", "delete-node", "create-edge", "delete-edge", "rename-node", "move-node", "change-data-property", "remove-data-property"];

//#endregion 🔖️Kinds


//#region 🔖️Subject
#[cfg(feature = "sut")]
mod subject {
    use semio_repo_test_host::{parse_json, Context, Json, Outcome};
    use semio_s_plugin_trinity::artifacts::jack::standards::v1::subsets::any::schema::mutations::{apply_trinity_graph_mutation_reporting, decode_trinity_graph_mutation_json, inverse_trinity_graph_mutation_steps, TrinityGraphMutation};
    use semio_s_plugin_trinity::artifacts::jack::standards::v1::subsets::any::schema::snapshot::{decode_jack_snapshot_json, encode_jack_snapshot_json, jack_scene_summary, parse_jack_dsl, print_jack_dsl, JackSnapshot};

    //#region 🔖️Plan
    /// 📤️ What parity compares: the six members both committed serializations of a jack scene carry.
    /// `content` is a digest handle and `manifest`/`manifestId` appear in only one of the two forms,
    /// so neither is comparable across languages; both are asserted in role below instead.
    const MEMBERS: &[&str] = &["schema", "name", "camera", "nodes", "edges", "rootNodeId"];

    /// 🧫️ The one declared fixture URI of this scenario's steps containing `needle`.
    fn uri_in(ctx: &Context, needle: &str) -> Result<String, String> {
        ctx.scenario
            .steps
            .iter()
            .flat_map(|(_, text)| text.split_whitespace())
            .find(|token| (token.starts_with("asset://") || token.starts_with("local://") || token.starts_with("shared://")) && token.contains(needle))
            .map(|token| token.to_string())
            .ok_or_else(|| format!("scenario {} declares no fixture URI containing {needle:?}", ctx.scenario.id))
    }

    /// 🧫️ The declared fixture's bytes as UTF-8 text.
    fn fixture_text(ctx: &Context, needle: &str) -> Result<String, String> {
        let uri = uri_in(ctx, needle)?;
        String::from_utf8(ctx.fixture_bytes(&uri)?).map_err(|error| format!("the declared fixture {uri} is not UTF-8: {error}"))
    }

    fn snapshot_of(text: &str, label: &str, kind: &str) -> Result<JackSnapshot, String> {
        decode_jack_snapshot_json(text).map_err(|error| format!("mutate-jack-1: the {label} snapshot for {kind:?} must decode: {error}"))
    }

    fn mutation_of(text: &str, label: &str, kind: &str) -> Result<TrinityGraphMutation, String> {
        decode_trinity_graph_mutation_json(text).map_err(|error| format!("mutate-jack-1: the {label} payload for {kind:?} must decode: {error}"))
    }

    /// 📤️ The comparable members of one scene, defaulting an absent `nodes`/`edges`/`rootNodeId` to
    /// the empty one — which is how the specification vectors write a scene with neither.
    fn projection(snapshot: &JackSnapshot) -> Result<Json, String> {
        let whole = parse_json(&encode_jack_snapshot_json(snapshot))?;
        let mut entries = Vec::new();
        for name in MEMBERS {
            let value = whole.get(name).cloned().unwrap_or(match *name {
                "nodes" | "edges" => Json::Array(Vec::new()),
                _ => Json::String(String::new()),
            });
            entries.push(((*name).to_string(), value));
        }
        Ok(Json::Object(entries))
    }

    fn disagreement(what: &str, got: &JackSnapshot, expected: &JackSnapshot) -> String {
        format!("{what}\n     got: {} {}\nexpected: {} {}", encode_jack_snapshot_json(got), jack_scene_summary(got), encode_jack_snapshot_json(expected), jack_scene_summary(expected))
    }

    /// 🗣️ The real committed tower, with its composed child resolved by the parse itself.
    fn tower(ctx: &Context) -> Result<JackSnapshot, String> {
        let parsed = parse_jack_dsl(&fixture_text(ctx, "📚️examples")?)?;
        let summary = jack_scene_summary(&parsed);
        if !summary.contains("jack_orphan") || !summary.contains("e-jack-prune") {
            return Err(format!("mutate-jack-1: the committed tower must resolve its composed child to the nine-piece scene these payloads address, got {summary}"));
        }
        Ok(parsed)
    }
    //#endregion 🔖️Plan

    //#region 🔖️Handlers
    /// 🎯️ Applies one kind to the REAL committed Nakagin Capsule Tower with the parameters the
    /// feature states. The observability law is asserted here in the sharpest form this artifact
    /// allows: the content-addressed child handle is a digest of the scene, so an applied mutation
    /// must MOVE it, and a mutation that quietly did nothing cannot pass.
    pub fn mutate(kind: &'static str) -> impl Fn(&Context) -> Result<Outcome, String> {
        move |ctx: &Context| {
            let base = tower(ctx)?;
            let payload = mutation_of(ctx.doc_string()?, "feature", kind)?;
            let mut applied = base.clone();
            let raised = apply_trinity_graph_mutation_reporting(&mut applied, &payload);
            if !raised.is_empty() {
                return Err(format!("mutate-{kind}: the feature's parameters were meant to APPLY to the committed tower, but the implementation raised {raised:?}"));
            }
            if applied.content.child_id == base.content.child_id {
                return Err(format!("mutate-{kind}: applying this kind to the tower left the content-addressed child handle at {} — the mutation never reached the scene ({})", base.content.child_id, jack_scene_summary(&applied)));
            }
            let projection = projection(&applied)?;
            Ok(Outcome::with_raw(projection.to_string().into_bytes(), projection))
        }
    }

    /// ↩️ Applies one kind to the REAL tower and then EVERY step of its OWN computed inverse. The
    /// restoring law is asserted on the WHOLE snapshot, content handle included — because that handle
    /// is a digest of the child, restoring it is the strongest available statement that the scene
    /// came back, piece order and port ids and all. The projection carries both scenes, so all eight
    /// rows do not project the same value.
    pub fn inverse(kind: &'static str) -> impl Fn(&Context) -> Result<Outcome, String> {
        move |ctx: &Context| {
            let base = tower(ctx)?;
            let payload = mutation_of(ctx.doc_string()?, "feature", kind)?;
            let mut current = base.clone();
            let raised = apply_trinity_graph_mutation_reporting(&mut current, &payload);
            if !raised.is_empty() {
                return Err(format!("inverse-{kind}: the forward mutation was rejected: {raised:?}"));
            }
            if current.content.child_id == base.content.child_id {
                return Err(format!("inverse-{kind}: the forward mutation left the content handle untouched, so restoring it proves nothing ({})", jack_scene_summary(&current)));
            }
            let mutated = projection(&current)?;
            for step in inverse_trinity_graph_mutation_steps(&payload, &base) {
                let undone = apply_trinity_graph_mutation_reporting(&mut current, &step);
                if undone.iter().any(|(code, _)| code != "mutation.no-op") {
                    return Err(format!("inverse-{kind}: an inverse step was rejected: {undone:?}"));
                }
            }
            if current != base {
                return Err(disagreement(&format!("inverse law violated: applying {kind:?} and then its own inverse did not restore the original"), &current, &base));
            }
            let projection = Json::Object(vec![("mutated".to_string(), mutated), ("restored".to_string(), projection(&current)?)]);
            Ok(Outcome::with_raw(projection.to_string().into_bytes(), projection))
        }
    }

    /// 📐️ Replays one committed handcrafted vector, read through the plan's declared fixtures — the
    /// same three files the Python reference reads. All eight are NEGATIVE, so the feature's
    /// `verdict` column states which refusal each commits to: `refused` must raise a fault and leave
    /// the document alone, `noop` must be ACCEPTED while leaving it alone. Both additionally require
    /// that the content-addressed child handle was NOT re-minted — without it a refusal that quietly
    /// rebuilt the child would look clean.
    pub fn spec_vector(kind: &'static str) -> impl Fn(&Context) -> Result<Outcome, String> {
        move |ctx: &Context| {
            let base = snapshot_of(&fixture_text(ctx, "⬅️before")?, "committed before", kind)?;
            let expected = snapshot_of(&fixture_text(ctx, "➡️after")?, "committed after", kind)?;
            let vector = mutation_of(&fixture_text(ctx, "🦠️mutation")?, "committed vector", kind)?;
            let verdict = ctx.doc_json()?.str("verdict");
            let mut replayed = base.clone();
            let raised = apply_trinity_graph_mutation_reporting(&mut replayed, &vector);
            match verdict.as_str() {
                "refused" => {
                    if raised.is_empty() {
                        return Err(format!("spec-vector-{kind}: the committed vector declares a refusal, but the implementation raised nothing at all"));
                    }
                }
                "noop" => {
                    if !raised.iter().all(|(code, _)| code == "mutation.no-op") {
                        return Err(format!("spec-vector-{kind}: the committed vector declares an accepted no-op, but the implementation raised {raised:?}"));
                    }
                }
                other => return Err(format!("spec-vector-{kind}: the feature declares an unknown verdict {other:?}")),
            }
            if replayed != expected {
                return Err(disagreement(&format!("spec-vector-{kind}: the vector must leave the document at the committed after-snapshot"), &replayed, &expected));
            }
            if replayed.content != base.content {
                return Err(format!("spec-vector-{kind}: the vector re-minted the composed child handle ({} -> {}) — the scene may be unchanged, but the document is no longer the same document", base.content.child_id, replayed.content.child_id));
            }
            let projection = projection(&replayed)?;
            Ok(Outcome::with_raw(projection.to_string().into_bytes(), projection))
        }
    }

    /// 🔁️ The real committed tower through its own DSL carrier. `.jack.dsl.semio` is a fixed-layout
    /// record grammar whose field values are hex-encoded, with no writer freedom at all, and the
    /// committed example is this codec's own output, committed as such. The wave's usual "output
    /// must not equal input" tripwire therefore does not apply and would be the wrong law here; the
    /// byte-exact law is asserted instead, together with a scene check a parser returning an
    /// unresolved child cannot satisfy. The projection is what the Python reference read out of the
    /// SAME committed bytes.
    pub fn round_trip(ctx: &Context) -> Result<Outcome, String> {
        let text = fixture_text(ctx, "📚️examples")?;
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
/// 🧭️ Registration entry point the generated host calls, by FULL expanded scenario id. SUBJECT only:
/// the reference for every scenario here is the Python implementation beside this file, and
/// registering an oracle handler as well would put this repository's answer on both sides.
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
        let _ = KINDS;
        built
    }
}
//#endregion 🔖️Registration
