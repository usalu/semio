use super::*;
use serde_json::{json, Value};

#[test]
fn portable_coordinate_matches_complete_original_production_descriptor_oracle() {
    let fixture: Value = serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();
    assert_eq!(fixture["coordinates"].as_array().unwrap().len(), CONTRIBUTIONS.len());
    let descriptors = <semio_s_artifact_space_space::standards::v1::subsets::any::schema::mutations::SSpaceMutation as Mutation<semio_s_artifact_space_space::standards::v1::subsets::any::schema::snapshot::SSpaceSnapshot>>::DESCRIPTORS;
    let coordinate = &CONTRIBUTIONS[0].coordinate;
    let row = &fixture["coordinates"][0];
    assert_eq!(coordinate.artifact, row["artifact"].as_str().unwrap());
    assert_eq!(coordinate.standard, row["standard"].as_str().unwrap());
    assert_eq!(coordinate.subset, row["subset"].as_str().unwrap());
    assert_eq!(coordinate.owner, row["owner"].as_str().unwrap());
    assert_eq!(coordinate.surface, None);
    let prefix = format!("{}/", coordinate.owner);
    let mut seen = std::collections::HashSet::new();
    let rows = descriptors.iter().filter(|descriptor| descriptor.owner.starts_with(&prefix) && !descriptor.owner[prefix.len()..].split('/').any(|segment| fixture["excludedOwnerSegments"].as_array().unwrap().iter().any(|value| value.as_str() == Some(segment))) && seen.insert(descriptor.semantic_kind)).map(|descriptor| json!({"id":descriptor.semantic_kind,"variant":descriptor.aggregate_variant,"outcomes":descriptor.outcome_classes.iter().map(|class| class.as_str()).collect::<Vec<_>>() })).collect::<Vec<_>>();
    assert!(!rows.is_empty());
    let expected = json!({"schema":"semio.repository-test.runtime-inventory/v2","artifact":coordinate.artifact,"standard":coordinate.standard,"subset":coordinate.subset,"bridgeVersion":1,"producedBy":fixture["producedBy"],"mutations":rows});
    let args = arguments(coordinate.artifact);
    let output = inventory_for_command(PRODUCED_BY, CONTRIBUTIONS, &args, &mut |_| Ok(())).unwrap();
    assert_eq!(serde_json::from_slice::<Value>(&output).unwrap(), expected);
    println!("[DEBUG] child={} mutations={} bytes={}", coordinate.artifact, expected["mutations"].as_array().unwrap().len(), output.len());
}

#[test]
fn child_refuses_actual_sibling_and_retains_progress_cancellation() {
    let fixture: Value = serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();
    let sibling = fixture["refusedArtifacts"][0].as_str().unwrap();
    assert_eq!(inventory_for_command(PRODUCED_BY, CONTRIBUTIONS, &arguments(sibling), &mut |_| Ok(())).unwrap_err(), MutationInventoryError::Coordinate);
    let mut callbacks = 0;
    let mut observe = |_| { callbacks += 1; Err(MutationInventoryError::Cancelled) };
    assert_eq!(inventory_for_command(PRODUCED_BY, CONTRIBUTIONS, &arguments(CONTRIBUTIONS[0].coordinate.artifact), &mut observe).unwrap_err(), MutationInventoryError::Cancelled);
    assert_eq!(callbacks, 1);
    println!("[DEBUG] child={} refusedSibling={} cancellationCallbacks={}", CONTRIBUTIONS[0].coordinate.artifact, sibling, callbacks);
}

fn arguments(artifact: &str) -> Vec<String> {
    ["list-mutations",artifact,"1","any","--maximum-units","10000000","--maximum-owned-bytes","1048576","--budget-ms","60000"].into_iter().map(String::from).collect()
}
