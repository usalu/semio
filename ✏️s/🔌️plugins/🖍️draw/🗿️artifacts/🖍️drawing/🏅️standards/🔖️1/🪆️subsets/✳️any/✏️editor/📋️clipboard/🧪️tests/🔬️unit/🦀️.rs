//! 🧪️ Clipboard fixtures preserve world placement, complete identities and atomic inverse restoration.
use super::*;
fn shape(id:&str)->DrawingLayerNode {let mut shape=crate::schema::create_drawing_shape_layer_rect(crate::schema::identity::DrawingIdentity::admit(((id)).to_string().into()).expect("nonempty authored identity"), id);layer_base_mut(&mut shape).id=id.into();shape}
fn apply(document:&mut DrawingSnapshot,mutations:&[DrawingMutation])->Vec<Vec<DrawingMutation>> {let mut inverses=Vec::new();for mutation in mutations {inverses.push(crate::mutations::inverse_drawing_mutation(document,mutation).unwrap());crate::standards::v1::subsets::any::io::text::mutations::apply_drawing_mutation(document,mutation).unwrap();}inverses}
#[test]
fn drawing_clipboard_dependency_fixture_and_repeated_identity_are_complete() {
    let a=shape("a");let b=shape("b");let mut result=crate::schema::create_drawing_boolean_layer(crate::schema::identity::DrawingIdentity::admit((("result")).to_string().into()).expect("nonempty authored identity"), "result","difference",vec!["a".into(),"b".into()].into());layer_base_mut(&mut result).id="result".into();
    let mut document=DrawingSnapshot {layers:vec![a,b,result].into(),..Default::default()};let before=document.clone();
    let fragment=copy(&document,&["result".into()]).unwrap();let packet=decode(&fragment).unwrap();assert_eq!(packet.roots.len(),3);assert_eq!(packet.selected,vec!["result"]);
    let mut selections=Vec::new();
    for _ in 0..2 {let (mutations,selected)=admitted_paste(&document,decode(&fragment).unwrap(),&PastePlacement {position:Some([0.0;3]),..Default::default()},None).unwrap();assert_eq!(selected.len(),1);let inverses=apply(&mut document,&mutations);let DrawingLayerNode::Boolean(body)=find_drawing_layer(&document,selected[0].as_str()).unwrap() else {panic!("Expected Boolean")};for reference in &body.children {assert!(find_drawing_layer(&document,reference).is_some());assert!(reference!="a" && reference!="b");}selections.push(selected[0].clone());if selections.len()==1 {let mut undo=document.clone();for inverse in inverses.into_iter().rev().flatten(){crate::standards::v1::subsets::any::io::text::mutations::apply_drawing_mutation(&mut undo,&inverse).unwrap();}assert_eq!(undo,before);}}
    assert_ne!(selections[0],selections[1]);let ids=crate::schema::flatten_drawing_layers(&document.layers).into_iter().map(|node|layer_base(node).id.to_string_owner()).collect::<BTreeSet<_>>();assert_eq!(ids.len(),9);
}

#[test]
fn drawing_clipboard_asset_entry_admission_preserves_original_samples_and_backing(){
 use semio_framework_value::{NativeDecodeControl,ToValue};
 let fixture:serde_json::Value=serde_json::from_str(include_str!("../../../../../🧱️structure/🧫️fixtures/🧬️mutations/📥️import-image-asset/➕️adds/📸️snapshot/➡️after/🔣️.json")).unwrap();
 let asset:DrawingImageAsset=serde_json::from_value(fixture["assets"]["bitmap"].clone()).unwrap();let mut candidate=Some(("bitmap".to_owned(),asset));let mut assets=DrawingClipboardAssets::default();let mut accepted=|_|true;
 let mut denied=NativeDecodeControl::new(0,&mut accepted);let(result,event)=semio_framework_trace::observe_heap_allocations_on_this_thread(||assets.append_candidate(&mut candidate,&mut denied));assert!(result.is_err());assert_eq!((event.requested_bytes,event.released_bytes),(0,0));assert!(assets.is_empty());assert!(candidate.is_some());
 let mut admitted=NativeDecodeControl::new(job::MAX_NATIVE_BYTES,&mut accepted);let(result,event)=semio_framework_trace::observe_heap_allocations_on_this_thread(||assets.append_candidate(&mut candidate,&mut admitted));result.unwrap();assert_eq!(event.requested_bytes,admitted.owned_bytes());assert_eq!(event.released_bytes,0);assert!(candidate.is_none());assert_eq!(semio_framework_pack_json::to_json_string(&assets.to_value()),serde_json::to_string(&fixture["assets"]).unwrap());
 let original=assets.entry_at(0).unwrap().1.clone();let mut duplicate=Some(("bitmap".to_owned(),original));let mut ordinal=0;let(result,event)=semio_framework_trace::observe_heap_allocations_on_this_thread(||assets.append_candidate_step(&mut duplicate,&mut ordinal,&mut admitted));assert!(result.is_err());assert!(duplicate.is_some());assert_eq!((event.requested_bytes,event.released_bytes),(0,0));retire(duplicate);
 let mut second=Some(("second".to_owned(),assets.entry_at(0).unwrap().1.clone()));let mut ordinal=0;let(result,event)=semio_framework_trace::observe_heap_allocations_on_this_thread(||assets.append_candidate_step(&mut second,&mut ordinal,&mut admitted));assert!(!result.unwrap());assert_eq!(ordinal,1);assert!(second.is_some());assert_eq!((event.requested_bytes,event.released_bytes),(0,0));let(result,event)=semio_framework_trace::observe_heap_allocations_on_this_thread(||assets.append_candidate_step(&mut second,&mut ordinal,&mut admitted));assert!(result.unwrap());assert!(second.is_none());assert_eq!(event.released_bytes,0);assert_eq!(ordinal,0);
 let key=assets.entry_at(0).unwrap().0.as_ptr();let(actual,event)=semio_framework_trace::observe_heap_allocations_on_this_thread(||std::mem::replace(assets.entry_at_mut(0).unwrap().1,DrawingImageAsset{width:0,height:0,samples:Default::default()}));assert_eq!((event.requested_bytes,event.released_bytes),(0,0));assert_eq!(assets.entry_at(0).unwrap().0.as_ptr(),key);assert_eq!(actual.samples[0],[13,27,89,255]);assert_eq!(assets.len(),2);retire(actual);retire(assets);
 eprintln!("[DEBUG] Native packet asset entry retains original samples on zero admission and retains actual key/page backing during mutation transfer");
}
#[test]
fn drawing_clipboard_world_placement_matches_neutral_fixtures() {
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/🔣️.json")).unwrap();
    let mut child=shape("child");layer_base_mut(&mut child).transform.x=10.0;layer_base_mut(&mut child).transform.y=10.0;
    let mut group=crate::schema::create_drawing_group_layer(crate::schema::identity::DrawingIdentity::admit((("parent")).to_string().into()).expect("nonempty authored identity"), "parent");layer_base_mut(&mut group).id="parent".into();layer_base_mut(&mut group).transform.x=10.0;layer_base_mut(&mut group).transform.y=10.0;layer_base_mut(&mut group).transform.scale_x=2.0;layer_base_mut(&mut group).transform.scale_y=3.0;
    let DrawingLayerNode::Group(body)=&mut group else {unreachable!()};body.children.push(child);
    let source=DrawingSnapshot {layers:vec![group].into(),..Default::default()};let fragment=copy(&source,&["child".into()]).unwrap();
    let mut destination=crate::schema::create_drawing_group_layer(crate::schema::identity::DrawingIdentity::admit((("target")).to_string().into()).expect("nonempty authored identity"), "target");layer_base_mut(&mut destination).id="target".into();layer_base_mut(&mut destination).transform.x=-7.0;layer_base_mut(&mut destination).transform.y=8.0;layer_base_mut(&mut destination).transform.rotation=0.4;layer_base_mut(&mut destination).transform.scale_x=1.7;layer_base_mut(&mut destination).transform.scale_y=-0.5;layer_base_mut(&mut destination).transform.shear=0.8;
    for case in fixture["placements"].as_array().unwrap() {let mut target=DrawingSnapshot {layers:vec![destination.clone()].into(),..Default::default()};let placement=PastePlacement {position:serde_json::from_value(case["position"].clone()).unwrap(),..Default::default()};let (mutations,selection)=admitted_paste(&target,decode(&fragment).unwrap(),&placement,Some("target")).unwrap();apply(&mut target,&mutations);let matrix=world(&target,&selection[0]).unwrap();let expected:[f64;2]=serde_json::from_value(case["expected"].clone()).unwrap();assert!((matrix[4]-expected[0]).abs()<1e-10);assert!((matrix[5]-expected[1]).abs()<1e-10);assert!((matrix[0]-2.0).abs()<1e-10);assert!((matrix[3]-3.0).abs()<1e-10);}
}
#[test]
fn drawing_clipboard_assets_and_cut_inverse_are_atomic() {
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/🔣️.json")).unwrap();let asset:DrawingImageAsset=serde_json::from_value(fixture["asset"].clone()).unwrap();
    let mut image=crate::schema::create_drawing_image_layer(crate::schema::identity::DrawingIdentity::admit((("image")).to_string().into()).expect("nonempty authored identity"), "image","bitmap");layer_base_mut(&mut image).id="image".into();
    let mut source=DrawingSnapshot {layers:vec![image].into(),assets:[("bitmap".into(),asset.clone())].into_iter().collect(),..Default::default()};let before=source.clone();let fragment=copy(&source,&["image".into()]).unwrap();let operations=cut(&source,&["image".into()]).unwrap();let inverses=apply(&mut source,&operations);assert!(source.layers.is_empty());for inverse in inverses.into_iter().rev().flatten(){crate::standards::v1::subsets::any::io::text::mutations::apply_drawing_mutation(&mut source,&inverse).unwrap();}assert_eq!(before,source);
    let mut target=DrawingSnapshot::default();let original=target.clone();let (operations,selection)=admitted_paste(&target,decode(&fragment).unwrap(),&PastePlacement::default(),None).unwrap();assert_eq!(operations.len(),2);let inverses=apply(&mut target,&operations);let DrawingLayerNode::Image(image)=find_drawing_layer(&target,selection[0].as_str()).unwrap() else {unreachable!()};assert_eq!(target.assets.get(&image.image_key),Some(&asset));for inverse in inverses.into_iter().rev().flatten(){crate::standards::v1::subsets::any::io::text::mutations::apply_drawing_mutation(&mut target,&inverse).unwrap();}assert_eq!(target,original);
    layer_base_mut(&mut source.layers[0]).locked=true;assert!(cut(&source,&["image".into()]).is_err());assert!(copy(&source,&["missing".into()]).is_err());
}
fn retire<T:semio_framework_value::retirement::RetireOwned>(value:T){
    use semio_framework_value::{retirement::controlled::ControlledRetirement,retained_clone::RetainedCloneGrant};
    let mut owner=ControlledRetirement::new(value).unwrap_or_else(|_|panic!("Clipboard owner must declare retirement"));
    for _ in 0..1_000_000 {if owner.terminal_is_empty(){return;}let copy=owner.next_copy_byte_demand().unwrap();let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:copy,maximum_capacity_bytes:owner.next_capacity_byte_demand(copy).unwrap(),maximum_release_bytes:owner.next_release_byte_demand().unwrap(),maximum_depth:owner.next_depth_demand().unwrap()};let step=owner.step(grant).unwrap();assert!(step.progress().fits(grant));}
    panic!("Clipboard retirement did not settle");
}
#[test]
fn drawing_clipboard_native_codec_matches_serde_and_resumes_large_text(){
    use semio_framework_value::{ToValue,NativeEncodeControl};
    let mut text=crate::schema::create_drawing_text_layer(crate::schema::identity::DrawingIdentity::admit((("Text")).to_string().into()).expect("nonempty authored identity"), "Text");layer_base_mut(&mut text).id="text".into();
    let DrawingLayerNode::Text(body)=&mut text else{unreachable!()};body.content=("α\n\"\\".repeat(16384)).into();
    let packet=DrawingClipboard{schema:SCHEMA.into(),roots:vec![text],selected:vec!["text".into()],assets:Default::default()};
    let expected=serde_json::to_value(&packet).unwrap();
    let mut encoder=semio_framework_pack_json::JsonWriteCursor::new(encode::ClipboardJsonSource{packet:packet.clone()});let mut receipt=None;let mut turns=0;
    let output=loop{let mut accepted=|_|true;let mut control=if let Some(receipt)=receipt.take(){NativeEncodeControl::resume(receipt,&mut accepted).unwrap()}else{NativeEncodeControl::new(MAX_BYTES,&mut accepted)};let output=encoder.step(17,&mut control).unwrap();receipt=Some(control.pause().unwrap());turns+=1;if let Some(output)=output{break output;}};
    assert!(turns>100);assert_eq!(serde_json::from_str::<serde_json::Value>(&output).unwrap(),expected);retire(encoder);
    let mut accepted=|_|true;let mut control=semio_framework_value::NativeDecodeControl::new(job::MAX_NATIVE_BYTES,&mut accepted);
    let mut hydration=decode::ClipboardHydration::new(packet.to_value());let mut turns=0;while !hydration.step(&mut control).unwrap(){turns+=1;assert!(turns<100000);}assert!(turns>64);assert_eq!(hydration.packet,packet);retire(hydration);
    let mut cancelled=decode::ClipboardHydration::new(packet.to_value());for _ in 0..32{assert!(!cancelled.step(&mut control).unwrap());}retire(cancelled);
}
#[test]
fn drawing_clipboard_boolean_operand_displacement_survives_parent_change(){
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/🔣️.json")).unwrap();let expected=&fixture["booleanPlacements"][0];
    let mut a=shape("a");layer_base_mut(&mut a).transform.x=3.0;
    let mut b=shape("b");layer_base_mut(&mut b).transform.x=5.0;
    let mut result=crate::schema::create_drawing_boolean_layer(crate::schema::identity::DrawingIdentity::admit((("result")).to_string().into()).expect("nonempty authored identity"), "result","union",vec!["a".into(),"b".into()].into());layer_base_mut(&mut result).id="result".into();layer_base_mut(&mut result).transform.x=7.0;
    let mut parent=crate::schema::create_drawing_group_layer(crate::schema::identity::DrawingIdentity::admit((("parent")).to_string().into()).expect("nonempty authored identity"), "parent");layer_base_mut(&mut parent).id="parent".into();layer_base_mut(&mut parent).transform.x=100.0;layer_base_mut(&mut parent).transform.scale_x=2.0;
    let DrawingLayerNode::Group(group)=&mut parent else{unreachable!()};group.children=vec![a,b,result].into();
    let source=DrawingSnapshot{layers:vec![parent].into(),..Default::default()};let packet=decode(&copy(&source,&["result".into()]).unwrap()).unwrap();
    let result=packet.roots.iter().find(|node|layer_base(node).id=="result").unwrap();let displacement=crate::schema::drawing_transform_to_matrix(&layer_base(result).transform);assert!((displacement[4]-14.0).abs()<1e-10);
    let mut target=crate::schema::create_drawing_group_layer(crate::schema::identity::DrawingIdentity::admit((("target")).to_string().into()).expect("nonempty authored identity"), "target");layer_base_mut(&mut target).id="target".into();layer_base_mut(&mut target).transform.x=-31.0;layer_base_mut(&mut target).transform.scale_x=3.0;
    let mut document=DrawingSnapshot{layers:vec![target].into(),..Default::default()};let (operations,selection)=admitted_paste(&document,packet,&PastePlacement{position:Some([0.0;3]),..Default::default()},Some("target")).unwrap();apply(&mut document,&operations);
    let DrawingLayerNode::Boolean(result)=find_drawing_layer(&document,&selection[0]).unwrap()else{unreachable!()};let local=crate::schema::drawing_transform_to_matrix(&result.base.transform);assert!((local[4]-expected["expectedLocal"][4].as_f64().unwrap()).abs()<1e-10);
}

#[test]
fn drawing_clipboard_hydration_preserves_admitted_identity_and_depth_refusal_owners(){
    use semio_framework_value::{ToValue,NativeDecodeControl};
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/🔣️.json")).unwrap();let cases=fixture["codecCases"].as_array().unwrap();
    let identity="a".repeat(cases[0]["identityBytes"].as_u64().unwrap()as usize);let a=shape(&identity);let b=shape("b");let mut boolean=crate::schema::create_drawing_boolean_layer(crate::schema::identity::DrawingIdentity::admit((("Result")).to_string().into()).expect("nonempty authored identity"), "Result","union",vec![identity.clone().into(),"b".into()].into());layer_base_mut(&mut boolean).id="result".into();
    let packet=DrawingClipboard{schema:SCHEMA.into(),roots:vec![a,b,boolean],selected:vec!["result".into()],assets:Default::default()};let mut accepted=|_|true;let mut control=NativeDecodeControl::new(job::MAX_NATIVE_BYTES,&mut accepted);let mut hydration=decode::ClipboardHydration::new(packet.to_value());while !hydration.step(&mut control).unwrap(){}assert_eq!(hydration.packet,packet);retire(hydration);
    let mut node=shape("leaf");for index in 0..cases[1]["depth"].as_u64().unwrap(){let mut group=crate::schema::create_drawing_group_layer(crate::schema::identity::DrawingIdentity::admit(((&format!("Group {index}"))).to_string().into()).expect("nonempty authored identity"), &format!("Group {index}"));layer_base_mut(&mut group).id=format!("group-{index}").into();let DrawingLayerNode::Group(body)=&mut group else{unreachable!()};body.children.push(node);node=group;}
    let selected=layer_base(&node).id.to_string_owner();let packet=DrawingClipboard{schema:SCHEMA.into(),roots:vec![node],selected:vec![selected],assets:Default::default()};let mut hydration=decode::ClipboardHydration::new(packet.to_value());let error=loop{match hydration.step(&mut control){Ok(false)=>{},Ok(true)=>panic!("Deep clipboard tree must be refused"),Err(error)=>break error}};assert_eq!(error.kind,semio_framework_value::ValueRefusalKind::InvalidValue);retire(hydration);
    eprintln!("[DEBUG] Native clipboard long identity {} bytes retained; depth {} refused with original partial owner; no publication",identity.len(),cases[1]["depth"]);
}
fn graph_layer(id:&str,nodes:&[serde_json::Value])->DrawingLayerNode{
    let row=nodes.iter().find(|row|row["id"]==id).unwrap();let edges=row["edges"].as_array().unwrap();
    let mut node=match row["kind"].as_str().unwrap(){
        "group"=>{let mut node=crate::schema::create_drawing_group_layer(crate::schema::identity::DrawingIdentity::admit(((id)).to_string().into()).expect("nonempty authored identity"), id);let DrawingLayerNode::Group(group)=&mut node else{unreachable!()};for child in edges{group.children.push(graph_layer(child.as_str().unwrap(),nodes));}node},
        "boolean"=>crate::schema::create_drawing_boolean_layer(crate::schema::identity::DrawingIdentity::admit(((id)).to_string().into()).expect("nonempty authored identity"), id,"union",edges.iter().map(|id|id.as_str().unwrap().into()).collect()),
        _=>shape(id)
    };layer_base_mut(&mut node).id=id.into();node
}
#[test]
fn drawing_clipboard_dependency_admission_replays_independent_graph_fixtures(){
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/🔣️.json")).unwrap();
    for row in fixture["dependencyCases"].as_array().unwrap(){let nodes=row["nodes"].as_array().unwrap();let packet=DrawingClipboard{schema:SCHEMA.into(),roots:row["roots"].as_array().unwrap().iter().map(|id|graph_layer(id.as_str().unwrap(),nodes)).collect(),selected:serde_json::from_value(row["selected"].clone()).unwrap(),assets:Default::default()};let before=packet.clone();assert_eq!(validate(&packet).is_ok(),row["accepted"].as_bool().unwrap(),"{}",row["name"]);assert_eq!(packet,before);}
    eprintln!("[DEBUG] Native clipboard graph admission replays neutral graphlib dependency/cycle witness rows");
}
#[test]
fn drawing_clipboard_maximum_body_roundtrips_shared_wire_envelope(){
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/🔣️.json")).unwrap();
    for row in fixture["envelopeCases"].as_array().unwrap(){let scalar=row["scalar"].as_str().unwrap();let text=scalar.repeat(row["bodyBytes"].as_u64().unwrap()as usize/scalar.len());assert_eq!(text.len(),semio_framework_plugin::kernel::CLIPBOARD_TEXT_MAX_BYTES);let actual=semio_framework_pack_json::to_json_string(&semio_framework_value::DslValue::String(text.clone()));assert_eq!(actual,serde_json::to_string(&text).unwrap());assert!(actual.len()+1024<=semio_framework_plugin::kernel::CLIPBOARD_FRAGMENT_MAX_WIRE_BYTES);let roundtrip:String=serde_json::from_str(&actual).unwrap();assert_eq!(roundtrip,text);}
    eprintln!("[DEBUG] Native maximum clipboard body JSON escape witness fits the shared producer/consumer wire envelope");
}

fn admitted_paste(document:&DrawingSnapshot,packet:DrawingClipboard,placement:&PastePlacement,parent:Option<&str>)->Result<(Vec<DrawingMutation>,Vec<String>),ClipboardError>{let mut observer=|_|true;let mut control=semio_framework_value::NativeEncodeControl::new(1024*1024,&mut observer);super::paste(document,packet,placement,parent,&mut control)}
