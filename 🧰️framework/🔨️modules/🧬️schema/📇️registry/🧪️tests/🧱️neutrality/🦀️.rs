//! 📇️ Public registry operations match independently authored atomic snapshots and mirror refusals.

use semio_framework_schema_registry::*;
use serde_json::{json, Value};
use std::sync::{mpsc, Arc, Barrier};
use std::time::Duration;

const CORPUS: &str = include_str!("../../../🧫️fixtures/🧱️neutrality/🔣️.json");

fn text(value: &Value) -> &'static str {
    Box::leak(value.as_str().expect("authored string").to_string().into_boxed_str())
}

fn leaves(value: &Value) -> FacetLeaves {
    FacetLeaves { rust: text(&value["rust"]), typescript: text(&value["typescript"]), graphql: text(&value["graphql"]), json_schema: text(&value["json_schema"]), proto: text(&value["proto"]) }
}

fn inference(value: &Value) -> ArtifactInferenceDescriptor {
    ArtifactInferenceDescriptor { id: text(&value["id"]), inference: leaves(&value["inference"]) }
}

fn leaf_json(value: FacetLeaves) -> Value {
    json!({"rust": value.rust, "typescript": value.typescript, "graphql": value.graphql, "json_schema": value.json_schema, "proto": value.proto})
}

#[test]
fn concurrent_conflicting_batches_publish_one_complete_independent_snapshot() {
    let corpus: Value = serde_json::from_str(CORPUS).expect("closed portable corpus");
    let schedules = corpus["concurrentSchedules"].as_array().expect("two schedules");
    assert_eq!(schedules.len(), 2);
    let actors = schedules[0]["actors"].as_array().expect("two actors");
    assert_eq!(actors.len(), 2);
    let ids: Vec<_> = actors.iter().flat_map(|actor| actor["descriptors"].as_array().expect("descriptors")).map(|descriptor| descriptor["id"].as_str().expect("id").to_string()).collect();
    assert!(with_artifact_inference_registry(|registry| ids.iter().all(|id| registry.get(id).is_none())));
    let barrier = Arc::new(Barrier::new(3));
    let handles: Vec<_> = actors.iter().map(|actor| {
        let name = actor["id"].as_str().expect("actor").to_string();
        let descriptors = actor["descriptors"].as_array().expect("batch").iter().map(inference).collect();
        let barrier = barrier.clone();
        std::thread::spawn(move || { barrier.wait(); (name, register_artifact_inference_descriptors(descriptors)) })
    }).collect();
    barrier.wait();
    let results: Vec<_> = handles.into_iter().map(|handle| handle.join().expect("owner thread")).collect();
    assert_eq!(results.iter().filter(|(_, result)| result.is_ok()).count(), 1);
    assert_eq!(results.iter().filter(|(_, result)| result.is_err()).count(), 1);
    let winner = &results.iter().find(|(_, result)| result.is_ok()).expect("winner").0;
    let failure = results.iter().find_map(|(_, result)| result.as_ref().err()).expect("conflict");
    assert_eq!(failure.registry, "artifact-inference");
    assert_eq!(failure.id, "same");
    let actual = with_artifact_inference_catalog(|entries| entries.iter().filter(|descriptor| ids.iter().any(|id| id == descriptor.id)).map(|descriptor| json!({"id": descriptor.id, "inference": leaf_json(descriptor.inference)})).collect::<Vec<_>>());
    let expected = schedules.iter().find(|schedule| schedule["expected"]["decisions"].as_array().expect("decisions").iter().any(|decision| decision["actor"].as_str() == Some(winner.as_str()) && decision["accepted"] == true)).expect("independent winning schedule");
    assert_eq!(Value::Array(actual), expected["expected"]["entries"]);
    for (actor, result) in results {
        let decision = expected["expected"]["decisions"].as_array().expect("decisions").iter().find(|decision| decision["actor"] == actor).expect("actor decision");
        assert_eq!(result.is_ok(), decision["accepted"].as_bool().expect("accepted"));
    }
}

#[test]
fn artifact_mirror_conflict_refuses_descriptor_and_batch_prefix_publication() {
    let corpus: Value = serde_json::from_str(CORPUS).expect("closed corpus");
    let rows = corpus["catalogs"].as_array().expect("catalogs");
    for (case_id, prefix) in [("artifact-mirror-export-conflict", "schema.native.mirror.single"), ("artifact-batch-mirror-export-conflict", "schema.native.mirror.batch")] {
        let row = rows.iter().find(|row| row["id"] == case_id).expect("independent mirror vector");
        let established = &row["exports"][0];
        register_scope_facet_leaves(prefix, [leaves(&established["artifact"]), leaves(&established["snapshot"]), leaves(&established["diff"]), leaves(&established["mutations"])]).expect("established mirror");
        let descriptors: Vec<_> = row["descriptors"].as_array().expect("batch").iter().map(|value| ArtifactSchemaDescriptor { id: if value["id"] == "a" { prefix } else { "schema.native.mirror.losing-prefix" }, artifact: leaves(&value["artifact"]), snapshot: leaves(&value["snapshot"]), diff: leaves(&value["diff"]), mutations: leaves(&value["mutations"]) }).collect();
        let result = if row["mode"] == "single" { register_artifact_schema_descriptor(descriptors[0]) } else { register_artifact_schema_descriptors(descriptors) };
        let error = result.expect_err("actual mirror refusal");
        assert_eq!(error.registry, "schema-export");
        assert_eq!(error.id, prefix);
        assert!(!artifact_schema_descriptor_registered(prefix));
        assert!(!artifact_schema_descriptor_registered("schema.native.mirror.losing-prefix"));
        assert!(!scope_schema_facets_registered("schema.native.mirror.losing-prefix"));
        with_schema_export_registry(|registry| {
            for facet in RESERVED_FACET_EXPORT_IDS { assert_eq!(registry.leaves(prefix, facet).expect("unchanged mirror"), leaves(&established[facet])); }
        });
    }
}

#[test]
fn user_catalog_callbacks_can_publish_and_read_without_retaining_the_owner_lock() {
    let (send, receive) = mpsc::channel();
    std::thread::spawn(move || {
        with_artifact_inference_catalog(|_| {
            let empty = FacetLeaves { rust: "", typescript: "", graphql: "", json_schema: "", proto: "" };
            register_app_schema_descriptor(AppSchemaDescriptor { id: "schema.native.callback.app", config: empty, presence: empty }).expect("reentrant publication");
            with_app_schema_catalog(|entries| assert!(entries.iter().any(|entry| entry.id == "schema.native.callback.app")));
            with_schema_export_registry(|_| assert!(app_schema_descriptor_registered("schema.native.callback.app")));
        });
        send.send(()).expect("completion");
    });
    receive.recv_timeout(Duration::from_secs(2)).expect("callback must release owner lock");
}

fn public_artifact(value:&Value,prefix:&str)->ArtifactSchemaDescriptor {
 ArtifactSchemaDescriptor{id:Box::leak(format!("{prefix}.{}",value["id"].as_str().unwrap()).into_boxed_str()),artifact:leaves(&value["artifact"]),snapshot:leaves(&value["snapshot"]),diff:leaves(&value["diff"]),mutations:leaves(&value["mutations"])}
}
fn public_inference(value:&Value,prefix:&str)->ArtifactInferenceDescriptor {
 ArtifactInferenceDescriptor{id:Box::leak(format!("{prefix}.{}",value["id"].as_str().unwrap()).into_boxed_str()),inference:leaves(&value["inference"])}
}
fn public_app(value:&Value,prefix:&str)->AppSchemaDescriptor {
 AppSchemaDescriptor{id:Box::leak(format!("{prefix}.{}",value["id"].as_str().unwrap()).into_boxed_str()),config:leaves(&value["config"]),presence:leaves(&value["presence"])}
}
fn public_artifact_json(descriptor:ArtifactSchemaDescriptor,prefix:&str)->Value {
 json!({"id":descriptor.id.strip_prefix(prefix).unwrap(),"artifact":leaf_json(descriptor.artifact),"snapshot":leaf_json(descriptor.snapshot),"diff":leaf_json(descriptor.diff),"mutations":leaf_json(descriptor.mutations)})
}
#[test]
fn all_authored_catalog_vectors_execute_real_public_initial_publication_and_candidate_operations(){
 let corpus:Value=serde_json::from_str(CORPUS).unwrap();let rows=corpus["catalogs"].as_array().unwrap();assert_eq!(rows.len(),19);
 for row in rows {
  let prefix=format!("schema.native.public.{}",row["id"].as_str().unwrap());let namespace=format!("{prefix}.");let family=row["family"].as_str().unwrap();
  for input in row["initial"].as_array().unwrap(){match family{"artifact"=>register_artifact_schema_descriptor(public_artifact(input,&prefix)),"inference"=>register_artifact_inference_descriptor(public_inference(input,&prefix)),"app"=>register_app_schema_descriptor(public_app(input,&prefix)),_=>panic!("closed family")}.unwrap();}
  for input in row["exports"].as_array().unwrap(){let descriptor=public_artifact(input,&prefix);register_scope_facet_leaves(descriptor.id,descriptor.facet_leaves()).unwrap();}
  let inputs=row["descriptors"].as_array().unwrap();let batch=row["mode"]=="batch";
  let result=match family{
   "artifact"=>{let items:Vec<_>=inputs.iter().map(|v|public_artifact(v,&prefix)).collect();if batch{register_artifact_schema_descriptors(items)}else{register_artifact_schema_descriptor(items[0])}},
   "inference"=>{let items:Vec<_>=inputs.iter().map(|v|public_inference(v,&prefix)).collect();if batch{register_artifact_inference_descriptors(items)}else{register_artifact_inference_descriptor(items[0])}},
   "app"=>{let items:Vec<_>=inputs.iter().map(|v|public_app(v,&prefix)).collect();if batch{register_app_schema_descriptors(items)}else{register_app_schema_descriptor(items[0])}},_=>panic!("closed family")
  };
  assert_eq!(result.is_ok(),row["expected"]["accepted"].as_bool().unwrap(),"{}",row["id"]);
  if let Err(error)=result{assert_eq!(error.registry,if row["expected"]["error"]=="export-conflict"{"schema-export"}else{match family{"artifact"=>"artifact-schema","inference"=>"artifact-inference","app"=>"app-schema",_=>unreachable!()}});}
  let actual:Vec<Value>=match family{
   "artifact"=>with_artifact_schema_catalog(|entries|entries.iter().filter(|d|d.id.starts_with(namespace.as_str())).map(|d|public_artifact_json(*d,&namespace)).collect()),
   "inference"=>with_artifact_inference_catalog(|entries|entries.iter().filter(|d|d.id.starts_with(namespace.as_str())).map(|d|json!({"id":d.id.strip_prefix(namespace.as_str()).unwrap(),"inference":leaf_json(d.inference)})).collect()),
   "app"=>with_app_schema_catalog(|entries|entries.iter().filter(|d|d.id.starts_with(namespace.as_str())).map(|d|json!({"id":d.id.strip_prefix(namespace.as_str()).unwrap(),"config":leaf_json(d.config),"presence":leaf_json(d.presence)})).collect()),_=>unreachable!()
  };
  assert_eq!(Value::Array(actual),row["expected"]["entries"],"{} descriptor snapshot",row["id"]);
  let mut ids:Vec<_>=row["initial"].as_array().unwrap().iter().chain(row["exports"].as_array().unwrap()).chain(inputs).map(|v|v["id"].as_str().unwrap()).collect();ids.sort();ids.dedup();
  let mirrors=with_schema_export_registry(|registry|ids.iter().filter_map(|id|{let full=format!("{prefix}.{id}");let artifact=registry.leaves(&full,"artifact").ok()?;Some(json!({"id":id,"artifact":leaf_json(artifact),"snapshot":leaf_json(registry.leaves(&full,"snapshot").unwrap()),"diff":leaf_json(registry.leaves(&full,"diff").unwrap()),"mutations":leaf_json(registry.leaves(&full,"mutations").unwrap())}))}).collect::<Vec<_>>());
  assert_eq!(Value::Array(mirrors),row["expected"]["exports"],"{} mirror snapshot",row["id"]);
 }
}
