//! 🧪️ Borrowed dynamic choices preserve exact neutral labels and refuse malformed source rows.
use super::*;
use semio_ui_contract::OriginalBuiltTreeRetirement;
use std::sync::atomic::{AtomicUsize,Ordering};
struct Choices{rows:Vec<(String,String)>,reads:AtomicUsize}
impl StagedScalarChoiceSource for Choices{
    fn len(&self)->usize{self.rows.len()}
    fn choice(&self,index:usize)->Option<StagedScalarChoice<'_>>{self.reads.fetch_add(1,Ordering::Relaxed);self.rows.get(index).map(|(key,label)|StagedScalarChoice{key,label})}
}
fn grant()->RetainedCloneGrant{RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:1<<20,maximum_capacity_bytes:1<<20,maximum_release_bytes:1<<20,maximum_depth:512}}
fn retire(cursor:&mut StagedScalarCursor){for _ in 0..16384{if matches!(cursor.close(grant()).unwrap(),RetainedCloneStep::Complete(_)){assert!(cursor.terminal_is_empty());return}}panic!("control did not retire")}
fn retire_node(node:BuiltNode){let mut owner=OriginalBuiltTreeRetirement::new(node);for _ in 0..16384{if owner.terminal_is_empty(){return}owner.step(grant()).unwrap();}panic!("node did not retire")}
#[test]
fn borrowed_dynamic_scalar_choices_match_neutral_json_and_reject_invalid_rows(){
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../../../../../../../🔨️modules/🛂️manifest/🧮️arg-schema/📝️staging/choice-source-cases.json")).unwrap();
    for row in fixture["cases"].as_array().unwrap(){
        let source=Choices{rows:row["choices"].as_array().unwrap().iter().map(|choice|(choice["key"].as_str().unwrap().into(),choice["label"].as_str().unwrap().into())).collect(),reads:AtomicUsize::new(0)};
        let spec=StagedScalarControl{owner:&source as*const _ as usize,address:7,controller:"norm",label:"Choice",description:None,disabled:false,kind:StagedScalarKind::Choice{value:row["value"].as_str().unwrap(),choices:&source}};
        let mut cursor=StagedScalarCursor::default();let mut args=Some(UiValue::Null);let mut output=None;let mut accepted=false;
        for _ in 0..512{match cursor.step(&spec,&mut args,&mut output,grant()){Err(_)=>break,Ok(RetainedCloneStep::Complete(_))=>{accepted=true;break},Ok(_)=>{}}}
        assert_eq!(accepted,row["allowed"].as_bool().unwrap(),"{}",row["id"]);
        if let Some(node)=output{let Component::Select(props)=&node.component else{panic!("select missing")};let actual=props.items.iter().map(|choice|serde_json::json!({"key":choice.value.as_ref(),"label":choice.label.0.as_ref()})).collect::<Vec<_>>();assert_eq!(serde_json::to_value(actual).unwrap(),row["choices"]);assert_eq!(props.value.as_ref(),row["value"].as_str().unwrap());retire_node(node);}
        retire(&mut cursor);
    }
}
#[test]
fn borrowed_dynamic_scalar_choices_do_not_read_under_denied_grants_and_cancel_every_prefix(){
    let source=Choices{rows:vec![("a".into(),"Größe".into()),("b".into(),"Width".into())],reads:AtomicUsize::new(0)};
    let spec=StagedScalarControl{owner:&source as*const _ as usize,address:9,controller:"norm",label:"Choice",description:None,disabled:false,kind:StagedScalarKind::Choice{value:"a",choices:&source}};
    for cut in 0..16{
        let mut cursor=StagedScalarCursor::default();let mut args=Some(UiValue::Null);let mut output=None;
        for _ in 0..cut{
            for denied in [RetainedCloneGrant{maximum_items:0,..grant()},RetainedCloneGrant{maximum_copy_bytes:0,..grant()},RetainedCloneGrant{maximum_depth:0,..grant()}]{
                let reads=source.reads.load(Ordering::Relaxed);let result=cursor.step(&spec,&mut args,&mut output,denied).unwrap();assert_eq!(result.progress(),RetainedCloneProgress::default());assert_eq!(reads,source.reads.load(Ordering::Relaxed));
            }
            if matches!(cursor.step(&spec,&mut args,&mut output,grant()).unwrap(),RetainedCloneStep::Complete(_)){break}
        }
        if let Some(node)=output{retire_node(node);}retire(&mut cursor);
    }
}
