//! 🦀️ DXF R12 exhaustive mutation case — Rust adapter. Ticket 26/08/23/END-TO-END-TESTING-REFACTOR
//! wave 7.
//!
//! Every scenario copies the derived, committed `🚏️bus-shelter` fixture into the case work
//! directory first; the committed asset is never written to. `oracle` drives the registered `dxf`
//! 0.6 reference implementation (`../../🏅️standards/🔖️r12/🪆️subsets/📰️header/🔮️oracle/🦀️.rs`'s
//! own `oracle_apply_mutation`/`oracle_apply_mutation_inverse`); `subject` drives this repository's
//! own `parse_dxf_document`/`print_dxf_document`/`apply_dxf_mutation` over the full 19-kind
//! `DxfMutation` vocabulary. Both results are read back by the SAME independent `project_dxf_r12`
//! (`dxf` 0.6) before the `semantic-dxf-r12-v1` profile compares them. The subject half is gated
//! behind the generated host's `sut` feature so the oracle-only run never compiles the local
//! implementation -- §5.3's own role separation, NOT a workaround for anything: the Rust subject
//! phase runs, and wave 14 ran the full differential comparison against the oracle.

use semio_repo_test_host::{Adapter, Context, Json, Outcome};
use semio_s_plugin_stdio_test_oracle::artifacts::dxf::standards::v_r12::subsets::any::{oracle_apply_mutation, oracle_apply_mutation_inverse, project_dxf_r12};

//#region 🔖️Kinds
/// 📇️ Kebab-case spelling of every `DxfMutation` variant, mirrored from
/// `../../🏅️standards/🔖️r12/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️.rs`'s own `KINDS` --
/// duplicated rather than imported because the ORACLE-only build of this adapter must never link
/// `semio-s-plugin-stdio` (see this file's own header); `kinds_const_matches_enum_variants_in_
/// declaration_order` on the production side and the framework's own catalog-completeness gate on
/// this side are what keep the two lists honest against each other.
const KINDS: &[&str] = &[
    "no-mutation",
    "set-snapshot",
    "set-header-var",
    "remove-header-var",
    "insert-layer",
    "remove-layer",
    "set-layer",
    "insert-style",
    "remove-style",
    "set-style",
    "insert-linetype",
    "remove-linetype",
    "set-linetype",
    "insert-entity",
    "remove-entity",
    "set-entity",
    "insert-block",
    "remove-block",
    "set-block",
];
//#endregion 🔖️Kinds

//#region 🔖️Input
const INPUT: &str = "asset://🏅️standards/🔖️r12/🪆️subsets/✳️any/📚️examples/🚏️bus-shelter/🖼️assets/🖊️.dxf";

/// 🧫️ Copies the immutable real asset into the work directory and returns the mutable copy's bytes.
fn mutable_input(ctx: &Context) -> Result<Vec<u8>, String> {
    let copy = ctx.copy_fixture(INPUT, Some("bus-shelter-r12.dxf"))?;
    std::fs::read(&copy).map_err(|error| error.to_string())
}
//#endregion 🔖️Input

//#region 🔖️Laws
/// 🔍️ First point at which two projections disagree, as a `path: expected != read` sentence -- a law
/// violation must name the field that broke it rather than dump two whole documents at the reader.
fn first_divergence(path: &str, expected: &Json, actual: &Json) -> Option<String> {
    match (expected, actual) {
        (Json::Object(left), Json::Object(right)) => {
            for (key, value) in left {
                match right.iter().find(|(name, _)| name == key) {
                    Some((_, other)) => {
                        if let Some(found) = first_divergence(&format!("{path}.{key}"), value, other) {
                            return Some(found);
                        }
                    }
                    None => return Some(format!("{path}.{key} is absent from the result")),
                }
            }
            right.iter().find(|(key, _)| !left.iter().any(|(name, _)| name == key)).map(|(key, _)| format!("{path}.{key} appeared in the result out of nowhere"))
        }
        (Json::Array(left), Json::Array(right)) => {
            if left.len() != right.len() {
                return Some(format!("{path} holds {} member(s), expected {}", right.len(), left.len()));
            }
            left.iter().zip(right.iter()).enumerate().find_map(|(index, (value, other))| first_divergence(&format!("{path}[{index}]"), value, other))
        }
        _ if expected == actual => None,
        _ => Some(format!("{path}: expected {} but read {}", expected.to_string(), actual.to_string())),
    }
}

/// ⚖️ Turns a projection law into a real verdict: `Ok` only when the two projections agree, otherwise
/// an `Err` naming the FIRST field that diverged. Without this an oracle handler asserts nothing and
/// its scenario passes whenever the reference library merely declined to error.
fn assert_same_projection(law: &str, expected: &Json, actual: &Json) -> Result<(), String> {
    match first_divergence("projection", expected, actual) {
        Some(divergence) => Err(format!("{law}: {divergence}")),
        None => Ok(()),
    }
}
//#endregion 🔖️Laws

//#region 🔖️Oracle
/// 🧾️ The `no-mutation` spec, spelled once -- both laws below need a baseline that has been through
/// exactly the same number of `dxf` load/save cycles as the document they judge, so that a
/// divergence names the mutation pair and never the reference writer's own normal form.
fn no_mutation() -> Json {
    Json::Object(vec![("kind".to_string(), Json::String("no-mutation".to_string())), ("params".to_string(), Json::Object(vec![]))])
}

/// 🔮️ One handler shared by every `mutate-<kind>` scenario id -- the scenario's own `<id>`/`<params>`
/// spec is carried in its doc string, not in the function it dispatches to. It asserts ONE thing in
/// role, before any parity comparison exists: every kind other than `no-mutation` must MOVE the
/// semantic projection. A row whose parameters make the mutation a no-op is not a test -- it passes
/// whenever the reference library declined to error, which is exactly the failure this platform
/// exists to prevent. The baseline runs one `no-mutation` cycle so the comparison isolates the
/// mutation rather than the writer's own normal form.
fn mutate_oracle(ctx: &Context) -> Result<Outcome, String> {
    let input = mutable_input(ctx)?;
    let spec = ctx.doc_json()?;
    let kind = spec.str("kind");
    let baseline = project_dxf_r12(&oracle_apply_mutation(&input, &no_mutation())?)?;
    let bytes = oracle_apply_mutation(&input, &spec)?;
    let projection = project_dxf_r12(&bytes)?;
    if kind != "no-mutation" && projection == baseline {
        return Err(format!("{kind:?} left the semantic projection of the R12 drawing unchanged -- a mutation that is not observable proves nothing, so this row's parameters do not exercise the kind they name"));
    }
    Ok(Outcome::with_raw(bytes, projection))
}

/// ↩️ One handler shared by every `inverse-<kind>` scenario id, and the ORACLE side of the inverse
/// law -- a law that is checkable in-role, without a subject: `dxf` applies the forward mutation and
/// then its own base-relative inverse (`oracle_apply_mutation_inverse`, one load/save cycle), and the
/// restored drawing MUST project exactly as the untouched drawing does. The baseline is taken
/// through one `no-mutation` cycle so the two sides carry the same serializer normalisation and the
/// comparison isolates the mutation pair itself.
fn inverse_oracle(ctx: &Context) -> Result<Outcome, String> {
    let input = mutable_input(ctx)?;
    let spec = ctx.doc_json()?;
    let baseline = project_dxf_r12(&oracle_apply_mutation(&input, &no_mutation())?)?;
    let bytes = oracle_apply_mutation_inverse(&input, &spec)?;
    let projection = project_dxf_r12(&bytes)?;
    assert_same_projection(&format!("inverse law violated for {:?} -- undoing it did not restore the drawing", spec.str("kind")), &baseline, &projection)?;
    Ok(Outcome::with_raw(bytes, projection))
}

/// 🔒️ The ORACLE side of the identity round trip, asserted in-role: `dxf` fully parses the real
/// document and re-serializes it from its own typed `Drawing` alone, so the re-encoded bytes MUST
/// carry the same semantic projection as the input AND MUST NOT be bit-identical to it. DXF R12 is
/// not a byte-preserving carrier -- `dxf` regenerates the whole group-code stream from its model --
/// so the byte tripwire is real evidence that the document was parsed rather than copied.
fn identity_round_trip_oracle(ctx: &Context) -> Result<Outcome, String> {
    let input = mutable_input(ctx)?;
    let before = project_dxf_r12(&input)?;
    let bytes = oracle_apply_mutation(&input, &no_mutation())?;
    if bytes == input {
        return Err("byte pass-through: the re-encoded output is bit-identical to the input, so nothing here proves the document was parsed".to_string());
    }
    let projection = project_dxf_r12(&bytes)?;
    assert_same_projection("identity round trip is not semantics-preserving", &before, &projection)?;
    Ok(Outcome::with_raw(bytes, projection))
}
//#endregion 🔖️Oracle

//#region 🔖️Subject
#[cfg(feature = "sut")]
mod subject {
    use super::{mutable_input, KINDS};
    use semio_repo_test_host::{Context, Json, Outcome};
    use semio_s_plugin_stdio::artifacts::dxf::standards::v_r12::subsets::any::schema::mutations::{
        apply_dxf_mutation, insert_block, insert_entity, insert_layer, insert_linetype, insert_style, remove_block, remove_entity, remove_header_var, remove_layer, remove_linetype, remove_style, set_block, set_entity, set_header_var, set_layer, set_linetype, set_snapshot, set_style, DxfMutation,
    };
    use semio_s_plugin_stdio::artifacts::dxf::standards::v_r12::subsets::any::schema::snapshot::{parse_dxf_document, print_dxf_document, DxfBlock, DxfEntity, DxfHeaderVar, DxfLayer, DxfLinetype, DxfStyle, DxfValue};
    use semio_s_plugin_stdio::artifacts::dxf::DxfSnapshot;
    use semio_s_plugin_stdio_test_oracle::artifacts::dxf::standards::v_r12::subsets::any::project_dxf_r12;

    //#region 🔖️SpecCodec
    fn number(v: &Json, key: &str) -> f64 {
        match v.get(key) {
            Some(Json::Number(n)) => *n,
            _ => 0.0,
        }
    }
    fn usize_field(v: &Json, key: &str) -> usize {
        number(v, key).max(0.0) as usize
    }
    fn point3(v: &Json, key: &str) -> [f64; 3] {
        point_of(v.get(key))
    }
    fn point_of(v: Option<&Json>) -> [f64; 3] {
        match v {
            Some(Json::Array(items)) => {
                let at = |i: usize| match items.get(i) {
                    Some(Json::Number(n)) => *n,
                    _ => 0.0,
                };
                [at(0), at(1), at(2)]
            }
            _ => [0.0, 0.0, 0.0],
        }
    }

    /// 🔎️ JSON → `DxfEntity`, over the six typed kinds this subset itself models -- the SAME
    /// `entityKind`/field-name grammar the oracle's own `build_entity` speaks, so a scenario's one
    /// `params` doc string means the same thing on both sides.
    fn json_to_entity(spec: &Json) -> Result<DxfEntity, String> {
        let layer = spec.str("layer");
        match spec.str("entityKind").as_str() {
            "line" => Ok(DxfEntity::Line { start: point3(spec, "start"), end: point3(spec, "end"), layer, unknown_group_codes: vec![] }),
            "circle" => Ok(DxfEntity::Circle { center: point3(spec, "center"), radius: number(spec, "radius"), layer, unknown_group_codes: vec![] }),
            "arc" => Ok(DxfEntity::Arc { center: point3(spec, "center"), radius: number(spec, "radius"), start_angle: number(spec, "startAngle"), end_angle: number(spec, "endAngle"), layer, unknown_group_codes: vec![] }),
            "text" => Ok(DxfEntity::Text { position: point3(spec, "position"), height: number(spec, "height"), value: spec.str("value"), layer, unknown_group_codes: vec![] }),
            "solid" => {
                let points = spec.array("points");
                let corner = |i: usize| point_of(points.get(i));
                Ok(DxfEntity::Solid { points: [corner(0), corner(1), corner(2), corner(3)], layer, unknown_group_codes: vec![] })
            }
            "insert" => Ok(DxfEntity::Insert { block_name: spec.str("blockName"), position: point3(spec, "position"), scale: [1.0, 1.0, 1.0], rotation: 0.0, layer, unknown_group_codes: vec![] }),
            other => Err(format!("dxf subject: unsupported entityKind {other:?}")),
        }
    }

    fn json_to_layer(spec: &Json) -> DxfLayer {
        DxfLayer { name: spec.str("name"), color: number(spec, "color") as i32, linetype: spec.str("linetype"), flags: 0, unknown_group_codes: vec![] }
    }
    fn json_to_style(spec: &Json) -> DxfStyle {
        DxfStyle { name: spec.str("name"), flags: 0, font_name: spec.str("font"), unknown_group_codes: vec![] }
    }
    fn json_to_linetype(spec: &Json) -> DxfLinetype {
        DxfLinetype { name: spec.str("name"), flags: 0, description: spec.str("description"), unknown_group_codes: vec![] }
    }
    fn json_to_block(spec: &Json) -> Result<DxfBlock, String> {
        let entities = spec.array("entities").iter().map(json_to_entity).collect::<Result<Vec<_>, String>>()?;
        Ok(DxfBlock { name: spec.str("name"), base_point: point3(spec, "basePoint"), entities, unknown_group_codes: vec![] })
    }

    /// ✏️ Inserts or replaces the header var named `name` in `snapshot.header_vars`, in place --
    /// `$INSBASE` (the one generic `$VAR` this case's `set-header-var`/`remove-header-var`/
    /// `set-snapshot` rows target) uses group code 10 per the point-component convention this
    /// subset's own header codec reads (`📸️snapshot/🦀️.rs`'s `parse_header_var`).
    fn upsert_header_var(snapshot: &mut DxfSnapshot, name: &str, group_code: i32, value: DxfValue) {
        match snapshot.header_vars.iter_mut().find(|v| v.name == name) {
            Some(existing) => existing.value = value,
            None => snapshot.header_vars.push(DxfHeaderVar { name: name.to_string(), group_code, value, extra_group_codes: vec![] }),
        }
    }

    /// 📄️ The scenario's `<id>`/`<params>` spec turned into the ONE typed `DxfMutation` this subset
    /// declares for it.
    fn mutation_from_spec(spec: &Json, base: &DxfSnapshot) -> Result<DxfMutation, String> {
        let params = spec.get("params").cloned().unwrap_or(Json::Null);
        match spec.str("kind").as_str() {
            "no-mutation" => Ok(DxfMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot: base.clone() })),
            "set-snapshot" => {
                let mut snapshot = base.clone();
                if params.get("insertionBase").is_some() {
                    upsert_header_var(&mut snapshot, "$INSBASE", 10, DxfValue::Point { value: point3(&params, "insertionBase") });
                }
                if let Some(Json::Array(items)) = params.get("layers") {
                    snapshot.tables.layers = items.iter().map(json_to_layer).collect();
                }
                if let Some(Json::Array(items)) = params.get("entities") {
                    snapshot.entities = items.iter().map(json_to_entity).collect::<Result<Vec<_>, String>>()?;
                }
                Ok(DxfMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot }))
            }
            "set-header-var" => {
                let name = params.str("name");
                Ok(DxfMutation::SetHeaderVar(set_header_var::SetHeaderVar { name: name.clone(), header_var: DxfHeaderVar { name, group_code: 10, value: DxfValue::Point { value: point3(&params, "value") }, extra_group_codes: vec![] } }))
            }
            "remove-header-var" => Ok(DxfMutation::RemoveHeaderVar(remove_header_var::RemoveHeaderVar { name: params.str("name") })),
            "insert-layer" => Ok(DxfMutation::InsertLayer(insert_layer::InsertLayer { index: usize_field(&params, "index"), layer: json_to_layer(&params) })),
            "remove-layer" => Ok(DxfMutation::RemoveLayer(remove_layer::RemoveLayer { name: params.str("name") })),
            "set-layer" => Ok(DxfMutation::SetLayer(set_layer::SetLayer { name: params.str("name"), layer: json_to_layer(&params) })),
            "insert-style" => Ok(DxfMutation::InsertStyle(insert_style::InsertStyle { index: usize_field(&params, "index"), style: json_to_style(&params) })),
            "remove-style" => Ok(DxfMutation::RemoveStyle(remove_style::RemoveStyle { name: params.str("name") })),
            "set-style" => Ok(DxfMutation::SetStyle(set_style::SetStyle { name: params.str("name"), style: json_to_style(&params) })),
            "insert-linetype" => Ok(DxfMutation::InsertLinetype(insert_linetype::InsertLinetype { index: usize_field(&params, "index"), linetype: json_to_linetype(&params) })),
            "remove-linetype" => Ok(DxfMutation::RemoveLinetype(remove_linetype::RemoveLinetype { name: params.str("name") })),
            "set-linetype" => Ok(DxfMutation::SetLinetype(set_linetype::SetLinetype { name: params.str("name"), linetype: json_to_linetype(&params) })),
            "insert-entity" => Ok(DxfMutation::InsertEntity(insert_entity::InsertEntity { index: usize_field(&params, "index"), entity: json_to_entity(&params)? })),
            "remove-entity" => Ok(DxfMutation::RemoveEntity(remove_entity::RemoveEntity { index: usize_field(&params, "index") })),
            "set-entity" => Ok(DxfMutation::SetEntity(set_entity::SetEntity { index: usize_field(&params, "index"), entity: json_to_entity(&params)? })),
            "insert-block" => Ok(DxfMutation::InsertBlock(insert_block::InsertBlock { index: usize_field(&params, "index"), block: json_to_block(&params)? })),
            "remove-block" => Ok(DxfMutation::RemoveBlock(remove_block::RemoveBlock { index: usize_field(&params, "index") })),
            "set-block" => Ok(DxfMutation::SetBlock(set_block::SetBlock { index: usize_field(&params, "index"), block: json_to_block(&params)? })),
            other => Err(format!("mutation kind {other:?} has no subject implementation")),
        }
    }
    //#endregion 🔖️SpecCodec

    //#region 🔖️Inverse
    /// ↩️ `DxfMutation::inverse`'s own per-variant contract (`../../🏅️standards/🔖️r12/🪆️subsets/
    /// ✳️any/🧬️schema/🧬️mutations/🦀️.rs`'s `agg_inverse`), transplanted in closed form -- same
    /// rationale `🌴️mutate-pdf-1-7`'s own `inverse_of` gives: this adapter needs no extra crate
    /// dependency (the `protocol::Mutation` trait itself) beyond `semio-s-plugin-stdio`.
    fn inverse_of(mutation: &DxfMutation, base: &DxfSnapshot) -> DxfMutation {
        // 🧷️ Where the forward mutation had nothing to act on (e.g. removing an already-absent
        // name), the undo step is this identity mutation rather than a dropped `NoMutation`
        // sentinel — reapplying `base` verbatim restores exactly the same state a true no-op
        // would have, since the forward step did not change anything in these fallback branches.
        let identity = || DxfMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot: base.clone() });
        match mutation {
            DxfMutation::SetSnapshot(_) => DxfMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot: base.clone() }),

            DxfMutation::SetHeaderVar(set_header_var::SetHeaderVar { name, .. }) => match base.header_vars.iter().find(|v| &v.name == name) {
                Some(v) => DxfMutation::SetHeaderVar(set_header_var::SetHeaderVar { name: name.clone(), header_var: v.clone() }),
                None => DxfMutation::RemoveHeaderVar(remove_header_var::RemoveHeaderVar { name: name.clone() }),
            },
            DxfMutation::RemoveHeaderVar(remove_header_var::RemoveHeaderVar { name }) => match base.header_vars.iter().find(|v| &v.name == name) {
                Some(v) => DxfMutation::SetHeaderVar(set_header_var::SetHeaderVar { name: name.clone(), header_var: v.clone() }),
                None => identity(),
            },

            DxfMutation::InsertLayer(insert_layer::InsertLayer { layer, .. }) => DxfMutation::RemoveLayer(remove_layer::RemoveLayer { name: layer.name.clone() }),
            DxfMutation::RemoveLayer(remove_layer::RemoveLayer { name }) => match base.tables.layers.iter().find(|l| &l.name == name) {
                Some(l) => DxfMutation::InsertLayer(insert_layer::InsertLayer { index: base.tables.layers.iter().position(|x| &x.name == name).unwrap_or(base.tables.layers.len()), layer: l.clone() }),
                None => identity(),
            },
            DxfMutation::SetLayer(set_layer::SetLayer { name, .. }) => match base.tables.layers.iter().find(|l| &l.name == name) {
                Some(l) => DxfMutation::SetLayer(set_layer::SetLayer { name: name.clone(), layer: l.clone() }),
                None => DxfMutation::RemoveLayer(remove_layer::RemoveLayer { name: name.clone() }),
            },

            DxfMutation::InsertStyle(insert_style::InsertStyle { style, .. }) => DxfMutation::RemoveStyle(remove_style::RemoveStyle { name: style.name.clone() }),
            DxfMutation::RemoveStyle(remove_style::RemoveStyle { name }) => match base.tables.styles.iter().find(|s| &s.name == name) {
                Some(s) => DxfMutation::InsertStyle(insert_style::InsertStyle { index: base.tables.styles.iter().position(|x| &x.name == name).unwrap_or(base.tables.styles.len()), style: s.clone() }),
                None => identity(),
            },
            DxfMutation::SetStyle(set_style::SetStyle { name, .. }) => match base.tables.styles.iter().find(|s| &s.name == name) {
                Some(s) => DxfMutation::SetStyle(set_style::SetStyle { name: name.clone(), style: s.clone() }),
                None => DxfMutation::RemoveStyle(remove_style::RemoveStyle { name: name.clone() }),
            },

            DxfMutation::InsertLinetype(insert_linetype::InsertLinetype { linetype, .. }) => DxfMutation::RemoveLinetype(remove_linetype::RemoveLinetype { name: linetype.name.clone() }),
            DxfMutation::RemoveLinetype(remove_linetype::RemoveLinetype { name }) => match base.tables.linetypes.iter().find(|l| &l.name == name) {
                Some(l) => DxfMutation::InsertLinetype(insert_linetype::InsertLinetype { index: base.tables.linetypes.iter().position(|x| &x.name == name).unwrap_or(base.tables.linetypes.len()), linetype: l.clone() }),
                None => identity(),
            },
            DxfMutation::SetLinetype(set_linetype::SetLinetype { name, .. }) => match base.tables.linetypes.iter().find(|l| &l.name == name) {
                Some(l) => DxfMutation::SetLinetype(set_linetype::SetLinetype { name: name.clone(), linetype: l.clone() }),
                None => DxfMutation::RemoveLinetype(remove_linetype::RemoveLinetype { name: name.clone() }),
            },

            DxfMutation::InsertEntity(insert_entity::InsertEntity { index, .. }) => DxfMutation::RemoveEntity(remove_entity::RemoveEntity { index: *index }),
            DxfMutation::RemoveEntity(remove_entity::RemoveEntity { index }) => match base.entities.get(*index) {
                Some(e) => DxfMutation::InsertEntity(insert_entity::InsertEntity { index: *index, entity: e.clone() }),
                None => identity(),
            },
            DxfMutation::SetEntity(set_entity::SetEntity { index, .. }) => match base.entities.get(*index) {
                Some(e) => DxfMutation::SetEntity(set_entity::SetEntity { index: *index, entity: e.clone() }),
                None => identity(),
            },

            DxfMutation::InsertBlock(insert_block::InsertBlock { index, .. }) => DxfMutation::RemoveBlock(remove_block::RemoveBlock { index: *index }),
            DxfMutation::RemoveBlock(remove_block::RemoveBlock { index }) => match base.blocks.get(*index) {
                Some(b) => DxfMutation::InsertBlock(insert_block::InsertBlock { index: *index, block: b.clone() }),
                None => identity(),
            },
            DxfMutation::SetBlock(set_block::SetBlock { index, .. }) => match base.blocks.get(*index) {
                Some(b) => DxfMutation::SetBlock(set_block::SetBlock { index: *index, block: b.clone() }),
                None => identity(),
            },
        }
    }
    //#endregion 🔖️Inverse

    //#region 🔖️Handlers
    /// 🚫️ A REFUSED mutation is a failure, never a silent no-op. `apply_dxf_mutation` returns a
    /// `MutationOutcome` whose messages carry the refusal and whose snapshot is left untouched, so
    /// discarding that value turned every rejection into a scenario that re-encoded the input
    /// unchanged and reported green: ten of this case's kinds did exactly that, against a real
    /// drawing, until this wave, and only the differential comparison against `dxf` 0.6 showed it.
    fn applied(snapshot: &mut DxfSnapshot, mutation: &DxfMutation, kind: &str) -> Result<(), String> {
        // 📨️ Every `DxfMutation::diff` arm builds its outcome with `MutationOutcome::new`, which
        // carries NO message, and `apply_dxf_mutation` attaches one only when the apply itself was
        // rejected — so a non-empty message list here IS a refusal, and needs no severity type the
        // adapter would have to reach into the kernel crate for.
        let outcome = apply_dxf_mutation(snapshot, mutation);
        match outcome.messages().first() {
            Some(refusal) => Err(format!("{kind}: the mutation was REFUSED and the document left untouched — {refusal:?}")),
            None => Ok(()),
        }
    }

    pub fn mutate(ctx: &Context) -> Result<Outcome, String> {
        let input = mutable_input(ctx)?;
        let text = std::str::from_utf8(&input).map_err(|error| error.to_string())?;
        let base = parse_dxf_document(text)?;
        let spec = ctx.doc_json()?;
        let mutation = mutation_from_spec(&spec, &base)?;
        let mut snapshot = base;
        applied(&mut snapshot, &mutation, &spec.str("kind"))?;
        let output = print_dxf_document(&snapshot).into_bytes();
        let projection = project_dxf_r12(&output)?;
        Ok(Outcome::with_raw(output, projection))
    }

    pub fn inverse(ctx: &Context) -> Result<Outcome, String> {
        let input = mutable_input(ctx)?;
        let text = std::str::from_utf8(&input).map_err(|error| error.to_string())?;
        let base = parse_dxf_document(text)?;
        let spec = ctx.doc_json()?;
        let mutation = mutation_from_spec(&spec, &base)?;
        let undo = inverse_of(&mutation, &base);
        let mut snapshot = base;
        let kind = spec.str("kind");
        applied(&mut snapshot, &mutation, &kind)?;
        applied(&mut snapshot, &undo, &format!("the inverse of {kind}"))?;
        let output = print_dxf_document(&snapshot).into_bytes();
        let projection = project_dxf_r12(&output)?;
        Ok(Outcome::with_raw(output, projection))
    }

    /// 🔒️ The no-byte-pass-through rule: the subject must fully parse the real artifact into its
    /// typed snapshot and re-serialize from the model alone -- `parse_dxf_document`/
    /// `print_dxf_document` are this subset's ONLY channel from input to output. `print_dxf_document`
    /// regenerates a canonical NORMAL FORM (documented in `📸️snapshot/🦀️.rs`'s own module
    /// doc), never raw byte preservation, so the tripwire is real rather than incidental.
    pub fn identity_round_trip(ctx: &Context) -> Result<Outcome, String> {
        let input = mutable_input(ctx)?;
        let text = std::str::from_utf8(&input).map_err(|error| error.to_string())?;
        let snapshot = parse_dxf_document(text)?;
        let output = print_dxf_document(&snapshot).into_bytes();
        if output == input {
            return Err("byte pass-through: output is bit-identical to the input".to_string());
        }
        let projection = project_dxf_r12(&output)?;
        Ok(Outcome::with_raw(output, projection))
    }
    //#endregion 🔖️Handlers

    /// 🧭️ Re-exported so `super::adapter()` can register the same 19-kind sweep for the subject role
    /// without duplicating `KINDS` a third time.
    pub const SUBJECT_KINDS: &[&str] = KINDS;
}
//#endregion 🔖️Subject

//#region 🔖️Registration
/// 🧭️ Registration entry point the generated host calls. `mutate-<kind>`/`inverse-<kind>` share ONE
/// handler per role across all 19 kinds -- the scenario id only selects which fixture row's
/// `<id>`/`<params>` doc string the shared handler reads, per `Adapter::oracle`/`subject`'s own
/// per-scenario dispatch table.
pub fn adapter() -> Adapter {
    let mut built = Adapter::new("rust");
    for kind in KINDS {
        built = built.oracle(&format!("mutate-{kind}"), mutate_oracle).oracle(&format!("inverse-{kind}"), inverse_oracle);
    }
    built = built.oracle("identity-round-trip", identity_round_trip_oracle);
    #[cfg(feature = "sut")]
    {
        for kind in subject::SUBJECT_KINDS {
            built = built.subject(&format!("mutate-{kind}"), subject::mutate).subject(&format!("inverse-{kind}"), subject::inverse);
        }
        built = built.subject("identity-round-trip", subject::identity_round_trip);
    }
    built
}
//#endregion 🔖️Registration
