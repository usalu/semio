//! 🧬️ ObjArtifact schema — full artifact state, mirrors `ObjSnapshot` field-for-field.

use crate::ObjSnapshot;
use framework_schema::ArtifactSchema;

//#region 🔖️Artifact
/// 🧬️ Full `stdio.obj` artifact state.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, ArtifactSchema, semio_framework_value::RetireOwned)]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.obj")]
pub struct ObjArtifact {
    #[state(artifact)]
    pub schema: String,
    #[state(artifact)]
    #[value(default)]
    pub vertices: Vec<crate::schema::snapshot::ObjVertex>,
    #[state(artifact)]
    #[value(default)]
    pub texcoords: Vec<crate::schema::snapshot::ObjTexCoord>,
    #[state(artifact)]
    #[value(default)]
    pub normals: Vec<crate::schema::snapshot::ObjNormal>,
    #[state(artifact)]
    #[value(default)]
    pub faces: Vec<crate::schema::snapshot::ObjFace>,
    #[state(artifact)]
    #[value(default)]
    pub groups: Vec<crate::schema::snapshot::ObjGroup>,
    #[state(artifact)]
    #[value(default)]
    pub objects: Vec<crate::schema::snapshot::ObjObject>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub mtllib: Option<String>,
    #[state(artifact)]
    #[value(default)]
    pub usemtl: Vec<crate::schema::snapshot::ObjUsemtlRange>,
    #[state(artifact)]
    #[value(default)]
    pub smoothing_groups: Vec<crate::schema::snapshot::ObjSmoothingRange>,
    #[state(artifact)]
    #[value(default)]
    pub unknown_statements: Vec<crate::schema::snapshot::ObjUnknownStatement>,
}
//#endregion 🔖️Artifact

//#region 🔖️Conversions
impl Default for ObjArtifact {
    fn default() -> Self {
        Self::from_snapshot(ObjSnapshot::default())
    }
}

impl ObjArtifact {
    /// 📸️ Persisted subset.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn to_snapshot(&self) -> ObjSnapshot {
        ObjSnapshot {
            schema: self.schema.clone(),
            vertices: self.vertices.clone(),
            texcoords: self.texcoords.clone(),
            normals: self.normals.clone(),
            faces: self.faces.clone(),
            groups: self.groups.clone(),
            objects: self.objects.clone(),
            mtllib: self.mtllib.clone(),
            usemtl: self.usemtl.clone(),
            smoothing_groups: self.smoothing_groups.clone(),
            unknown_statements: self.unknown_statements.clone(),
        }
    }

    /// 🧬️ Builds a full artifact from a snapshot.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn from_snapshot(snapshot: ObjSnapshot) -> Self {
        Self {
            schema: snapshot.schema,
            vertices: snapshot.vertices,
            texcoords: snapshot.texcoords,
            normals: snapshot.normals,
            faces: snapshot.faces,
            groups: snapshot.groups,
            objects: snapshot.objects,
            mtllib: snapshot.mtllib,
            usemtl: snapshot.usemtl,
            smoothing_groups: snapshot.smoothing_groups,
            unknown_statements: snapshot.unknown_statements,
        }
    }

    /// 🔄 Writes persistent fields from a snapshot into this artifact.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn set_snapshot(&mut self, snapshot: ObjSnapshot) {
        self.schema = snapshot.schema;
        self.vertices = snapshot.vertices;
        self.texcoords = snapshot.texcoords;
        self.normals = snapshot.normals;
        self.faces = snapshot.faces;
        self.groups = snapshot.groups;
        self.objects = snapshot.objects;
        self.mtllib = snapshot.mtllib;
        self.usemtl = snapshot.usemtl;
        self.smoothing_groups = snapshot.smoothing_groups;
        self.unknown_statements = snapshot.unknown_statements;
    }
}
//#endregion 🔖️Conversions

//#region 🔖️Descriptor
/// 🧬️ Descriptor for `s.stdio.obj`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn obj_artifact_schema_descriptor() -> semio_framework_schema_registry::ArtifactSchemaDescriptor {
    semio_framework_schema_registry::ArtifactSchemaDescriptor {
        id: "s.stdio.obj",
        artifact: semio_framework_schema_registry::FacetLeaves { rust: include_str!("🦀️.rs"), typescript: include_str!("🟦️.ts"), graphql: include_str!("🔗️.graphql"), json_schema: include_str!("🔣️.json"), proto: include_str!("🛰️.proto") },
        snapshot: semio_framework_schema_registry::FacetLeaves {
            rust: include_str!("📸️snapshot/🦀️.rs"),
            typescript: include_str!("📸️snapshot/🟦️.ts"),
            graphql: include_str!("📸️snapshot/🔗️.graphql"),
            json_schema: include_str!("📸️snapshot/🔣️.json"),
            proto: include_str!("📸️snapshot/🛰️.proto"),
        },
        diff: semio_framework_schema_registry::FacetLeaves {
            rust: include_str!("🔺️diff/🦀️.rs"),
            typescript: include_str!("🔺️diff/🟦️.ts"),
            graphql: include_str!("🔺️diff/🔗️.graphql"),
            json_schema: include_str!("🔺️diff/🔣️.json"),
            proto: include_str!("🔺️diff/🛰️.proto"),
        },
        mutations: semio_framework_schema_registry::FacetLeaves {
            rust: include_str!("🧬️mutations/🦀️.rs"),
            typescript: include_str!("🧬️mutations/🟦️.ts"),
            graphql: include_str!("🧬️mutations/🔗️.graphql"),
            json_schema: include_str!("🧬️mutations/🔣️.json"),
            proto: include_str!("🧬️mutations/🛰️.proto"),
        },
    }
}
//#endregion 🔖️Descriptor
//#region 🏗️DerivedConstruction

//#endregion 🏗️DerivedConstruction

//#region 🧐️DerivedAnalysis

//#endregion 🧐️DerivedAnalysis

//#region 🧬️DerivedArtifactFacets

//#endregion 🧬️DerivedArtifactFacets

//#region 🔖️DocumentHelpers
/// 🌱 Empty persisted snapshot. Dissolved out of `⚙️engine`
/// (ticket 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES) — reached as
/// `crate::engine::empty_obj_snapshot` through the `engine` barrel shim.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn empty_obj_snapshot() -> ObjSnapshot {
    ObjSnapshot::default()
}

/// 📄️ Raw demo Wavefront OBJ text — a two-triangle quad split across two named groups/materials,
/// matching `📚️examples/🎬️demo/🖼️assets/🧊️example.obj` verbatim (this module's own single source
/// of truth for the demo fixture — exercises every statement kind: `mtllib`, `v`/`vt`/`vn`
/// (incl. `w`-omitted forms), `f` (`v/vt/vn` triangles), `o`, `g`, `usemtl`, `s` (both numeric and
/// `off`), a `#` comment, and one genuinely-unrecognized keyword line retained via
/// `unknown_statements`).
const DEMO_OBJ_TEXT: &str = "# stdio.obj demo -- a two-triangle quad split across two named groups/materials\n\
mtllib demo.mtl\n\
v 0 0 0\nv 1 0 0\nv 1 1 0\nv 0 1 0\n\
vt 0 0\nvt 1 0\nvt 1 1\nvt 0 1\n\
vn 0 0 1\n\
o Quad\ng Front\nusemtl Red\ns 1\n\
f 1/1/1 2/2/1 3/3/1\n\
f 1/1/1 3/3/1 4/4/1\n\
g Back\nusemtl Blue\ns off\n\
f 3/3/1 2/2/1 1/1/1\n\
# trailing note retained verbatim\n\
weird_directive foo bar\n";

/// 📄️ The `demo` example snapshot, parsed once from [`DEMO_OBJ_TEXT`] and stabilized to the
/// SECOND-generation decode/encode fixed point this module's own doc comment documents
/// (`unknown_statements[].line_index` renumbers into the trailer on the first re-encode) — so
/// `print_dsl(demo_obj_snapshot())` is genuinely stable, matching `🗣️.dsl.semio`'s own
/// `fixture_honesty_law` requirement exactly. Same pattern `stdio.txt`'s own
/// `demo_txt_snapshot()`/`stdio.csv`'s own `demo_csv_snapshot()` establish.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn demo_obj_snapshot() -> ObjSnapshot {
    let gen1 = crate::engine::decode_obj(DEMO_OBJ_TEXT).unwrap_or_else(|_| empty_obj_snapshot());
    let gen2_text = crate::engine::encode_obj(&gen1);
    crate::engine::decode_obj(&gen2_text).unwrap_or(gen1)
}
//#endregion 🔖️DocumentHelpers

//#region 🔖️RegisterSchemaSpecs
/// 📇️ P2-FG1: `dsl::registry::register_schema_spec` (P2-M3's `FullResolver` insertion API) — a
/// real, non-fabricated call: `ObjSnapshot` genuinely derives `#[derive(dsl::DslRecord)]`
/// (`📸️snapshot/🦀️.rs`), so `__dsl_spec` is a real generated constructor. `ObjDiff` is
/// hand-rolled (§3b tri-state blocker — `Option<Option<T>>` fields — see `🔺️diff/🦀️.rs`'s
/// own module doc comment), NOT `#[derive(dsl::DslDiff)]`, so it has no `__dsl_diff_spec` to
/// register under `"stdio.obj#diff"` — filed as a `mechanism_gaps` entry rather than fabricating a
/// `RecordSpec` that would diverge from the real hand-rolled diff codec (same treatment
/// gif89a/svg's own hand-rolled diffs get). `#[cfg]`-gated to match `os_dsl::registry`'s own
/// `#[cfg(not(target_arch = "wasm32"))]`. Dissolved out of `⚙️engine` (ticket 26/08/12/
/// ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES) — one of the ten deliberate imperative
/// `engine::register()`-family calls left in place at the stdio plugin root's own
/// `.setup(crate::engine::register_schema_specs)`, reached through the `engine`
/// barrel shim.
#[cfg(not(target_arch = "wasm32"))]
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn register_schema_specs() {
    ::semio_framework_async::poll::resolve_ready(dsl::registry::register_schema_spec("stdio.obj", ObjSnapshot::__dsl_spec));
}

#[cfg(target_arch = "wasm32")]
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn register_schema_specs() {}
//#endregion 🔖️RegisterSchemaSpecs
