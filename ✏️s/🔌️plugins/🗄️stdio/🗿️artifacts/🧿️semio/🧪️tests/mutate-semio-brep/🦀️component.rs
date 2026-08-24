//! 🦀️ Semio BREP exhaustive mutation case — Rust adapter. Ticket 26/08/23/END-TO-END-TESTING-
//! REFACTOR. Recorded no-oracle decision `semio-brep-mutation-semantics` (`../../🏅️standards/
//! 🔖️v1/🪆️subsets/✳️brep/🧪️oracle/🔣️component.json`): `s.stdio.semio.brep` is a semio-NATIVE
//! format with no third-party reader or writer, so the evidence rests entirely on the committed,
//! independently handcrafted per-kind specification fixtures under `../../🏅️standards/🔖️v1/
//! 🪆️subsets/✳️brep/🧬️schema/🧬️mutations/<kind>/🧪️tests/<fixture>/`.
//!
//! BOTH roles read those fixtures directly, through `Context::fixture_json`: each scenario's
//! `Given` step names its `before`/`mutation`/`after` files as `asset://` URIs resolved against
//! this artifact's own root, and the planner digest-pins every one before a host sees it. Nothing
//! about a fixture is transcribed into this file — an earlier draft hand-transcribed the before-
//! snapshots and mutation payloads into Rust literals (the generated test host links only
//! `semio-repo-test-host` and, behind `sut`, this subset's own crate, so `serde_json` is not
//! available to parse them), but a literal can silently stop matching the fixture it claims to
//! mirror, which is exactly the drift a specification-vector substitute cannot afford. The typed
//! values the production entry point needs are instead decoded from the committed JSON by the
//! hand-rolled, dependency-free `Decode` region below, mirroring this subset's own serde shape
//! (camelCase snapshot fields, `"kind"`-tagged `BrepCurve`/`BrepSurface`, externally tagged
//! mutation variants with snake_case payload fields).
//!
//! `oracle` returns the committed snapshot literally — no recomputation, no reimplementation of
//! mutation semantics. `subject` drives this repository's own `apply_semio_brep_mutation`, the
//! entry point this ticket added, over the full 13-kind `SemioBrepMutation` vocabulary, then
//! projects the result back to structural JSON for `ordered-json-v1` to compare. The subject half
//! is gated behind the generated host's `sut` feature so the oracle-only run never compiles the
//! local implementation (fleet brief §5.3); the Rust SUBJECT phase is blocked this wave by a
//! concurrent os-kernel refactor (see the fleet brief), so it is written and gated but not run.

use semio_repo_test_host::{Adapter, Context, Outcome};

//#region 🔖️Kinds
/// 🏷️ Mirrors `SemioBrepMutation::KINDS` (`../../🏅️standards/🔖️v1/🪆️subsets/✳️brep/🧬️schema/
/// 🧬️mutations/🦀️component.rs`) — duplicated, not imported, because the oracle-only build must not
/// link the subject crate. The contract's mutation-coverage gate keeps this list honest against the
/// catalog; `kinds_match_the_enum_and_the_catalog` in that production file keeps it honest against
/// the enum.
const KINDS: &[&str] = &["create-vertex", "delete-vertex", "create-edge", "delete-edge", "create-face", "delete-face", "create-shell", "delete-shell", "create-solid", "delete-solid", "replace-curve", "replace-surface", "move-vertex"];
//#endregion 🔖️Kinds

//#region 🔖️FixtureBinding
/// 🧫️ The `asset://` URI the scenario's own `Given` table binds to one role (`before`, `mutation`
/// or `after`). The feature file is the single source of truth for which fixture each kind uses —
/// this adapter never spells a fixture path itself, so a renamed or re-pointed fixture is a
/// one-file edit that both roles pick up together.
fn fixture_uri(ctx: &Context, role: &str) -> Result<String, String> {
    let table = ctx.data_table()?;
    let header = table.first().ok_or_else(|| format!("scenario {} has an empty fixture table", ctx.scenario.id))?;
    let role_column = header.iter().position(|cell| cell == "role").ok_or_else(|| format!("scenario {}'s fixture table has no 'role' column", ctx.scenario.id))?;
    let fixture_column = header.iter().position(|cell| cell == "fixture").ok_or_else(|| format!("scenario {}'s fixture table has no 'fixture' column", ctx.scenario.id))?;
    for row in table.iter().skip(1) {
        if row.get(role_column).map(String::as_str) == Some(role) {
            return row.get(fixture_column).cloned().ok_or_else(|| format!("scenario {}'s {role:?} row carries no fixture URI", ctx.scenario.id));
        }
    }
    Err(format!("scenario {} binds no {role:?} fixture", ctx.scenario.id))
}
//#endregion 🔖️FixtureBinding

//#region 🔖️Oracle
/// 🔮️ The reference answer for either direction, read literally from the committed fixture the
/// scenario names: the AFTER snapshot for `mutate-*`, the BEFORE snapshot for `inverse-*` (undoing
/// a mutation must return to exactly where the specification vector started).
fn snapshot_oracle_for(role: &'static str) -> impl Fn(&Context) -> Result<Outcome, String> {
    move |ctx: &Context| {
        let uri = fixture_uri(ctx, role)?;
        let raw = ctx.fixture_bytes(&uri)?;
        let projection = ctx.fixture_json(&uri)?;
        Ok(Outcome::with_raw(raw, projection))
    }
}
//#endregion 🔖️Oracle

//#region 🔖️Subject
#[cfg(feature = "sut")]
mod subject {
    use super::fixture_uri;
    use protocol::Mutation;
    use semio_repo_test_host::{Context, Json, Outcome};
    use semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::any::schema::geometry::SemioPoint3;
    use semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::brep::schema::mutations::{
        apply_semio_brep_mutation, create_edge, create_face, create_shell, create_solid, create_vertex, delete_edge, delete_face, delete_shell, delete_solid, delete_vertex, move_vertex, replace_curve, replace_surface, SemioBrepMutation,
    };
    use semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::{
        BrepCurve, BrepEdge, BrepFace, BrepLoop, BrepLoopEdge, BrepShell, BrepShellFace, BrepSolid, BrepSolidShell, BrepSurface, BrepVertex, SemioBrepSnapshot,
    };

    //#region 🔖️Decode
    /// 🔎️ Strict accessors over the framework's own dependency-free `Json`. Every one of them
    /// fails loudly rather than defaulting, so a fixture that stops carrying a field this subset's
    /// schema requires surfaces as an error instead of a silently wrong value.
    fn field<'j>(value: &'j Json, key: &str) -> Result<&'j Json, String> {
        value.get(key).ok_or_else(|| format!("fixture object is missing the {key:?} field"))
    }
    fn as_str(value: &Json) -> Result<String, String> {
        match value {
            Json::String(text) => Ok(text.clone()),
            other => Err(format!("expected a JSON string, got {other:?}")),
        }
    }
    fn as_f64(value: &Json) -> Result<f64, String> {
        match value {
            Json::Number(number) => Ok(*number),
            other => Err(format!("expected a JSON number, got {other:?}")),
        }
    }
    fn as_u32(value: &Json) -> Result<u32, String> {
        Ok(as_f64(value)? as u32)
    }
    fn as_bool(value: &Json) -> Result<bool, String> {
        match value {
            Json::Bool(flag) => Ok(*flag),
            other => Err(format!("expected a JSON bool, got {other:?}")),
        }
    }
    fn as_array(value: &Json) -> Result<&Vec<Json>, String> {
        match value {
            Json::Array(items) => Ok(items),
            other => Err(format!("expected a JSON array, got {other:?}")),
        }
    }
    /// 🔎️ A `#[serde(default)]` collection: absent means empty, present must still be an array.
    fn optional_list<T>(value: &Json, key: &str, decode: impl Fn(&Json) -> Result<T, String>) -> Result<Vec<T>, String> {
        match value.get(key) {
            None | Some(Json::Null) => Ok(Vec::new()),
            Some(present) => as_array(present)?.iter().map(decode).collect(),
        }
    }
    fn list<T>(value: &Json, key: &str, decode: impl Fn(&Json) -> Result<T, String>) -> Result<Vec<T>, String> {
        as_array(field(value, key)?)?.iter().map(decode).collect()
    }

    fn point_of(value: &Json) -> Result<SemioPoint3, String> {
        Ok(SemioPoint3 { x: as_f64(field(value, "x")?)?, y: as_f64(field(value, "y")?)?, z: as_f64(field(value, "z")?)? })
    }
    /// 📈️ `BrepCurve`'s `#[serde(tag = "kind", rename_all = "camelCase")]` wire shape.
    fn curve_of(value: &Json) -> Result<BrepCurve, String> {
        match as_str(field(value, "kind")?)?.as_str() {
            "line" => Ok(BrepCurve::Line { origin: point_of(field(value, "origin")?)?, direction: point_of(field(value, "direction")?)? }),
            "circle" => Ok(BrepCurve::Circle { center: point_of(field(value, "center")?)?, axis: point_of(field(value, "axis")?)?, radius: as_f64(field(value, "radius")?)? }),
            "ellipse" => Ok(BrepCurve::Ellipse {
                center: point_of(field(value, "center")?)?,
                axis: point_of(field(value, "axis")?)?,
                radius_major: as_f64(field(value, "radiusMajor")?)?,
                radius_minor: as_f64(field(value, "radiusMinor")?)?,
            }),
            "nurbs" => Ok(BrepCurve::Nurbs {
                control_points: list(value, "controlPoints", point_of)?,
                weights: list(value, "weights", as_f64)?,
                degree: as_u32(field(value, "degree")?)?,
                knots: list(value, "knots", as_f64)?,
            }),
            other => Err(format!("curve: unknown kind {other:?}")),
        }
    }
    /// 🗺️ `BrepSurface`'s `#[serde(tag = "kind", rename_all = "camelCase")]` wire shape.
    fn surface_of(value: &Json) -> Result<BrepSurface, String> {
        match as_str(field(value, "kind")?)?.as_str() {
            "plane" => Ok(BrepSurface::Plane { origin: point_of(field(value, "origin")?)?, normal: point_of(field(value, "normal")?)? }),
            "cylinder" => Ok(BrepSurface::Cylinder { origin: point_of(field(value, "origin")?)?, axis: point_of(field(value, "axis")?)?, radius: as_f64(field(value, "radius")?)? }),
            "cone" => Ok(BrepSurface::Cone {
                origin: point_of(field(value, "origin")?)?,
                axis: point_of(field(value, "axis")?)?,
                radius: as_f64(field(value, "radius")?)?,
                half_angle: as_f64(field(value, "halfAngle")?)?,
            }),
            "sphere" => Ok(BrepSurface::Sphere { center: point_of(field(value, "center")?)?, radius: as_f64(field(value, "radius")?)? }),
            "torus" => Ok(BrepSurface::Torus {
                center: point_of(field(value, "center")?)?,
                axis: point_of(field(value, "axis")?)?,
                major_radius: as_f64(field(value, "majorRadius")?)?,
                minor_radius: as_f64(field(value, "minorRadius")?)?,
            }),
            "nurbs" => Ok(BrepSurface::Nurbs {
                control_points: list(value, "controlPoints", point_of)?,
                weights: list(value, "weights", as_f64)?,
                u_count: as_u32(field(value, "uCount")?)?,
                v_count: as_u32(field(value, "vCount")?)?,
                degree_u: as_u32(field(value, "degreeU")?)?,
                degree_v: as_u32(field(value, "degreeV")?)?,
                knots_u: list(value, "knotsU", as_f64)?,
                knots_v: list(value, "knotsV", as_f64)?,
            }),
            other => Err(format!("surface: unknown kind {other:?}")),
        }
    }
    fn vertex_of(value: &Json) -> Result<BrepVertex, String> {
        Ok(BrepVertex { id: as_str(field(value, "id")?)?, point: point_of(field(value, "point")?)? })
    }
    fn edge_of(value: &Json) -> Result<BrepEdge, String> {
        Ok(BrepEdge { id: as_str(field(value, "id")?)?, start_vertex: as_str(field(value, "startVertex")?)?, end_vertex: as_str(field(value, "endVertex")?)?, curve: curve_of(field(value, "curve")?)? })
    }
    fn loop_edge_of(value: &Json) -> Result<BrepLoopEdge, String> {
        Ok(BrepLoopEdge { edge: as_str(field(value, "edge")?)?, orientation: as_bool(field(value, "orientation")?)? })
    }
    fn loop_of(value: &Json) -> Result<BrepLoop, String> {
        Ok(BrepLoop { id: as_str(field(value, "id")?)?, edges: optional_list(value, "edges", loop_edge_of)? })
    }
    fn face_of(value: &Json) -> Result<BrepFace, String> {
        Ok(BrepFace {
            id: as_str(field(value, "id")?)?,
            outer_loop: as_str(field(value, "outerLoop")?)?,
            inner_loops: optional_list(value, "innerLoops", as_str)?,
            surface: surface_of(field(value, "surface")?)?,
            orientation: as_bool(field(value, "orientation")?)?,
        })
    }
    fn shell_face_of(value: &Json) -> Result<BrepShellFace, String> {
        Ok(BrepShellFace { face: as_str(field(value, "face")?)?, orientation: as_bool(field(value, "orientation")?)? })
    }
    fn shell_of(value: &Json) -> Result<BrepShell, String> {
        Ok(BrepShell { id: as_str(field(value, "id")?)?, faces: optional_list(value, "faces", shell_face_of)? })
    }
    fn solid_shell_of(value: &Json) -> Result<BrepSolidShell, String> {
        Ok(BrepSolidShell { shell: as_str(field(value, "shell")?)?, is_void: as_bool(field(value, "isVoid")?)? })
    }
    fn solid_of(value: &Json) -> Result<BrepSolid, String> {
        Ok(BrepSolid { id: as_str(field(value, "id")?)?, shells: optional_list(value, "shells", solid_shell_of)? })
    }
    /// 📸️ `SemioBrepSnapshot`'s `#[serde(rename_all = "camelCase")]` wire shape — the six id-keyed
    /// collections all carry `#[serde(default)]`, so an absent collection decodes as empty.
    fn snapshot_of(value: &Json) -> Result<SemioBrepSnapshot, String> {
        Ok(SemioBrepSnapshot {
            schema: as_str(field(value, "schema")?)?,
            vertices: optional_list(value, "vertices", vertex_of)?,
            edges: optional_list(value, "edges", edge_of)?,
            loops: optional_list(value, "loops", loop_of)?,
            faces: optional_list(value, "faces", face_of)?,
            shells: optional_list(value, "shells", shell_of)?,
            solids: optional_list(value, "solids", solid_of)?,
        })
    }
    /// 🦠️ `SemioBrepMutation`'s externally tagged wire shape: one `{"VariantName": {payload}}`
    /// member, whose payload struct fields are plain snake_case (the payload structs carry no
    /// `rename_all`), while nested snapshot value types keep their own camelCase spelling.
    fn mutation_of(value: &Json) -> Result<SemioBrepMutation, String> {
        let (variant, payload) = match value {
            Json::Object(entries) => entries.first().ok_or_else(|| "mutation fixture is an empty object".to_string())?,
            other => return Err(format!("mutation fixture must be an object, got {other:?}")),
        };
        match variant.as_str() {
            "CreateVertex" => Ok(SemioBrepMutation::CreateVertex(create_vertex::mutation::CreateVertex { id: as_str(field(payload, "id")?)?, point: point_of(field(payload, "point")?)? })),
            "DeleteVertex" => Ok(SemioBrepMutation::DeleteVertex(delete_vertex::mutation::DeleteVertex { id: as_str(field(payload, "id")?)? })),
            "CreateEdge" => Ok(SemioBrepMutation::CreateEdge(create_edge::mutation::CreateEdge {
                id: as_str(field(payload, "id")?)?,
                start_vertex: as_str(field(payload, "start_vertex")?)?,
                end_vertex: as_str(field(payload, "end_vertex")?)?,
                curve: curve_of(field(payload, "curve")?)?,
            })),
            "DeleteEdge" => Ok(SemioBrepMutation::DeleteEdge(delete_edge::mutation::DeleteEdge { id: as_str(field(payload, "id")?)? })),
            "CreateFace" => Ok(SemioBrepMutation::CreateFace(create_face::mutation::CreateFace {
                id: as_str(field(payload, "id")?)?,
                outer_loop: as_str(field(payload, "outer_loop")?)?,
                inner_loops: optional_list(payload, "inner_loops", as_str)?,
                surface: surface_of(field(payload, "surface")?)?,
                orientation: as_bool(field(payload, "orientation")?)?,
            })),
            "DeleteFace" => Ok(SemioBrepMutation::DeleteFace(delete_face::mutation::DeleteFace { id: as_str(field(payload, "id")?)? })),
            "CreateShell" => Ok(SemioBrepMutation::CreateShell(create_shell::mutation::CreateShell { id: as_str(field(payload, "id")?)?, faces: optional_list(payload, "faces", shell_face_of)? })),
            "DeleteShell" => Ok(SemioBrepMutation::DeleteShell(delete_shell::mutation::DeleteShell { id: as_str(field(payload, "id")?)? })),
            "CreateSolid" => Ok(SemioBrepMutation::CreateSolid(create_solid::mutation::CreateSolid { id: as_str(field(payload, "id")?)?, shells: optional_list(payload, "shells", solid_shell_of)? })),
            "DeleteSolid" => Ok(SemioBrepMutation::DeleteSolid(delete_solid::mutation::DeleteSolid { id: as_str(field(payload, "id")?)? })),
            "ReplaceCurve" => Ok(SemioBrepMutation::ReplaceCurve(replace_curve::mutation::ReplaceCurve { edge_id: as_str(field(payload, "edge_id")?)?, new_curve: curve_of(field(payload, "new_curve")?)? })),
            "ReplaceSurface" => Ok(SemioBrepMutation::ReplaceSurface(replace_surface::mutation::ReplaceSurface { face_id: as_str(field(payload, "face_id")?)?, new_surface: surface_of(field(payload, "new_surface")?)? })),
            "MoveVertex" => Ok(SemioBrepMutation::MoveVertex(move_vertex::mutation::MoveVertex { vertex_id: as_str(field(payload, "vertex_id")?)?, new_point: point_of(field(payload, "new_point")?)? })),
            other => Err(format!("mutation: unknown variant {other:?}")),
        }
    }

    /// 🧫️ The committed `(before, mutation)` pair the scenario binds, decoded into the typed values
    /// `apply_semio_brep_mutation` consumes.
    fn fixture_for(ctx: &Context) -> Result<(SemioBrepSnapshot, SemioBrepMutation), String> {
        let before = snapshot_of(&ctx.fixture_json(&fixture_uri(ctx, "before")?)?)?;
        let mutation = mutation_of(&ctx.fixture_json(&fixture_uri(ctx, "mutation")?)?)?;
        Ok((before, mutation))
    }
    //#endregion 🔖️Decode

    //#region 🔖️Projection
    fn point_json(p: &SemioPoint3) -> Json {
        Json::Object(vec![("x".to_string(), Json::Number(p.x)), ("y".to_string(), Json::Number(p.y)), ("z".to_string(), Json::Number(p.z))])
    }
    fn curve_json(c: &BrepCurve) -> Json {
        match c {
            BrepCurve::Line { origin, direction } => Json::Object(vec![("kind".to_string(), Json::String("line".to_string())), ("origin".to_string(), point_json(origin)), ("direction".to_string(), point_json(direction))]),
            BrepCurve::Circle { center, axis, radius } => Json::Object(vec![
                ("kind".to_string(), Json::String("circle".to_string())),
                ("center".to_string(), point_json(center)),
                ("axis".to_string(), point_json(axis)),
                ("radius".to_string(), Json::Number(*radius)),
            ]),
            BrepCurve::Ellipse { center, axis, radius_major, radius_minor } => Json::Object(vec![
                ("kind".to_string(), Json::String("ellipse".to_string())),
                ("center".to_string(), point_json(center)),
                ("axis".to_string(), point_json(axis)),
                ("radiusMajor".to_string(), Json::Number(*radius_major)),
                ("radiusMinor".to_string(), Json::Number(*radius_minor)),
            ]),
            BrepCurve::Nurbs { control_points, weights, degree, knots } => Json::Object(vec![
                ("kind".to_string(), Json::String("nurbs".to_string())),
                ("controlPoints".to_string(), Json::Array(control_points.iter().map(point_json).collect())),
                ("weights".to_string(), Json::Array(weights.iter().map(|w| Json::Number(*w)).collect())),
                ("degree".to_string(), Json::Number(*degree as f64)),
                ("knots".to_string(), Json::Array(knots.iter().map(|k| Json::Number(*k)).collect())),
            ]),
        }
    }
    fn surface_json(s: &BrepSurface) -> Json {
        match s {
            BrepSurface::Plane { origin, normal } => Json::Object(vec![("kind".to_string(), Json::String("plane".to_string())), ("origin".to_string(), point_json(origin)), ("normal".to_string(), point_json(normal))]),
            BrepSurface::Cylinder { origin, axis, radius } => Json::Object(vec![
                ("kind".to_string(), Json::String("cylinder".to_string())),
                ("origin".to_string(), point_json(origin)),
                ("axis".to_string(), point_json(axis)),
                ("radius".to_string(), Json::Number(*radius)),
            ]),
            BrepSurface::Cone { origin, axis, radius, half_angle } => Json::Object(vec![
                ("kind".to_string(), Json::String("cone".to_string())),
                ("origin".to_string(), point_json(origin)),
                ("axis".to_string(), point_json(axis)),
                ("radius".to_string(), Json::Number(*radius)),
                ("halfAngle".to_string(), Json::Number(*half_angle)),
            ]),
            BrepSurface::Sphere { center, radius } => Json::Object(vec![("kind".to_string(), Json::String("sphere".to_string())), ("center".to_string(), point_json(center)), ("radius".to_string(), Json::Number(*radius))]),
            BrepSurface::Torus { center, axis, major_radius, minor_radius } => Json::Object(vec![
                ("kind".to_string(), Json::String("torus".to_string())),
                ("center".to_string(), point_json(center)),
                ("axis".to_string(), point_json(axis)),
                ("majorRadius".to_string(), Json::Number(*major_radius)),
                ("minorRadius".to_string(), Json::Number(*minor_radius)),
            ]),
            BrepSurface::Nurbs { control_points, weights, u_count, v_count, degree_u, degree_v, knots_u, knots_v } => Json::Object(vec![
                ("kind".to_string(), Json::String("nurbs".to_string())),
                ("controlPoints".to_string(), Json::Array(control_points.iter().map(point_json).collect())),
                ("weights".to_string(), Json::Array(weights.iter().map(|w| Json::Number(*w)).collect())),
                ("uCount".to_string(), Json::Number(*u_count as f64)),
                ("vCount".to_string(), Json::Number(*v_count as f64)),
                ("degreeU".to_string(), Json::Number(*degree_u as f64)),
                ("degreeV".to_string(), Json::Number(*degree_v as f64)),
                ("knotsU".to_string(), Json::Array(knots_u.iter().map(|k| Json::Number(*k)).collect())),
                ("knotsV".to_string(), Json::Array(knots_v.iter().map(|k| Json::Number(*k)).collect())),
            ]),
        }
    }
    fn vertex_json(v: &BrepVertex) -> Json {
        Json::Object(vec![("id".to_string(), Json::String(v.id.clone())), ("point".to_string(), point_json(&v.point))])
    }
    fn edge_json(e: &BrepEdge) -> Json {
        Json::Object(vec![
            ("id".to_string(), Json::String(e.id.clone())),
            ("startVertex".to_string(), Json::String(e.start_vertex.clone())),
            ("endVertex".to_string(), Json::String(e.end_vertex.clone())),
            ("curve".to_string(), curve_json(&e.curve)),
        ])
    }
    fn loop_edge_json(le: &BrepLoopEdge) -> Json {
        Json::Object(vec![("edge".to_string(), Json::String(le.edge.clone())), ("orientation".to_string(), Json::Bool(le.orientation))])
    }
    fn loop_json(l: &BrepLoop) -> Json {
        Json::Object(vec![("id".to_string(), Json::String(l.id.clone())), ("edges".to_string(), Json::Array(l.edges.iter().map(loop_edge_json).collect()))])
    }
    fn face_json(f: &BrepFace) -> Json {
        Json::Object(vec![
            ("id".to_string(), Json::String(f.id.clone())),
            ("outerLoop".to_string(), Json::String(f.outer_loop.clone())),
            ("innerLoops".to_string(), Json::Array(f.inner_loops.iter().map(|s| Json::String(s.clone())).collect())),
            ("surface".to_string(), surface_json(&f.surface)),
            ("orientation".to_string(), Json::Bool(f.orientation)),
        ])
    }
    fn shell_face_json(sf: &BrepShellFace) -> Json {
        Json::Object(vec![("face".to_string(), Json::String(sf.face.clone())), ("orientation".to_string(), Json::Bool(sf.orientation))])
    }
    fn shell_json(sh: &BrepShell) -> Json {
        Json::Object(vec![("id".to_string(), Json::String(sh.id.clone())), ("faces".to_string(), Json::Array(sh.faces.iter().map(shell_face_json).collect()))])
    }
    fn solid_shell_json(ss: &BrepSolidShell) -> Json {
        Json::Object(vec![("shell".to_string(), Json::String(ss.shell.clone())), ("isVoid".to_string(), Json::Bool(ss.is_void))])
    }
    fn solid_json(so: &BrepSolid) -> Json {
        Json::Object(vec![("id".to_string(), Json::String(so.id.clone())), ("shells".to_string(), Json::Array(so.shells.iter().map(solid_shell_json).collect()))])
    }
    /// 🎯️ The projection every scenario compares under `ordered-json-v1`: the snapshot's own
    /// structural JSON shape, matching the committed fixtures field for field (camelCase keys,
    /// matching `SemioBrepSnapshot`'s own `#[serde(rename_all = "camelCase")]`). This is the exact
    /// inverse of the `Decode` region above, so a fixture that round-trips through both is proof the
    /// two agree.
    fn snapshot_json(snapshot: &SemioBrepSnapshot) -> Json {
        Json::Object(vec![
            ("schema".to_string(), Json::String(snapshot.schema.clone())),
            ("vertices".to_string(), Json::Array(snapshot.vertices.iter().map(vertex_json).collect())),
            ("edges".to_string(), Json::Array(snapshot.edges.iter().map(edge_json).collect())),
            ("loops".to_string(), Json::Array(snapshot.loops.iter().map(loop_json).collect())),
            ("faces".to_string(), Json::Array(snapshot.faces.iter().map(face_json).collect())),
            ("shells".to_string(), Json::Array(snapshot.shells.iter().map(shell_json).collect())),
            ("solids".to_string(), Json::Array(snapshot.solids.iter().map(solid_json).collect())),
        ])
    }
    //#endregion 🔖️Projection

    //#region 🔖️Handlers
    pub fn mutate(ctx: &Context) -> Result<Outcome, String> {
        let (mut current, mutation) = fixture_for(ctx)?;
        let outcome = apply_semio_brep_mutation(&mut current, &mutation);
        if !outcome.messages().is_empty() {
            return Err(format!("{}: mutation rejected: {:?}", ctx.scenario.id, outcome.messages()));
        }
        let projection = snapshot_json(&current);
        let bytes = projection.to_string().into_bytes();
        Ok(Outcome::with_raw(bytes, projection))
    }

    pub fn inverse(ctx: &Context) -> Result<Outcome, String> {
        let (base, mutation) = fixture_for(ctx)?;
        let mut current = base.clone();
        let outcome = apply_semio_brep_mutation(&mut current, &mutation);
        if !outcome.messages().is_empty() {
            return Err(format!("{}: forward mutation rejected: {:?}", ctx.scenario.id, outcome.messages()));
        }
        for step in &mutation.inverse(&base) {
            let step_outcome = apply_semio_brep_mutation(&mut current, step);
            if !step_outcome.messages().is_empty() {
                return Err(format!("{}: inverse step rejected: {:?}", ctx.scenario.id, step_outcome.messages()));
            }
        }
        let projection = snapshot_json(&current);
        let bytes = projection.to_string().into_bytes();
        Ok(Outcome::with_raw(bytes, projection))
    }
    //#endregion 🔖️Handlers
}
//#endregion 🔖️Subject

//#region 🔖️Registration
/// 🧭️ Registration entry point the generated host calls.
pub fn adapter() -> Adapter {
    let mut built = Adapter::new("rust");
    for kind in KINDS {
        built = built.oracle(&format!("mutate-{kind}"), snapshot_oracle_for("after")).oracle(&format!("inverse-{kind}"), snapshot_oracle_for("before"));
        #[cfg(feature = "sut")]
        {
            built = built.subject(&format!("mutate-{kind}"), subject::mutate).subject(&format!("inverse-{kind}"), subject::inverse);
        }
    }
    built
}
//#endregion 🔖️Registration
