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
//!
//! 🧬️ Ticket 26/08/29/S-END-TO-END (mutation-leaf migration): `protocol::Mutation<P>` now requires
//! `DESCRIPTORS`/`descriptor()`, which only `#[derive(dsl::Mutations)]` synthesizes. Every variant
//! moved to its own mutation-leaf folder beside this file (`../🖼️tiff/…/🧱️baseline/🧬️schema/🧬️mutations/`
//! is the reference shape). `NoMutation` was dropped: the derive requires every variant to wrap
//! exactly one leaf payload (a unit variant wraps none), and `"no"` is not an `APPROVED_VERB`
//! (`🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🎮️command/🦀️.rs`) a derived `SEMANTICS.verb`
//! could hold anyway.

use crate::schema::diff::{self, StlDiff};
use crate::schema::snapshot::StlTriangle;
use crate::StlSnapshot;
use protocol::Mutation;
use protocol::{OpBinary, OpText};

//#region 🔖️Mutations
#[path = "➕insert-triangle/🦀️.rs"]
pub mod insert_triangle;
#[path = "➖remove-triangle/🦀️.rs"]
pub mod remove_triangle;
/// 📐️ Typed content mutation for `stdio.stl`.
//#region 🔖️Leaves
#[path = "📸️set-snapshot/🦀️.rs"]
pub mod set_snapshot;
#[path = "🏷️set-solid-name/🦀️.rs"]
pub mod set_solid_name;
#[path = "🧭set-triangle-normal/🦀️.rs"]
pub mod set_triangle_normal;
#[path = "📐set-triangle-vertices/🦀️.rs"]
pub mod set_triangle_vertices;
//#endregion 🔖️Leaves

/// 📐️ Typed mutation for this subset. `NoMutation` was dropped: `#[derive(dsl::Mutations)]` requires
/// every variant to wrap exactly one leaf payload and a unit variant wraps none.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::Mutations)]
#[mutations(snapshot = StlSnapshot, diff = StlDiff, schema = "StlMutation")]
#[value(tag = "mutation", rename_all = "camelCase")]
pub enum StlMutation {
    SetSnapshot(set_snapshot::SetSnapshot),
    SetSolidName(set_solid_name::SetSolidName),
    InsertTriangle(insert_triangle::InsertTriangle),
    RemoveTriangle(remove_triangle::RemoveTriangle),
    SetTriangleNormal(set_triangle_normal::SetTriangleNormal),
    SetTriangleVertices(set_triangle_vertices::SetTriangleVertices),
}
//#endregion 🔖️Mutations

//#region 🔖️Kinds
/// 🧾️ Kebab-case spelling of every `StlMutation` variant, in declaration order — the vocabulary
/// `../../🔮️oracle/🔣️.json`'s `stl-ascii-any` catalog is measured against. Kept honest by
/// `kinds_match_enum_and_catalog` below (the framework never parses Rust to learn this list).
pub const KINDS: &[&str] = &["set-snapshot", "set-solid-name", "insert-triangle", "remove-triangle", "set-triangle-normal", "set-triangle-vertices"];
//#endregion 🔖️Kinds

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
/// ▶️ Lifted verbatim from the former hand-rolled `impl Mutation<StlSnapshot> for StlMutation`;
/// every leaf's `MutationKind::diff` reconstructs its `StlMutation` and delegates here, so this
/// stays the single place the forward semantics are written.
// 🚫️async: E1 pure codec/computation helper — lifted verbatim from the former `impl Mutation`.
pub(crate) fn agg_diff(this: &StlMutation, base: &StlSnapshot) -> protocol::MutationOutcome<StlDiff> {
    protocol::MutationOutcome::new(match this {
        StlMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot }) => diff::diff_set_snapshot(base, snapshot),
        StlMutation::SetSolidName(set_solid_name::SetSolidName { name }) => diff::diff_set_solid_name(name),
        StlMutation::InsertTriangle(insert_triangle::InsertTriangle { index, triangle }) => diff::diff_insert_triangle(*index, *triangle),
        StlMutation::RemoveTriangle(remove_triangle::RemoveTriangle { index }) => diff::diff_remove_triangle(*index),
        StlMutation::SetTriangleNormal(set_triangle_normal::SetTriangleNormal { index, normal }) => diff::diff_set_triangle_normal(*index, *normal),
        StlMutation::SetTriangleVertices(set_triangle_vertices::SetTriangleVertices { index, vertices }) => diff::diff_set_triangle_vertices(*index, *vertices),
    })
}

/// ↩️ Lifted verbatim from the former hand-rolled `impl Mutation<StlSnapshot> for StlMutation`.
/// Index-targeted variants look the prior value up in `base`; a stale/out-of-range index inverts
/// to the EMPTY inverse (`Vec::new()`) rather than a `NoMutation` stand-in, now that the derive
/// forbids a unit variant.
// 🚫️async: E1 pure codec/computation helper — lifted verbatim from the former `impl Mutation`.
pub(crate) fn agg_inverse(this: &StlMutation, base: &StlSnapshot) -> Vec<StlMutation> {
    match this {
        StlMutation::SetSnapshot(_) => vec![StlMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot: base.clone() })],
        StlMutation::SetSolidName(_) => vec![StlMutation::SetSolidName(set_solid_name::SetSolidName { name: base.solid_name.clone() })],
        StlMutation::InsertTriangle(insert_triangle::InsertTriangle { index, .. }) => {
            vec![StlMutation::RemoveTriangle(remove_triangle::RemoveTriangle { index: (*index).min(base.triangles.len()) })]
        }
        StlMutation::RemoveTriangle(remove_triangle::RemoveTriangle { index }) => match base.triangles.get(*index) {
            Some(t) => vec![StlMutation::InsertTriangle(insert_triangle::InsertTriangle { index: *index, triangle: *t })],
            None => Vec::new(),
        },
        StlMutation::SetTriangleNormal(set_triangle_normal::SetTriangleNormal { index, .. }) => match base.triangles.get(*index) {
            Some(t) => vec![StlMutation::SetTriangleNormal(set_triangle_normal::SetTriangleNormal { index: *index, normal: t.normal })],
            None => Vec::new(),
        },
        StlMutation::SetTriangleVertices(set_triangle_vertices::SetTriangleVertices { index, .. }) => match base.triangles.get(*index) {
            Some(t) => vec![StlMutation::SetTriangleVertices(set_triangle_vertices::SetTriangleVertices { index: *index, vertices: t.vertices })],
            None => Vec::new(),
        },
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
fn enc_snapshot(s: &StlSnapshot) -> String {
    let stl_triangle_separator = ",";
    format!("[{},{},[{}]]", diff::hex_encode_str(&s.schema), diff::hex_encode_str(&s.solid_name), s.triangles.iter().map(diff::enc_triangle).collect::<Vec<_>>().join(stl_triangle_separator),)
}
fn dec_snapshot(s: &str) -> Result<StlSnapshot, String> {
    let parts = diff::split_top_level(diff::strip_brackets(s)?, ',');
    let [schema, solid_name, triangles] = parts.as_slice() else {
        return Err(format!("snapshot: expected 3 fields, got {}", parts.len()));
    };
    let triangles = diff::split_top_level(diff::strip_brackets(triangles)?, ',').into_iter().filter(|s| !s.is_empty()).map(diff::dec_triangle).collect::<Result<Vec<_>, String>>()?;
    Ok(StlSnapshot { schema: diff::hex_decode_str(schema)?, solid_name: diff::hex_decode_str(solid_name)?, triangles })
}

fn print_stl_op(m: &StlMutation) -> String {
    match m {
        StlMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot }) => format!("set-snapshot snapshot={}", enc_snapshot(snapshot)),
        StlMutation::SetSolidName(set_solid_name::SetSolidName { name }) => format!("set-solid-name name={}", diff::hex_encode_str(name)),
        StlMutation::InsertTriangle(insert_triangle::InsertTriangle { index, triangle }) => format!("insert-triangle index={index} triangle={}", diff::enc_triangle(triangle)),
        StlMutation::RemoveTriangle(remove_triangle::RemoveTriangle { index }) => format!("remove-triangle index={index}"),
        StlMutation::SetTriangleNormal(set_triangle_normal::SetTriangleNormal { index, normal }) => format!("set-triangle-normal index={index} normal={}", diff::enc_vec3(normal)),
        StlMutation::SetTriangleVertices(set_triangle_vertices::SetTriangleVertices { index, vertices }) => format!("set-triangle-vertices index={index} vertices={}", diff::enc_vertices(vertices)),
    }
}
fn parse_stl_op(line: &str) -> Result<StlMutation, String> {
    let mut tokens = line.split(' ');
    let keyword = tokens.next().ok_or_else(|| "stl op: empty line".to_string())?;
    let args: Vec<&str> = tokens.collect();
    let get = |key: &str| -> Result<&str, String> {
        let probe = format!("{key}=");
        args.iter().find_map(|t| t.strip_prefix(probe.as_str())).ok_or_else(|| format!("stl op: missing '{key}=' in {line:?}"))
    };
    match keyword {
        "set-snapshot" => Ok(StlMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot: dec_snapshot(get("snapshot")?)? })),
        "set-solid-name" => Ok(StlMutation::SetSolidName(set_solid_name::SetSolidName { name: diff::hex_decode_str(get("name")?)? })),
        "insert-triangle" => Ok(StlMutation::InsertTriangle(insert_triangle::InsertTriangle { index: diff::parse_usize(get("index")?)?, triangle: diff::dec_triangle(get("triangle")?)? })),
        "remove-triangle" => Ok(StlMutation::RemoveTriangle(remove_triangle::RemoveTriangle { index: diff::parse_usize(get("index")?)? })),
        "set-triangle-normal" => Ok(StlMutation::SetTriangleNormal(set_triangle_normal::SetTriangleNormal { index: diff::parse_usize(get("index")?)?, normal: diff::dec_vec3(get("normal")?)? })),
        "set-triangle-vertices" => Ok(StlMutation::SetTriangleVertices(set_triangle_vertices::SetTriangleVertices { index: diff::parse_usize(get("index")?)?, vertices: diff::dec_vertices(get("vertices")?)? })),
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
fn enc_snapshot_bin(s: &StlSnapshot, out: &mut Vec<u8>) {
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
/// `../💾️binary/📡️.protocol.semio`'s `header fixed 2` + `chain payload bytes` shape —
/// upgraded from the prior `print_stl_op(self).into_bytes()` text-as-binary shortcut. `tag` is
/// the `StlMutation` variant's declaration-order ordinal (1=`SetSnapshot` .. 6=
/// `SetTriangleVertices`, same order `enum StlMutation` declares them; `0` is retired along with
/// `NoMutation`, not reused). Every variant's payload is real field-by-field binary
/// (`write_varint_u64` for `index: usize`, `write_f64_bin`/`enc_vec3_bin`/`enc_vertices_bin`/
/// `enc_triangle_bin`/`enc_snapshot_bin` for the rest) — `StlMutation`'s payload tree has ZERO
/// self-recursion, so nothing here is opaque at the Rust layer; only the protocol-dialect file
/// still frames the payload as one opaque trailing chain (`SetSnapshot`'s `Vec<StlTriangle>` is a
/// variable-length vector-of-records, the same `protocol-array-of-records` `walk_protocol` gap the
/// sibling diff protocol file documents).
impl OpBinary for StlMutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        let mut out = vec![store::pack_rt::OP_BINARY_FORMAT, 0u8];
        let tag: u8 = match self {
            StlMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot }) => {
                enc_snapshot_bin(snapshot, &mut out);
                1
            }
            StlMutation::SetSolidName(set_solid_name::SetSolidName { name }) => {
                diff::write_str_bin(&mut out, name);
                2
            }
            StlMutation::InsertTriangle(insert_triangle::InsertTriangle { index, triangle }) => {
                store::pack_rt::write_varint_u64(&mut out, *index as u64);
                diff::enc_triangle_bin(triangle, &mut out);
                3
            }
            StlMutation::RemoveTriangle(remove_triangle::RemoveTriangle { index }) => {
                store::pack_rt::write_varint_u64(&mut out, *index as u64);
                4
            }
            StlMutation::SetTriangleNormal(set_triangle_normal::SetTriangleNormal { index, normal }) => {
                store::pack_rt::write_varint_u64(&mut out, *index as u64);
                diff::enc_vec3_bin(normal, &mut out);
                5
            }
            StlMutation::SetTriangleVertices(set_triangle_vertices::SetTriangleVertices { index, vertices }) => {
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
            1 => {
                let snapshot = dec_snapshot_bin(&mut reader).map_err(|e| protocol::ProtocolError::Malformed { what: "op snapshot", offset: reader.position() as u64, detail: e })?;
                Ok(StlMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot }))
            }
            2 => {
                let name = diff::read_str_bin(&mut reader).map_err(|e| protocol::ProtocolError::Malformed { what: "op name", offset: reader.position() as u64, detail: e })?;
                Ok(StlMutation::SetSolidName(set_solid_name::SetSolidName { name }))
            }
            3 => {
                let index = reader.read_varint_u64().map_err(|e| protocol::ProtocolError::Malformed { what: "op index", offset: reader.position() as u64, detail: e.to_string() })? as usize;
                let triangle = diff::dec_triangle_bin(&mut reader).map_err(|e| protocol::ProtocolError::Malformed { what: "op triangle", offset: reader.position() as u64, detail: e })?;
                Ok(StlMutation::InsertTriangle(insert_triangle::InsertTriangle { index, triangle }))
            }
            4 => {
                let index = reader.read_varint_u64().map_err(|e| protocol::ProtocolError::Malformed { what: "op index", offset: reader.position() as u64, detail: e.to_string() })? as usize;
                Ok(StlMutation::RemoveTriangle(remove_triangle::RemoveTriangle { index }))
            }
            5 => {
                let index = reader.read_varint_u64().map_err(|e| protocol::ProtocolError::Malformed { what: "op index", offset: reader.position() as u64, detail: e.to_string() })? as usize;
                let normal = diff::dec_vec3_bin(&mut reader).map_err(|e| protocol::ProtocolError::Malformed { what: "op normal", offset: reader.position() as u64, detail: e })?;
                Ok(StlMutation::SetTriangleNormal(set_triangle_normal::SetTriangleNormal { index, normal }))
            }
            6 => {
                let index = reader.read_varint_u64().map_err(|e| protocol::ProtocolError::Malformed { what: "op index", offset: reader.position() as u64, detail: e.to_string() })? as usize;
                let vertices = diff::dec_vertices_bin(&mut reader).map_err(|e| protocol::ProtocolError::Malformed { what: "op vertices", offset: reader.position() as u64, detail: e })?;
                Ok(StlMutation::SetTriangleVertices(set_triangle_vertices::SetTriangleVertices { index, vertices }))
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
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn demo_mutation_cases() -> Vec<StlMutation> {
    let base = StlSnapshot { schema: crate::STDIO_STL_DOCUMENT_SCHEMA.into(), solid_name: "mesh".into(), triangles: vec![StlTriangle { normal: [0.0, 0.0, 1.0], vertices: [[0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0]] }] };
    vec![
        StlMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot: StlSnapshot { solid_name: "renamed".into(), ..base } }),
        StlMutation::SetSolidName(set_solid_name::SetSolidName { name: "renamed".into() }),
        StlMutation::InsertTriangle(insert_triangle::InsertTriangle { index: 1, triangle: StlTriangle { normal: [1.0, 0.0, 0.0], vertices: [[99.0, 0.0, 0.0], [100.0, 0.0, 0.0], [99.0, 1.0, 0.0]] } }),
        StlMutation::RemoveTriangle(remove_triangle::RemoveTriangle { index: 1 }),
        StlMutation::SetTriangleNormal(set_triangle_normal::SetTriangleNormal { index: 0, normal: [1.0, 0.0, 0.0] }),
        StlMutation::SetTriangleVertices(set_triangle_vertices::SetTriangleVertices { index: 0, vertices: [[9.0, 9.0, 9.0], [8.0, 8.0, 8.0], [7.0, 7.0, 7.0]] }),
    ]
}
//#endregion 🔖️DemoCases

//#region Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion Tests

//#region 🧪️FixtureCases
/// 🧪️ Handcrafted `📸️set-snapshot` fixture cases, wired from this tree's own mutations root so
/// `🦀️.rs` stays untouched (`#[path]` on a non-inline module resolves against this file's own
/// directory).
#[cfg(test)]
#[path = "📸️set-snapshot/🧪️tests/✏️renames-the-solid-and-closes-the-wedge-with-a-third-facet/🦀️.rs"]
mod set_snapshot_renames_the_solid_and_closes_the_wedge_with_a_third_facet;
//#endregion 🧪️FixtureCases
