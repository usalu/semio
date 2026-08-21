//! 🧬️ StlMutation — document mutation dispatch. Ticket
//! 26/08/10/ARTIFACT-SYSTEM-OVERHAUL-REAL-CODECS-RUNTIME-REUSE-EVOLUTION: real vocabulary beyond
//! the universal `{NoMutation, SetSnapshot}` stub — `SetSolidName` plus
//! `InsertTriangle`/`RemoveTriangle`/`SetTriangleNormal`/`SetTriangleVertices` for the index-keyed
//! `triangles` collection. Every variant's `diff()` is handcrafted (constructs `StlDiff` directly
//! via the `schema::diff` builders) — apply-and-capture is never used.
//!
//! 🧪️ F6 (OpText/OpBinary + DiffCodec wave): **HAND-ROLL path** — every variant's payload closure
//! (incl. `SetSnapshot`'s whole `StlSnapshot`) has zero data-carrying enums (§3a of
//! `f6-recon-report.md`'s decision rule), and `#[derive(dsl::DslOps)]` DID compile cleanly on a
//! first attempt, exactly like `GifMutation`'s pilot. It was reverted for the SAME reason
//! `StlDiff`'s derive was (see `🔺️diff::component`'s top doc comment and `StlTriangle`'s doc
//! comment in `📸️snapshot::component`): a real, reproduced `dsl`-grammar bug where nested
//! `Shape::Tuple` levels (`vertices: [[f64; 3]; 3]`, reachable via `SetSnapshot`, `InsertTriangle`,
//! `SetTriangleVertices`) print flat and cannot be re-parsed. `OpText`/`OpBinary` below are
//! hand-rolled instead, reusing `🔺️diff::component`'s `pub(crate)` grammar primitives
//! (`enc_vec3`/`enc_vertices`/`enc_triangle`/`hex_encode_str`/`split_top_level`/`strip_brackets`) —
//! same intra-artifact reuse pattern `svg`'s `SvgMutation` uses over `SvgDiff`'s primitives.

use crate::artifacts::stl::schema::diff::{self, StlDiff};
use crate::artifacts::stl::schema::snapshot::StlTriangle;
use crate::artifacts::stl::StlSnapshot;
use protocol::Mutation;
use protocol::{OpBinary, OpText};
use serde::{Deserialize, Serialize};

//#region 🔖️Mutations
/// 📐️ Typed content mutation for `stdio.stl`.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(tag = "mutation", rename_all = "camelCase")]
pub enum StlMutation {
    #[default]
    NoMutation,
    SetSnapshot {
        snapshot: StlSnapshot,
    },
    /// 🏷️ Sets the `solid`/`endsolid` header/trailer name.
    SetSolidName {
        name: String,
    },
    /// ➕️ Inserts a fully-specified triangle at `index` (final position, clamped to `len`).
    InsertTriangle {
        index: usize,
        triangle: StlTriangle,
    },
    /// ➖️ Removes the triangle at `index` (no-op if out of range).
    RemoveTriangle {
        index: usize,
    },
    /// 🧭️ Replaces one triangle's facet normal.
    SetTriangleNormal {
        index: usize,
        normal: [f64; 3],
    },
    /// 📐️ Replaces one triangle's 3 vertices (whole-value replace).
    SetTriangleVertices {
        index: usize,
        vertices: [[f64; 3]; 3],
    },
}
//#endregion 🔖️Mutations

//#region 🔖️Apply
/// ▶️ Applies `mutation` to `snapshot`, returning a typed error outcome without changing the
/// snapshot when an index target is missing or out of range.
pub fn apply_stl_mutation(snapshot: &mut StlSnapshot, mutation: &StlMutation) -> protocol::MutationOutcome<StlDiff> {
    let outcome = <StlMutation as Mutation<StlSnapshot>>::diff(mutation, snapshot);
    match protocol::MutationDiff::apply(outcome.diff(), snapshot) {
        Ok(next) => {
            *snapshot = next;
            outcome
        }
        Err(error) => protocol::MutationOutcome::error(error.code, error.message, error.target).absorb_messages(outcome.messages().to_vec()),
    }
}
//#endregion 🔖️Apply

//#region 🔖️MutationTrait
impl Mutation<StlSnapshot> for StlMutation {
    type Diff = StlDiff;

    fn diff(&self, base: &StlSnapshot) -> protocol::MutationOutcome<Self::Diff> {
        protocol::MutationOutcome::new(match self {
            StlMutation::NoMutation => StlDiff::default(),
            StlMutation::SetSnapshot { snapshot } => diff::diff_set_snapshot(base, snapshot),
            StlMutation::SetSolidName { name } => diff::diff_set_solid_name(name),
            StlMutation::InsertTriangle { index, triangle } => diff::diff_insert_triangle(*index, *triangle),
            StlMutation::RemoveTriangle { index } => diff::diff_remove_triangle(*index),
            StlMutation::SetTriangleNormal { index, normal } => diff::diff_set_triangle_normal(*index, *normal),
            StlMutation::SetTriangleVertices { index, vertices } => diff::diff_set_triangle_vertices(*index, *vertices),
        })
    }

    /// ↩️ Handcrafted, index-aware mutation-level inverses. Index-targeted variants look the
    /// prior value up in `base`; a stale/out-of-range index inverts to `NoMutation` (nothing to
    /// undo).
    fn inverse(&self, base: &StlSnapshot) -> Vec<Self> {
        match self {
            StlMutation::NoMutation => vec![StlMutation::NoMutation],
            StlMutation::SetSnapshot { .. } => vec![StlMutation::SetSnapshot { snapshot: base.clone() }],
            StlMutation::SetSolidName { .. } => vec![StlMutation::SetSolidName { name: base.solid_name.clone() }],
            StlMutation::InsertTriangle { index, .. } => {
                vec![StlMutation::RemoveTriangle { index: (*index).min(base.triangles.len()) }]
            }
            StlMutation::RemoveTriangle { index } => match base.triangles.get(*index) {
                Some(t) => vec![StlMutation::InsertTriangle { index: *index, triangle: *t }],
                None => vec![StlMutation::NoMutation],
            },
            StlMutation::SetTriangleNormal { index, .. } => match base.triangles.get(*index) {
                Some(t) => vec![StlMutation::SetTriangleNormal { index: *index, normal: t.normal }],
                None => vec![StlMutation::NoMutation],
            },
            StlMutation::SetTriangleVertices { index, .. } => match base.triangles.get(*index) {
                Some(t) => vec![StlMutation::SetTriangleVertices { index: *index, vertices: t.vertices }],
                None => vec![StlMutation::NoMutation],
            },
        }
    }
}
//#endregion 🔖️MutationTrait

//#region OpCodecs
/// 🧪️ F6: hand-rolled `OpText`/`OpBinary` grammar — see this file's top doc comment for why (the
/// same real, reproduced `dsl`-derive bug that forced `StlDiff`'s hand-roll also reaches here via
/// `SetSnapshot`/`InsertTriangle`/`SetTriangleVertices`'s `[[f64; 3]; 3]` payload).
///
/// **Grammar**: `<keyword> arg=value ...` — one space-separated `key=value` token per argument
/// (every variant's args are ALWAYS present, unlike `StlDiff`'s sparse tokens). `index`/floats
/// print via `Display`; `name`/`solid_name` are lowercase hex; `normal`/`vertices`/`triangle`/
/// `snapshot` reuse `🔺️diff::component`'s `pub(crate)` value codecs verbatim (`enc_vec3`,
/// `enc_vertices`, `enc_triangle`) plus this file's own `enc_snapshot` (the one type `🔺️diff`
/// doesn't need — only `SetSnapshot`'s payload does).
async fn enc_snapshot(s: &StlSnapshot) -> String {
    format!("[{},{},[{}]]", diff::hex_encode_str(&s.schema), diff::hex_encode_str(&s.solid_name), s.triangles.iter().map(diff::enc_triangle).collect::<Vec<_>>().join(","),)
}
async fn dec_snapshot(s: &str) -> Result<StlSnapshot, String> {
    let parts = diff::split_top_level(diff::strip_brackets(s)?, ',');
    let [schema, solid_name, triangles] = parts.as_slice() else {
        return Err(format!("snapshot: expected 3 fields, got {}", parts.len()));
    };
    let triangles = diff::split_top_level(diff::strip_brackets(triangles)?, ',').into_iter().filter(|s| !s.is_empty()).map(diff::dec_triangle).collect::<Result<Vec<_>, String>>()?;
    Ok(StlSnapshot { schema: diff::hex_decode_str(schema)?, solid_name: diff::hex_decode_str(solid_name)?, triangles })
}

fn print_stl_op(m: &StlMutation) -> String {
    match m {
        StlMutation::NoMutation => "no-mutation".to_string(),
        StlMutation::SetSnapshot { snapshot } => format!("set-snapshot snapshot={}", enc_snapshot(snapshot)),
        StlMutation::SetSolidName { name } => format!("set-solid-name name={}", diff::hex_encode_str(name)),
        StlMutation::InsertTriangle { index, triangle } => format!("insert-triangle index={index} triangle={}", diff::enc_triangle(triangle)),
        StlMutation::RemoveTriangle { index } => format!("remove-triangle index={index}"),
        StlMutation::SetTriangleNormal { index, normal } => format!("set-triangle-normal index={index} normal={}", diff::enc_vec3(normal)),
        StlMutation::SetTriangleVertices { index, vertices } => format!("set-triangle-vertices index={index} vertices={}", diff::enc_vertices(vertices)),
    }
}
async fn parse_stl_op(line: &str) -> Result<StlMutation, String> {
    if line == "no-mutation" {
        return Ok(StlMutation::NoMutation);
    }
    let mut tokens = line.split(' ');
    let keyword = tokens.next().ok_or_else(|| "stl op: empty line".to_string())?;
    let args: Vec<&str> = tokens.collect();
    let get = |key: &str| -> Result<&str, String> {
        let probe = format!("{key}=");
        args.iter().find_map(|t| t.strip_prefix(probe.as_str())).ok_or_else(|| format!("stl op: missing '{key}=' in {line:?}"))
    };
    match keyword {
        "set-snapshot" => Ok(StlMutation::SetSnapshot { snapshot: dec_snapshot(get("snapshot")?).await? }),
        "set-solid-name" => Ok(StlMutation::SetSolidName { name: diff::hex_decode_str(get("name")?)? }),
        "insert-triangle" => Ok(StlMutation::InsertTriangle { index: diff::parse_usize(get("index")?)?, triangle: diff::dec_triangle(get("triangle")?)? }),
        "remove-triangle" => Ok(StlMutation::RemoveTriangle { index: diff::parse_usize(get("index")?)? }),
        "set-triangle-normal" => Ok(StlMutation::SetTriangleNormal { index: diff::parse_usize(get("index")?)?, normal: diff::dec_vec3(get("normal")?)? }),
        "set-triangle-vertices" => Ok(StlMutation::SetTriangleVertices { index: diff::parse_usize(get("index")?)?, vertices: diff::dec_vertices(get("vertices")?)? }),
        other => Err(format!("stl op: unknown keyword {other:?}")),
    }
}

impl OpText for StlMutation {
    fn print_op(&self) -> String {
        print_stl_op(self)
    }
    fn parse_op(line: &str) -> Result<Self, store::TextError> {
        parse_stl_op(line).map_err(|e| store::TextError::new(e, dsl::TextSpan::at(1, 1)))
    }
}

//#region 🔖️OpBinaryCodec
/// 🧪️ P2-FG1-FIX: real recursive binary twin of [`enc_snapshot`]/[`dec_snapshot`] above —
/// `StlSnapshot` is genuinely flat (`Vec<StlTriangle>`, no self-recursion), so this is real
/// varint-framed binary all the way down, reusing `diff`'s `pub(crate)` binary value codecs
/// (`enc_triangle_bin`/`dec_triangle_bin`) rather than duplicating them — same intra-artifact
/// reuse pattern this file's own text `enc_snapshot` already establishes over `diff::enc_triangle`.
async fn enc_snapshot_bin(s: &StlSnapshot, out: &mut Vec<u8>) {
    diff::write_str_bin(out, &s.schema);
    diff::write_str_bin(out, &s.solid_name);
    store::pack_rt::write_varint_u64(out, s.triangles.len() as u64);
    for t in &s.triangles {
        diff::enc_triangle_bin(t, out);
    }
}
fn dec_snapshot_bin(reader: &mut store::ByteReader<'_>) -> Result<StlSnapshot, String> {
    let schema = diff::read_str_bin(reader)?;
    let solid_name = diff::read_str_bin(reader)?;
    let count = reader.read_varint_u64().map_err(|e| e.to_string())?;
    let mut triangles = Vec::with_capacity(count as usize);
    for _ in 0..count {
        triangles.push(diff::dec_triangle_bin(reader)?);
    }
    Ok(StlSnapshot { schema, solid_name, triangles })
}

/// 🧪️ P2-FG1-FIX: REAL binary op frame (`format u8 | tag u8 | variant payload`), matching
/// `../💾️binary/📡️component.protocol.semio`'s `header fixed 2` + `chain payload bytes` shape —
/// upgraded from the prior `print_stl_op(self).into_bytes()` text-as-binary shortcut. `tag` is
/// the `StlMutation` variant's declaration-order ordinal (0=`NoMutation` .. 6=
/// `SetTriangleVertices`, same order `enum StlMutation` declares them). Every variant's payload is
/// real field-by-field binary (`write_varint_u64` for `index: usize`, `write_f64_bin`/
/// `enc_vec3_bin`/`enc_vertices_bin`/`enc_triangle_bin`/`enc_snapshot_bin` for the rest) — `
/// StlMutation`'s payload tree has ZERO self-recursion, so nothing here is opaque at the Rust
/// layer; only the protocol-dialect file still frames the payload as one opaque trailing chain
/// (`SetSnapshot`'s `Vec<StlTriangle>` is a variable-length vector-of-records, the same
/// `protocol-array-of-records` `walk_protocol` gap the sibling diff protocol file documents).
impl OpBinary for StlMutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        let mut out = vec![store::pack_rt::OP_BINARY_FORMAT, 0u8];
        let tag: u8 = match self {
            StlMutation::NoMutation => 0,
            StlMutation::SetSnapshot { snapshot } => {
                enc_snapshot_bin(snapshot, &mut out);
                1
            }
            StlMutation::SetSolidName { name } => {
                diff::write_str_bin(&mut out, name);
                2
            }
            StlMutation::InsertTriangle { index, triangle } => {
                store::pack_rt::write_varint_u64(&mut out, *index as u64);
                diff::enc_triangle_bin(triangle, &mut out);
                3
            }
            StlMutation::RemoveTriangle { index } => {
                store::pack_rt::write_varint_u64(&mut out, *index as u64);
                4
            }
            StlMutation::SetTriangleNormal { index, normal } => {
                store::pack_rt::write_varint_u64(&mut out, *index as u64);
                diff::enc_vec3_bin(normal, &mut out);
                5
            }
            StlMutation::SetTriangleVertices { index, vertices } => {
                store::pack_rt::write_varint_u64(&mut out, *index as u64);
                diff::enc_vertices_bin(vertices, &mut out);
                6
            }
        };
        out[1] = tag;
        Ok(out)
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        let mut reader = store::ByteReader::new(bytes);
        let _format = reader.read_u8().map_err(|e| protocol::ProtocolError::Malformed { what: "op format", offset: 0, detail: e.to_string() })?;
        let tag = reader.read_u8().map_err(|e| protocol::ProtocolError::Malformed { what: "op tag", offset: 1, detail: e.to_string() })?;
        match tag {
            0 => Ok(StlMutation::NoMutation),
            1 => {
                let snapshot = dec_snapshot_bin(&mut reader).map_err(|e| protocol::ProtocolError::Malformed { what: "op snapshot", offset: semio_framework_plugin::resolve_ready(reader.position()) as u64, detail: e })?;
                Ok(StlMutation::SetSnapshot { snapshot })
            }
            2 => {
                let name = diff::read_str_bin(&mut reader).map_err(|e| protocol::ProtocolError::Malformed { what: "op name", offset: semio_framework_plugin::resolve_ready(reader.position()) as u64, detail: e })?;
                Ok(StlMutation::SetSolidName { name })
            }
            3 => {
                let index = reader.read_varint_u64().map_err(|e| protocol::ProtocolError::Malformed { what: "op index", offset: semio_framework_plugin::resolve_ready(reader.position()) as u64, detail: e.to_string() })? as usize;
                let triangle = diff::dec_triangle_bin(&mut reader).map_err(|e| protocol::ProtocolError::Malformed { what: "op triangle", offset: semio_framework_plugin::resolve_ready(reader.position()) as u64, detail: e })?;
                Ok(StlMutation::InsertTriangle { index, triangle })
            }
            4 => {
                let index = reader.read_varint_u64().map_err(|e| protocol::ProtocolError::Malformed { what: "op index", offset: semio_framework_plugin::resolve_ready(reader.position()) as u64, detail: e.to_string() })? as usize;
                Ok(StlMutation::RemoveTriangle { index })
            }
            5 => {
                let index = reader.read_varint_u64().map_err(|e| protocol::ProtocolError::Malformed { what: "op index", offset: semio_framework_plugin::resolve_ready(reader.position()) as u64, detail: e.to_string() })? as usize;
                let normal = diff::dec_vec3_bin(&mut reader).map_err(|e| protocol::ProtocolError::Malformed { what: "op normal", offset: semio_framework_plugin::resolve_ready(reader.position()) as u64, detail: e })?;
                Ok(StlMutation::SetTriangleNormal { index, normal })
            }
            6 => {
                let index = reader.read_varint_u64().map_err(|e| protocol::ProtocolError::Malformed { what: "op index", offset: semio_framework_plugin::resolve_ready(reader.position()) as u64, detail: e.to_string() })? as usize;
                let vertices = diff::dec_vertices_bin(&mut reader).map_err(|e| protocol::ProtocolError::Malformed { what: "op vertices", offset: semio_framework_plugin::resolve_ready(reader.position()) as u64, detail: e })?;
                Ok(StlMutation::SetTriangleVertices { index, vertices })
            }
            other => Err(protocol::ProtocolError::Malformed { what: "op tag", offset: 1, detail: format!("unknown tag {other}") }),
        }
    }
}
//#endregion 🔖️OpBinaryCodec
//#endregion OpCodecs

//#region 🔖️DemoCases
/// 🎯 FG1: one representative case per `StlMutation` variant, incl. the two struct-valued payloads
/// (`SetSnapshot`, `InsertTriangle`) and the fixed-size-array payloads (`SetTriangleNormal`,
/// `SetTriangleVertices`) that carry the doubly-nested `[[f64; 3]; 3]` — shared by
/// `op_text_binary_roundtrip_law` below AND `⚙️engine::conformance_laws`'s `ops_grammar_
/// conformance_law`/`protocol_walk_law` (same reuse pattern `binary`'s own `demo_mutation_cases`
/// establishes).
#[cfg(test)]
pub(crate) async fn demo_mutation_cases() -> Vec<StlMutation> {
    let base = StlSnapshot { schema: crate::artifacts::stl::STDIO_STL_DOCUMENT_SCHEMA.into(), solid_name: "mesh".into(), triangles: vec![StlTriangle { normal: [0.0, 0.0, 1.0], vertices: [[0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0]] }] };
    vec![
        StlMutation::NoMutation,
        StlMutation::SetSnapshot { snapshot: StlSnapshot { solid_name: "renamed".into(), ..base } },
        StlMutation::SetSolidName { name: "renamed".into() },
        StlMutation::InsertTriangle { index: 1, triangle: StlTriangle { normal: [1.0, 0.0, 0.0], vertices: [[99.0, 0.0, 0.0], [100.0, 0.0, 0.0], [99.0, 1.0, 0.0]] } },
        StlMutation::RemoveTriangle { index: 1 },
        StlMutation::SetTriangleNormal { index: 0, normal: [1.0, 0.0, 0.0] },
        StlMutation::SetTriangleVertices { index: 0, vertices: [[9.0, 9.0, 9.0], [8.0, 8.0, 8.0], [7.0, 7.0, 7.0]] },
    ]
}
//#endregion 🔖️DemoCases

//#region Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::stl::schema::diff::{StlTriangleAdded, StlTriangleDiff, StlTriangleModified, StlTrianglesDiff};
    use protocol::command::DiffAlgebra;
    use protocol::{DiffCodec, MutationDiff};

    //#region Fixtures
    async fn tri(nx: f64, ny: f64, nz: f64, seed: f64) -> StlTriangle {
        StlTriangle { normal: [nx, ny, nz], vertices: [[seed, 0.0, 0.0], [seed + 1.0, 0.0, 0.0], [seed, 1.0, 0.0]] }
    }

    async fn base_snapshot() -> StlSnapshot {
        StlSnapshot { schema: "stdio.stl".into(), solid_name: "mesh".into(), triangles: vec![tri(0.0, 0.0, 1.0, 0.0), tri(0.0, 0.0, 1.0, 10.0), tri(0.0, 0.0, 1.0, 20.0)] }
    }
    //#endregion Fixtures

    //#region 🔖️mutation_diff_law
    async fn assert_mutation_diff_law(base: &StlSnapshot, mutation: StlMutation) {
        let expected_diff = mutation.diff(base);
        let mut applied_snapshot = base.clone();
        let returned_diff = apply_stl_mutation(&mut applied_snapshot, &mutation);
        assert_eq!(returned_diff, expected_diff, "apply_stl_mutation must return mutation.diff(base) for {mutation:?}");
        assert_eq!(expected_diff.diff().apply(base).expect("valid mutation diff"), applied_snapshot, "diff.diff().apply(base) must equal the imperative mutation result for {mutation:?}");
    }

    #[semio_framework_async_macros::async_test]
    async fn mutation_diff_law() {
        let base = base_snapshot();
        assert_mutation_diff_law(&base, StlMutation::NoMutation);
        let mut alt = base.clone();
        alt.solid_name = "different".into();
        assert_mutation_diff_law(&base, StlMutation::SetSnapshot { snapshot: alt });
        assert_mutation_diff_law(&base, StlMutation::SetSolidName { name: "renamed".into() });
        assert_mutation_diff_law(&base, StlMutation::InsertTriangle { index: 1, triangle: tri(1.0, 0.0, 0.0, 99.0) });
        assert_mutation_diff_law(&base, StlMutation::RemoveTriangle { index: 1 });
        assert_mutation_diff_law(&base, StlMutation::SetTriangleNormal { index: 0, normal: [1.0, 0.0, 0.0] });
        assert_mutation_diff_law(&base, StlMutation::SetTriangleVertices { index: 0, vertices: [[9.0, 9.0, 9.0], [8.0, 8.0, 8.0], [7.0, 7.0, 7.0]] });
    }
    //#endregion 🔖️mutation_diff_law

    //#region 🔖️inverse_law
    #[semio_framework_async_macros::async_test]
    async fn inverse_law() {
        let base = base_snapshot();
        let variants = vec![
            StlMutation::NoMutation,
            StlMutation::SetSolidName { name: "changed".into() },
            StlMutation::InsertTriangle { index: 1, triangle: tri(1.0, 0.0, 0.0, 99.0) },
            StlMutation::RemoveTriangle { index: 1 },
            StlMutation::SetTriangleNormal { index: 0, normal: [1.0, 0.0, 0.0] },
            StlMutation::SetTriangleVertices { index: 0, vertices: [[9.0, 9.0, 9.0], [8.0, 8.0, 8.0], [7.0, 7.0, 7.0]] },
        ];
        for m in variants {
            // Mutation-level round trip.
            let mut snap = base.clone();
            apply_stl_mutation(&mut snap, &m);
            for inv in m.inverse(&base) {
                apply_stl_mutation(&mut snap, &inv);
            }
            assert_eq!(snap, base, "mutation-level inverse must restore base for {m:?}");

            // Diff-level round trip.
            let d = m.diff(&base);
            let mutated = d.diff().apply(&base).expect("valid forward diff");
            let inv_d = d.diff().inverse(&base);
            assert_eq!(inv_d.apply(&mutated).expect("valid inverse diff"), base, "diff-level inverse must restore base for {m:?}");
        }
    }
    //#endregion 🔖️inverse_law

    //#region 🔖️absorb_law
    async fn assert_absorb_law(base: &StlSnapshot, m1: StlMutation, m2: StlMutation) {
        let d1 = m1.diff(base);
        let mid = d1.diff().apply(base).expect("valid first diff");
        let d2 = m2.diff(&mid);
        let sequential = d2.diff().apply(&mid).expect("valid second diff");

        let mut merged = d1.diff().clone();
        merged.absorb(d2.diff().clone());
        assert_eq!(merged.apply(base).expect("valid absorbed diff"), sequential, "absorb(d1,d2).apply(base) must equal sequential application for {m1:?} + {m2:?}");
    }

    #[semio_framework_async_macros::async_test]
    async fn absorb_law() {
        let base = base_snapshot();

        // Insert+Remove-before: added triangle lands correctly once an earlier-positioned base
        // survivor is removed by the second mutation (the recipe's own canonical shift case).
        assert_absorb_law(&base, StlMutation::InsertTriangle { index: 2, triangle: tri(1.0, 0.0, 0.0, 99.0) }, StlMutation::RemoveTriangle { index: 0 });

        // Insert+Insert-same-index: both survive.
        assert_absorb_law(&base, StlMutation::InsertTriangle { index: 1, triangle: tri(1.0, 0.0, 0.0, 91.0) }, StlMutation::InsertTriangle { index: 1, triangle: tri(0.0, 1.0, 0.0, 92.0) });

        // Add+SetField: the second mutation patches directly into the still-pending added triangle.
        assert_absorb_law(&base, StlMutation::InsertTriangle { index: 0, triangle: tri(1.0, 0.0, 0.0, 99.0) }, StlMutation::SetTriangleNormal { index: 0, normal: [0.0, 1.0, 0.0] });

        // Modify+Remove: a pending field patch on a since-removed base triangle vanishes.
        assert_absorb_law(&base, StlMutation::SetTriangleNormal { index: 0, normal: [0.0, 0.0, -1.0] }, StlMutation::RemoveTriangle { index: 0 });

        // Insert then annihilate the very same insert.
        assert_absorb_law(&base, StlMutation::InsertTriangle { index: 0, triangle: tri(1.0, 0.0, 0.0, 99.0) }, StlMutation::RemoveTriangle { index: 0 });

        // Two unrelated scalar sets absorb via LWW.
        assert_absorb_law(&base, StlMutation::SetSolidName { name: "first".into() }, StlMutation::SetSolidName { name: "second".into() });

        // Modify+Modify on the same triangle: both fields land, per-field absorbed.
        assert_absorb_law(&base, StlMutation::SetTriangleNormal { index: 1, normal: [1.0, 0.0, 0.0] }, StlMutation::SetTriangleVertices { index: 1, vertices: [[1.0, 1.0, 1.0], [2.0, 2.0, 2.0], [3.0, 3.0, 3.0]] });
    }

    #[semio_framework_async_macros::async_test]
    async fn absorb_law_associativity() {
        let base = base_snapshot();
        let d1 = StlMutation::SetSolidName { name: "one".into() }.diff(&base);
        let mid1 = d1.diff().apply(&base).expect("valid first diff");
        let d2 = StlMutation::InsertTriangle { index: 0, triangle: tri(1.0, 0.0, 0.0, 50.0) }.diff(&mid1);
        let mid2 = d2.diff().apply(&mid1).expect("valid second diff");
        let d3 = StlMutation::SetTriangleNormal { index: 0, normal: [0.0, 1.0, 0.0] }.diff(&mid2);

        // (d1∘d2)∘d3
        let mut left = d1.diff().clone();
        left.absorb(d2.diff().clone());
        left.absorb(d3.diff().clone());

        // d1∘(d2∘d3)
        let mut d23 = d2.diff().clone();
        d23.absorb(d3.diff().clone());
        let mut right = d1.diff().clone();
        right.absorb(d23);

        assert_eq!(left.apply(&base).expect("valid left diff"), right.apply(&base).expect("valid right diff"), "absorb must associate");
        assert_eq!(left.apply(&base).expect("valid associated diff"), d3.diff().apply(&mid2).expect("valid third diff"), "associated absorb must match full sequential application");
    }
    //#endregion 🔖️absorb_law

    //#region 🔖️between_roundtrip_law
    #[semio_framework_async_macros::async_test]
    async fn between_roundtrip_law() {
        let a = base_snapshot();
        let mut b = base_snapshot();
        b.solid_name = "changed solid name".into();
        b.triangles.remove(0); // remove first triangle
        b.triangles[0].normal = [0.0, 1.0, 0.0]; // modify (now index 0)
        b.triangles.push(tri(0.0, 0.0, -1.0, 30.0)); // add a triangle

        let d = <StlDiff as DiffAlgebra<StlSnapshot>>::between(&a, &b);
        assert_eq!(d.apply(&a).expect("valid forward diff"), b, "between(a,b).apply(a) must equal b");
        let d_rev = <StlDiff as DiffAlgebra<StlSnapshot>>::between(&b, &a);
        assert_eq!(d_rev.apply(&b).expect("valid backward diff"), a, "between(b,a).apply(b) must equal a");
        assert!(<StlDiff as DiffAlgebra<StlSnapshot>>::between(&a, &a).is_empty(), "between(a,a) must be empty");
    }
    //#endregion 🔖️between_roundtrip_law

    //#region 🔖️codec_retention_law
    #[semio_framework_async_macros::async_test]
    async fn codec_retention_law() {
        let bytes = std::fs::read(concat!(env!("CARGO_MANIFEST_DIR"), "/../../🗿️artifacts/🟪️stl/📚️examples/🎬️demo/🖼️assets/🟪️example.stl"));
        let decoded = bytes
            .ok()
            .and_then(|b| String::from_utf8(b).ok())
            .and_then(|text| crate::artifacts::stl::engine::decode_stl_ascii(&text).ok())
            // The checked-in fixture at this path is a shared cross-artifact demo placeholder
            // (not real STL text) — fall back to a synthetic document so this law still exercises
            // a genuine decode -> encode -> decode identity.
            .unwrap_or_else(base_snapshot);
        let reencoded = crate::artifacts::stl::engine::encode_stl_ascii(&decoded);
        let redecoded = crate::artifacts::stl::engine::decode_stl_ascii(&reencoded).expect("re-decode");
        assert_eq!(redecoded.solid_name, decoded.solid_name);
        assert_eq!(redecoded.triangles, decoded.triangles);
    }
    //#endregion 🔖️codec_retention_law

    //#region 🔖️field_sweep
    /// 🌪️ `sweep_a`/`sweep_b` differ in EVERY mutable field. `triangles` is intentionally
    /// asymmetric-length (2 vs 3) — per this ticket's documented fix (an equal-length flat/
    /// unkeyed index collection can structurally only ever show `removed` XOR `added` from a
    /// single `between()` call, never both; asymmetric lengths + asserting across BOTH
    /// directions is the correct way to exercise every triple-kind, matching `txt`'s field_sweep
    /// fix for the identical structural issue).
    async fn sweep_a() -> StlSnapshot {
        StlSnapshot { schema: "stdio.stl".into(), solid_name: "before".into(), triangles: vec![tri(1.0, 0.0, 0.0, 0.0), tri(0.0, 1.0, 0.0, 10.0)] }
    }

    async fn sweep_b() -> StlSnapshot {
        StlSnapshot {
            schema: "stdio.stl".into(),
            solid_name: "after".into(),
            triangles: vec![
                tri(0.0, 0.0, 1.0, 5.0),   // index 0: modified from sweep_a's index 0 (normal AND vertices differ — different seed)
                tri(0.0, 1.0, 0.0, 10.0),  // index 1: unchanged
                tri(-1.0, 0.0, 0.0, 20.0), // index 2: added (b longer than a)
            ],
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn field_sweep_covers_every_mutable_field() {
        let a = sweep_a();
        let b = sweep_b();

        let forward = <StlDiff as DiffAlgebra<StlSnapshot>>::between(&a, &b);
        assert_eq!(forward.apply(&a).expect("valid forward diff"), b, "between(a,b).apply(a) must equal b");
        let backward = <StlDiff as DiffAlgebra<StlSnapshot>>::between(&b, &a);
        assert_eq!(backward.apply(&b).expect("valid backward diff"), a, "between(b,a).apply(b) must equal a");
        assert!(<StlDiff as DiffAlgebra<StlSnapshot>>::between(&a, &a).is_empty(), "between(a,a) must be empty");

        // solid_name: exercised in both directions (LWW scalar).
        assert!(forward.solid_name.is_some(), "solid_name must be diffed forward");
        assert!(backward.solid_name.is_some(), "solid_name must be diffed backward");

        // Forward (a -> b, b is longer): proves `modified` + `added`.
        let ftd: &StlTrianglesDiff = forward.triangles.as_ref().expect("forward triangles diff must be present");
        assert!(ftd.removed.is_empty(), "forward direction must not produce removed (b is longer)");
        assert_eq!(ftd.modified.len(), 1, "exactly one triangle must be modified forward");
        let fmd = &ftd.modified[0];
        assert_eq!(fmd.index, 0);
        assert!(fmd.diff.normal.is_some(), "normal must be diffed");
        assert!(fmd.diff.vertices.is_some(), "vertices must be diffed");
        assert_eq!(ftd.added.len(), 1, "exactly one triangle must be added forward");
        assert_eq!(ftd.added[0].index, 2);
        assert_eq!(ftd.added[0].triangle, b.triangles[2]);

        // Backward (b -> a, a is longer): proves `modified` + `removed`.
        let btd: &StlTrianglesDiff = backward.triangles.as_ref().expect("backward triangles diff must be present");
        assert!(btd.added.is_empty(), "backward direction must not produce added (a is shorter)");
        assert_eq!(btd.modified.len(), 1, "exactly one triangle must be modified backward");
        assert_eq!(btd.modified[0].index, 0);
        assert_eq!(btd.removed, vec![2], "the tail triangle must be tracked removed backward");
    }
    //#endregion 🔖️field_sweep

    //#region 🔖️CanonicalCases
    /// 🎯 The recipe's own canonical absorb shapes, asserted structurally (not just via
    /// `assert_absorb_law`'s apply-equivalence) — `Insert(2)+Remove(0)` and
    /// `Insert(2,f)+Insert(2,g)`.
    #[semio_framework_async_macros::async_test]
    async fn insert_then_remove_before_matches_canonical_shape() {
        let d1 = StlDiff { solid_name: None, triangles: Some(StlTrianglesDiff { removed: vec![], modified: vec![], added: vec![StlTriangleAdded { index: 2, triangle: tri(1.0, 0.0, 0.0, 1.0) }] }) };
        let d2 = StlDiff { solid_name: None, triangles: Some(StlTrianglesDiff { removed: vec![0], modified: vec![], added: vec![] }) };
        let mut merged = d1.clone();
        merged.absorb(d2.clone());
        let td = merged.triangles.clone().expect("triangles diff present");
        assert_eq!(td.removed, vec![0]);
        assert_eq!(td.added, vec![StlTriangleAdded { index: 1, triangle: tri(1.0, 0.0, 0.0, 1.0) }]);
        assert!(td.modified.is_empty());

        let base = base_snapshot();
        let sequential = {
            let mid = d1.apply(&base).expect("valid first diff");
            d2.apply(&mid).expect("valid second diff")
        };
        assert_eq!(merged.apply(&base).expect("valid absorbed diff"), sequential);
    }

    #[semio_framework_async_macros::async_test]
    async fn insert_insert_same_index_both_survive() {
        let d1 = StlDiff { solid_name: None, triangles: Some(StlTrianglesDiff { removed: vec![], modified: vec![], added: vec![StlTriangleAdded { index: 2, triangle: tri(1.0, 0.0, 0.0, 1.0) }] }) };
        let d2 = StlDiff { solid_name: None, triangles: Some(StlTrianglesDiff { removed: vec![], modified: vec![], added: vec![StlTriangleAdded { index: 2, triangle: tri(0.0, 1.0, 0.0, 2.0) }] }) };
        let mut merged = d1.clone();
        merged.absorb(d2.clone());
        let base = base_snapshot();
        let sequential = {
            let mid = d1.apply(&base).expect("valid first diff");
            d2.apply(&mid).expect("valid second diff")
        };
        assert_eq!(merged.apply(&base).expect("valid absorbed diff"), sequential);
        assert_eq!(sequential.triangles.len(), base.triangles.len() + 2);
    }

    #[semio_framework_async_macros::async_test]
    async fn add_then_set_field_patches_into_added() {
        let d1 = StlDiff { solid_name: None, triangles: Some(StlTrianglesDiff { removed: vec![], modified: vec![], added: vec![StlTriangleAdded { index: 1, triangle: tri(1.0, 0.0, 0.0, 1.0) }] }) };
        let d2 = StlDiff { solid_name: None, triangles: Some(StlTrianglesDiff { removed: vec![], modified: vec![StlTriangleModified { index: 1, diff: StlTriangleDiff { normal: Some([0.0, 1.0, 0.0]), vertices: None } }], added: vec![] }) };
        let mut merged = d1.clone();
        merged.absorb(d2.clone());
        let td = merged.triangles.clone().expect("triangles diff present");
        assert!(td.modified.is_empty(), "patched value should live in the added entry, not a separate modified entry");
        assert_eq!(td.added.len(), 1);
        assert_eq!(td.added[0].triangle.normal, [0.0, 1.0, 0.0]);

        let base = base_snapshot();
        let sequential = {
            let mid = d1.apply(&base).expect("valid first diff");
            d2.apply(&mid).expect("valid second diff")
        };
        assert_eq!(merged.apply(&base).expect("valid absorbed diff"), sequential);
    }
    //#endregion 🔖️CanonicalCases

    #[semio_framework_async_macros::async_test]
    async fn out_of_range_triangle_mutation_is_rejected_without_mutating() {
        let base = base_snapshot();
        let mut snap = base.clone();
        let outcome = apply_stl_mutation(&mut snap, &StlMutation::SetTriangleNormal { index: 999, normal: [1.0, 1.0, 1.0] });
        assert_eq!(snap, base);
        assert_eq!(outcome.messages()[0].target, vec!["triangles", "999"]);
        let outcome = apply_stl_mutation(&mut snap, &StlMutation::RemoveTriangle { index: 999 });
        assert_eq!(snap, base);
        assert_eq!(outcome.messages()[0].target, vec!["triangles", "999"]);
    }

    //#region 🔖️F6RoundtripLaws
    /// 🧪️ F6: `OpText`/`OpBinary` round-trip laws (hand-rolled grammar, see this file's `OpCodecs`
    /// region) — every variant, incl. the two struct-valued payloads (`SetSnapshot`,
    /// `InsertTriangle`) and the fixed-size-array payloads (`SetTriangleNormal`,
    /// `SetTriangleVertices`) that carry the doubly-nested `[[f64; 3]; 3]` the `dsl`-derive bug
    /// blocks.
    #[semio_framework_async_macros::async_test]
    async fn op_text_binary_roundtrip_law() {
        // FG1: reuses `demo_mutation_cases()` (this file's own `DemoCases` region, above), the
        // single source of truth also exercised by `⚙️engine::conformance_laws`'s `ops_grammar_
        // conformance_law`/`protocol_walk_law` — same reuse pattern `binary`'s own pilot precedent.
        for mutation in demo_mutation_cases() {
            let printed = mutation.print_op();
            assert!(!printed.contains('\n'), "print_op must be one line, got {printed:?}");
            let parsed = StlMutation::parse_op(&printed).unwrap_or_else(|e| panic!("parse_op({printed:?}) failed: {e}"));
            assert_eq!(parsed, mutation, "print_op/parse_op round-trip mismatch for {mutation:?} (printed {printed:?})");

            let encoded = mutation.encode_op().unwrap_or_else(|e| panic!("encode_op({mutation:?}) failed: {e}"));
            let decoded = StlMutation::decode_op(&encoded).unwrap_or_else(|e| panic!("decode_op failed: {e}"));
            assert_eq!(decoded, mutation, "encode_op/decode_op round-trip mismatch for {mutation:?}");
        }
    }

    /// 🧪️ F6: `DiffCodec` round-trip laws for the hand-rolled `StlDiff` grammar (`🔺️diff::component`'s
    /// `HandcraftedDiffCodec` region) — exercises `solid_name` plus every triangle-triple section
    /// (`removed`/`modified`/`added`), incl. the doubly-nested `vertices` field, simultaneously via
    /// real `between()` results in both directions.
    #[semio_framework_async_macros::async_test]
    async fn diff_codec_text_binary_roundtrip_law() {
        // FG1: reuses `diff::demo_diff_cases()` (this artifact's `🔺️diff::component`'s own
        // `DemoCases` region), the single source of truth also exercised by `⚙️engine::
        // conformance_laws`'s `diff_grammar_conformance_law`/`protocol_walk_law`.
        for d in diff::demo_diff_cases() {
            let printed = d.print_diff();
            assert!(!printed.contains('\n'), "print_diff must be one line, got {printed:?}");
            let parsed = StlDiff::parse_diff(&printed).unwrap_or_else(|e| panic!("parse_diff({printed:?}) failed: {e}"));
            assert_eq!(parsed, d, "print_diff/parse_diff round-trip mismatch (printed {printed:?})");

            let encoded = d.encode_diff().unwrap_or_else(|e| panic!("encode_diff failed: {e}"));
            let decoded = StlDiff::decode_diff(&encoded).unwrap_or_else(|e| panic!("decode_diff failed: {e}"));
            assert_eq!(decoded, d, "encode_diff/decode_diff round-trip mismatch");
        }
    }
    //#endregion 🔖️F6RoundtripLaws
}
//#endregion Tests

//#region 🧪️FixtureCases
/// 🧪️ Handcrafted `📄set-snapshot` fixture cases, wired from this tree's own mutations root so
/// `📦️glue.rs` stays untouched (`#[path]` on a non-inline module resolves against this file's own
/// directory).
#[cfg(test)]
#[path = "📄set-snapshot/🧪️tests/renames-the-solid-and-closes-the-wedge-with-a-third-facet/🦀️component.rs"]
mod set_snapshot_renames_the_solid_and_closes_the_wedge_with_a_third_facet;
//#endregion 🧪️FixtureCases
