use super::*;
use crate::energy_simulation_session::{encode_surface_energy_payload, energy_surface_payload_bytes, ENERGY_SURFACE_PAYLOAD_MAXIMUM_ROWS};
use crate::results::SurfaceEnergySummary;

fn summary(id: u32, loss: f64, gain: f64, transmitted: f64, absorbed: f64) -> SurfaceEnergySummary {
    SurfaceEnergySummary { id: crate::model::EntityId(id), conduction_loss_kwh: loss, conduction_gain_kwh: gain, solar_transmitted_kwh: transmitted, solar_absorbed_kwh: absorbed }
}

#[test]
fn a_published_payload_round_trips_back_into_the_same_per_surface_map() {
    let rows = vec![summary(1, 412.25, 0.5, 0.0, 12.5), summary(7, 0.0, 33.0, 180.0, 44.0), summary(4_000_000_001, 1.0, 2.0, 3.0, 4.0)];
    let bytes = encode_surface_energy_payload(rows.iter().copied());
    assert_eq!(bytes.len(), energy_surface_payload_bytes(rows.len()), "the wire length is exactly header + rows");
    let map = decode_run_payload(&bytes).expect("a well-formed payload decodes");
    assert_eq!(map.len(), rows.len());
    for row in &rows {
        let decoded = map.get(row.id.0).expect("every encoded row decodes");
        // f32 on the wire: assert to single-precision, not to the f64 the engine accumulated in.
        for (decoded, original) in [
            (decoded.conduction_loss_kwh, row.conduction_loss_kwh),
            (decoded.conduction_gain_kwh, row.conduction_gain_kwh),
            (decoded.solar_transmitted_kwh, row.solar_transmitted_kwh),
            (decoded.solar_absorbed_kwh, row.solar_absorbed_kwh),
        ] {
            assert!((decoded - original).abs() <= original.abs() * 1e-6 + 1e-6, "{decoded} != {original}");
        }
    }
}

#[test]
fn a_payload_never_outgrows_the_tick_budget_even_for_two_thousand_surfaces() {
    let rows: Vec<_> = (0..2_000).map(|id| summary(id, f64::from(id), 0.0, 0.0, 0.0)).collect();
    let bytes = encode_surface_energy_payload(rows.iter().copied());
    assert_eq!(bytes.len(), 40_008, "2 000 surfaces are 8 header bytes plus 2 000 × 20");
    assert!(bytes.len() * 4 < semio_framework_tool_run::TOOL_RUN_TICK_BYTES_MAX, "2 000 surfaces leave at least a 4× margin inside one tick");
    assert_eq!(decode_run_payload(&bytes).expect("decodes").len(), 2_000);
    // The hard ceiling itself still fits, with room for the progress record and the step ring.
    assert!(energy_surface_payload_bytes(ENERGY_SURFACE_PAYLOAD_MAXIMUM_ROWS) < semio_framework_tool_run::TOOL_RUN_TICK_BYTES_MAX);
}

#[test]
fn the_encoder_truncates_at_its_own_row_ceiling() {
    let rows: Vec<_> = (0..(ENERGY_SURFACE_PAYLOAD_MAXIMUM_ROWS as u32 + 17)).map(|id| summary(id, 1.0, 0.0, 0.0, 0.0)).collect();
    let bytes = encode_surface_energy_payload(rows.into_iter());
    assert_eq!(bytes.len(), energy_surface_payload_bytes(ENERGY_SURFACE_PAYLOAD_MAXIMUM_ROWS));
    assert_eq!(decode_run_payload(&bytes).expect("decodes").len(), ENERGY_SURFACE_PAYLOAD_MAXIMUM_ROWS);
}

#[test]
fn malformed_and_absent_payloads_are_refused_rather_than_panicking() {
    assert_eq!(decode_run_payload(&[]), None, "empty bytes");
    assert_eq!(decode_run_payload(b"ESF"), None, "shorter than the header");
    assert_eq!(decode_run_payload(b"XXXX\0\0\0\0"), None, "wrong magic");
    let mut truncated = encode_surface_energy_payload([summary(1, 1.0, 0.0, 0.0, 0.0), summary(2, 2.0, 0.0, 0.0, 0.0)].into_iter());
    truncated.truncate(truncated.len() - 3);
    assert_eq!(decode_run_payload(&truncated), None, "a body shorter than its own row count");
    assert_eq!(surface_energy_from_run(None), None, "no run at all");
    let empty = encode_surface_energy_payload(std::iter::empty());
    assert_eq!(decode_run_payload(&empty).expect("an empty map is still well formed").len(), 0);
}

#[test]
fn the_colour_ramp_puts_the_extremes_on_its_own_end_stops() {
    let rows = [summary(1, 0.0, 0.0, 0.0, 0.0), summary(2, 200.0, 0.0, 0.0, 0.0), summary(3, 400.0, 0.0, 0.0, 0.0)];
    let map = decode_run_payload(&encode_surface_energy_payload(rows.into_iter())).expect("decodes");
    let (colors, min, max) = surface_colors(&map, ResultField::ConductionLoss);
    assert_eq!((min, max), (0.0, 400.0));
    assert_eq!(colors.len(), 3);
    assert_eq!(colors[&1], hex_to_rgb01(SURFACE_ENERGY_BANDS[0]), "the least loss takes the coldest band");
    assert_eq!(colors[&3], hex_to_rgb01(SURFACE_ENERGY_BANDS[7]), "the greatest loss takes the hottest band");
    assert_ne!(colors[&2], colors[&1], "a middle value does not collapse onto an end stop");
    assert_eq!(band_color(-5.0, 0.0, 400.0), SURFACE_ENERGY_BANDS[0], "below the range clamps to the first band");
    assert_eq!(band_color(9_999.0, 0.0, 400.0), SURFACE_ENERGY_BANDS[7], "above the range clamps to the last band");
}

#[test]
fn an_empty_map_colours_nothing_and_a_flat_map_still_colours() {
    let (colors, min, max) = surface_colors(&SurfaceEnergyMap::default(), ResultField::ConductionLoss);
    assert!(colors.is_empty());
    assert_eq!((min, max), (0.0, 0.0));
    let flat = decode_run_payload(&encode_surface_energy_payload([summary(1, 5.0, 0.0, 0.0, 0.0), summary(2, 5.0, 0.0, 0.0, 0.0)].into_iter())).expect("decodes");
    let (colors, min, max) = surface_colors(&flat, ResultField::ConductionLoss);
    assert_eq!(colors.len(), 2);
    assert_eq!((min, max), (5.0, 5.0));
    assert_eq!(colors[&1], colors[&2]);
}

#[test]
fn every_field_reads_its_own_column_and_its_own_wire_id() {
    let map = decode_run_payload(&encode_surface_energy_payload([summary(9, 1.0, 2.0, 3.0, 4.0)].into_iter())).expect("decodes");
    let energy = *map.get(9).expect("row 9");
    for (field, expected, id) in [
        (ResultField::ConductionLoss, 1.0, "conductionLoss"),
        (ResultField::ConductionGain, 2.0, "conductionGain"),
        (ResultField::SolarTransmitted, 3.0, "solarTransmitted"),
        (ResultField::SolarAbsorbed, 4.0, "solarAbsorbed"),
    ] {
        assert_eq!(energy.field(field), expected, "{id}");
        assert_eq!(field.id(), id);
        assert_eq!(ResultField::from_id(id), Some(field));
    }
    assert_eq!(ResultField::from_id("nope"), None);
    assert_eq!(ResultField::default(), ResultField::ConductionLoss);
}

#[test]
fn the_config_selects_the_field_and_an_unknown_string_falls_back_to_the_default() {
    assert_eq!(result_field(&EnergyModelConfig::default()), ResultField::ConductionLoss);
    let mut cfg = EnergyModelConfig::default();
    cfg.result_field = "solarTransmitted".into();
    assert_eq!(result_field(&cfg), ResultField::SolarTransmitted);
    cfg.result_field = "not-a-field".into();
    assert_eq!(result_field(&cfg), ResultField::ConductionLoss);
}

#[test]
fn the_legend_names_the_field_and_its_range() {
    assert_eq!(legend_caption(ResultField::ConductionLoss, 0.0, 412.34), "Conduction loss · 0.0 – 412.3 kWh");
    assert_eq!(legend_caption(ResultField::SolarTransmitted, 1.25, 9.0), "Solar transmitted · 1.2 – 9.0 kWh");
}
