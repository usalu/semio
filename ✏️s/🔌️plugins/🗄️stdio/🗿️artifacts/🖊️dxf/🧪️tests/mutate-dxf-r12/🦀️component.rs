//! 🦀️ DXF R12 exhaustive mutation case — Rust adapter. Ticket 26/08/23/END-TO-END-TESTING-REFACTOR
//! wave 7.
//!
//! Every scenario copies the derived, committed `🚏️bus-shelter` fixture into the case work
//! directory first; the committed asset is never written to. `oracle` drives the registered `dxf`
//! 0.6 reference implementation (`../../🏅️standards/🔖️r12/🪆️subsets/✳️any/🧪️oracle/🦀️component.rs`'s
//! own `oracle_apply_mutation`/`oracle_apply_mutation_inverse`); `subject` drives this repository's
//! own `parse_dxf_document`/`print_dxf_document`/`apply_dxf_mutation` over the full 19-kind
//! `DxfMutation` vocabulary. Both results are read back by the SAME independent `project_dxf_r12`
//! (`dxf` 0.6) before the `semantic-dxf-r12-v1` profile compares them. The subject half is gated
//! behind the generated host's `sut` feature so the oracle-only run never compiles the local
//! implementation -- which cannot compile this wave regardless (a concurrent os-kernel refactor),
//! per the fleet brief's own note.

use semio_repo_test_host::{Adapter, Context, Json, Outcome};
use semio_s_plugin_stdio_test_oracle::artifacts::dxf::standards::v_r12::subsets::any::{oracle_apply_mutation, oracle_apply_mutation_inverse, project_dxf_r12};

//#region 🔖️Kinds
/// 📇️ Kebab-case spelling of every `DxfMutation` variant, mirrored from
/// `../../🏅️standards/🔖️r12/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs`'s own `KINDS` --
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
const INPUT: &str = "asset://🏅️standards/🔖️r12/🪆️subsets/✳️any/📚️examples/🚏️bus-shelter/🖼️assets/🖊️bus-shelter-r12.dxf";

/// 🧫️ Copies the immutable real asset into the work directory and returns the mutable copy's bytes.
fn mutable_input(ctx: &Context) -> Result<Vec<u8>, String> {
    let copy = ctx.copy_fixture(INPUT, Some("bus-shelter-r12.dxf"))?;
    std::fs::read(&copy).map_err(|error| error.to_string())
}
//#endregion 🔖️Input

//#region 🔖️Oracle
/// 🔮️ One handler shared by every `mutate-<kind>` scenario id -- the scenario's own `<id>`/`<params>`
/// spec is carried in its doc string, not in the function it dispatches to.
fn mutate_oracle(ctx: &Context) -> Result<Outcome, String> {
    let input = mutable_input(ctx)?;
    let spec = ctx.doc_json()?;
    let bytes = oracle_apply_mutation(&input, &spec)?;
    let projection = project_dxf_r12(&bytes)?;
    Ok(Outcome::with_raw(bytes, projection))
}

/// 🔮️ One handler shared by every `inverse-<kind>` scenario id.
fn inverse_oracle(ctx: &Context) -> Result<Outcome, String> {
    let input = mutable_input(ctx)?;
    let spec = ctx.doc_json()?;
    let bytes = oracle_apply_mutation_inverse(&input, &spec)?;
    let projection = project_dxf_r12(&bytes)?;
    Ok(Outcome::with_raw(bytes, projection))
}

/// 🔒️ The ORACLE side of the no-byte-pass-through law: `dxf` fully parses the real document and
/// re-serializes it from its own typed `Drawing` alone (the same "no-mutation" routing
/// `oracle_apply_mutation` already gives every other kind), independent evidence that a full
/// parse/re-serialize is possible before the SUBJECT is held to the same standard below.
fn identity_round_trip_oracle(ctx: &Context) -> Result<Outcome, String> {
    let input = mutable_input(ctx)?;
    let no_mutation = Json::Object(vec![("kind".to_string(), Json::String("no-mutation".to_string())), ("params".to_string(), Json::Object(vec![]))]);
    let bytes = oracle_apply_mutation(&input, &no_mutation)?;
    let projection = project_dxf_r12(&bytes)?;
    Ok(Outcome::with_raw(bytes, projection))
}
//#endregion 🔖️Oracle

//#region 🔖️Subject
#[cfg(feature = "sut")]
mod subject {
    use super::{mutable_input, KINDS};
    use semio_repo_test_host::{Context, Json, Outcome};
    use semio_s_plugin_stdio::artifacts::dxf::standards::v_r12::subsets::any::schema::mutations::{apply_dxf_mutation, DxfMutation};
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
    /// subset's own header codec reads (`📸️snapshot/🦀️component.rs`'s `parse_header_var`).
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
            "no-mutation" => Ok(DxfMutation::NoMutation),
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
                Ok(DxfMutation::SetSnapshot { snapshot })
            }
            "set-header-var" => {
                let name = params.str("name");
                Ok(DxfMutation::SetHeaderVar { name: name.clone(), header_var: DxfHeaderVar { name, group_code: 10, value: DxfValue::Point { value: point3(&params, "value") }, extra_group_codes: vec![] } })
            }
            "remove-header-var" => Ok(DxfMutation::RemoveHeaderVar { name: params.str("name") }),
            "insert-layer" => Ok(DxfMutation::InsertLayer { index: usize_field(&params, "index"), layer: json_to_layer(&params) }),
            "remove-layer" => Ok(DxfMutation::RemoveLayer { name: params.str("name") }),
            "set-layer" => Ok(DxfMutation::SetLayer { name: params.str("name"), layer: json_to_layer(&params) }),
            "insert-style" => Ok(DxfMutation::InsertStyle { index: usize_field(&params, "index"), style: json_to_style(&params) }),
            "remove-style" => Ok(DxfMutation::RemoveStyle { name: params.str("name") }),
            "set-style" => Ok(DxfMutation::SetStyle { name: params.str("name"), style: json_to_style(&params) }),
            "insert-linetype" => Ok(DxfMutation::InsertLinetype { index: usize_field(&params, "index"), linetype: json_to_linetype(&params) }),
            "remove-linetype" => Ok(DxfMutation::RemoveLinetype { name: params.str("name") }),
            "set-linetype" => Ok(DxfMutation::SetLinetype { name: params.str("name"), linetype: json_to_linetype(&params) }),
            "insert-entity" => Ok(DxfMutation::InsertEntity { index: usize_field(&params, "index"), entity: json_to_entity(&params)? }),
            "remove-entity" => Ok(DxfMutation::RemoveEntity { index: usize_field(&params, "index") }),
            "set-entity" => Ok(DxfMutation::SetEntity { index: usize_field(&params, "index"), entity: json_to_entity(&params)? }),
            "insert-block" => Ok(DxfMutation::InsertBlock { index: usize_field(&params, "index"), block: json_to_block(&params)? }),
            "remove-block" => Ok(DxfMutation::RemoveBlock { index: usize_field(&params, "index") }),
            "set-block" => Ok(DxfMutation::SetBlock { index: usize_field(&params, "index"), block: json_to_block(&params)? }),
            other => Err(format!("mutation kind {other:?} has no subject implementation")),
        }
    }
    //#endregion 🔖️SpecCodec

    //#region 🔖️Inverse
    /// ↩️ `DxfMutation::inverse`'s own per-variant contract (`../../🏅️standards/🔖️r12/🪆️subsets/
    /// ✳️any/🧬️schema/🧬️mutations/🦀️component.rs`'s `impl Mutation<DxfSnapshot> for DxfMutation`),
    /// transplanted in closed form -- same rationale `mutate-pdf-1-7`'s own `inverse_of` gives:
    /// this adapter needs no extra crate dependency (the `protocol::Mutation` trait itself) beyond
    /// `semio-s-plugin-stdio`.
    fn inverse_of(mutation: &DxfMutation, base: &DxfSnapshot) -> DxfMutation {
        match mutation {
            DxfMutation::NoMutation => DxfMutation::NoMutation,
            DxfMutation::SetSnapshot { .. } => DxfMutation::SetSnapshot { snapshot: base.clone() },

            DxfMutation::SetHeaderVar { name, .. } => match base.header_vars.iter().find(|v| &v.name == name) {
                Some(v) => DxfMutation::SetHeaderVar { name: name.clone(), header_var: v.clone() },
                None => DxfMutation::RemoveHeaderVar { name: name.clone() },
            },
            DxfMutation::RemoveHeaderVar { name } => match base.header_vars.iter().find(|v| &v.name == name) {
                Some(v) => DxfMutation::SetHeaderVar { name: name.clone(), header_var: v.clone() },
                None => DxfMutation::NoMutation,
            },

            DxfMutation::InsertLayer { layer, .. } => DxfMutation::RemoveLayer { name: layer.name.clone() },
            DxfMutation::RemoveLayer { name } => match base.tables.layers.iter().find(|l| &l.name == name) {
                Some(l) => DxfMutation::InsertLayer { index: base.tables.layers.iter().position(|x| &x.name == name).unwrap_or(base.tables.layers.len()), layer: l.clone() },
                None => DxfMutation::NoMutation,
            },
            DxfMutation::SetLayer { name, .. } => match base.tables.layers.iter().find(|l| &l.name == name) {
                Some(l) => DxfMutation::SetLayer { name: name.clone(), layer: l.clone() },
                None => DxfMutation::RemoveLayer { name: name.clone() },
            },

            DxfMutation::InsertStyle { style, .. } => DxfMutation::RemoveStyle { name: style.name.clone() },
            DxfMutation::RemoveStyle { name } => match base.tables.styles.iter().find(|s| &s.name == name) {
                Some(s) => DxfMutation::InsertStyle { index: base.tables.styles.iter().position(|x| &x.name == name).unwrap_or(base.tables.styles.len()), style: s.clone() },
                None => DxfMutation::NoMutation,
            },
            DxfMutation::SetStyle { name, .. } => match base.tables.styles.iter().find(|s| &s.name == name) {
                Some(s) => DxfMutation::SetStyle { name: name.clone(), style: s.clone() },
                None => DxfMutation::RemoveStyle { name: name.clone() },
            },

            DxfMutation::InsertLinetype { linetype, .. } => DxfMutation::RemoveLinetype { name: linetype.name.clone() },
            DxfMutation::RemoveLinetype { name } => match base.tables.linetypes.iter().find(|l| &l.name == name) {
                Some(l) => DxfMutation::InsertLinetype { index: base.tables.linetypes.iter().position(|x| &x.name == name).unwrap_or(base.tables.linetypes.len()), linetype: l.clone() },
                None => DxfMutation::NoMutation,
            },
            DxfMutation::SetLinetype { name, .. } => match base.tables.linetypes.iter().find(|l| &l.name == name) {
                Some(l) => DxfMutation::SetLinetype { name: name.clone(), linetype: l.clone() },
                None => DxfMutation::RemoveLinetype { name: name.clone() },
            },

            DxfMutation::InsertEntity { index, .. } => DxfMutation::RemoveEntity { index: *index },
            DxfMutation::RemoveEntity { index } => match base.entities.get(*index) {
                Some(e) => DxfMutation::InsertEntity { index: *index, entity: e.clone() },
                None => DxfMutation::NoMutation,
            },
            DxfMutation::SetEntity { index, .. } => match base.entities.get(*index) {
                Some(e) => DxfMutation::SetEntity { index: *index, entity: e.clone() },
                None => DxfMutation::NoMutation,
            },

            DxfMutation::InsertBlock { index, .. } => DxfMutation::RemoveBlock { index: *index },
            DxfMutation::RemoveBlock { index } => match base.blocks.get(*index) {
                Some(b) => DxfMutation::InsertBlock { index: *index, block: b.clone() },
                None => DxfMutation::NoMutation,
            },
            DxfMutation::SetBlock { index, .. } => match base.blocks.get(*index) {
                Some(b) => DxfMutation::SetBlock { index: *index, block: b.clone() },
                None => DxfMutation::NoMutation,
            },
        }
    }
    //#endregion 🔖️Inverse

    //#region 🔖️Handlers
    pub fn mutate(ctx: &Context) -> Result<Outcome, String> {
        let input = mutable_input(ctx)?;
        let text = std::str::from_utf8(&input).map_err(|error| error.to_string())?;
        let base = parse_dxf_document(text)?;
        let mutation = mutation_from_spec(&ctx.doc_json()?, &base)?;
        let mut snapshot = base;
        apply_dxf_mutation(&mut snapshot, &mutation);
        let output = print_dxf_document(&snapshot).into_bytes();
        let projection = project_dxf_r12(&output)?;
        Ok(Outcome::with_raw(output, projection))
    }

    pub fn inverse(ctx: &Context) -> Result<Outcome, String> {
        let input = mutable_input(ctx)?;
        let text = std::str::from_utf8(&input).map_err(|error| error.to_string())?;
        let base = parse_dxf_document(text)?;
        let mutation = mutation_from_spec(&ctx.doc_json()?, &base)?;
        let undo = inverse_of(&mutation, &base);
        let mut snapshot = base;
        apply_dxf_mutation(&mut snapshot, &mutation);
        apply_dxf_mutation(&mut snapshot, &undo);
        let output = print_dxf_document(&snapshot).into_bytes();
        let projection = project_dxf_r12(&output)?;
        Ok(Outcome::with_raw(output, projection))
    }

    /// 🔒️ The no-byte-pass-through rule: the subject must fully parse the real artifact into its
    /// typed snapshot and re-serialize from the model alone -- `parse_dxf_document`/
    /// `print_dxf_document` are this subset's ONLY channel from input to output. `print_dxf_document`
    /// regenerates a canonical NORMAL FORM (documented in `📸️snapshot/🦀️component.rs`'s own module
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
