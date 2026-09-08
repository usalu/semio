
use super::*;

#[test]
fn mixer_blends_by_mass_flow() {
    let nodes = vec![FluidNode { id: 0, temperature_c: 10.0, humidity_ratio: 0.005, pressure_pa: 101_325.0, mass_flow_kg_s: 1.0 }, FluidNode { id: 1, temperature_c: 30.0, humidity_ratio: 0.015, pressure_pa: 101_325.0, mass_flow_kg_s: 1.0 }];
    let mixer = Mixer { id: 0, inlets: vec![0, 1], outlet: 2 };
    let out = mixer.blend(&nodes);
    assert!((out.temperature_c - 20.0).abs() < 1e-9);
    assert!((out.mass_flow_kg_s - 2.0).abs() < 1e-9);
}

#[test]
fn splitter_preserves_mass() {
    let inlet = FluidNode { id: 0, temperature_c: 20.0, humidity_ratio: 0.01, pressure_pa: 101_325.0, mass_flow_kg_s: 2.0 };
    let splitter = Splitter { id: 0, inlet: 0, outlets: vec![(1, 0.6), (2, 0.4)] };
    let outs = splitter.distribute(&inlet);
    let m_sum: f64 = outs.iter().map(|n| n.mass_flow_kg_s).sum();
    assert!((m_sum - 2.0).abs() < 1e-9);
}

#[test]
fn valid_topology_passes() {
    let nodes = vec![FluidNode::new(0), FluidNode::new(1)];
    let branches = vec![Branch { id: 0, inlet: 0, outlet: 1, component: BranchComponent::Bypass }];
    let diag = validate_topology(&nodes, &branches, &[], &[]);
    assert!(!diag.has_fatal());
}

#[test]
fn branch_with_invalid_node_index_is_fatal() {
    let nodes = vec![FluidNode::new(0)];
    let branches = vec![Branch { id: 0, inlet: 0, outlet: 5, component: BranchComponent::Bypass }];
    let diag = validate_topology(&nodes, &branches, &[], &[]);
    assert!(diag.has_fatal());
}

#[test]
fn branch_with_identical_inlet_outlet_is_severe() {
    let nodes = vec![FluidNode::new(0), FluidNode::new(1)];
    let branches = vec![Branch { id: 0, inlet: 0, outlet: 0, component: BranchComponent::Bypass }];
    let diag = validate_topology(&nodes, &branches, &[], &[]);
    assert!(!diag.has_fatal());
    assert_eq!(diag.messages.len(), 1);
}

#[test]
fn splitter_fraction_mismatch_warns() {
    let nodes = vec![FluidNode::new(0), FluidNode::new(1), FluidNode::new(2)];
    let splitters = vec![Splitter { id: 0, inlet: 0, outlets: vec![(1, 0.3), (2, 0.3)] }];
    let diag = validate_topology(&nodes, &[], &splitters, &[]);
    assert_eq!(diag.messages.len(), 1);
}

#[test]
fn splitter_invalid_inlet_is_fatal() {
    let nodes = vec![FluidNode::new(0)];
    let splitters = vec![Splitter { id: 0, inlet: 9, outlets: vec![(0, 1.0)] }];
    let diag = validate_topology(&nodes, &[], &splitters, &[]);
    assert!(diag.has_fatal());
}

#[test]
fn mixer_invalid_inlet_and_outlet_are_fatal() {
    let nodes = vec![FluidNode::new(0)];
    let mixers = vec![Mixer { id: 0, inlets: vec![9], outlet: 8 }];
    let diag = validate_topology(&nodes, &[], &[], &mixers);
    assert!(diag.has_fatal());
    assert_eq!(diag.messages.len(), 2);
}

#[test]
fn mixer_blend_with_zero_flow_returns_default_node() {
    let nodes = [FluidNode { id: 0, temperature_c: 10.0, humidity_ratio: 0.005, pressure_pa: 101_325.0, mass_flow_kg_s: 0.0 }];
    let mixer = Mixer { id: 0, inlets: vec![0], outlet: 7 };
    let out = mixer.blend(&nodes);
    assert_eq!(out.id, 7);
    assert_eq!(out.mass_flow_kg_s, 0.0);
}

#[test]
fn air_loop_validate_flags_invalid_zone_outlet() {
    let loop_topo = AirLoop {
        id: 0,
        name: "AL".into(),
        nodes: vec![FluidNode::new(0), FluidNode::new(1)],
        branches: vec![],
        splitters: vec![],
        mixers: vec![],
        supply_inlet: 0,
        supply_outlet: 1,
        return_inlet: 0,
        return_outlet: 1,
        zone_outlets: vec![9],
        zone_returns: vec![],
    };
    let diag = loop_topo.validate();
    assert!(!diag.messages.is_empty());
}

#[test]
fn plant_loop_validate_delegates_to_validate_topology() {
    let loop_topo = PlantLoop {
        id: 0,
        name: "PL".into(),
        fluid: PlantFluid::Water,
        nodes: vec![FluidNode::new(0), FluidNode::new(1)],
        branches: vec![Branch { id: 0, inlet: 0, outlet: 1, component: BranchComponent::Bypass }],
        splitters: vec![],
        mixers: vec![],
        supply_inlet: 0,
        supply_outlet: 1,
        demand_inlet: 1,
        demand_outlet: 0,
    };
    assert!(!loop_topo.validate().has_fatal());
}

#[test]
fn branch_pressure_drop_for_each_component_kind() {
    let inlet = FluidNode { id: 0, temperature_c: 60.0, humidity_ratio: 0.008, pressure_pa: 200_000.0, mass_flow_kg_s: 1.0 };
    let outlet = FluidNode { id: 1, temperature_c: 40.0, humidity_ratio: 0.008, pressure_pa: 150_000.0, mass_flow_kg_s: 1.0 };

    let duct = Branch { id: 0, inlet: 0, outlet: 1, component: BranchComponent::Duct { hydraulic_diameter_m: 0.3, length_m: 10.0 } };
    assert!(duct.pressure_drop_pa(&inlet, &outlet) > 0.0);

    let pipe = Branch { id: 1, inlet: 0, outlet: 1, component: BranchComponent::Pipe { diameter_m: 0.05, length_m: 20.0 } };
    assert!(pipe.pressure_drop_pa(&inlet, &outlet) > 0.0);

    let pump = Branch { id: 2, inlet: 0, outlet: 1, component: BranchComponent::Pump { design_head_pa: 100_000.0, design_flow_kg_s: 1.0 } };
    assert!(pump.pressure_drop_pa(&inlet, &outlet) < 0.0);

    let coil = Branch { id: 3, inlet: 0, outlet: 1, component: BranchComponent::Coil { ua_w_per_k: 500.0 } };
    assert!(coil.pressure_drop_pa(&inlet, &outlet) > 0.0);

    let valve = Branch { id: 4, inlet: 0, outlet: 1, component: BranchComponent::Valve { cv: 10.0 } };
    assert!(valve.pressure_drop_pa(&inlet, &outlet) > 0.0);

    let bypass = Branch { id: 5, inlet: 0, outlet: 1, component: BranchComponent::Bypass };
    assert_eq!(bypass.pressure_drop_pa(&inlet, &outlet), 5.0);
}
