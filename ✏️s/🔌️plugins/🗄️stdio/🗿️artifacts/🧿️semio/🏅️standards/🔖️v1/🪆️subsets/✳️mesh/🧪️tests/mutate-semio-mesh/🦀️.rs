//! 🦀️ Semio MESH exhaustive mutation case — Rust SUBJECT adapter. Ticket 26/08/23/END-TO-END-
//! TESTING-REFACTOR.
//!
//! **This file no longer serves the oracle role.** The reference for `semio-v1-mesh-mutate` is the
//! registered oracle `semio-mesh-typescript-three-independent` (`../../🏅️standards/🔖️v1/🪆️subsets/
//! ✳️mesh/🔣️oracle.json`) — three.js for the geometry it genuinely speaks, plus an
//! independent TypeScript implementation of the semio-native carrier and the seventeen verbs, living
//! beside this file as `🟦️.ts`. The runner dispatches the oracle role there and the subject
//! role here, and compares the two projections under `@comparison-ordered-json-v1`. Registering
//! oracle handlers here as well would put this repository's own answer on both sides of that
//! comparison, which is the precise failure the platform exists to prevent.
//!
//! **Where three.js bites.** Every scenario that produces primitives projects a `geometry` report
//! beside the document: per primitive, its attribute counts, the bounding box of its position buffer
//! and — when its index buffer addresses that buffer — the flat vertex stream the index expands to.
//! The oracle side obtains those from a real `THREE.BufferGeometry`; this side computes them here,
//! by hand, from the same primitives. A permuted index buffer or a lost vertex therefore shows up as
//! a difference in the expanded stream rather than passing quietly.
//!
//! **What the handlers assert in role.** Parity across the two implementations is the primary
//! evidence, but each side still states its own law so a scenario can fail for the right reason with
//! a readable message: `inverse-<kind>` requires the mutation's OWN computed inverse to restore the
//! model, `spec-vector-<kind>` requires the applied snapshot to be the committed after-snapshot, and
//! `identity-round-trip` requires both committed encodings to be reproduced byte for byte through
//! `law::carrier_is_exact`.
//!
//! **How the fixture reaches typed values.** The generated test host links only
//! `semio-repo-test-host` and, behind `sut`, this subset's own crate — no `serde`, no `serde_json` —
//! so every fixture is decoded through the bridges this subset's own production code exports
//! (`decode_semio_mesh_snapshot_json`/`encode_semio_mesh_snapshot_json` and
//! `decode_semio_mesh_mutation_json`/`inverse_semio_mesh_mutation`), whose signatures name only
//! reachable types. Every input is read from a fixture the FEATURE declares, so neither adapter holds
//! a transcription that could drift away from what the other one read.

use semio_repo_test_host::Adapter;

//#region 🔖️Kinds
/// 🏷️ Mirrors `SemioMeshMutation::KINDS` (`../../🏅️standards/🔖️v1/🪆️subsets/✳️mesh/🧬️schema/
/// 🧬️mutations/🦀️.rs`) — duplicated, not imported, because the generated host builds this
/// file with and without the subject crate. The contract's mutation-coverage gate keeps this list
/// honest against the catalog; `kinds_match_the_enum_and_the_catalog` in that production file keeps
/// it honest against the enum.
#[cfg(feature = "sut")]
const KINDS: &[&str] = &[
    "create-mesh",
    "delete-mesh",
    "create-primitive",
    "delete-primitive",
    "set-primitive-topology",
    "replace-primitive-geometry",
    "set-primitive-material",
    "create-material",
    "delete-material",
    "change-material-base-color",
    "change-material-metallic",
    "change-material-roughness",
    "create-texture",
    "delete-texture",
    "change-texture-mime",
    "replace-texture-bytes",
    "move-vertex",
];
//#endregion 🔖️Kinds

//#region 🔖️Subject
#[cfg(feature = "sut")]
mod subject {
    use semio_repo_test_host::{digest, parse_json, Context, Json, Outcome};
    use semio_s_plugin_stdio_test_oracle::law::carrier_is_exact;
    use semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::base::schema::mutations::semio_mutation_refusals;
    use semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::mesh::schema::mutations::{apply_semio_mesh_mutation, decode_semio_mesh_mutation_json, inverse_semio_mesh_mutation, SemioMeshMutation};
    use semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::mesh::schema::snapshot::{decode_mesh_pack, decode_semio_mesh_snapshot_json, encode_mesh_pack, encode_semio_mesh_snapshot_json, parse_mesh_dsl, print_mesh_dsl, SemioMeshSnapshot};

    //#region 🔖️Input
    /// 🔺️ The real derived model — 271 meshes and 459 primitives read once out of the committed
    /// Metabolism `🧊️base.glb` and written out through the independent TypeScript implementation.
    const ARTIFACT_DSL: &str = "local://🗣️.dsl.semio";
    /// 🎒️ The same model in its binary envelope, written by a separate codec from the DSL text.
    const ARTIFACT_PACK: &str = "local://🎒️.pack.semio";

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

    /// 🔺️ The real derived model, parsed through this repository's own DSL codec.
    fn artifact(ctx: &Context) -> Result<SemioMeshSnapshot, String> {
        let text = String::from_utf8(ctx.fixture_bytes(ARTIFACT_DSL)?).map_err(|error| format!("the derived model artifact is not UTF-8: {error}"))?;
        parse_mesh_dsl(&text).map_err(|error| error.to_string())
    }

    /// 📜️ The scenario's own committed mutation payload — the feature owns the vector.
    fn payload(ctx: &Context) -> Result<SemioMeshMutation, String> {
        let uri = step_uris(ctx, "local://🦠️").into_iter().next().ok_or_else(|| format!("{}: the scenario names no mutation payload", ctx.scenario.id))?;
        let text = String::from_utf8(ctx.fixture_bytes(&uri)?).map_err(|error| format!("{uri} is not UTF-8: {error}"))?;
        decode_semio_mesh_mutation_json(&text).map_err(|error| format!("{}: the mutation payload must decode: {error}", ctx.scenario.id))
    }

    /// 🧫️ One committed specification-vector file, read as text and decoded as a snapshot.
    fn vector(ctx: &Context, position: usize, label: &str) -> Result<String, String> {
        let uri = step_uris(ctx, "asset://").into_iter().nth(position).ok_or_else(|| format!("{}: the scenario names no {label} asset", ctx.scenario.id))?;
        String::from_utf8(ctx.fixture_bytes(&uri)?).map_err(|error| format!("{uri} is not UTF-8: {error}"))
    }

    fn apply(current: &mut SemioMeshSnapshot, step: &SemioMeshMutation, what: &str) -> Result<(), String> {
        let outcome = apply_semio_mesh_mutation(current, step);
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
    fn snapshot_json(snapshot: &SemioMeshSnapshot) -> Result<Json, String> {
        parse_json(&encode_semio_mesh_snapshot_json(snapshot))
    }

    /// 🔺️ The three facts the oracle obtains from a real `THREE.BufferGeometry`, computed here by
    /// hand from the same primitives: the attribute counts, the bounding box of the position buffer,
    /// and the flat vertex stream the index buffer expands to. An index that points outside the
    /// position buffer cannot be expanded, which is reported rather than silently dropped. It reads
    /// the snapshot's own JSON wire form rather than the typed value, so the topology spelling is
    /// the wire form's and this adapter never re-declares the enumeration.
    fn geometry_json(document: &Json) -> Json {
        let axis_of = |point: &Json, key: &str| match point.get(key) {
            Some(Json::Number(value)) => *value,
            _ => f64::NAN,
        };
        let mut primitives = Vec::new();
        for mesh in document.array("meshes") {
            for primitive in mesh.array("primitives") {
                let positions = primitive.array("positions");
                let indices: Vec<usize> = primitive
                    .array("indices")
                    .iter()
                    .map(|entry| match entry {
                        Json::Number(value) => *value as usize,
                        _ => usize::MAX,
                    })
                    .collect();
                let addressable = !indices.is_empty() && indices.iter().all(|index| *index < positions.len());
                let bounding = match positions.first() {
                    None => Json::Null,
                    Some(first) => {
                        let mut low = [axis_of(first, "x"), axis_of(first, "y"), axis_of(first, "z")];
                        let mut high = low;
                        for point in &positions {
                            for (at, key) in ["x", "y", "z"].into_iter().enumerate() {
                                let value = axis_of(point, key);
                                low[at] = low[at].min(value);
                                high[at] = high[at].max(value);
                            }
                        }
                        Json::Object(vec![
                            ("min".to_string(), Json::Array(low.iter().map(|value| Json::Number(*value)).collect())),
                            ("max".to_string(), Json::Array(high.iter().map(|value| Json::Number(*value)).collect())),
                        ])
                    }
                };
                let expanded = match addressable {
                    false => Json::Null,
                    true => Json::Array(
                        indices
                            .iter()
                            .flat_map(|index| {
                                let point = &positions[*index];
                                [Json::Number(axis_of(point, "x")), Json::Number(axis_of(point, "y")), Json::Number(axis_of(point, "z"))]
                            })
                            .collect(),
                    ),
                };
                primitives.push(Json::Object(vec![
                    ("meshId".to_string(), Json::String(mesh.str("id"))),
                    ("primitiveId".to_string(), Json::String(primitive.str("id"))),
                    ("topology".to_string(), Json::String(primitive.str("topology"))),
                    (
                        "counts".to_string(),
                        Json::Object(vec![
                            ("positions".to_string(), number(positions.len())),
                            ("normals".to_string(), number(primitive.array("normals").len())),
                            ("uvs".to_string(), number(primitive.array("uvs").len())),
                            ("colors".to_string(), number(primitive.array("colors").len())),
                            ("indices".to_string(), number(indices.len())),
                        ]),
                    ),
                    ("addressable".to_string(), Json::Bool(addressable)),
                    ("boundingBox".to_string(), bounding),
                    ("nonIndexedPositions".to_string(), expanded),
                ]));
            }
        }
        Json::Object(vec![("library".to_string(), Json::String("three.js".to_string())), ("primitives".to_string(), Json::Array(primitives))])
    }

    /// 🚨️ A failure message that names WHAT disagreed — trimmed, because the real model carries 271
    /// meshes and an embedded texture.
    fn disagreement(what: &str, got: &SemioMeshSnapshot, expected: &SemioMeshSnapshot) -> String {
        let short = |model: &SemioMeshSnapshot| {
            let meshes = model.meshes.iter().map(|mesh| format!("{}({})", mesh.id, mesh.primitives.len())).collect::<Vec<_>>().join(",");
            let materials = model.materials.iter().map(|material| material.id.clone()).collect::<Vec<_>>().join(",");
            let textures = model.textures.iter().map(|texture| format!("{}:{}:{}", texture.id, texture.mime, texture.bytes.len())).collect::<Vec<_>>().join(",");
            format!("meshes={} [{}] materials=[{}] textures=[{}] digest={}", model.meshes.len(), meshes.chars().take(300).collect::<String>(), materials, textures, digest(encode_semio_mesh_snapshot_json(model).as_bytes()))
        };
        format!("{what}\n     got: {}\nexpected: {}", short(got), short(expected))
    }
    //#endregion 🔖️Projection

    //#region 🔖️Handlers
    /// 🎯️ One verb applied to the real derived model by this repository's codec alone.
    pub fn mutate(ctx: &Context) -> Result<Outcome, String> {
        let mut current = artifact(ctx)?;
        apply(&mut current, &payload(ctx)?, &ctx.scenario.id)?;
        let document = snapshot_json(&current)?;
        let geometry = geometry_json(&document);
        Ok(Outcome::projection(Json::Object(vec![("document".to_string(), document), ("geometry".to_string(), geometry)])))
    }

    /// ↩️ The metamorphic inverse law on the real model: applying the verb and then its OWN computed
    /// inverse must restore it exactly — pool ORDER, the parallel attribute arrays and the texture
    /// payload included.
    pub fn inverse(ctx: &Context) -> Result<Outcome, String> {
        let base = artifact(ctx)?;
        let step = payload(ctx)?;
        let mut current = base.clone();
        apply(&mut current, &step, &ctx.scenario.id)?;
        let mutated = snapshot_json(&current)?;
        for undo in &inverse_semio_mesh_mutation(&step, &base) {
            apply(&mut current, undo, &ctx.scenario.id)?;
        }
        if current != base {
            return Err(disagreement(&format!("{}: undoing the mutation did not restore the model", ctx.scenario.id), &current, &base));
        }
        Ok(Outcome::projection(Json::Object(vec![("mutated".to_string(), mutated), ("restored".to_string(), snapshot_json(&current)?)])))
    }

    /// 🧫️ The same verb on its committed handcrafted `(before, mutation, after)` vector — a THIRD
    /// statement of what the verb means, independent of both implementations.
    pub fn spec_vector(ctx: &Context) -> Result<Outcome, String> {
        let mut current = decode_semio_mesh_snapshot_json(&vector(ctx, 0, "before-snapshot")?)?;
        let step = decode_semio_mesh_mutation_json(&vector(ctx, 1, "mutation")?)?;
        let expected = decode_semio_mesh_snapshot_json(&vector(ctx, 2, "after-snapshot")?)?;
        apply(&mut current, &step, &ctx.scenario.id)?;
        if current != expected {
            return Err(disagreement(&format!("{}: the applied snapshot does not match the committed after-snapshot", ctx.scenario.id), &current, &expected));
        }
        Ok(Outcome::projection(snapshot_json(&current)?))
    }

    /// 🔁️ Both committed encodings of the real derived model, each re-emitted from the parsed
    /// document.
    ///
    /// 🔒️ **The byte half of the identity law — asserted, and asserted as `carrier_is_exact`.**
    /// `.dsl.semio` is a fixed-layout record grammar and `.pack.semio` is its binary twin; the two
    /// committed artifacts this scenario reads were produced by the INDEPENDENT TypeScript
    /// implementation from the same grammar, so reproducing them BYTE FOR BYTE is the correct answer
    /// here and `law::reparsed_not_copied` would be exactly backwards — the same reading
    /// `mutate-dag-1` records for `.dag.dsl.semio`. Nor is it a self-comparison: the bytes this side
    /// must match were written by the other implementation, and the digests of what each side
    /// emitted are what the runner compares.
    pub fn identity_round_trip(ctx: &Context) -> Result<Outcome, String> {
        let dsl_bytes = ctx.fixture_bytes(ARTIFACT_DSL)?;
        let text = String::from_utf8(dsl_bytes.clone()).map_err(|error| format!("identity-round-trip: the derived model artifact is not UTF-8: {error}"))?;
        let parsed = parse_mesh_dsl(&text).map_err(|error| error.to_string())?;
        let printed = print_mesh_dsl(&parsed);
        carrier_is_exact(printed.as_bytes(), &dsl_bytes)?;
        let reparsed = parse_mesh_dsl(&printed).map_err(|error| error.to_string())?;
        if reparsed != parsed {
            return Err(disagreement("identity-round-trip: printing the snapshot back to DSL and reparsing it lost content", &reparsed, &parsed));
        }
        let pack_bytes = ctx.fixture_bytes(ARTIFACT_PACK)?;
        let unpacked = decode_mesh_pack(&pack_bytes).map_err(|error| error.to_string())?;
        if unpacked != parsed {
            return Err(disagreement("identity-round-trip: the committed binary twin decodes to a different model than the committed text artifact", &unpacked, &parsed));
        }
        let repacked_bytes = encode_mesh_pack(&parsed);
        carrier_is_exact(&repacked_bytes, &pack_bytes)?;
        let repacked = decode_mesh_pack(&repacked_bytes).map_err(|error| error.to_string())?;
        if repacked != parsed {
            return Err(disagreement("identity-round-trip: encoding the snapshot to a pack and decoding it back lost content", &repacked, &parsed));
        }
        let document = snapshot_json(&parsed)?;
        let geometry = geometry_json(&document);
        Ok(Outcome::projection(Json::Object(vec![
            ("document".to_string(), document),
            ("geometry".to_string(), geometry),
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
/// registered: the oracle role belongs to `🟦️.ts`.
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
