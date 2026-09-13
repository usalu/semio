use super::*;
use protocol::{Mutation, MutationDiff, OpBinary, OpText};

const SCHEMA: &str = include_str!("../../🧬️schema/🔣️.json");

#[test]
fn language_neutral_settings_mutations_match_the_serde_oracle_and_restore_the_base() {
    let vectors: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🔁️mutations.json")).unwrap();
    for vector in vectors.as_array().unwrap() {
        let base: EnergyModelConfig = dsl::json::from_json_str(&vector["base"].to_string()).unwrap();
        let mutation: EnergyModelConfigMutation = dsl::json::from_json_str(&vector["mutation"].to_string()).unwrap();
        assert_eq!(mutation, serde_json::from_value::<EnergyModelConfigMutation>(vector["mutation"].clone()).unwrap());
        assert_eq!(base, serde_json::from_value::<EnergyModelConfig>(vector["base"].clone()).unwrap());
        assert_eq!(mutation.descriptor().semantic_kind, vector["kind"].as_str().unwrap());
        let next = mutation.diff(&base).diff().apply(&base).unwrap();
        assert_eq!(next, serde_json::from_value::<EnergyModelConfig>(vector["after"].clone()).unwrap());
        assert_eq!(next.is_valid(), vector["valid"].as_bool().unwrap());
        assert_eq!(EnergyModelConfigMutation::decode_op(&mutation.encode_op().unwrap()).unwrap(), mutation);
        assert_eq!(EnergyModelConfigMutation::parse_op(&mutation.print_op()).unwrap(), mutation);
        let mut restored = next;
        for inverse in mutation.inverse(&base) {
            restored = inverse.diff(&restored).diff().apply(&restored).unwrap();
        }
        assert_eq!(restored, base);
    }
}

#[test]
fn defaults_and_ranges_follow_the_schema_source_of_record() {
    let schema: serde_json::Value = serde_json::from_str(SCHEMA).unwrap();
    let defaults = serde_json::to_value(EnergyModelConfig::default()).unwrap();
    for (field, spec) in schema["properties"].as_object().unwrap() {
        assert_eq!(defaults[field], spec["default"], "default of {field}");
        let mut probe = defaults.clone();
        probe[field] = spec["maximum"].as_u64().map(|maximum| maximum + 1).into();
        assert!(!serde_json::from_value::<EnergyModelConfig>(probe).unwrap().is_valid(), "{field} above its maximum is admitted");
    }
    let template = EnergyModelConfig::default().simulation_template();
    let engine = SimulationConfig::default();
    assert_eq!((template.zone_timestep_minutes, template.system_timestep_minutes, template.warmup_days), (engine.zone_timestep_minutes, engine.system_timestep_minutes, engine.warmup_days), "config defaults stay in lockstep with the engine");
}

#[test]
fn the_config_round_trips_through_its_pack_and_dsl_codecs() {
    use store::{ArtifactDsl, ArtifactPack};
    let config = EnergyModelConfig { zone_timestep_minutes: 10, system_timestep_minutes: 2, warmup_days: 3 };
    assert_eq!(EnergyModelConfig::decode_pack(&config.encode_pack()).unwrap(), config);
    assert_eq!(EnergyModelConfig::parse_dsl(&config.print_dsl()).unwrap(), config);
}
