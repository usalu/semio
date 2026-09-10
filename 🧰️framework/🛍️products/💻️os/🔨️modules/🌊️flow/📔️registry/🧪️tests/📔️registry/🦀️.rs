//! 🧪️ Real contributed registry replacement, borrowed catalogues, and maintenance ownership.

use super::*;

fn third_party_json<T: crate::os_dsl::ToValue>(value: &T) -> serde_json::Value {
    serde_json::from_str(&crate::os_pack::json::to_json_string(value)).expect("first-party JSON must remain valid RFC 8259")
}

//#region 🧪️RetainedReplacement
#[test]
fn contributed_registry_replacement_preserves_readers_and_drains_old_versions() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🔣️.json")).unwrap();
    let plugin = fixture["pluginId"].as_str().unwrap();
    let manifest = serde_json::to_string(&fixture["manifest"]).unwrap();
    install_flow_extension_manifest(plugin, &manifest).unwrap();
    let reader = flow_extension_registry();
    assert_eq!(third_party_json(reader.schema("owned").unwrap()), fixture["manifest"]["contributes"]["schemas"][0]);
    assert_eq!(third_party_json(reader.operator_info("owned.echo").unwrap()), fixture["manifest"]["contributes"]["operators"][0]);
    let mut replacement = fixture["manifest"].clone();
    replacement["name"] = "Replacement".into();
    replacement["contributes"]["schemas"][0]["name"] = "New Schema".into();
    install_flow_extension_manifest(plugin, &replacement.to_string()).unwrap();
    assert_eq!(reader.schema("owned").unwrap().name, "Owned");
    assert_eq!(flow_extension_registry().schema("owned").unwrap().name, "New Schema");
    let catalogue: serde_json::Value = serde_json::from_str(&flow_neuron_kind_infos_json()).unwrap();
    assert!(catalogue.as_array().unwrap().iter().any(|item| item["id"] == "owned.echo"));
    assert!(!flow_catalogue_sections().is_empty());
    assert!(!flow_operator_catalogue_records().is_empty());
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
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🔣️.json")).unwrap();
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
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🔣️.json")).unwrap();
    let plugin = fixture["pluginId"].as_str().unwrap();
    let manifest = fixture["manifest"].to_string();
    install_flow_extension_manifest(plugin, &manifest).unwrap();
    let reader = flow_extension_registry();
    let expected = third_party_json(reader.schema("owned").unwrap());
    let capacity = fixture["retiredCapacity"].as_u64().unwrap() as usize;
    assert_eq!(RETIRED_REGISTRY_CAPACITY, capacity);
    for _ in 1..capacity { install_flow_extension_manifest(plugin, &manifest).unwrap(); }
    let generation = flow_extension_state().lock().unwrap().generation;
    assert_eq!(install_flow_extension_manifest(plugin, &manifest), Err("flow.registry-retirement-full"));
    assert_eq!(uninstall_flow_extension("owned"), Err("flow.registry-retirement-full"));
    assert_eq!(flow_extension_state().lock().unwrap().generation, generation);
    assert_eq!(third_party_json(reader.schema("owned").unwrap()), expected);
    assert_eq!(third_party_json(flow_extension_registry().schema("owned").unwrap()), expected);
    drop(reader);
    for _ in 0..100_000 {
        if retire_flow_extension_registries_step(1, 64).unwrap() == neural::ValueRetirementStep::Complete { break; }
    }
    let maximum = fixture["maximumGeneration"].as_str().unwrap().parse::<u64>().unwrap();
    flow_extension_state().lock().unwrap().generation = maximum;
    assert_eq!(install_flow_extension_manifest(plugin, &manifest), Err("flow.registry-generation-exhausted"));
    assert_eq!(sync_host_flow_extension_contributions("[]".to_string()), Err("flow.registry-generation-exhausted"));
    assert_eq!(flow_extension_state().lock().unwrap().generation, maximum);
    assert_eq!(third_party_json(flow_extension_registry().schema("owned").unwrap()), expected);
    assert!(flow_extension_state().lock().unwrap().retired.is_empty());
    flow_extension_state().lock().unwrap().generation = generation;
    sync_host_flow_extension_contributions("[]".to_string()).unwrap();
    assert!(flow_extension_registry().schema("owned").is_none());
    for _ in 0..100_000 {
        if retire_flow_extension_registries_step(1, 64).unwrap() == neural::ValueRetirementStep::Complete { break; }
    }
    assert!(flow_extension_state().lock().unwrap().retired.is_empty());
}
//#endregion 🧪️RetainedReplacement

//#region 🪪️InvocationAddress
/// ⚖️ LAW: a contributed operator's pending-extension request is addressed by the CONTRIBUTING
/// PLUGIN's id, never by the flow manifest's own extension id.
///
/// The host resolves an extension actor by `pluginId` alone (`dispatchInvokeExtensionEffect`,
/// `🏛️ShellHost/🟦️.tsx`), and the framework's own invocation fixture
/// (`🏛️ShellHost/🧫️fixtures/🔣️extension-invocation.json`) pins that same shape. Raising
/// `manifest.id` here made every browser evaluation fault `extension.missing` and stall the tick
/// chain at its first extension node (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
#[test]
fn contributed_operators_are_addressed_by_their_contributing_plugin_id() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🔣️.json")).unwrap();
    let plugin_id = fixture["pluginId"].as_str().unwrap();
    let flow_extension_id = fixture["manifest"]["id"].as_str().unwrap();
    let operator_id = fixture["manifest"]["contributes"]["operators"][0]["id"].as_str().unwrap();
    assert_ne!(plugin_id, flow_extension_id, "the fixture must keep the two ids distinct or this law proves nothing");
    install_flow_extension_manifest(plugin_id, &fixture["manifest"].to_string()).unwrap();
    let registry = flow_extension_registry();
    let input = neural::ColdOwner::new(Dictionary::new());
    let pending = registry.dispatch(operator_id, &input).expect_err("a contributed operator must defer to its owning plugin");
    match pending {
        EvalError::PendingExtension { extension_id, operator_id: pending_operator, .. } => {
            assert_eq!(extension_id, plugin_id, "the invocation address must be the contributing plugin id, not the flow extension id");
            assert_eq!(pending_operator, operator_id);
        }
        other => panic!("expected a pending-extension deferral, got {other:?}"),
    }
    assert_eq!(flow_extension_invocation_address(flow_extension_id).as_deref(), Ok(plugin_id));
    let miss = flow_extension_invocation_address("not-contributed").expect_err("an uncontributed extension id has no invocation address");
    assert_eq!(miss.extension_id, "not-contributed");
    assert!(miss.contributed.contains(&(flow_extension_id.to_string(), plugin_id.to_string())), "the miss names every live translation: {:?}", miss.contributed);
    let (english, german) = miss.labels();
    assert!(english.contains("not-contributed") && german.contains("not-contributed"), "both languages name the extension id");
    drop(registry);
    uninstall_flow_extension(flow_extension_id).unwrap();
    for _ in 0..100_000 {
        if retire_flow_extension_registries_step(1, 64).unwrap() == neural::ValueRetirementStep::Complete { break; }
    }
}
//#endregion 🪪️InvocationAddress
