use super::*;
use super::shared::IfcDetailsProfile;
use semio_framework_ui_locale::Locale;
use semio_framework_value::DslValue;
use semio_s_artifact_stdio_contract::editing::{self,SnapshotDetailPathSegment as P,SnapshotDetailsProvider};
use semio_s_artifact_stdio_ifc_4_model::standards::v4::subsets::any::schema::snapshot::{IfcEntity,IfcSnapshot,IfcValue};
use semio_s_artifact_stdio_ifc_4_mutations::standards::v4::subsets::any::schema::mutations::IfcMutation;
fn path(index:usize,suffix:&[P])->Vec<P>{[vec![P::Key("entities".into()),P::Index(0),P::Key("args".into()),P::Index(index)],suffix.to_vec()].concat()}
#[test]
fn neutral_controls_use_the_ifc4_catalog_and_real_detail_provider(){
    let fixture:semio_framework_pack_json::Value=semio_framework_pack_json::parse(include_str!("../../../../../../../🔎️details/🧪️tests/🔣️.json"),semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
    for case in fixture["cases"].as_array().unwrap(){let name=case["entity"].as_str().unwrap();let attributes=Ifc4DetailsProfile::catalog().attribute_declarations(name).unwrap();let index=case["index"].as_u64().unwrap()as usize;let mut args=vec![IfcValue::Unset;attributes.len()];if case.get("enum").is_some(){args[index]=IfcValue::Enum("STANDARD".into());}if case.get("aggregate").is_some(){args[index]=IfcValue::Aggregate(vec![IfcValue::Integer(0)]);}if case["derived"].as_bool()==Some(true){args[index]=IfcValue::Derived;}let snapshot=IfcSnapshot{entities:vec![IfcEntity{id:1,name:name.into(),args,complex:Vec::new()}],..Default::default()};let provider=Ifc4DetailsProvider::new(&snapshot);assert!(provider.presentation(&path(index,&[]),Locale::En).unwrap().label.contains(case["attribute"].as_str().unwrap()));if case["derived"].as_bool()==Some(true){assert!(!provider.item_capabilities(&path(index,&[P::Key("value".into())])).edit);}if let Some(members)=case.get("enum"){let expected:Vec<DslValue>=members.as_array().unwrap().iter().map(|value|DslValue::String(value.as_str().unwrap().into())).collect();assert_eq!(provider.enum_values(&path(index,&[P::Key("value".into())])),expected);}}
}
#[test]
fn exact_reference_and_complete_type_variants_use_existing_mutations(){
    let snapshot=IfcSnapshot{entities:vec![IfcEntity{id:1,name:"IFCWALL".into(),args:vec![IfcValue::String("0123456789012345678901".into()),IfcValue::Unset,IfcValue::String("Wall".into()),IfcValue::Unset,IfcValue::Unset,IfcValue::Unset,IfcValue::Unset,IfcValue::Unset,IfcValue::Enum("STANDARD".into())],complex:Vec::new()},IfcEntity{id:u64::MAX,name:"IFCLOCALPLACEMENT".into(),args:vec![IfcValue::Unset,IfcValue::Unset],complex:Vec::new()}],..Default::default()};
    let event=editing::SnapshotEditEvent::SetValue{path:"/entities/0/args/5/reference".into(),value:DslValue::String(u64::MAX.to_string())};assert!(details_edit_is_admitted(&event,&snapshot));let mutations=details_edit_special(&event,&snapshot).unwrap().unwrap();assert!(matches!(&mutations[0],IfcMutation::SetEntityArg(edit)if edit.value==IfcValue::Reference(u64::MAX)));
    let provider=Ifc4DetailsProvider::new(&snapshot);let entity_path=[P::Key("entities".into()),P::Index(0)];let variant=(0..provider.variant_count(&entity_path,Locale::En)).filter_map(|index|provider.variant(&entity_path,Locale::En,index)).find(|variant|variant.label.eq_ignore_ascii_case("IFCDOOR")).unwrap();let change=editing::SnapshotEditEvent::SetValue{path:"/entities/0".into(),value:variant.value};let mutations=details_edit_special(&change,&snapshot).unwrap().unwrap();assert!(mutations.iter().any(|mutation|matches!(mutation,IfcMutation::SetEntityName(edit)if edit.id==1&&edit.name=="IFCDOOR")));
}

#[test]
fn type_switch_batch_uses_central_diff_inverse_and_checks_each_planning_stop(){
    use protocol::Mutation;
    let snapshot=IfcSnapshot{entities:vec![IfcEntity{id:1,name:"IFCWALL".into(),args:vec![IfcValue::String("0123456789012345678901".into()),IfcValue::Unset,IfcValue::String("Wall".into()),IfcValue::Unset,IfcValue::Unset,IfcValue::Unset,IfcValue::Unset,IfcValue::String("Tag".into()),IfcValue::Enum("STANDARD".into())],complex:Vec::new()}],..Default::default()};
    let event=editing::SnapshotEditEvent::SetValue{path:"/entities/0/name".into(),value:DslValue::String("IFCDOOR".into())};let mut work=0;let mutations=details_edit_special_controlled(&event,&snapshot,&mut ||{work+=1;Ok(())}).unwrap().unwrap();assert!(mutations.len()>=2);
    let mut next=snapshot.clone();let mut inverse=Vec::new();for mutation in &mutations{inverse.splice(0..0,mutation.inverse(&next).unwrap());next=protocol::apply_diff(mutation.diff(&next).diff(),&next).unwrap();}assert_eq!(next.entities[0].name,"IFCDOOR");assert_eq!(next.entities[0].args[0],snapshot.entities[0].args[0]);assert_eq!(next.entities[0].args[2],snapshot.entities[0].args[2]);for mutation in inverse{next=protocol::apply_diff(mutation.diff(&next).diff(),&next).unwrap();}assert_eq!(next,snapshot);
    for stop in 1..=work{let mut count=0;assert!(details_edit_special_controlled(&event,&snapshot,&mut ||{count+=1;if count==stop{Err("cancelled".into())}else{Ok(())}}).is_err());}
}

#[test]
fn declared_reference_target_search_observes_every_original_callback_stop(){
    let snapshot=IfcSnapshot{entities:vec![IfcEntity{id:1,name:"IFCWALL".into(),args:vec![IfcValue::Unset;9],complex:Vec::new()},IfcEntity{id:u64::MAX,name:"IFCLOCALPLACEMENT".into(),args:Vec::new(),complex:Vec::new()}],..Default::default()};let event=editing::SnapshotEditEvent::SetValue{path:"/entities/0/args/5/reference".into(),value:DslValue::String(u64::MAX.to_string())};let mut work=0;assert!(shared::normalized_reference_edit_controlled::<Ifc4DetailsProfile>(&event,&snapshot,true,&mut ||{work+=1;Ok(())}).unwrap().is_some());assert!(work>=3);for stop in 1..=work{let mut count=0;assert!(shared::normalized_reference_edit_controlled::<Ifc4DetailsProfile>(&event,&snapshot,true,&mut ||{count+=1;if count==stop{Err("cancelled".into())}else{Ok(())}}).is_err());}
}

#[test]
fn neutral_select_names_and_complex_reference_paths_use_the_version_catalog(){
    use semio_s_artifact_stdio_ifc_4_model::standards::v4::subsets::any::schema::snapshot::{IfcComplexType,IfcTypedValue};
    let fixture:semio_framework_pack_json::Value=semio_framework_pack_json::parse(include_str!("../../../../../../../🔎️details/🧪️tests/🔣️.json"),semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();let select=&fixture["select"];let snapshot=IfcSnapshot{entities:vec![IfcEntity{id:1,name:select["entity"].as_str().unwrap().into(),args:vec![IfcValue::String("Load".into()),IfcValue::Unset,IfcValue::TypedValue(IfcTypedValue{name:"IFCLABEL".into(),items:vec![IfcValue::String("Design".into())]}),IfcValue::Unset],complex:Vec::new()}],..Default::default()};let provider=Ifc4DetailsProvider::new(&snapshot);let variants=provider.variants(&path(select["index"].as_u64().unwrap()as usize,&[]),Locale::En);for member in select["requiredLeaves"].as_array().unwrap(){assert!(variants.iter().any(|variant|variant.label==member.as_str().unwrap()));}
    let row=&fixture["complexReference"];let id=row["targetId"].as_str().unwrap().parse::<u64>().unwrap();let snapshot=IfcSnapshot{entities:vec![IfcEntity{id:1,name:row["primary"].as_str().unwrap().into(),args:vec![IfcValue::Unset;4],complex:vec![IfcComplexType{name:row["constituent"].as_str().unwrap().into(),args:vec![IfcValue::Unset;2]}]},IfcEntity{id,name:"IFCROOT".into(),args:Vec::new(),complex:vec![IfcComplexType{name:row["target"].as_str().unwrap().into(),args:vec![IfcValue::Unset;2]}]}],..Default::default()};let event=editing::SnapshotEditEvent::SetValue{path:format!("/entities/0/complex/0/args/{}/reference",row["index"].as_u64().unwrap()),value:DslValue::String(id.to_string())};let mutations=details_edit_special(&event,&snapshot).unwrap().unwrap();assert!(mutations.iter().any(|mutation|matches!(mutation,IfcMutation::SetEntityComplex(edit)if edit.complex[0].args[0]==IfcValue::Reference(id))));
}
