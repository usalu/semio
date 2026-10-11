//! 🧬️ StlMutation — document mutation dispatch. Ticket
//! 26/08/10/ARTIFACT-SYSTEM-OVERHAUL-REAL-CODECS-RUNTIME-REUSE-EVOLUTION: real vocabulary beyond
//! the universal stub — `SetSolidName` plus
//! `InsertTriangle`/`RemoveTriangle`/`SetTriangleNormal`/`SetTriangleVertices` for the index-keyed
//! `triangles` collection. Every variant's `diff()` is handcrafted (constructs `StlDiff` directly
//! via the `schema::diff` builders) — apply-and-capture is never used.
//!
//! 🧪️ F6 (OpText/OpBinary + DiffCodec wave): **HAND-ROLL path** — every variant's payload closure
//! (incl. `InsertTriangle`'s triangle) has zero data-carrying enums (§3a of
//! `f6-recon-report.md`'s decision rule), and `#[derive(dsl::DslOps)]` DID compile cleanly on a
//! first attempt, exactly like `GifMutation`'s pilot. It was reverted for the SAME reason
//! `StlDiff`'s derive was (see `🔺️diff::component`'s top doc comment and `StlTriangle`'s doc
//! comment in `📸️snapshot::component`): a real, reproduced `dsl`-grammar bug where nested
//! `Shape::Tuple` levels (`vertices: [[f64; 3]; 3]`, reachable via `InsertTriangle`,
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


//#region 🔖️Mutations
#[path = "➕insert-triangle/🦀️.rs"]
pub mod insert_triangle;
#[path = "➖remove-triangle/🦀️.rs"]
pub mod remove_triangle;
/// 📐️ Typed content mutation for `stdio.stl`.
//#region 🔖️Leaves
#[path = "🏷️set-solid-name/🦀️.rs"]
pub mod set_solid_name;
#[path = "🧭set-triangle-normal/🦀️.rs"]
pub mod set_triangle_normal;
#[path = "📐set-triangle-vertices/🦀️.rs"]
pub mod set_triangle_vertices;
//#endregion 🔖️Leaves

/// 📐️ Typed mutation for this subset. `NoMutation` was dropped: `#[derive(dsl::Mutations)]` requires
/// every variant to wrap exactly one leaf payload and a unit variant wraps none.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::Mutations, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutations(snapshot = StlSnapshot, diff = StlDiff, schema = "StlMutation")]
#[value(tag = "mutation", rename_all = "camelCase")]
pub enum StlMutation {
    SetSolidName(set_solid_name::SetSolidName),
    InsertTriangle(insert_triangle::InsertTriangle),
    RemoveTriangle(remove_triangle::RemoveTriangle),
    SetTriangleNormal(set_triangle_normal::SetTriangleNormal),
    SetTriangleVertices(set_triangle_vertices::SetTriangleVertices),
}
//#endregion 🔖️Mutations

//#region 🔖️Kinds
/// 🧾️ Kebab-case spelling of every `StlMutation` variant, in declaration order — the vocabulary
/// `../../🔮️oracles/🔣️.json`'s `stl-ascii-any` catalog is measured against. Kept honest by
/// `kinds_match_enum_and_catalog` below (the framework never parses Rust to learn this list).
pub const KINDS: &[&str] = &["set-solid-name", "insert-triangle", "remove-triangle", "set-triangle-normal", "set-triangle-vertices"];
//#endregion 🔖️Kinds


//#endregion 🔖️Apply


//#endregion 🔖️MutationTrait


//#region OpCodecs








//#region 🔖️OpBinaryCodec






//#endregion 🔖️OpBinaryCodec
//#endregion OpCodecs

//#region 🔖️DemoCases
/// 🎯 FG1: one representative case per `StlMutation` variant, incl. the two struct-valued payloads
/// (`InsertTriangle`) and the fixed-size-array payloads (`SetTriangleNormal`,
/// `SetTriangleVertices`) that carry the doubly-nested `[[f64; 3]; 3]` — shared by
/// `op_text_binary_roundtrip_law` below AND `⚙️engine::conformance_laws`'s `ops_grammar_
/// conformance_law`/`protocol_walk_law` (same reuse pattern `binary`'s own `demo_mutation_cases`
/// establishes).
#[cfg(test)]
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn demo_mutation_cases() -> Vec<StlMutation> {
    let base = StlSnapshot { schema: crate::STDIO_STL_DOCUMENT_SCHEMA.into(), solid_name: "mesh".into(), triangles: vec![StlTriangle { normal: [0.0, 0.0, 1.0], vertices: [[0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0]] }] };
    vec![
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
//#endregion 🧪️FixtureCases

#[cfg(test)]
use protocol::{OpBinary,OpText};
