//! 🦀️ Semio MESH exhaustive mutation case — Rust adapter. Ticket 26/08/23/END-TO-END-TESTING-
//! REFACTOR. Recorded no-oracle decision `semio-mesh-mutation-semantics` (`../../🏅️standards/
//! 🔖️v1/🪆️subsets/✳️mesh/🧪️oracle/🔣️component.json`): `s.stdio.semio.mesh` is a semio-NATIVE
//! format with no third-party reader or writer, so `oracle` here reads the committed,
//! independently handcrafted per-kind specification fixtures (`../../🏅️standards/🔖️v1/🪆️subsets/
//! ✳️mesh/🧬️schema/🧬️mutations/<kind>/🧪️tests/<fixture>/`) literally — no recomputation, no
//! reimplementation of mutation semantics. `subject` drives this repository's own
//! `apply_semio_mesh_mutation`, the entry point this ticket added, over the full 17-kind
//! `SemioMeshMutation` vocabulary. Both sides project the snapshot to structural JSON and
//! `ordered-json-v1` compares them. The oracle-only build must never link the subject crate (fleet
//! brief §5.3), so the fixtures' BEFORE snapshot and MUTATION payload are transcribed once, by
//! hand, as `SemioMeshSnapshot`/`SemioMeshMutation` Rust literals inside the `sut`-gated `subject`
//! module below — mechanically identical to the committed JSON, never independently invented
//! (compare against the JSON embedded via `include_str!` in `oracle_fixture`). The generated
//! test-host crate carries no `serde_json` dependency (only `semio-repo-test-host` and, behind
//! `sut`, this subset's own crate), so parsing committed JSON straight into typed structs is not an
//! option here; the framework's own dependency-free `protocol::Json`/`parse_json` carries the
//! oracle side instead. The subject half is gated behind the generated host's `sut` feature so the
//! oracle-only run never compiles the local implementation; the Rust SUBJECT phase is blocked this
//! wave by a concurrent os-kernel refactor (see the fleet brief), so it is written and gated but
//! not run.

use semio_repo_test_host::{parse_json, Adapter, Context, Json, Outcome};

//#region 🔖️Kinds
/// 🏷️ Mirrors `SemioMeshMutation::KINDS` (`../../🏅️standards/🔖️v1/🪆️subsets/✳️mesh/🧬️schema/
/// 🧬️mutations/🦀️component.rs`) — duplicated, not imported, because the oracle-only build must not
/// link the subject crate. The contract's mutation-coverage gate keeps this list honest against the
/// catalog; `kinds_match_the_enum_and_the_catalog` in that production file keeps it honest against
/// the enum.
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

//#region 🔖️OracleFixtures
/// 🧫️ The committed `(before, after)` snapshot JSON for one kind, read literally — this IS the
/// independently handcrafted specification vector the no-oracle decision rests on, never
/// recomputed.
fn oracle_fixture(kind: &str) -> (&'static str, &'static str) {
    match kind {
        "create-mesh" => (
            include_str!("../../🏅️standards/🔖️v1/🪆️subsets/✳️mesh/🧬️schema/🧬️mutations/🕸️create-mesh/🧪️tests/adds-an-empty-second-mesh-at-the-end/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️v1/🪆️subsets/✳️mesh/🧬️schema/🧬️mutations/🕸️create-mesh/🧪️tests/adds-an-empty-second-mesh-at-the-end/📸️snapshot/➡️after/🔣️component.json"),
        ),
        "delete-mesh" => (
            include_str!("../../🏅️standards/🔖️v1/🪆️subsets/✳️mesh/🧬️schema/🧬️mutations/🗑️delete-mesh/🧪️tests/removes-the-leading-mesh-and-keeps-the-trailing-one/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️v1/🪆️subsets/✳️mesh/🧬️schema/🧬️mutations/🗑️delete-mesh/🧪️tests/removes-the-leading-mesh-and-keeps-the-trailing-one/📸️snapshot/➡️after/🔣️component.json"),
        ),
        "create-primitive" => (
            include_str!("../../🏅️standards/🔖️v1/🪆️subsets/✳️mesh/🧬️schema/🧬️mutations/🔺create-primitive/🧪️tests/adds-a-second-primitive-inside-the-existing-mesh/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️v1/🪆️subsets/✳️mesh/🧬️schema/🧬️mutations/🔺create-primitive/🧪️tests/adds-a-second-primitive-inside-the-existing-mesh/📸️snapshot/➡️after/🔣️component.json"),
        ),
        "delete-primitive" => (
            include_str!("../../🏅️standards/🔖️v1/🪆️subsets/✳️mesh/🧬️schema/🧬️mutations/✂️delete-primitive/🧪️tests/removes-the-leading-primitive-and-keeps-the-trailing-one/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️v1/🪆️subsets/✳️mesh/🧬️schema/🧬️mutations/✂️delete-primitive/🧪️tests/removes-the-leading-primitive-and-keeps-the-trailing-one/📸️snapshot/➡️after/🔣️component.json"),
        ),
        "set-primitive-topology" => (
            include_str!("../../🏅️standards/🔖️v1/🪆️subsets/✳️mesh/🧬️schema/🧬️mutations/🔀set-primitive-topology/🧪️tests/switches-the-primitive-to-a-triangle-strip/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️v1/🪆️subsets/✳️mesh/🧬️schema/🧬️mutations/🔀set-primitive-topology/🧪️tests/switches-the-primitive-to-a-triangle-strip/📸️snapshot/➡️after/🔣️component.json"),
        ),
        "replace-primitive-geometry" => (
            include_str!("../../🏅️standards/🔖️v1/🪆️subsets/✳️mesh/🧬️schema/🧬️mutations/📐replace-primitive-geometry/🧪️tests/swaps-the-triangle-for-a-textured-quad/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️v1/🪆️subsets/✳️mesh/🧬️schema/🧬️mutations/📐replace-primitive-geometry/🧪️tests/swaps-the-triangle-for-a-textured-quad/📸️snapshot/➡️after/🔣️component.json"),
        ),
        "set-primitive-material" => (
            include_str!("../../🏅️standards/🔖️v1/🪆️subsets/✳️mesh/🧬️schema/🧬️mutations/🔗set-primitive-material/🧪️tests/binds-the-primitive-to-the-existing-material/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️v1/🪆️subsets/✳️mesh/🧬️schema/🧬️mutations/🔗set-primitive-material/🧪️tests/binds-the-primitive-to-the-existing-material/📸️snapshot/➡️after/🔣️component.json"),
        ),
        "create-material" => (
            include_str!("../../🏅️standards/🔖️v1/🪆️subsets/✳️mesh/🧬️schema/🧬️mutations/🎨create-material/🧪️tests/adds-a-second-material-at-the-end/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️v1/🪆️subsets/✳️mesh/🧬️schema/🧬️mutations/🎨create-material/🧪️tests/adds-a-second-material-at-the-end/📸️snapshot/➡️after/🔣️component.json"),
        ),
        "delete-material" => (
            include_str!("../../🏅️standards/🔖️v1/🪆️subsets/✳️mesh/🧬️schema/🧬️mutations/🚮delete-material/🧪️tests/removes-the-leading-material-and-keeps-the-trailing-one/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️v1/🪆️subsets/✳️mesh/🧬️schema/🧬️mutations/🚮delete-material/🧪️tests/removes-the-leading-material-and-keeps-the-trailing-one/📸️snapshot/➡️after/🔣️component.json"),
        ),
        "change-material-base-color" => (
            include_str!("../../🏅️standards/🔖️v1/🪆️subsets/✳️mesh/🧬️schema/🧬️mutations/🌈change-material-base-color/🧪️tests/repaints-the-material-from-red-to-blue/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️v1/🪆️subsets/✳️mesh/🧬️schema/🧬️mutations/🌈change-material-base-color/🧪️tests/repaints-the-material-from-red-to-blue/📸️snapshot/➡️after/🔣️component.json"),
        ),
        "change-material-metallic" => (
            include_str!("../../🏅️standards/🔖️v1/🪆️subsets/✳️mesh/🧬️schema/🧬️mutations/⚙️change-material-metallic/🧪️tests/raises-the-metallic-factor-to-fully-metallic/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️v1/🪆️subsets/✳️mesh/🧬️schema/🧬️mutations/⚙️change-material-metallic/🧪️tests/raises-the-metallic-factor-to-fully-metallic/📸️snapshot/➡️after/🔣️component.json"),
        ),
        "change-material-roughness" => (
            include_str!("../../🏅️standards/🔖️v1/🪆️subsets/✳️mesh/🧬️schema/🧬️mutations/🧱change-material-roughness/🧪️tests/lowers-the-roughness-factor-to-a-quarter/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️v1/🪆️subsets/✳️mesh/🧬️schema/🧬️mutations/🧱change-material-roughness/🧪️tests/lowers-the-roughness-factor-to-a-quarter/📸️snapshot/➡️after/🔣️component.json"),
        ),
        "create-texture" => (
            include_str!("../../🏅️standards/🔖️v1/🪆️subsets/✳️mesh/🧬️schema/🧬️mutations/🖼️create-texture/🧪️tests/adds-a-second-texture-at-the-end/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️v1/🪆️subsets/✳️mesh/🧬️schema/🧬️mutations/🖼️create-texture/🧪️tests/adds-a-second-texture-at-the-end/📸️snapshot/➡️after/🔣️component.json"),
        ),
        "delete-texture" => (
            include_str!("../../🏅️standards/🔖️v1/🪆️subsets/✳️mesh/🧬️schema/🧬️mutations/🕳️delete-texture/🧪️tests/removes-the-leading-texture-and-keeps-the-trailing-one/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️v1/🪆️subsets/✳️mesh/🧬️schema/🧬️mutations/🕳️delete-texture/🧪️tests/removes-the-leading-texture-and-keeps-the-trailing-one/📸️snapshot/➡️after/🔣️component.json"),
        ),
        "change-texture-mime" => (
            include_str!("../../🏅️standards/🔖️v1/🪆️subsets/✳️mesh/🧬️schema/🧬️mutations/🏷️change-texture-mime/🧪️tests/retags-the-texture-as-jpeg-without-touching-its-bytes/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️v1/🪆️subsets/✳️mesh/🧬️schema/🧬️mutations/🏷️change-texture-mime/🧪️tests/retags-the-texture-as-jpeg-without-touching-its-bytes/📸️snapshot/➡️after/🔣️component.json"),
        ),
        "replace-texture-bytes" => (
            include_str!("../../🏅️standards/🔖️v1/🪆️subsets/✳️mesh/🧬️schema/🧬️mutations/📀replace-texture-bytes/🧪️tests/swaps-the-texture-payload-without-retagging-its-mime/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️v1/🪆️subsets/✳️mesh/🧬️schema/🧬️mutations/📀replace-texture-bytes/🧪️tests/swaps-the-texture-payload-without-retagging-its-mime/📸️snapshot/➡️after/🔣️component.json"),
        ),
        "move-vertex" => (
            include_str!("../../🏅️standards/🔖️v1/🪆️subsets/✳️mesh/🧬️schema/🧬️mutations/📍move-vertex/🧪️tests/lifts-the-third-vertex-of-the-triangle/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️v1/🪆️subsets/✳️mesh/🧬️schema/🧬️mutations/📍move-vertex/🧪️tests/lifts-the-third-vertex-of-the-triangle/📸️snapshot/➡️after/🔣️component.json"),
        ),
        other => panic!("mutate-semio-mesh: no fixture registered for kind {other:?}"),
    }
}

/// 🔎️ Parses one embedded fixture file into the framework's own dependency-free `Json`.
fn canonical(text: &str) -> Json {
    parse_json(text).unwrap_or_else(|error| panic!("committed fixture JSON must parse: {error}"))
}
//#endregion 🔖️OracleFixtures

//#region 🔖️Oracle
/// 🔮️ The forward reference answer: the committed AFTER snapshot, read literally.
fn mutate_oracle_for(kind: &'static str) -> impl Fn(&Context) -> Result<Outcome, String> {
    move |_ctx: &Context| {
        let (_before, after) = oracle_fixture(kind);
        Ok(Outcome::with_raw(after.as_bytes().to_vec(), canonical(after)))
    }
}

/// 🔮️ The inverse reference answer: the committed BEFORE snapshot — undoing any mutation must
/// return to exactly where the specification vector started.
fn inverse_oracle_for(kind: &'static str) -> impl Fn(&Context) -> Result<Outcome, String> {
    move |_ctx: &Context| {
        let (before, _after) = oracle_fixture(kind);
        Ok(Outcome::with_raw(before.as_bytes().to_vec(), canonical(before)))
    }
}
//#endregion 🔖️Oracle

//#region 🔖️Subject
#[cfg(feature = "sut")]
mod subject {
    use semio_repo_test_host::{Context, Json, Outcome};
    use semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::any::schema::geometry::{SemioPoint3, SemioRgba, SemioUv};
    use semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::mesh::schema::mutations::{
        change_material_base_color, change_material_metallic, change_material_roughness, change_texture_mime, create_material, create_mesh, create_primitive, create_texture, delete_material, delete_mesh, delete_primitive, delete_texture, move_vertex, replace_primitive_geometry,
        replace_texture_bytes, set_primitive_material, set_primitive_topology, SemioMeshMutation,
    };
    use semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::mesh::schema::snapshot::{SemioMaterial, SemioMesh, SemioMeshSnapshot, SemioPrimitive, SemioTexture, SemioTopology};
    use protocol::Mutation;

    //#region 🔖️HandcraftedFixtures
    /// 🧫️ The SAME specification vector `../🦀️component.rs::oracle_fixture` embeds as JSON,
    /// transcribed once by hand into real `SemioMeshSnapshot`/`SemioMeshMutation` values — the
    /// oracle-only build must never link this crate, so there is no way to share one physical
    /// source between the two roles; committed side by side under the same kind's `🧪️tests/`
    /// directory, so a drift between them is a one-file diff away from being caught by eye.
    fn p3(x: f64, y: f64, z: f64) -> SemioPoint3 {
        SemioPoint3 { x, y, z }
    }
    fn uv(u: f64, v: f64) -> SemioUv {
        SemioUv { u, v }
    }
    fn rgba(r: f32, g: f32, b: f32, a: f32) -> SemioRgba {
        SemioRgba { r, g, b, a }
    }

    /// 🔺 `prim-a`: the base fixture's sole primitive — a plain, unmaterialed CCW triangle.
    fn prim_a() -> SemioPrimitive {
        SemioPrimitive { id: "prim-a".into(), topology: SemioTopology::Triangles, positions: vec![p3(0.0, 0.0, 0.0), p3(1.0, 0.0, 0.0), p3(0.0, 1.0, 0.0)], normals: vec![], uvs: vec![], colors: vec![], indices: vec![0, 1, 2], material_id: None }
    }
    /// 🎨 `mat-a`: the base fixture's sole material — opaque red, non-metallic, half-rough.
    fn mat_a() -> SemioMaterial {
        SemioMaterial { id: "mat-a".into(), base_color: rgba(1.0, 0.0, 0.0, 1.0), metallic: 0.0, roughness: 0.5 }
    }
    /// 🖼️ `tex-a`: the base fixture's sole texture — a tiny 4-byte PNG-tagged payload.
    fn tex_a() -> SemioTexture {
        SemioTexture { id: "tex-a".into(), mime: "image/png".into(), bytes: vec![1, 2, 3, 4] }
    }
    /// 🕸️ The shared base snapshot every single-mutation fixture starts from: one mesh (`mesh-a`)
    /// holding `prim_a()`, one material (`mat_a()`), one texture (`tex_a()`) — matches every
    /// committed `⬅️before/🔣️component.json` this kind's fixture directory carries except
    /// `delete-material`/`delete-texture`, whose `before` additionally carries a second entity kept
    /// in `after` (handled inline in `fixture_for` below).
    fn base_snapshot() -> SemioMeshSnapshot {
        SemioMeshSnapshot { schema: "stdio.semio.mesh".into(), meshes: vec![SemioMesh { id: "mesh-a".into(), primitives: vec![prim_a()] }], materials: vec![mat_a()], textures: vec![tex_a()] }
    }

    fn fixture_for(kind: &str) -> (SemioMeshSnapshot, SemioMeshMutation) {
        match kind {
            "create-mesh" => (base_snapshot(), SemioMeshMutation::CreateMesh(create_mesh::mutation::CreateMesh { mesh: SemioMesh { id: "mesh-b".into(), primitives: vec![] } })),
            "delete-mesh" => {
                let mut before = base_snapshot();
                before.meshes.push(SemioMesh { id: "mesh-b".into(), primitives: vec![] });
                (before, SemioMeshMutation::DeleteMesh(delete_mesh::mutation::DeleteMesh { id: "mesh-a".into() }))
            }
            "create-primitive" => {
                let primitive = SemioPrimitive { id: "prim-b".into(), topology: SemioTopology::Lines, positions: vec![p3(0.0, 0.0, 0.0), p3(0.0, 0.0, 1.0)], normals: vec![], uvs: vec![], colors: vec![], indices: vec![0, 1], material_id: None };
                (base_snapshot(), SemioMeshMutation::CreatePrimitive(create_primitive::mutation::CreatePrimitive { mesh_id: "mesh-a".into(), primitive }))
            }
            "delete-primitive" => {
                let mut before = base_snapshot();
                let second = SemioPrimitive { id: "prim-b".into(), topology: SemioTopology::Lines, positions: vec![p3(0.0, 0.0, 0.0), p3(0.0, 0.0, 1.0)], normals: vec![], uvs: vec![], colors: vec![], indices: vec![0, 1], material_id: None };
                before.meshes[0].primitives.push(second);
                (before, SemioMeshMutation::DeletePrimitive(delete_primitive::mutation::DeletePrimitive { mesh_id: "mesh-a".into(), primitive_id: "prim-a".into() }))
            }
            "set-primitive-topology" => (base_snapshot(), SemioMeshMutation::SetPrimitiveTopology(set_primitive_topology::mutation::SetPrimitiveTopology { mesh_id: "mesh-a".into(), primitive_id: "prim-a".into(), topology: SemioTopology::TriangleStrip })),
            "replace-primitive-geometry" => (
                base_snapshot(),
                SemioMeshMutation::ReplacePrimitiveGeometry(replace_primitive_geometry::mutation::ReplacePrimitiveGeometry {
                    mesh_id: "mesh-a".into(),
                    primitive_id: "prim-a".into(),
                    positions: vec![p3(0.0, 0.0, 0.0), p3(2.0, 0.0, 0.0), p3(0.0, 2.0, 0.0), p3(2.0, 2.0, 0.0)],
                    normals: vec![p3(0.0, 0.0, 1.0); 4],
                    uvs: vec![uv(0.0, 0.0), uv(1.0, 0.0), uv(0.0, 1.0), uv(1.0, 1.0)],
                    colors: vec![rgba(1.0, 1.0, 1.0, 1.0); 4],
                    indices: vec![0, 1, 2, 1, 3, 2],
                }),
            ),
            "set-primitive-material" => (base_snapshot(), SemioMeshMutation::SetPrimitiveMaterial(set_primitive_material::mutation::SetPrimitiveMaterial { mesh_id: "mesh-a".into(), primitive_id: "prim-a".into(), material_id: Some("mat-a".into()) })),
            "create-material" => (base_snapshot(), SemioMeshMutation::CreateMaterial(create_material::mutation::CreateMaterial { material: SemioMaterial { id: "mat-b".into(), base_color: rgba(0.0, 0.0, 1.0, 1.0), metallic: 1.0, roughness: 0.25 } })),
            "delete-material" => {
                let mut before = base_snapshot();
                before.materials.push(SemioMaterial { id: "mat-b".into(), base_color: rgba(0.0, 0.0, 1.0, 1.0), metallic: 1.0, roughness: 0.25 });
                (before, SemioMeshMutation::DeleteMaterial(delete_material::mutation::DeleteMaterial { id: "mat-a".into() }))
            }
            "change-material-base-color" => (base_snapshot(), SemioMeshMutation::ChangeMaterialBaseColor(change_material_base_color::mutation::ChangeMaterialBaseColor { id: "mat-a".into(), new_base_color: rgba(0.0, 0.5, 1.0, 1.0) })),
            "change-material-metallic" => (base_snapshot(), SemioMeshMutation::ChangeMaterialMetallic(change_material_metallic::mutation::ChangeMaterialMetallic { id: "mat-a".into(), new_metallic: 1.0 })),
            "change-material-roughness" => (base_snapshot(), SemioMeshMutation::ChangeMaterialRoughness(change_material_roughness::mutation::ChangeMaterialRoughness { id: "mat-a".into(), new_roughness: 0.25 })),
            "create-texture" => (base_snapshot(), SemioMeshMutation::CreateTexture(create_texture::mutation::CreateTexture { texture: SemioTexture { id: "tex-b".into(), mime: "image/jpeg".into(), bytes: vec![9, 8, 7] } })),
            "delete-texture" => {
                let mut before = base_snapshot();
                before.textures.push(SemioTexture { id: "tex-b".into(), mime: "image/jpeg".into(), bytes: vec![9, 8, 7] });
                (before, SemioMeshMutation::DeleteTexture(delete_texture::mutation::DeleteTexture { id: "tex-a".into() }))
            }
            "change-texture-mime" => (base_snapshot(), SemioMeshMutation::ChangeTextureMime(change_texture_mime::mutation::ChangeTextureMime { id: "tex-a".into(), new_mime: "image/jpeg".into() })),
            "replace-texture-bytes" => (base_snapshot(), SemioMeshMutation::ReplaceTextureBytes(replace_texture_bytes::mutation::ReplaceTextureBytes { id: "tex-a".into(), new_bytes: vec![10, 20, 30, 40, 50] })),
            "move-vertex" => (base_snapshot(), SemioMeshMutation::MoveVertex(move_vertex::mutation::MoveVertex { mesh_id: "mesh-a".into(), primitive_id: "prim-a".into(), vertex_index: 2, new_point: p3(0.0, 1.0, 0.5) })),
            other => panic!("mutate-semio-mesh: no fixture registered for kind {other:?}"),
        }
    }
    //#endregion 🔖️HandcraftedFixtures

    //#region 🔖️Projection
    fn topology_str(topology: SemioTopology) -> &'static str {
        match topology {
            SemioTopology::Points => "points",
            SemioTopology::Lines => "lines",
            SemioTopology::LineStrip => "lineStrip",
            SemioTopology::Triangles => "triangles",
            SemioTopology::TriangleStrip => "triangleStrip",
            SemioTopology::TriangleFan => "triangleFan",
        }
    }
    fn point3_json(p: &SemioPoint3) -> Json {
        Json::Object(vec![("x".to_string(), Json::Number(p.x)), ("y".to_string(), Json::Number(p.y)), ("z".to_string(), Json::Number(p.z))])
    }
    fn uv_json(v: &SemioUv) -> Json {
        Json::Object(vec![("u".to_string(), Json::Number(v.u)), ("v".to_string(), Json::Number(v.v))])
    }
    fn rgba_json(c: &SemioRgba) -> Json {
        Json::Object(vec![("r".to_string(), Json::Number(c.r as f64)), ("g".to_string(), Json::Number(c.g as f64)), ("b".to_string(), Json::Number(c.b as f64)), ("a".to_string(), Json::Number(c.a as f64))])
    }
    fn primitive_json(p: &SemioPrimitive) -> Json {
        Json::Object(vec![
            ("id".to_string(), Json::String(p.id.clone())),
            ("topology".to_string(), Json::String(topology_str(p.topology).to_string())),
            ("positions".to_string(), Json::Array(p.positions.iter().map(point3_json).collect())),
            ("normals".to_string(), Json::Array(p.normals.iter().map(point3_json).collect())),
            ("uvs".to_string(), Json::Array(p.uvs.iter().map(uv_json).collect())),
            ("colors".to_string(), Json::Array(p.colors.iter().map(rgba_json).collect())),
            ("indices".to_string(), Json::Array(p.indices.iter().map(|v| Json::Number(*v as f64)).collect())),
            ("materialId".to_string(), match &p.material_id { Some(id) => Json::String(id.clone()), None => Json::Null }),
        ])
    }
    fn mesh_json(m: &SemioMesh) -> Json {
        Json::Object(vec![("id".to_string(), Json::String(m.id.clone())), ("primitives".to_string(), Json::Array(m.primitives.iter().map(primitive_json).collect()))])
    }
    fn material_json(m: &SemioMaterial) -> Json {
        Json::Object(vec![("id".to_string(), Json::String(m.id.clone())), ("baseColor".to_string(), rgba_json(&m.base_color)), ("metallic".to_string(), Json::Number(m.metallic as f64)), ("roughness".to_string(), Json::Number(m.roughness as f64))])
    }
    fn texture_json(t: &SemioTexture) -> Json {
        Json::Object(vec![("id".to_string(), Json::String(t.id.clone())), ("mime".to_string(), Json::String(t.mime.clone())), ("bytes".to_string(), Json::Array(t.bytes.iter().map(|b| Json::Number(*b as f64)).collect()))])
    }
    /// 🎯️ The projection every scenario compares under `ordered-json-v1`: the snapshot's own
    /// structural JSON shape, matching the committed fixtures field for field.
    fn snapshot_json(snapshot: &SemioMeshSnapshot) -> Json {
        Json::Object(vec![
            ("schema".to_string(), Json::String(snapshot.schema.clone())),
            ("meshes".to_string(), Json::Array(snapshot.meshes.iter().map(mesh_json).collect())),
            ("materials".to_string(), Json::Array(snapshot.materials.iter().map(material_json).collect())),
            ("textures".to_string(), Json::Array(snapshot.textures.iter().map(texture_json).collect())),
        ])
    }
    //#endregion 🔖️Projection

    //#region 🔖️Handlers
    pub fn mutate(kind: &'static str) -> impl Fn(&Context) -> Result<Outcome, String> {
        move |_ctx: &Context| {
            let (mut base, mutation) = fixture_for(kind);
            let outcome = semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::mesh::schema::mutations::apply_semio_mesh_mutation(&mut base, &mutation);
            if !outcome.messages().is_empty() {
                return Err(format!("mutate-{kind}: mutation rejected: {:?}", outcome.messages()));
            }
            let projection = snapshot_json(&base);
            let bytes = projection.to_string().into_bytes();
            Ok(Outcome::with_raw(bytes, projection))
        }
    }

    pub fn inverse(kind: &'static str) -> impl Fn(&Context) -> Result<Outcome, String> {
        move |_ctx: &Context| {
            let (base, mutation) = fixture_for(kind);
            let mut current = base.clone();
            let outcome = semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::mesh::schema::mutations::apply_semio_mesh_mutation(&mut current, &mutation);
            if !outcome.messages().is_empty() {
                return Err(format!("inverse-{kind}: forward mutation rejected: {:?}", outcome.messages()));
            }
            let undo = mutation.inverse(&base);
            for step in &undo {
                let step_outcome = semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::mesh::schema::mutations::apply_semio_mesh_mutation(&mut current, step);
                if !step_outcome.messages().is_empty() {
                    return Err(format!("inverse-{kind}: inverse step rejected: {:?}", step_outcome.messages()));
                }
            }
            let projection = snapshot_json(&current);
            let bytes = projection.to_string().into_bytes();
            Ok(Outcome::with_raw(bytes, projection))
        }
    }
    //#endregion 🔖️Handlers
}
//#endregion 🔖️Subject

//#region 🔖️Registration
/// 🧭️ Registration entry point the generated host calls.
pub fn adapter() -> Adapter {
    let mut built = Adapter::new("rust");
    for kind in KINDS {
        built = built.oracle(&format!("mutate-{kind}"), mutate_oracle_for(kind)).oracle(&format!("inverse-{kind}"), inverse_oracle_for(kind));
        #[cfg(feature = "sut")]
        {
            built = built.subject(&format!("mutate-{kind}"), subject::mutate(kind)).subject(&format!("inverse-{kind}"), subject::inverse(kind));
        }
    }
    built
}
//#endregion 🔖️Registration
