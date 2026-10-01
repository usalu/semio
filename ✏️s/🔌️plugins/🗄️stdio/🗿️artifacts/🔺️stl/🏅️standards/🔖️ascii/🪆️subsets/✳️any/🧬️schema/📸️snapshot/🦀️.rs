//! 🧬️ StlSnapshot schema — persistent fields + real codecs. Ticket
//! 26/08/10/ARTIFACT-SYSTEM-OVERHAUL-REAL-CODECS-RUNTIME-REUSE-EVOLUTION: complete per the ASCII
//! STL spec (https://en.wikipedia.org/wiki/STL_(file_format)) — `solid <name>` header/trailer plus
//! an index-keyed, self-contained triangle soup (`StlTriangle{normal, vertices}`, each triangle
//! owning its own 3 vertices — real STL has no shared vertex index space). Replaces the shared,
//! byte-identical `MeshVertex`/`MeshTriangle` indexed-mesh types (W0-confirmed duplicated verbatim
//! with 🧱️ply) with this artifact's own, format-accurate model.

use crate::STDIO_STL_DOCUMENT_SCHEMA;
use framework_schema::ArtifactSchema;

//#region 🔖️StlTriangle
/// 🔺️ One ASCII/binary STL facet: `facet normal ni nj nk` / `outer loop` / 3×`vertex vx vy vz` /
/// `endloop` / `endfacet`. The facet normal is persisted exactly as read (never recomputed from
/// vertex winding on encode — many real-world writers legitimately emit `facet normal 0 0 0` and
/// rely on readers to recompute it downstream; silently overwriting that on re-encode would be a
/// fabrication this recipe forbids). Vertices are NOT index-shared across triangles — the ASCII
/// and binary STL grammars both write every triangle's 3 vertices independently, so this type
/// owns them directly rather than referencing a separate vertex pool.
/// 🧪️ F6 (OpText/OpBinary + DiffCodec wave): **HAND-ROLL path**, and a genuinely NEW third
/// blocker beyond `f6-recon-report.md`'s documented §3a (enum-in-tree)/§3b (tri-state) rules —
/// confirmed by a real, reproduced test failure, not assumed: `vertices: [[f64; 3]; 3]` is a
/// doubly-nested fixed-size array. `dsl`'s `impl<T: DslField, const N: usize> DslField for [T; N]`
/// binds it fine at COMPILE time (`[f64; 3]: DslField` via the same blanket impl, one level down),
/// but `dsl`'s grammar engine's `Shape::Tuple` printer
/// (`🧰️framework/…/🗣️dsl/🧬️schema/🦀️component.rs::print_shape`, the `(FieldValue::Tuple(items),
/// Shape::Tuple(elem, _))` arm) prints EVERY tuple level as a flat, unbracketed comma-join with no
/// depth marker — `[[1,2,3],[4,5,6],[7,8,9]]` prints as the indistinguishable-from-flat
/// `"1,2,3,4,5,6,7,8,9"` — and the matching `parse_shape` arm's inner recursive call greedily
/// consumes every remaining top-level comma (it has no way to know the inner tuple is only 3 wide),
/// so round-tripping a real value fails: `parse_diff("… vertices=1,2,3,4,5,6,7,8,9 …")` errors
/// `"tuple expects 3 elements, found 9"` (reproduced verbatim via a real `cargo test` run — see
/// `f6-stl-report.md`). This is a `dsl` framework bug, out of this artifact's ownership boundary to
/// fix (`🗣️dsl` is a shared module) — `StlTriangle` therefore carries NO `dsl` derive; `StlDiff`'s
/// `DiffCodec` and `StlMutation`'s `OpText`/`OpBinary` are hand-rolled instead (see those files),
/// using an explicit `[…]`-bracketed grammar for `normal`/`vertices` that DOES mark nesting depth
/// (`enc_vec3`/`enc_vertices` in `🔺️diff::component`), sidestepping the bug entirely.
#[derive(Clone, Copy, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct StlTriangle {
    pub normal: [f64; 3],
    pub vertices: [[f64; 3]; 3],
}

impl Default for StlTriangle {
    fn default() -> Self {
        Self { normal: [0.0; 3], vertices: [[0.0; 3]; 3] }
    }
}
//#endregion 🔖️StlTriangle

//#region 🔖️Snapshot
/// 📸️ Persisted `stdio.stl` snapshot — complete per the ASCII STL spec: `solid_name` is the
/// `solid`/`endsolid` header/trailer's name token (empty string when the file omits it, as many
/// real-world writers do), `triangles` is the ordered facet list, index-keyed for diffing.
/// 🧪️ F6: no `dsl` derive here either — `StlSnapshot` embeds `StlTriangle` (see its doc comment
/// for the real, reproduced nested-`[T;N]` grammar bug this artifact's whole `dsl`-derive attempt
/// hit) — `StlMutation::SetSnapshot`'s payload is hand-encoded via `enc_snapshot`/`dec_snapshot`
/// in `🧬️mutations::component` instead.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, ArtifactSchema)]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.stl")]
pub struct StlSnapshot {
    #[state(artifact)]
    pub schema: String,
    #[state(artifact)]
    #[value(default)]
    pub solid_name: String,
    #[state(artifact)]
    #[value(default)]
    pub triangles: Vec<StlTriangle>,
}

impl Default for StlSnapshot {
    fn default() -> Self {
        Self { schema: STDIO_STL_DOCUMENT_SCHEMA.into(), solid_name: String::new(), triangles: Vec::new() }
    }
}
//#endregion 🔖️Snapshot

//#region 🔖️HandcraftedArtifactCodecs
// 🔗 Real ASCII/binary grammar lives in `⚙️engine::encode_stl_ascii`/`decode_stl_ascii` and
// `encode_stl_binary`/`decode_stl_binary` (https://en.wikipedia.org/wiki/STL_(file_format)).
#[path="📦️pack/🦀️.rs"]
mod native_pack;
//#endregion 🔖️HandcraftedArtifactCodecs

#[path = "🪶️sqlite/🦀️.rs"]
mod sqlite;
#[cfg(test)]
#[path = "🧪️tests/🪶️sqlite/🦀️.rs"]
mod sqlite_tests;
