//! 🧪️ The bundled BESTEST case 600 loads into the native persistent owner and survives the text and pack carriers.
use crate::snapshot::EnergyModelSnapshot;
use pack::snapshot::{ArtifactDsl, ArtifactPack};

fn done(snapshot: EnergyModelSnapshot) {
    snapshot.retire().expect("funded retirement");
}

fn case_600() -> EnergyModelSnapshot {
    EnergyModelSnapshot::from_model(&semio_s_plugin_energy_engine_bestest::model("600").expect("case 600 is registered")).expect("case 600 is an admissible native document")
}

#[test]
fn bestest_600_round_trips_the_text_carrier() {
    let snapshot = case_600();
    let text = snapshot.print_dsl();
    let parsed = EnergyModelSnapshot::parse_dsl(&text).expect("canonical text parses");
    assert_eq!(parsed, snapshot);
    assert_eq!(parsed.print_dsl(), text);
    done(parsed);
    done(snapshot);
}

#[test]
fn bestest_600_round_trips_the_pack_carrier_and_the_reference_projection() {
    let snapshot = case_600();
    let bytes = snapshot.encode_pack();
    let decoded = EnergyModelSnapshot::decode_pack(&bytes).expect("canonical pack decodes");
    assert_eq!(decoded, snapshot);
    done(decoded);
    let reference = snapshot.to_reference().expect("projects to plain rows");
    assert_eq!(reference.model, semio_s_plugin_energy_engine_bestest::model("600").unwrap());
    let back = EnergyModelSnapshot::from_reference(&reference).expect("receives back");
    assert_eq!(back, snapshot);
    done(back);
    done(snapshot);
}

#[test]
fn the_derived_owned_spec_hashes_to_the_authored_native_schema_hash() {
    let spec = <EnergyModelSnapshot as ArtifactPack>::record_spec().expect("authored record spec");
    assert_eq!(pack::record::schema_hash(&spec), crate::original::pack::SCHEMA_HASH);
}

#[test]
fn a_refused_cold_diff_leaves_the_native_base_untouched() {
    use crate::diff::{ModelPatch, ReferenceEnergyModelDiff, ZonePatch};
    use semio_s_artifact_energy_model_diff_patch::Rows;
    let mut base = case_600();
    let before = base.clone();
    let ghost = ReferenceEnergyModelDiff::of(ModelPatch { zones: Rows::modifying(ZonePatch { name: Some("ghost".into()), ..ZonePatch::of(crate::model::EntityId(4_242_424)) }), ..ModelPatch::default() });
    assert!(protocol::apply_diff_cold_into(&ghost, &mut base).is_err());
    assert_eq!(base, before);
    done(before);
    done(base);
}
