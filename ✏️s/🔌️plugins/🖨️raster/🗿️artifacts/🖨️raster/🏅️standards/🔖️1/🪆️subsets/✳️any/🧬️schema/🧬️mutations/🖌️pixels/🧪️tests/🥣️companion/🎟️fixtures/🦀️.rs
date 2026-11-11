//! 🎟️ Pixel laws receive flat fixture rows through genuine original construction and retain every resulting root.
use semio_plugin::artifact_app_laws::OriginalWorkFixture;
use semio_framework_value::{DslValue,FromValue,ToValue,retirement::RetireOwned};
use semio_s_artifact_raster_raster_model::{RasterSelectionSpans,RasterSelectionSpan,snapshot::RasterSnapshot};
use semio_s_artifact_raster_raster_mutations::RasterMutation;

fn loan()->semio_framework_value::RetainedCloneGrant{semio_framework_value::RetainedCloneGrant{maximum_items:64,maximum_copy_bytes:32768,maximum_capacity_bytes:1<<20,maximum_release_bytes:1<<20,maximum_depth:4096}}
fn rows(values:&[(u32,u32,u32)])->DslValue{values.iter().map(|&(start,length,coverage)|RasterSelectionSpan{start,length,coverage}).collect::<Vec<_>>().to_value()}
fn receive(value:DslValue)->RasterSelectionSpans{let mut caller=OriginalWorkFixture::new();let output=caller.receive_native(value,loan());caller.finish();output}
pub fn selection_fixture(values:&[(u32,u32,u32)])->RasterSelectionSpans{receive(rows(values))}
pub fn retire_fixture<T:RetireOwned>(value:T){let mut caller=OriginalWorkFixture::new();caller.retire_owned(value,loan());caller.finish();}
pub fn for_pixel_cases(cases:Vec<(&'static str,RasterSnapshot,RasterMutation)>,mut run:impl FnMut(&'static str,&RasterSnapshot,&RasterMutation)){for(name,before,mutation)in cases{run(name,&before,&mutation);retire_fixture(mutation);semio_s_artifact_raster_raster_model::snapshot::retire_raster_snapshot(before);}}
pub fn mutation_fixture(text:&str)->RasterMutation{
    let mut value=semio_framework_pack_json::from_json_str::<DslValue>(text,semio_framework_pack_json::JsonMemberPolicy::Reject).expect("pixel fixture is strict JSON");
    let selection=match &mut value{DslValue::Object(fields)=>fields.iter_mut().find(|(key,_)|key=="selection").map(|(_,value)|std::mem::replace(value,DslValue::Null)),_=>None};
    let mut mutation=RasterMutation::from_value(value).expect("pixel fixture header decodes without constructing persistent roots");
    if let Some(selection)=selection.filter(|value|!matches!(value,DslValue::Null)){let selection=receive(selection);match &mut mutation{RasterMutation::PaintStroke(value)=>value.selection=Some(selection),RasterMutation::FillRegion(value)=>value.selection=Some(selection),RasterMutation::FillSelection(value)=>value.selection=Some(selection),RasterMutation::ApplyFilter(value)=>value.selection=Some(selection),_=>{retire_fixture(selection);panic!("selection fixture belongs to a pixel command")}}}mutation
}
pub fn assert_selection_refused(values:&[(u32,u32,u32)]){let mut caller=OriginalWorkFixture::new();let result=caller.try_receive_native::<DslValue,RasterSelectionSpans>(rows(values),loan());match result{Ok(value)=>{caller.retire_owned(value,loan());panic!("invalid ordered selection fixture was admitted")},Err(error)=>{assert!(error.receiving().is_some());caller.retire_owned(error,loan());}}caller.finish();}

#[test]
fn original_pixel_fixture_rows_preserve_flat_wire_and_refuse_overlap(){let fixture:serde_json::Value=serde_json::from_str(include_str!("🔣️.json")).unwrap();for value in fixture["valid"].as_array().unwrap(){let values:Vec<(u32,u32,u32)>=serde_json::from_value(value.clone()).unwrap();let selection=selection_fixture(&values);let expected=serde_json::Value::Array(values.iter().map(|&(start,length,coverage)|serde_json::json!({"start":start,"length":length,"coverage":coverage})).collect());assert_eq!(serde_json::Value::from(&selection.to_value()),expected);retire_fixture(selection);}for value in fixture["invalid"].as_array().unwrap(){let values:Vec<(u32,u32,u32)>=serde_json::from_value(value.clone()).unwrap();assert_selection_refused(&values);}}
