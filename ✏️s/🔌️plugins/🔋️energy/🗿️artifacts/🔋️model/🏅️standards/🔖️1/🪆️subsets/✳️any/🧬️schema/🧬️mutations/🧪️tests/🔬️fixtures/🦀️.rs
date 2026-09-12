use super::EnergyModelMutation;
use crate::diff::EnergyModelDiff;
use crate::EnergyModelSnapshot;
use protocol::{Mutation, MutationDiff, SemanticMutation};
use semio_framework_os_kernel::ToValue;

/// 🧫️ One committed specification vector and the typed scenario it was generated from.
pub struct Case {
    pub kind: &'static str,
    pub directory: &'static str,
    pub before: &'static str,
    pub after: &'static str,
    pub mutation: &'static str,
    pub diff: &'static str,
    pub outcome: &'static str,
    pub scenario: fn() -> (EnergyModelSnapshot, EnergyModelMutation),
}

/// 🏠️ One conditioned zone with the BESTEST 600 envelope's own air volume.
pub fn zone(id: u32, name: &str) -> crate::model::Zone {
    crate::model::Zone { id: crate::model::EntityId(id), name: name.to_string(), volume_m3: 129.6, multiplier: 1, conditioned: true, part_of_total_floor_area: true }
}

/// 📸️ The persisted snapshot a typed model lives in, both composed child handles minted.
pub fn snapshot(model: crate::model::Model) -> EnergyModelSnapshot {
    crate::energy_snapshot_with_state(crate::ENERGY_MODEL_DOCUMENT_SCHEMA, &model, None)
}

/// 🔗️ A head-pinned forward link to another artifact.
pub fn link(uri: &str, role: &str) -> store::ArtifactLink {
    store::ArtifactLink { target: store::os_io::ArtifactRef::parse_uri(uri).expect("fixture link uri parses"), pin: store::LinkPin::Head, role: role.to_string() }
}

//#region 🧰️G1Constructors
/// 🪑️ One space inside a zone.
pub fn space(id: u32, name: &str, zone_id: u32) -> crate::model::Space {
    crate::model::Space { id: crate::model::EntityId(id), name: name.to_string(), zone_id: crate::model::EntityId(zone_id), floor_area_m2: 48.0 }
}

/// 🧱️ One opaque material layer — the BESTEST 600 lightweight wall's plasterboard.
pub fn material(id: u32, name: &str) -> crate::model::Material {
    crate::model::Material {
        id: crate::model::EntityId(id),
        name: name.to_string(),
        thickness_m: 0.012,
        conductivity_w_m_k: 0.16,
        density_kg_m3: 950.0,
        specific_heat_j_kg_k: 840.0,
        thermal_absorptance: 0.9,
        solar_absorptance: 0.6,
        visible_absorptance: 0.6,
    }
}

/// 🧱️ One single-layer construction over `material_id`.
pub fn construction(id: u32, name: &str, material_id: u32) -> crate::model::Construction {
    crate::model::Construction { id: crate::model::EntityId(id), name: name.to_string(), layer_material_ids: vec![crate::model::EntityId(material_id)] }
}

/// 📐️ One sun- and wind-exposed 8 m × 2.7 m exterior wall on `zone_id`.
pub fn surface(id: u32, name: &str, zone_id: u32, construction_id: u32) -> crate::model::Surface {
    crate::model::Surface {
        id: crate::model::EntityId(id),
        name: name.to_string(),
        zone_id: crate::model::EntityId(zone_id),
        class: crate::model::SurfaceClass::ExteriorWall,
        vertices_m: vec![[0.0, 0.0, 0.0], [8.0, 0.0, 0.0], [8.0, 0.0, 2.7], [0.0, 0.0, 2.7]],
        construction_id: crate::model::EntityId(construction_id),
        outside_boundary_condition: crate::model::OutsideBoundary::OutdoorAir,
        sun_exposed: true,
        wind_exposed: true,
        multiplier: 1,
    }
}

/// 🪟️ One 3 m × 2 m window carrying ANSI/ASHRAE 140 §5.2's own quoted optics.
pub fn window(id: u32, name: &str, surface_id: u32) -> crate::model::Fenestration {
    crate::model::Fenestration {
        id: crate::model::EntityId(id),
        name: name.to_string(),
        surface_id: crate::model::EntityId(surface_id),
        u_value_w_m2k: 3.0,
        shgc: 0.787,
        vlt: 0.86,
        area_m2: 6.0,
        height_m: 2.0,
        sill_height_m: 0.5,
        frame_conductance_w_k: 0.0,
        divider_conductance_w_k: 0.0,
        overhang_depth_m: 0.0,
        overhang_offset_m: 0.0,
        fin_depth_m: 0.0,
        fin_offset_m: 0.0,
        glazing_construction_id: None,
    }
}

/// 🌳️ One free-standing site shading surface.
pub fn shading(id: u32, name: &str) -> crate::model::ShadingSurface {
    crate::model::ShadingSurface { id: crate::model::EntityId(id), name: name.to_string(), vertices_m: vec![[0.0, -2.0, 3.0], [8.0, -2.0, 3.0], [8.0, 0.0, 3.0], [0.0, 0.0, 3.0]], transmittance_schedule_id: None }
}
//#endregion 🧰️G1Constructors

fn committed(case: &Case, label: &str, text: &str) -> pack::json::Value {
    pack::json::parse(text).unwrap_or_else(|error| panic!("{}/{}: committed {label} is not valid JSON: {error}", case.kind, case.directory))
}

fn decode(case: &Case, label: &str, text: &str) -> EnergyModelSnapshot {
    pack::json::from_json_str(text).unwrap_or_else(|error| panic!("{}/{}: committed {label} does not decode: {error}", case.kind, case.directory))
}

fn built(case: &Case) -> protocol::MutationOutcome<EnergyModelDiff> {
    let base = decode(case, "before-snapshot", case.before);
    let mutation: EnergyModelMutation = pack::json::from_json_str(case.mutation).expect("committed mutation payload decodes");
    <EnergyModelMutation as Mutation<EnergyModelSnapshot>>::diff(&mutation, &base)
}

fn level_name(level: protocol::Severity) -> &'static str {
    match level {
        protocol::Severity::Info => "info",
        protocol::Severity::Warning => "warning",
        protocol::Severity::Error => "error",
        protocol::Severity::Fatal => "fatal",
    }
}

/// 🎯️ The declared outcome document a produced `MutationOutcome` corresponds to: a refusal names
/// one fault code and the offending address, an application names its ordered message list.
fn outcome_document(outcome: &protocol::MutationOutcome<EnergyModelDiff>) -> pack::json::Value {
    let rejected = outcome.worst_level().is_some_and(|level| level >= protocol::Severity::Error);
    if rejected {
        let first = outcome.messages().iter().find(|message| message.level >= protocol::Severity::Error).expect("a refusal carries its own message");
        return pack::json::object([
            ("status".to_string(), pack::json::Value::String("rejected".to_string())),
            ("code".to_string(), pack::json::Value::String(first.code.0.clone())),
            ("path".to_string(), pack::json::Value::Array(first.target.iter().map(|entry| pack::json::Value::String(entry.clone())).collect())),
        ]);
    }
    pack::json::object([
        ("status".to_string(), pack::json::Value::String("applied".to_string())),
        (
            "messages".to_string(),
            pack::json::Value::Array(
                outcome.messages().iter().map(|message| pack::json::object([("level".to_string(), pack::json::Value::String(level_name(message.level).to_string())), ("code".to_string(), pack::json::Value::String(message.code.0.clone()))])).collect(),
            ),
        ),
    ])
}

fn write_file(path: std::path::PathBuf, text: &str) {
    std::fs::create_dir_all(path.parent().expect("fixture file has a parent")).expect("fixture directory is writable");
    std::fs::write(path, format!("{text}\n")).expect("fixture file is writable");
}

/// 🏗️ Materializes the five committed JSON files from the typed scenario. Off unless
/// `SEMIO_ENERGY_WRITE_FIXTURES=1`, so an ordinary test run never touches the source tree.
pub fn write_when_requested(case: &Case) {
    if std::env::var("SEMIO_ENERGY_WRITE_FIXTURES").ok().as_deref() != Some("1") {
        return;
    }
    let (before, mutation) = (case.scenario)();
    let outcome = <EnergyModelMutation as Mutation<EnergyModelSnapshot>>::diff(&mutation, &before);
    let after = MutationDiff::apply(outcome.diff(), &before).expect("the scenario's forward diff applies");
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations").join(case.directory);
    let json = |value: pack::json::Value| pack::json::to_string_pretty(&value);
    write_file(root.join("📸️snapshot/⬅️before/🔣️.json"), &json(pack::json::from_dsl_value(&before.to_value())));
    write_file(root.join("📸️snapshot/➡️after/🔣️.json"), &json(pack::json::from_dsl_value(&after.to_value())));
    write_file(root.join("🦠️mutation/🔣️.json"), &json(pack::json::from_dsl_value(&mutation.to_value())));
    write_file(root.join("🔺️diff/🔣️.json"), &json(pack::json::from_dsl_value(&outcome.diff().to_value())));
    write_file(root.join("🎯️outcome/🔣️.json"), &json(outcome_document(&outcome)));
}

/// ▶️ Applying the committed mutation to the committed before-snapshot reproduces the committed
/// after-snapshot exactly, composed child handles included.
pub fn assert_forward(case: &Case) {
    let base = decode(case, "before-snapshot", case.before);
    let applied = MutationDiff::apply(built(case).diff(), &base).expect("committed mutation applies to its committed before-snapshot");
    assert_eq!(applied, decode(case, "after-snapshot", case.after), "{}/{}: the applied document is not the committed after-snapshot", case.kind, case.directory);
    assert_eq!((applied.structure.child_id.as_str(), applied.zones.child_id.as_str()), (base.structure.child_id.as_str(), base.zones.child_id.as_str()), "{}/{}: structure and zones must stay one scene", case.kind, case.directory);
}

/// ↩️ Applying the committed mutation and then its OWN computed inverse restores the committed
/// before-snapshot; an inverse step that is itself refused fails here rather than passing quietly.
pub fn assert_inverse(case: &Case) {
    let base = decode(case, "before-snapshot", case.before);
    let mutation: EnergyModelMutation = pack::json::from_json_str(case.mutation).expect("committed mutation payload decodes");
    let mut snapshot = MutationDiff::apply(built(case).diff(), &base).expect("committed mutation applies");
    for step in <EnergyModelMutation as Mutation<EnergyModelSnapshot>>::inverse(&mutation, &base) {
        let outcome = <EnergyModelMutation as Mutation<EnergyModelSnapshot>>::diff(&step, &snapshot);
        assert!(!outcome.worst_level().is_some_and(|level| level >= protocol::Severity::Error), "{}/{}: an inverse step was itself refused", case.kind, case.directory);
        snapshot = MutationDiff::apply(outcome.diff(), &snapshot).expect("the inverse step applies");
    }
    assert_eq!(snapshot, base, "{}/{}: undoing did not land back on the committed before-snapshot", case.kind, case.directory);
}

/// 🔣️ Both committed documents and the committed payload re-encode to themselves.
pub fn assert_canonical(case: &Case) {
    for (label, text) in [("before", case.before), ("after", case.after)] {
        let decoded = decode(case, label, text);
        let reencoded = pack::json::from_dsl_value(&decoded.to_value());
        assert!(pack::json::value_eq_ignoring_object_order(&reencoded, &committed(case, label, text)), "{}/{}: committed {label} snapshot is not canonical", case.kind, case.directory);
    }
    let mutation: EnergyModelMutation = pack::json::from_json_str(case.mutation).expect("committed mutation payload decodes");
    let reencoded = pack::json::from_dsl_value(&mutation.to_value());
    assert!(pack::json::value_eq_ignoring_object_order(&reencoded, &committed(case, "mutation", case.mutation)), "{}/{}: committed mutation payload is not canonical", case.kind, case.directory);
}

/// 🎯️ The diagnostics the implementation raises are the ones the committed outcome declares.
pub fn assert_outcome(case: &Case) {
    let produced = outcome_document(&built(case));
    let declared = committed(case, "outcome", case.outcome);
    assert!(pack::json::value_eq_ignoring_object_order(&produced, &declared), "{}/{}: produced outcome {produced:?} differs from the committed one {declared:?}", case.kind, case.directory);
}

/// 🔺️ The produced delta is the committed delta — which pins WHICH fields the kind may touch,
/// not merely where the document ended up.
pub fn assert_diff(case: &Case) {
    let produced = pack::json::from_dsl_value(&built(case).diff().to_value());
    let declared = committed(case, "diff", case.diff);
    assert!(pack::json::value_eq_ignoring_object_order(&produced, &declared), "{}/{}: produced diff {produced:?} differs from the committed 🔺️diff/🔣️.json {declared:?}", case.kind, case.directory);
}

/// 🔣️ The committed delta decodes to the real `EnergyModelDiff` and re-encodes unchanged.
pub fn assert_diff_canonical(case: &Case) {
    let decoded: EnergyModelDiff = pack::json::from_json_str(case.diff).expect("committed diff decodes");
    let reencoded = pack::json::from_dsl_value(&decoded.to_value());
    assert!(pack::json::value_eq_ignoring_object_order(&reencoded, &committed(case, "diff", case.diff)), "{}/{}: committed diff is not canonical", case.kind, case.directory);
}

/// 🩹 The committed delta ALONE carries the before-document to the after-document.
pub fn assert_diff_applies(case: &Case) {
    let decoded: EnergyModelDiff = pack::json::from_json_str(case.diff).expect("committed diff decodes");
    let produced = MutationDiff::apply(&decoded, &decode(case, "before-snapshot", case.before)).expect("committed diff applies to the before-document");
    assert_eq!(produced, decode(case, "after-snapshot", case.after), "{}/{}: the committed diff did not carry before to after", case.kind, case.directory);
}

/// 🧭️ The kind's semantic descriptor uses an approved verb and its inverse closes the loop.
pub async fn assert_semantics(case: &Case) {
    let base = decode(case, "before-snapshot", case.before);
    let mutation: EnergyModelMutation = pack::json::from_json_str(case.mutation).expect("committed mutation payload decodes");
    let descriptor = SemanticMutation::semantics(&mutation);
    assert!(protocol::is_approved_verb(descriptor.verb), "{}: {:?} is not an approved verb", case.kind, descriptor.verb);
    assert_eq!(descriptor.kind, case.kind);
    assert_eq!(<EnergyModelMutation as SemanticMutation<EnergyModelSnapshot>>::kinds().len(), super::KINDS.len());
    assert_inverse(case);
    let _ = base;
}

/// ⚖️ The framework's own inverse and diff-absorb laws, run in role against this vector.
pub async fn assert_laws(case: &Case) {
    let base = decode(case, "before-snapshot", case.before);
    let mutation: EnergyModelMutation = pack::json::from_json_str(case.mutation).expect("committed mutation payload decodes");
    protocol::os_spr::protocol_laws::assert_mutation_inverse_law(&base, &mutation).await;
    let first = built(case).diff().clone();
    let second = built(case).diff().clone();
    protocol::os_spr::protocol_laws::assert_mutation_diff_absorb_law(&base, first, second).await;
}

/// 📝️ Every declared kind survives the text and binary op codecs the `dsl::DslEnum` derive
/// generates — one loop, no per-kind round-trip test to forget.
#[semio_framework_async_macros::async_test]
async fn every_kind_round_trips_through_text_and_binary() {
    for operation in super::wire_probes() {
        store::os_store::test_support::assert_op_line_round_trip(&operation);
        store::os_store::test_support::assert_op_text_binary_equivalence(&operation);
    }
}
