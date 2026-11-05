//! 🎟️ Dedicated bake issuers reach the original app command-work admission gate.
use super::*;
#[test]
fn dedicated_bake_tools_are_accepted_by_the_shared_command_work_gate(){
 let fixture:serde_json::Value=serde_json::from_str(include_str!("🧫️fixtures/🎟️routes.json")).unwrap();
 for row in fixture["cases"].as_array().unwrap(){assert_eq!(raster_retained_accepts(row["tool"].as_str().unwrap()),row["accepted"].as_bool().unwrap());}
}

#[test]
fn pixel_semantic_routes_have_a_dedicated_schema_derived_resumable_contract(){
 use semio_framework::{ToolJobFactory,ToolExecutionShape};
 let law:serde_json::Value=serde_json::from_str(include_str!("../../🎮️commands/🖼️pixel-edit/🧪️tests/execution.json")).unwrap();let factory=RasterPixelEditJobFactory::new("raster");let contract=factory.execution_contract();assert_eq!(contract.shape,ToolExecutionShape::Resumable);
 assert_eq!(contract.max_raw_wire_bytes,law["maximumRawBytes"].as_u64().unwrap()as usize);assert_eq!(contract.max_decoded_items,law["maximumDecodedItems"].as_u64().unwrap()as usize);assert_eq!(contract.max_work_units_per_step,law["maximumWorkUnitsPerStep"].as_u64().unwrap());assert_eq!(contract.max_output_bytes,law["maximumOutputBytes"].as_u64().unwrap()as usize);assert_eq!(contract.max_step_micros,law["maximumStepMicros"].as_u64().unwrap()as u32);assert_eq!(contract.checkpoint_every_steps,law["checkpointEverySteps"].as_u64().unwrap()as u32);assert_eq!(contract.progress_every_steps,law["progressEverySteps"].as_u64().unwrap()as u32);
 let points=law["maximumPoints"].as_u64().unwrap();let field=law["maximumIdentifierUtf8Bytes"].as_u64().unwrap();let escape=law["maximumEscapedBytesPerUtf8Byte"].as_u64().unwrap();let raw=2*points*(law["maximumNumberBytes"].as_u64().unwrap()+1)+law["rawIdentifierFields"].as_u64().unwrap()*field*escape+law["rawSmallFieldUtf8Bytes"].as_u64().unwrap()*escape+law["rawHeaderBytes"].as_u64().unwrap();assert_eq!(raw,contract.max_raw_wire_bytes as u64);assert_eq!(2*points+law["decodedHeaderItems"].as_u64().unwrap(),contract.max_decoded_items as u64);
 let output=law["maximumPixels"].as_u64().unwrap()*(law["maximumSpanJsonBytes"].as_u64().unwrap()+1)+2+points*(law["maximumPointJsonBytes"].as_u64().unwrap()+1)+2+law["outputIdentifierFields"].as_u64().unwrap()*field*escape+law["outputStateFields"].as_u64().unwrap()*law["maximumStateUtf8Bytes"].as_u64().unwrap()*escape+law["outputHeaderBytes"].as_u64().unwrap();assert_eq!(output,contract.max_output_bytes as u64);
 for id in law["tools"].as_array().unwrap(){let id=id.as_str().unwrap();assert!(RASTER_PIXEL_EDIT_TOOL_IDS.contains(&id));assert!(!RASTER_RETAINED_TOOL_IDS.contains(&id));assert!(raster_retained_accepts(id));assert!(RasterPixelEditJobFactory::PUBLICATION_CONTRACTS.iter().any(|route|route.tool_id==id));}assert_eq!(RASTER_PIXEL_EDIT_TOOL_IDS.len(),5);
}
