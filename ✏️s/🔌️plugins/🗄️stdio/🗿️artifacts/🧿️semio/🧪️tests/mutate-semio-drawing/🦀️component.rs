//! 🦀️ Semio DRAWING exhaustive mutation case — Rust SUBJECT adapter. Ticket 26/08/23/END-TO-END-
//! TESTING-REFACTOR.
//!
//! **This file no longer serves the oracle role.** The reference for `semio-v1-drawing-mutate` is
//! the registered oracle `semio-drawing-python-independent` (`../../🏅️standards/🔖️v1/🪆️subsets/
//! ✳️drawing/🧪️oracle/🔣️component.json`) — an independent Python implementation of the semio drawing
//! carrier, its recursive `DrawNode` tree and all seventeen verbs, written from the committed
//! grammar and protocol documents and living beside this file as `🐍️component.py`. The runner
//! dispatches the oracle role there and the subject role here, and compares the two projections
//! under `@comparison-ordered-json-v1`. Registering oracle handlers here as well would put this
//! repository's own answer on both sides of that comparison, which is the precise failure the
//! platform exists to prevent.
//!
//! **The drawing under test is a real one.** `local://🗣️artifact.dsl.semio` and its binary twin were
//! derived ONCE from two real committed SVG documents —
//! `🗿️artifacts/🎨️svg/🧫️fixtures/mouse.svg` and `…/🎨️svg/🧫️fixtures/qr-code.svg` — by an independent
//! Python SVG reader built on `xml.etree` plus a path-data scanner written from the SVG 1.1 §8.3
//! command grammar, never through this repository's own svg bridge. It carries three layers, 1 006
//! nodes nested five deep, 1 728 real path segments, four styles and a real 5 476-byte embedded
//! image.
//!
//! **What the handlers assert in role.** Parity across the two implementations is the primary
//! evidence, but each side still states its own law so a scenario can fail for the right reason with
//! a readable message: `inverse-<kind>` requires the mutation's OWN computed inverse to restore the
//! drawing with scene-graph order and nesting intact, `spec-vector-<kind>` requires the applied
//! drawing to be the committed after-snapshot, and `identity-round-trip` requires both committed
//! encodings to be reproduced byte for byte through `law::carrier_is_exact`.
//!
//! **How the fixture reaches typed values.** The generated test host links only
//! `semio-repo-test-host` and, behind `sut`, this subset's own crate — no `serde`, no `serde_json` —
//! so every fixture is decoded through the bridges this subset's own production code exports
//! (`decode_semio_drawing_snapshot_json`/`encode_semio_drawing_snapshot_json` and
//! `decode_semio_drawing_mutation_json`/`inverse_semio_drawing_mutation`), whose signatures name only
//! reachable types. Every input is read from a fixture the FEATURE declares, so neither adapter holds
//! a transcription that could drift away from what the other one read.

use semio_repo_test_host::Adapter;

//#region 🔖️Kinds
/// 🏷️ Mirrors `SemioDrawingMutation::KINDS` (`../../🏅️standards/🔖️v1/🪆️subsets/✳️drawing/🧬️schema/
/// 🧬️mutations/🦀️component.rs`) — duplicated, not imported, because the generated host builds this
/// file with and without the subject crate. The contract's mutation-coverage gate keeps this list
/// honest against the catalog; `kinds_match_the_enum_and_the_catalog` in that production file keeps
/// it honest against the enum.
#[cfg(feature = "sut")]
const KINDS: &[&str] = &[
    "create-layer",
    "delete-layer",
    "create-node",
    "delete-node",
    "move-node",
    "drag-nodes",
    "rotate",
    "scale",
    "reorder-nodes",
    "group",
    "ungroup",
    "flatten",
    "unflatten",
    "replace-path",
    "replace-fill",
    "change-stroke-color",
    "change-stroke-width",
];
//#endregion 🔖️Kinds

//#region 🔖️Subject
#[cfg(feature = "sut")]
mod subject {
    use semio_repo_test_host::{digest, parse_json, Context, Json, Outcome};
    use semio_s_plugin_stdio_test_oracle::law::carrier_is_exact;
    use semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::any::schema::mutations::semio_mutation_refusals;
    use semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::drawing::schema::mutations::{apply_semio_drawing_mutation, decode_semio_drawing_mutation_json, inverse_semio_drawing_mutation, SemioDrawingMutation};
    use semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::drawing::schema::snapshot::{
        decode_semio_drawing_pack, decode_semio_drawing_snapshot_json, encode_semio_drawing_pack, encode_semio_drawing_snapshot_json, parse_semio_drawing_dsl, print_semio_drawing_dsl, SemioDrawingSnapshot,
    };

    //#region 🔖️Input
    /// 🖍️ The real derived drawing — the committed `mouse.svg` and `qr-code.svg` read once by an
    /// independent SVG reader and written out through the independent Python implementation.
    const ARTIFACT_DSL: &str = "local://🗣️artifact.dsl.semio";
    /// 🎒️ The same drawing in its binary envelope, written by a separate codec from the DSL text.
    const ARTIFACT_PACK: &str = "local://🎒️artifact.pack.semio";

    /// 🧫️ Every fixture URI of one scheme the scenario's steps name, in step order. The feature is
    /// the single place those paths are written down; both adapters read them from there.
    fn step_uris(ctx: &Context, scheme: &str) -> Vec<String> {
        let mut found = Vec::new();
        for (_, text) in &ctx.scenario.steps {
            for token in text.split_whitespace() {
                if token.starts_with(scheme) {
                    found.push(token.to_string());
                }
            }
        }
        found
    }

    fn fixture_text(ctx: &Context, uri: &str) -> Result<String, String> {
        String::from_utf8(ctx.fixture_bytes(uri)?).map_err(|error| format!("{uri} is not UTF-8: {error}"))
    }

    /// 🖍️ The real derived drawing, parsed through this repository's own DSL codec.
    fn artifact(ctx: &Context) -> Result<SemioDrawingSnapshot, String> {
        parse_semio_drawing_dsl(&fixture_text(ctx, ARTIFACT_DSL)?)
    }

    /// 📜️ The scenario's own committed mutation payload — the feature owns the vector.
    fn payload(ctx: &Context) -> Result<SemioDrawingMutation, String> {
        let uri = step_uris(ctx, "local://🦠️").into_iter().next().ok_or_else(|| format!("{}: the scenario names no mutation payload", ctx.scenario.id))?;
        decode_semio_drawing_mutation_json(&fixture_text(ctx, &uri)?).map_err(|error| format!("{}: the mutation payload must decode: {error}", ctx.scenario.id))
    }

    fn vector(ctx: &Context, position: usize, label: &str) -> Result<String, String> {
        let uri = step_uris(ctx, "asset://").into_iter().nth(position).ok_or_else(|| format!("{}: the scenario names no {label} asset", ctx.scenario.id))?;
        fixture_text(ctx, &uri)
    }

    fn apply(current: &mut SemioDrawingSnapshot, step: &SemioDrawingMutation, what: &str) -> Result<(), String> {
        let outcome = apply_semio_drawing_mutation(current, step);
        let refusals = semio_mutation_refusals(&outcome);
        if refusals.is_empty() {
            return Ok(());
        }
        Err(format!("{what}: the mutation was rejected: {refusals:?}"))
    }
    //#endregion 🔖️Input

    //#region 🔖️Projection
    fn number(value: usize) -> Json {
        Json::Number(value as f64)
    }

    /// 🎯️ The projection every scenario compares under `ordered-json-v1`: the snapshot's own
    /// structural JSON shape, exactly as the committed specification vectors are written.
    fn snapshot_json(snapshot: &SemioDrawingSnapshot) -> Result<Json, String> {
        parse_json(&encode_semio_drawing_snapshot_json(snapshot))
    }

    /// 🌳️ A structural census of the scene graph, so a mutation that lands in the wrong branch shows
    /// up as a shape difference and not only as a deep value difference: per layer, the node kind
    /// histogram, the maximum depth and the total segment count. Computed over the snapshot's own
    /// JSON wire form, so this adapter never re-declares the node union.
    fn shape_json(document: &Json) -> Json {
        fn walk(node: &Json, level: usize, counts: &mut [usize; 4], depth: &mut usize, segments: &mut usize) {
            let kind = node.str("kind");
            let at = ["path", "text", "group", "image"].iter().position(|candidate| *candidate == kind.as_str()).unwrap_or(0);
            counts[at] += 1;
            *depth = (*depth).max(level);
            if kind == "path" {
                *segments += node.array("segments").len();
            }
            for child in node.array("children") {
                walk(&child, level + 1, counts, depth, segments);
            }
        }
        let layers = document
            .array("layers")
            .iter()
            .map(|layer| {
                let mut counts = [0usize; 4];
                let mut depth = 0usize;
                let mut segments = 0usize;
                if let Some(root) = layer.get("root") {
                    walk(root, 0, &mut counts, &mut depth, &mut segments);
                }
                Json::Object(vec![
                    ("id".to_string(), Json::String(layer.str("id"))),
                    ("visible".to_string(), Json::Bool(matches!(layer.get("visible"), Some(Json::Bool(true))))),
                    (
                        "nodes".to_string(),
                        Json::Object(vec![
                            ("path".to_string(), number(counts[0])),
                            ("text".to_string(), number(counts[1])),
                            ("group".to_string(), number(counts[2])),
                            ("image".to_string(), number(counts[3])),
                        ]),
                    ),
                    ("depth".to_string(), number(depth)),
                    ("segments".to_string(), number(segments)),
                ])
            })
            .collect();
        Json::Object(vec![
            ("layers".to_string(), Json::Array(layers)),
            ("styles".to_string(), Json::Array(document.array("styles").iter().map(|style| Json::String(style.str("name"))).collect())),
        ])
    }

    /// 🚨️ A failure message that names WHAT disagreed — trimmed, because the real drawing carries 1 006
    /// nodes and an embedded image.
    fn disagreement(what: &str, got: &SemioDrawingSnapshot, expected: &SemioDrawingSnapshot) -> String {
        let short = |drawing: &SemioDrawingSnapshot| {
            let layers = drawing.layers.iter().map(|layer| format!("{}({})", layer.id, layer.visible)).collect::<Vec<_>>().join(",");
            let styles = drawing.styles.iter().map(|style| style.name.clone()).collect::<Vec<_>>().join(",");
            format!("canvas={}x{} layers=[{layers}] styles=[{styles}] digest={}", drawing.canvas.width, drawing.canvas.height, digest(encode_semio_drawing_snapshot_json(drawing).as_bytes()))
        };
        format!("{what}\n     got: {}\nexpected: {}", short(got), short(expected))
    }
    //#endregion 🔖️Projection

    //#region 🔖️Handlers
    /// 🎯️ One verb applied to the real derived drawing by this repository's codec alone.
    pub fn mutate(ctx: &Context) -> Result<Outcome, String> {
        let mut current = artifact(ctx)?;
        apply(&mut current, &payload(ctx)?, &ctx.scenario.id)?;
        let document = snapshot_json(&current)?;
        let shape = shape_json(&document);
        Ok(Outcome::projection(Json::Object(vec![("document".to_string(), document), ("shape".to_string(), shape)])))
    }

    /// ↩️ The metamorphic inverse law on the real drawing: applying the verb and then its OWN computed
    /// inverse must restore it exactly — scene-graph ORDER, nesting depth and every transform
    /// included, which is what the four hierarchy verbs make load-bearing.
    pub fn inverse(ctx: &Context) -> Result<Outcome, String> {
        let base = artifact(ctx)?;
        let step = payload(ctx)?;
        let mut current = base.clone();
        apply(&mut current, &step, &ctx.scenario.id)?;
        let mutated = snapshot_json(&current)?;
        for undo in &inverse_semio_drawing_mutation(&step, &base) {
            apply(&mut current, undo, &ctx.scenario.id)?;
        }
        if current != base {
            return Err(disagreement(&format!("{}: undoing the mutation did not restore the drawing", ctx.scenario.id), &current, &base));
        }
        Ok(Outcome::projection(Json::Object(vec![("mutated".to_string(), mutated), ("restored".to_string(), snapshot_json(&current)?)])))
    }

    /// 🧫️ The same verb on its committed handcrafted `(before, mutation, after)` vector — a THIRD
    /// statement of what the verb means, independent of both implementations.
    pub fn spec_vector(ctx: &Context) -> Result<Outcome, String> {
        let mut current = decode_semio_drawing_snapshot_json(&vector(ctx, 0, "before-snapshot")?)?;
        let step = decode_semio_drawing_mutation_json(&vector(ctx, 1, "mutation")?)?;
        let expected = decode_semio_drawing_snapshot_json(&vector(ctx, 2, "after-snapshot")?)?;
        apply(&mut current, &step, &ctx.scenario.id)?;
        if current != expected {
            return Err(disagreement(&format!("{}: the applied drawing does not match the committed after-snapshot", ctx.scenario.id), &current, &expected));
        }
        Ok(Outcome::projection(snapshot_json(&current)?))
    }

    /// 🔁️ Both committed encodings of the real derived drawing, each re-emitted from the parsed
    /// document.
    ///
    /// 🔒️ **The byte half of the identity law — asserted, and asserted as `carrier_is_exact`.**
    /// `.dsl.semio` is a fixed-layout record grammar and `.pack.semio` is its binary twin; the two
    /// committed artifacts this scenario reads were produced by the INDEPENDENT Python implementation
    /// from the same grammar, so reproducing them BYTE FOR BYTE is the correct answer here and
    /// `law::reparsed_not_copied` would be exactly backwards — the same reading `mutate-dag-1`
    /// records for `.dag.dsl.semio`. Nor is it a self-comparison: the bytes this side must match were
    /// written by the other implementation, and the digests of what each side emitted are what the
    /// runner compares.
    pub fn identity_round_trip(ctx: &Context) -> Result<Outcome, String> {
        let dsl_bytes = ctx.fixture_bytes(ARTIFACT_DSL)?;
        let text = String::from_utf8(dsl_bytes.clone()).map_err(|error| format!("identity-round-trip: the derived drawing artifact is not UTF-8: {error}"))?;
        let parsed = parse_semio_drawing_dsl(&text)?;
        let printed = print_semio_drawing_dsl(&parsed);
        carrier_is_exact(printed.as_bytes(), &dsl_bytes)?;
        let reparsed = parse_semio_drawing_dsl(&printed)?;
        if reparsed != parsed {
            return Err(disagreement("identity-round-trip: printing the snapshot back to DSL and reparsing it lost content", &reparsed, &parsed));
        }
        let pack_bytes = ctx.fixture_bytes(ARTIFACT_PACK)?;
        let unpacked = decode_semio_drawing_pack(&pack_bytes)?;
        if unpacked != parsed {
            return Err(disagreement("identity-round-trip: the committed binary twin decodes to a different drawing than the committed text artifact", &unpacked, &parsed));
        }
        let repacked_bytes = encode_semio_drawing_pack(&parsed);
        carrier_is_exact(&repacked_bytes, &pack_bytes)?;
        let repacked = decode_semio_drawing_pack(&repacked_bytes)?;
        if repacked != parsed {
            return Err(disagreement("identity-round-trip: encoding the snapshot to a pack and decoding it back lost content", &repacked, &parsed));
        }
        let document = snapshot_json(&parsed)?;
        let shape = shape_json(&document);
        Ok(Outcome::projection(Json::Object(vec![
            ("document".to_string(), document),
            ("shape".to_string(), shape),
            ("dslDigest".to_string(), Json::String(digest(printed.as_bytes()))),
            ("packDigest".to_string(), Json::String(digest(&repacked_bytes))),
            ("dslLength".to_string(), number(printed.len())),
            ("packLength".to_string(), number(repacked_bytes.len())),
        ])))
    }
    //#endregion 🔖️Handlers
}
//#endregion 🔖️Subject

//#region 🔖️Registration
/// 🧭️ Registration entry point the generated host calls. Registration is by FULL expanded scenario
/// id, so the loop mirrors the feature's `Examples` tables exactly. Only subject handlers are
/// registered: the oracle role belongs to `🐍️component.py`.
pub fn adapter() -> Adapter {
    #[allow(unused_mut)]
    let mut built = Adapter::new("rust");
    #[cfg(feature = "sut")]
    {
        for kind in KINDS {
            built = built
                .subject(&format!("mutate-{kind}"), subject::mutate)
                .subject(&format!("inverse-{kind}"), subject::inverse)
                .subject(&format!("spec-vector-{kind}"), subject::spec_vector);
        }
        built = built.subject("identity-round-trip", subject::identity_round_trip);
    }
    built
}
//#endregion 🔖️Registration
