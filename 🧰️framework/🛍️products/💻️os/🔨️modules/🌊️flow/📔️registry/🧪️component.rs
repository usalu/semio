//! 🧪️ Real contributed registry replacement, borrowed catalogues, and maintenance ownership.

use super::*;

//#region 🧪️RetainedReplacement
#[test]
fn contributed_registry_replacement_preserves_readers_and_drains_old_versions() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("🧪️fixtures/🔣️.json")).unwrap();
    let plugin = fixture["pluginId"].as_str().unwrap();
    let manifest = serde_json::to_string(&fixture["manifest"]).unwrap();
    install_flow_extension_manifest(plugin, &manifest).unwrap();
    let reader = flow_extension_registry();
    assert_eq!(serde_json::to_value(reader.schema("owned").unwrap()).unwrap(), fixture["manifest"]["contributes"]["schemas"][0]);
    assert_eq!(serde_json::to_value(reader.operator_info("owned.echo").unwrap()).unwrap(), fixture["manifest"]["contributes"]["operators"][0]);
    let mut replacement = fixture["manifest"].clone();
    replacement["name"] = "Replacement".into();
    replacement["contributes"]["schemas"][0]["name"] = "New Schema".into();
    install_flow_extension_manifest(plugin, &replacement.to_string()).unwrap();
    assert_eq!(reader.schema("owned").unwrap().name, "Owned");
    assert_eq!(flow_extension_registry().schema("owned").unwrap().name, "New Schema");
    let catalogue: serde_json::Value = serde_json::from_str(&crate::catalogue::flow_neuron_kind_infos_json()).unwrap();
    assert!(catalogue.as_array().unwrap().iter().any(|item| item["id"] == "owned.echo"));
    assert!(!flow_catalogue_sections().is_empty());
    assert!(!crate::catalogue::flow_operator_catalogue_records().is_empty());
    assert_eq!(retire_flow_extension_registries_step(0, 64).unwrap(), neural::ValueRetirementStep::Blocked);
    for _ in 0..1000 {
        if retire_flow_extension_registries_step(1, 64).unwrap() == neural::ValueRetirementStep::Blocked { break; }
    }
    assert!(!flow_extension_state().lock().unwrap().retired.is_empty());
    drop(reader);
    uninstall_flow_extension("owned").unwrap();
    for _ in 0..100_000 {
        if retire_flow_extension_registries_step(1, 1).unwrap() == neural::ValueRetirementStep::Complete { break; }
    }
    assert!(flow_extension_state().lock().unwrap().retired.is_empty());
}

#[test]
fn registry_maintenance_does_not_initialize_a_registry() {
    assert!(FLOW_EXTENSION_STATE.get().is_none());
    assert_eq!(retire_flow_extension_registries_step(1, 64).unwrap(), neural::ValueRetirementStep::Complete);
    assert!(FLOW_EXTENSION_STATE.get().is_none());
}

struct FaultingOperator { text: String }
impl neural::Operator for FaultingOperator {
    fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> { Ok(input.clone()) }
    fn retirement_is_empty(&self) -> bool { self.text.is_empty() }
    fn retire_step(&mut self, _: usize, _: usize, values: &mut neural::ValueRetirement) -> Result<neural::ValueRetirementStep, &'static str> {
        values.text(std::mem::take(&mut self.text));
        panic!("fixture retained operator fault");
    }
}

#[test]
fn registry_maintenance_retains_cursor_outside_a_faulted_worker() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("🧪️fixtures/🔣️.json")).unwrap();
    let mut registry = neural::Registry::new();
    registry.register_operator(neural::OperatorInfo::default(), vec![OperatorImpl { schemas: vec![], operator: Box::new(FaultingOperator { text: fixture["pluginId"].as_str().unwrap().into() }) }], &[]);
    {
        let mut state = flow_extension_state().lock().unwrap();
        begin_flow_registry_replacement(&mut state).unwrap().publish(registry);
        begin_flow_registry_replacement(&mut state).unwrap().publish(neural::Registry::new());
    }
    let mut fault = None;
    for _ in 0..1000 {
        if let Err(reason) = retire_flow_extension_registries_step(1, 1) { fault = Some(reason); break; }
    }
    assert_eq!(fault, Some("flow.registry-retirement-panicked"));
    assert!(!flow_extension_state().is_poisoned());
    assert!(!flow_extension_state().lock().unwrap().retired.is_empty());
    for _ in 0..1000 {
        if retire_flow_extension_registries_step(1, 1).unwrap() == neural::ValueRetirementStep::Complete { break; }
    }
    assert!(flow_extension_state().lock().unwrap().retired.is_empty());
}

#[test]
fn registry_replacement_admission_preserves_roots_on_capacity_and_generation_exhaustion() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("🧪️fixtures/🔣️.json")).unwrap();
    let plugin = fixture["pluginId"].as_str().unwrap();
    let manifest = fixture["manifest"].to_string();
    install_flow_extension_manifest(plugin, &manifest).unwrap();
    let reader = flow_extension_registry();
    let expected = serde_json::to_value(reader.schema("owned").unwrap()).unwrap();
    let capacity = fixture["retiredCapacity"].as_u64().unwrap() as usize;
    assert_eq!(RETIRED_REGISTRY_CAPACITY, capacity);
    for _ in 1..capacity { install_flow_extension_manifest(plugin, &manifest).unwrap(); }
    let generation = flow_extension_state().lock().unwrap().generation;
    assert_eq!(install_flow_extension_manifest(plugin, &manifest), Err("flow.registry-retirement-full"));
    assert_eq!(uninstall_flow_extension("owned"), Err("flow.registry-retirement-full"));
    assert_eq!(flow_extension_state().lock().unwrap().generation, generation);
    assert_eq!(serde_json::to_value(reader.schema("owned").unwrap()).unwrap(), expected);
    assert_eq!(serde_json::to_value(flow_extension_registry().schema("owned").unwrap()).unwrap(), expected);
    drop(reader);
    for _ in 0..100_000 {
        if retire_flow_extension_registries_step(1, 64).unwrap() == neural::ValueRetirementStep::Complete { break; }
    }
    let maximum = fixture["maximumGeneration"].as_str().unwrap().parse::<u64>().unwrap();
    flow_extension_state().lock().unwrap().generation = maximum;
    assert_eq!(install_flow_extension_manifest(plugin, &manifest), Err("flow.registry-generation-exhausted"));
    assert_eq!(sync_host_flow_extension_contributions("[]"), Err("flow.registry-generation-exhausted"));
    assert_eq!(flow_extension_state().lock().unwrap().generation, maximum);
    assert_eq!(serde_json::to_value(flow_extension_registry().schema("owned").unwrap()).unwrap(), expected);
    assert!(flow_extension_state().lock().unwrap().retired.is_empty());
    flow_extension_state().lock().unwrap().generation = generation;
    sync_host_flow_extension_contributions("[]").unwrap();
    assert!(flow_extension_registry().schema("owned").is_none());
    for _ in 0..100_000 {
        if retire_flow_extension_registries_step(1, 64).unwrap() == neural::ValueRetirementStep::Complete { break; }
    }
    assert!(flow_extension_state().lock().unwrap().retired.is_empty());
}
//#endregion 🧪️RetainedReplacement
