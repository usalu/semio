//! 🧬️ Transparent energy-model semantic mutation aggregate. Every variant is a single-field tuple
//! wrapping a handcrafted `protocol::MutationKind` payload (the `🧬️mutations/<slug>/` leaves);
//! `#[derive(dsl::Mutations)]` generates the `protocol::Mutation`/`protocol::SemanticMutation`
//! dispatch and `#[derive(dsl::DslEnum)]` generates the `dsl::DslVariants` binding that
//! `📝️text/🦀️.rs`'s op codecs are written against — so a new kind costs one enum variant and one
//! re-export line here, never a hand-written match arm, an opcode constant or a tag registry
//! (ticket 26/09/06/ENERGY-PLUGIN-END-TO-END, adopting `📸️remodel`'s architecture).
//!
//! There is deliberately NO whole-document replace in this enum: `📓️derivation-rules.md` rule 6
//! routes file-open / import / load-example through `store::ArtifactStore::reset`, outside history.

use crate::artifacts::model::diff::EnergyModelDiff;
use crate::artifacts::model::EnergyModelSnapshot;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️LinkRoles
/// 🌦️ The `weather_link` slot's role name — one constant shared by the bind/unbind pair.
pub const WEATHER_LINK_ROLE: &str = "weather";
/// 🪢️ The `referenced_model` slot's role name — one constant shared by the connect/disconnect pair.
pub const REFERENCED_MODEL_LINK_ROLE: &str = "model";
//#endregion 🔖️LinkRoles

//#region 🔖️Reexports
pub use super::rename_model::{rename_model, RenameModel};
pub use super::change_model_version::{change_model_version, ChangeModelVersion};
pub use super::update_site::{update_site, UpdateSite};
pub use super::update_ground_temperature::{update_ground_temperature, UpdateGroundTemperature};
pub use super::update_run_period::{update_run_period, UpdateRunPeriod};
pub use super::replace_airflow_network::{replace_airflow_network, ReplaceAirflowNetwork};
pub use super::add_output_variable::{add_output_variable, AddOutputVariable};
pub use super::remove_output_variable::{remove_output_variable, RemoveOutputVariable};
pub use super::bind_weather_file::{bind_weather_file, BindWeatherFile};
pub use super::unbind_weather_file::{unbind_weather_file, UnbindWeatherFile};
pub use super::connect_referenced_model::{connect_referenced_model, ConnectReferencedModel};
pub use super::disconnect_referenced_model::{disconnect_referenced_model, DisconnectReferencedModel};
pub use super::rename_zone::{rename_zone, RenameZone};
pub use super::change_zone_volume::{change_zone_volume, ChangeZoneVolume};
pub use super::change_zone_multiplier::{change_zone_multiplier, ChangeZoneMultiplier};
pub use super::change_zone_conditioned::{change_zone_conditioned, ChangeZoneConditioned};
pub use super::change_zone_floor_area_participation::{change_zone_floor_area_participation, ChangeZoneFloorAreaParticipation};
//#endregion 🔖️Reexports

//#region 🔖️Aggregate
/// 🧬️ Closed semantic mutation vocabulary for an energy model.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, dsl::DslEnum, dsl::Mutations)]
#[value(tag = "mutation", rename_all = "camelCase")]
#[mutations(snapshot = EnergyModelSnapshot, diff = EnergyModelDiff, schema = "energy.model")]
pub enum EnergyModelMutation {
    RenameModel(RenameModel),
    ChangeModelVersion(ChangeModelVersion),
    UpdateSite(UpdateSite),
    UpdateGroundTemperature(UpdateGroundTemperature),
    UpdateRunPeriod(UpdateRunPeriod),
    ReplaceAirflowNetwork(ReplaceAirflowNetwork),
    AddOutputVariable(AddOutputVariable),
    RemoveOutputVariable(RemoveOutputVariable),
    BindWeatherFile(BindWeatherFile),
    UnbindWeatherFile(UnbindWeatherFile),
    ConnectReferencedModel(ConnectReferencedModel),
    DisconnectReferencedModel(DisconnectReferencedModel),
    RenameZone(RenameZone),
    ChangeZoneVolume(ChangeZoneVolume),
    ChangeZoneMultiplier(ChangeZoneMultiplier),
    ChangeZoneConditioned(ChangeZoneConditioned),
    ChangeZoneFloorAreaParticipation(ChangeZoneFloorAreaParticipation),
}

/// 🏷️ Direct semantic roster exported for the language-neutral test adapter, in aggregate
/// declaration order — which is also the binary ordinal order `dsl::variants_binary` writes.
pub const KINDS: &[&str] = &[
    "rename-model",
    "change-model-version",
    "update-site",
    "update-ground-temperature",
    "update-run-period",
    "replace-airflow-network",
    "add-output-variable",
    "remove-output-variable",
    "bind-weather-file",
    "unbind-weather-file",
    "connect-referenced-model",
    "disconnect-referenced-model",
    "rename-zone",
    "change-zone-volume",
    "change-zone-multiplier",
    "change-zone-conditioned",
    "change-zone-floor-area-participation",
];

/// 🗂️ `(semanticKind, leaf directory name)` for every declared kind — the single place the
/// emoji-carrying directory names are stated in Rust.
pub const DIRECTORIES: &[(&str, &str)] = &[
    ("rename-model", "🏷️rename-model"),
    ("change-model-version", "🔢️change-model-version"),
    ("update-site", "🌍️update-site"),
    ("update-ground-temperature", "🌡️update-ground-temperature"),
    ("update-run-period", "📅️update-run-period"),
    ("replace-airflow-network", "🫧️replace-airflow-network"),
    ("add-output-variable", "📊️add-output-variable"),
    ("remove-output-variable", "📉️remove-output-variable"),
    ("bind-weather-file", "🌦️bind-weather-file"),
    ("unbind-weather-file", "🌤️unbind-weather-file"),
    ("connect-referenced-model", "🪢️connect-referenced-model"),
    ("disconnect-referenced-model", "✂️disconnect-referenced-model"),
    ("rename-zone", "🏠️rename-zone"),
    ("change-zone-volume", "📦️change-zone-volume"),
    ("change-zone-multiplier", "✖️change-zone-multiplier"),
    ("change-zone-conditioned", "🌬️change-zone-conditioned"),
    ("change-zone-floor-area-participation", "📐️change-zone-floor-area-participation"),
];
//#endregion 🔖️Aggregate

//#region 🌉️TestBridge
/// 🔮️ Reports the forward and inverse behavior of one committed language-neutral vector.
pub fn energy_model_mutation_report_json(base_json: &str, mutation_json: &str, after_json: &str) -> Result<String, String> {
    use semio_framework_os_kernel::ToValue;
    let decode_snapshot = |text: &str| -> Result<EnergyModelSnapshot, String> { pack::json::from_json_str::<EnergyModelSnapshot>(text).map_err(|error| error.to_string()) };
    let base = decode_snapshot(base_json)?;
    let expected = decode_snapshot(after_json)?;
    let mutation: EnergyModelMutation = pack::json::from_json_str(mutation_json).map_err(|error| error.to_string())?;
    let mut applied = base.clone();
    let forward = <EnergyModelMutation as protocol::Mutation<EnergyModelSnapshot>>::diff(&mutation, &base).apply_to(&mut applied);
    let inverse = <EnergyModelMutation as protocol::Mutation<EnergyModelSnapshot>>::inverse(&mutation, &base);
    let mut undone = applied.clone();
    let mut inverse_messages = Vec::new();
    for step in &inverse {
        let outcome = <EnergyModelMutation as protocol::Mutation<EnergyModelSnapshot>>::diff(step, &undone).apply_to(&mut undone);
        inverse_messages.extend(outcome.messages().iter().cloned());
    }
    // 🌉️ `MutationMessage` (`🧰️framework/🔨️modules/📡️replication/🎮️mutation/🦀️.rs`) is a
    // framework-owned type that has not itself gained `ToValue`/`FromValue` — its two call sites
    // here go through the PRE-EXISTING `protocol::to_dsl_value` serde bridge (framework-internal,
    // exempt) and land in `pack::json::Value` via `pack::json::from_dsl_value`.
    let messages_json = protocol::to_dsl_value(forward.messages()).map(|value| pack::json::from_dsl_value(&value)).map_err(|error| error.to_string())?;
    let inverse_messages_json = protocol::to_dsl_value(&inverse_messages).map(|value| pack::json::from_dsl_value(&value)).map_err(|error| error.to_string())?;
    let report = pack::json::object([
        ("base".to_string(), pack::json::from_dsl_value(&base.to_value())),
        ("expectedSnapshot".to_string(), pack::json::from_dsl_value(&expected.to_value())),
        ("snapshot".to_string(), pack::json::from_dsl_value(&applied.to_value())),
        ("diff".to_string(), pack::json::from_dsl_value(&forward.diff().to_value())),
        ("messages".to_string(), messages_json),
        ("inverseSteps".to_string(), pack::json::from_dsl_value(&inverse.to_value())),
        ("inverseSnapshot".to_string(), pack::json::from_dsl_value(&undone.to_value())),
        ("inverseMessages".to_string(), inverse_messages_json),
    ]);
    Ok(pack::json::to_string(&report))
}
//#endregion 🌉️TestBridge

//#region 🧵️WireProbes
/// 🧵️ One representative value per declared variant — the codec round-trip corpus, so wire coverage
/// grows with the enum instead of with a hand-maintained list of tests.
#[cfg(test)]
pub fn wire_probes() -> Vec<EnergyModelMutation> {
    vec![
        rename_model("Probe".to_string()),
        change_model_version("2".to_string()),
        update_site(52.4, 9.7, 55.0, 1.0, 0.0),
        update_ground_temperature(vec![10.0; 12], vec![9.0; 12], 8.0),
        update_run_period(1, 1, 1, 31, 2026),
        replace_airflow_network(true, vec![1], vec![1], 0, vec![7]),
        add_output_variable("Zone Mean Air Temperature".to_string(), "ZONE ONE".to_string(), crate::model::OutputReportFrequency::Hourly),
        remove_output_variable("Zone Mean Air Temperature".to_string(), "ZONE ONE".to_string()),
        bind_weather_file("hannover!s.stdio.semio@v1/epw".to_string()),
        unbind_weather_file(),
        connect_referenced_model("doc-2!s.stdio.semio@v1/model".to_string()),
        disconnect_referenced_model(),
        rename_zone(crate::model::EntityId(1), "ZONE 1".to_string()),
        change_zone_volume(crate::model::EntityId(1), 129.6),
        change_zone_multiplier(crate::model::EntityId(1), 4),
        change_zone_conditioned(crate::model::EntityId(1), false),
        change_zone_floor_area_participation(crate::model::EntityId(1), false),
    ]
}
//#endregion 🧵️WireProbes

//#region 🧰️Fixtures
/// 🧰️ Shared fixture-case machinery: the eight laws every committed `(before, mutation, after,
/// diff, outcome)` vector is held to, plus the `SEMIO_ENERGY_WRITE_FIXTURES=1` generator that
/// materializes those five JSON files from one typed scenario. Every `<kind>/🧪️tests/<case>/🦀️.rs`
/// is a thin declaration over this module, so a new kind's fixture case is data, not code.
#[cfg(test)]
pub mod fixtures {
    use super::EnergyModelMutation;
    use crate::artifacts::model::diff::EnergyModelDiff;
    use crate::artifacts::model::EnergyModelSnapshot;
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
        crate::artifacts::model::energy_snapshot_with_state(crate::artifacts::model::ENERGY_MODEL_DOCUMENT_SCHEMA, &model, None)
    }

    /// 🔗️ A head-pinned forward link to another artifact.
    pub fn link(uri: &str, role: &str) -> store::ArtifactLink {
        store::ArtifactLink { target: store::os_io::ArtifactRef::parse_uri(uri).expect("fixture link uri parses"), pin: store::LinkPin::Head, role: role.to_string() }
    }

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
                    outcome
                        .messages()
                        .iter()
                        .map(|message| pack::json::object([("level".to_string(), pack::json::Value::String(level_name(message.level).to_string())), ("code".to_string(), pack::json::Value::String(message.code.0.clone()))]))
                        .collect(),
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
        let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations").join(case.directory);
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
        protocol::os_spr::testkit::assert_mutation_inverse_law(&base, &mutation).await;
        let first = built(case).diff().clone();
        let second = built(case).diff().clone();
        protocol::os_spr::testkit::assert_mutation_diff_absorb_law(&base, first, second).await;
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
}
//#endregion 🧰️Fixtures

//#region 🧪️StructuralCorrespondence
#[cfg(test)]
mod structural_correspondence_tests {
    use super::*;
    use protocol::SemanticMutation;

    /// 🧭️ Every declared kind owns a leaf directory whose descriptor, payload schema and behaviour
    /// facets agree with the aggregate enum AND with the subset's language-neutral oracle catalog —
    /// which lives at `✳️any/🔮️oracle/🔣️.json`, never at a flat `🔣️oracle.json`.
    #[test]
    fn direct_owner_descriptors_and_catalog_correspond() {
        let mutation_root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations");
        let catalog_source = std::fs::read_to_string(mutation_root.join("../../🔮️oracle/🔣️.json")).expect("language-neutral oracle catalog");
        let catalog: pack::json::Value = pack::json::parse(&catalog_source).expect("language-neutral oracle catalog must be valid JSON");
        let catalog_kinds: Vec<String> = catalog["mutationCatalogs"][0]["kinds"].as_array().expect("catalog kinds").iter().map(|kind| kind.as_str().expect("catalog kind is a string").to_string()).collect();
        let descriptors = EnergyModelMutation::kinds();
        assert_eq!(descriptors.len(), KINDS.len());
        assert_eq!(descriptors.iter().map(|descriptor| descriptor.kind).collect::<Vec<_>>(), KINDS.to_vec());
        assert_eq!(catalog_kinds, KINDS.iter().map(|kind| (*kind).to_string()).collect::<Vec<_>>());
        assert_eq!(DIRECTORIES.len(), KINDS.len());
        for (kind, directory) in DIRECTORIES {
            let owner = mutation_root.join(directory);
            let source = std::fs::read_to_string(owner.join("🦀️.rs")).expect("direct Rust owner");
            let descriptor_source = std::fs::read_to_string(owner.join("🔣️.json")).expect("direct language-neutral descriptor");
            let descriptor: pack::json::Value = pack::json::parse(&descriptor_source).expect("direct descriptor must be valid JSON");
            let payload_schema_source = std::fs::read_to_string(owner.join("🧬️.schema.json")).expect("direct payload schema");
            let payload_schema: pack::json::Value = pack::json::parse(&payload_schema_source).expect("direct payload schema must be valid JSON");
            assert!(source.contains("protocol::MutationKind"), "{kind} owns no MutationKind impl");
            assert!(!source.contains(concat!("::", "mutation::")));
            assert_eq!(descriptor["semanticKind"].as_str(), Some(*kind));
            assert_eq!(descriptor["textOpcode"].as_str(), Some(*kind));
            assert_eq!(descriptor["payloadSchema"].as_str(), Some("🧬️.schema.json"));
            assert_eq!(payload_schema["title"].as_str(), descriptor["aggregateVariant"].as_str());
            assert!(owner.join("🔺️diff/🦀️.rs").exists(), "{kind} owns no diff leaf");
            assert!(owner.join("↩️inverse/🦀️.rs").exists(), "{kind} owns no inverse leaf");
        }
    }
}
//#endregion 🧪️StructuralCorrespondence
