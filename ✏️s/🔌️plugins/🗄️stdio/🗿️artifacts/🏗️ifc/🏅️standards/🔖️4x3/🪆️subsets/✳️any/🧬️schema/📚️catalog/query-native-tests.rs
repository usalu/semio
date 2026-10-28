//! 🧪️ Actual original native receiving drives the same qualified borrowed-slot corpus.
use super::*;
use semio_framework_os_kernel::NativeSnapshotDecodeOwner;
use semio_framework_value::{NativeDecodeControl,RetainedCloneGrant,ValueRefusalKind};
use semio_framework_value_native_owned::native_decoding::NativeDecodeRetirementRecipient;
use semio_s_artifact_stdio_ifc_4_model::standards::v4::subsets::any::schema::snapshot::{IfcEntity,IfcComplexType};
use std::cell::Cell;
fn grant()->RetainedCloneGrant{RetainedCloneGrant{maximum_items:1_000_000,maximum_copy_bytes:64*1024*1024,maximum_capacity_bytes:1024*1024,maximum_release_bytes:1024*1024,maximum_depth:32}}
fn fixture()->serde_json::Value{serde_json::from_str(include_str!("../../../../../../📚️catalog/🔎️query/🧪️tests/cases.json")).unwrap()}
fn source(row:&serde_json::Value)->IfcSnapshot{let values=|row:&serde_json::Value|row["args"].as_array().unwrap().iter().map(|value|IfcValue::String(value.as_str().unwrap().into())).collect();let mut entities=(0..7).map(|id|IfcEntity{id,name:"IFCREPRESENTATIONITEM".into(),args:vec![],complex:vec![]}).collect::<Vec<_>>();let entity=&row["entity"];entities.push(IfcEntity{id:71,name:entity["name"].as_str().unwrap().into(),args:values(entity),complex:entity["complex"].as_array().map_or_else(Vec::new,|parts|parts.iter().map(|part|IfcComplexType{name:part["name"].as_str().unwrap().into(),args:values(part)}).collect())});IfcSnapshot{entities,..Default::default()}}
fn status(value:IfcAttributeQueryStatus)->&'static str{match value{IfcAttributeQueryStatus::Missing=>"missing",IfcAttributeQueryStatus::Unique=>"unique",IfcAttributeQueryStatus::Ambiguous=>"ambiguous",IfcAttributeQueryStatus::Unknown=>"unknown"}}
fn close(native:&mut NativeDecodeControl<'_>){for _ in 0..10000{if !native.has_retirement_owner(){return;}native.close_retirement_recipient(grant()).unwrap();}panic!("original native query recipient did not close");}
#[test]
fn native_owner_query_reuses_neutral_exact_source_and_static_declaration_slots(){
 let data=fixture();
 for row in data["cases"].as_array().unwrap(){let snapshot=source(row);let mut observe=|_|true;let mut recipient=NativeDecodeRetirementRecipient::new();let mut native=NativeDecodeControl::new(1024*1024,&mut observe);native.install_retirement_recipient(&mut recipient).unwrap();let mut owner=NativeSnapshotDecodeOwner::new(&mut native,grant());
  let found=with_ifc4x3_native_attribute(&snapshot,7,row["name"].as_str().unwrap(),row["owner"].as_str(),31,&mut owner,|found,_|{
   assert_eq!(status(found.status),row["status"].as_str().unwrap());let mut count=0;
   for entry in found.matches.iter().flatten(){let Some(IfcValue::String(value))=entry.value else{panic!("marker absent")};assert_eq!(value,row["values"][count].as_str().unwrap());assert_eq!(entry.address.to_string(),row["pointers"][count].as_str().unwrap());let part=if entry.address.part==0{snapshot.entities[7].args.as_slice()}else{snapshot.entities[7].complex[entry.address.part-1].args.as_slice()};assert!(std::ptr::eq(entry.value.unwrap(),&part[entry.address.index]));assert!(std::ptr::eq(entry.type_expr,&entry.declaration.type_expr));assert!(entry.declaration.declared_by.eq_ignore_ascii_case(row["owner"].as_str().unwrap()));count+=1;}Ok(count)
  }).unwrap();assert_eq!(found,row["values"].as_array().unwrap().len());assert!(owner.progress().copied_items>1&&owner.progress().copied_bytes>0&&owner.progress().retained_capacity_bytes>0&&owner.progress().released_bytes>0);drop(owner);assert!(!native.has_retirement_owner());
 }
}
#[test]
fn native_owner_query_preserves_unique_declaration_with_absent_value(){
 let mut snapshot=source(&fixture()["cases"][0]);snapshot.entities[7].args.clear();let mut observe=|_|true;let mut recipient=NativeDecodeRetirementRecipient::new();let mut native=NativeDecodeControl::new(1024*1024,&mut observe);native.install_retirement_recipient(&mut recipient).unwrap();let mut owner=NativeSnapshotDecodeOwner::new(&mut native,grant());
 with_ifc4x3_native_attribute(&snapshot,7,"XDim",None,31,&mut owner,|found,_|{assert_eq!(found.status,IfcAttributeQueryStatus::Unique);let entry=found.matches[0].as_ref().unwrap();assert!(entry.value.is_none());assert_eq!(entry.address.index,3);Ok(())}).unwrap();drop(owner);assert!(!native.has_retirement_owner());
}
#[test]
fn native_owner_query_lends_actual_static_declaration_and_expression_addresses(){
 let snapshot=source(&fixture()["cases"][0]);let declaration=crate::validation::metadata::entity("IFCRECTANGLEPROFILEDEF").unwrap();let mut observe=|_|true;let mut recipient=NativeDecodeRetirementRecipient::new();let mut native=NativeDecodeControl::new(1024*1024,&mut observe);native.install_retirement_recipient(&mut recipient).unwrap();let mut owner=NativeSnapshotDecodeOwner::new(&mut native,grant());
 with_ifc4x3_native_attribute(&snapshot,7,"XDim",Some("IFCRECTANGLEPROFILEDEF"),31,&mut owner,|found,_|{let entry=found.matches[0].as_ref().unwrap();assert!(std::ptr::eq(entry.declaration,&declaration.all_attributes[3]));assert!(std::ptr::eq(entry.type_expr,&declaration.all_attributes[3].type_expr));Ok(())}).unwrap();drop(owner);assert!(!native.has_retirement_owner());
}
#[test]
fn native_owner_query_reports_original_frame_birth_and_close_heap_receipts(){
 let snapshot=source(&fixture()["cases"][0]);let mut observe=|_|true;let mut recipient=NativeDecodeRetirementRecipient::new();let mut native=NativeDecodeControl::new(1024*1024,&mut observe);native.install_retirement_recipient(&mut recipient).unwrap();let mut owner=NativeSnapshotDecodeOwner::new(&mut native,grant());
 let(result,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||with_ifc4x3_native_attribute(&snapshot,7,"Position",Some("IFCPARAMETERIZEDPROFILEDEF"),31,&mut owner,|found,_|{assert_eq!(found.status,IfcAttributeQueryStatus::Unique);Ok(())}));result.unwrap();let progress=owner.progress();assert_eq!(heap.requested_bytes,progress.retained_capacity_bytes);assert_eq!(heap.released_bytes,progress.released_bytes);assert!(heap.requested_bytes>0);drop(owner);assert!(!native.has_retirement_owner());
}
#[test]
fn native_owner_query_closed_lifecycle_guards_use_only_original_grants(){
 let contract:serde_json::Value=serde_json::from_str(include_str!("../../../../../../📚️catalog/🔎️query/native/cases.json")).unwrap();let snapshot=source(&fixture()["cases"][0]);
 for law in contract["laws"].as_array().unwrap(){let id=law["id"].as_str().unwrap();if !matches!(id,"no-items"|"no-copy"|"no-capacity"|"no-depth"|"cancel-entry"){continue;}let mut budget=grant();match id{"no-items"=>budget.maximum_items=0,"no-copy"=>budget.maximum_copy_bytes=0,"no-capacity"=>budget.maximum_capacity_bytes=0,"no-depth"=>budget.maximum_depth=0,_=>{}}let mut observe=|_|id!="cancel-entry";let mut recipient=NativeDecodeRetirementRecipient::new();let mut native=NativeDecodeControl::new(1024*1024,&mut observe);native.install_retirement_recipient(&mut recipient).unwrap();let mut owner=NativeSnapshotDecodeOwner::new(&mut native,budget);let mut published=false;let result=with_ifc4x3_native_attribute(&snapshot,7,"Position",Some("IFCPARAMETERIZEDPROFILEDEF"),31,&mut owner,|_,_|{published=true;Ok(())});assert!(result.is_err());assert!(!published);if id=="cancel-entry"{assert_eq!(result.unwrap_err().kind,ValueRefusalKind::Canceled);}drop(owner);if id!="cancel-entry"{close(&mut native);}assert!(!native.has_retirement_owner());
 }
}
#[test]
fn native_owner_query_preserves_prior_receipts_and_retains_source_error_prefix(){
 let snapshot=source(&fixture()["cases"][0]);let mut observe=|_|true;let mut recipient=NativeDecodeRetirementRecipient::new();let mut native=NativeDecodeControl::new(1024*1024,&mut observe);native.install_retirement_recipient(&mut recipient).unwrap();let mut owner=NativeSnapshotDecodeOwner::new(&mut native,grant());
 with_ifc4x3_native_attribute(&snapshot,7,"Position",Some("IFCPARAMETERIZEDPROFILEDEF"),31,&mut owner,|_,_|Ok(())).unwrap();let before=owner.progress();
 assert!(with_ifc4x3_native_attribute(&snapshot,8,"Position",Some("IFCPARAMETERIZEDPROFILEDEF"),31,&mut owner,|_,_|Ok(())).is_err());assert!(owner.progress().copied_items>before.copied_items&&owner.progress().copied_bytes>before.copied_bytes);drop(owner);assert!(native.has_retirement_owner());close(&mut native);
}
#[test]
fn native_owner_query_restores_the_original_finite_parent_stage(){
 let snapshot=source(&fixture()["cases"][0]);for row in [7,8]{let last=Cell::new((0usize,0usize));let child=Cell::new(false);let mut observe=|progress:semio_framework_value_native_owned::native_decoding::NativeDecodeProgress|{last.set((progress.completed,progress.total));if progress.total==0{child.set(true);}true};let mut recipient=NativeDecodeRetirementRecipient::new();let mut native=NativeDecodeControl::new(1024*1024,&mut observe);native.install_retirement_recipient(&mut recipient).unwrap();native.begin_stage(1).unwrap();
 {let mut owner=NativeSnapshotDecodeOwner::new(&mut native,grant());let result=with_ifc4x3_native_attribute(&snapshot,row,"Position",Some("IFCPARAMETERIZEDPROFILEDEF"),31,&mut owner,|_,_|Ok(()));assert_eq!(result.is_ok(),row==7);assert!(owner.progress().copied_items>1);}assert!(child.get());close(&mut native);native.step().unwrap();native.checkpoint().unwrap();assert_eq!(last.get(),(1,1));assert!(!native.has_retirement_owner());}
}
#[test]
fn native_owner_query_every_native_observer_prefix_cancels_and_closes_original(){
 let data=fixture();let row=data["cases"].as_array().unwrap().iter().find(|row|row["id"]=="unknown-after-duplicates").unwrap();let snapshot=source(row);let total=Cell::new(0usize);
 {let mut observe=|_|{total.set(total.get()+1);true};let mut recipient=NativeDecodeRetirementRecipient::new();let mut native=NativeDecodeControl::new(1024*1024,&mut observe);native.install_retirement_recipient(&mut recipient).unwrap();let mut owner=NativeSnapshotDecodeOwner::new(&mut native,grant());with_ifc4x3_native_attribute(&snapshot,7,row["name"].as_str().unwrap(),row["owner"].as_str(),31,&mut owner,|found,_|{assert_eq!(found.status,IfcAttributeQueryStatus::Unknown);Ok(())}).unwrap();}
 for stop in 1..=total.get(){let events=Cell::new(0usize);let mut observe=|_|{events.set(events.get()+1);events.get()!=stop};let mut recipient=NativeDecodeRetirementRecipient::new();let mut native=NativeDecodeControl::new(1024*1024,&mut observe);native.install_retirement_recipient(&mut recipient).unwrap();let mut owner=NativeSnapshotDecodeOwner::new(&mut native,grant());let result=with_ifc4x3_native_attribute(&snapshot,7,row["name"].as_str().unwrap(),row["owner"].as_str(),31,&mut owner,|_,_|Ok(()));assert_eq!(result.unwrap_err().kind,ValueRefusalKind::Canceled);drop(owner);close(&mut native);}
}
#[test]
fn native_owner_query_owned_output_is_retained_before_original_publication_cancellation(){
 let snapshot=source(&fixture()["cases"][0]);let refuse=Cell::new(false);let mut observe=|_|!refuse.replace(false);let mut recipient=NativeDecodeRetirementRecipient::new();let mut native=NativeDecodeControl::new(1024*1024,&mut observe);native.install_retirement_recipient(&mut recipient).unwrap();let mut owner=NativeSnapshotDecodeOwner::new(&mut native,grant());
 let result=with_ifc4x3_native_attribute(&snapshot,7,"Position",Some("IFCPARAMETERIZEDPROFILEDEF"),31,&mut owner,|_,original|{
  let output=original.receive::<Vec<u8>,Vec<u8>>(|slot,native,body|{*slot=Some(Vec::new());body.allocate_vec_into(native,128,slot.as_mut().unwrap(),1)?;Ok(slot.take().unwrap())})?;refuse.set(true);Ok(output)
 });assert_eq!(result.unwrap_err().kind,ValueRefusalKind::Canceled);assert!(owner.progress().retained_capacity_bytes>=128);drop(owner);assert!(native.has_retirement_owner());close(&mut native);
}

