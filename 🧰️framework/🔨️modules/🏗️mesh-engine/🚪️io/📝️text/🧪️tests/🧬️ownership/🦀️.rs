//! 🧪️ Neutral native mesh admission and physical closure grants.
use super::*;
use pack::value::retained_clone::{RetainedCloneGrant,RetainedCloneProgress};
fn step(cursor:&mut PolygonSourcePreparation,text:&str,grant:PolygonSourceGrant)->Result<PolygonSourceStep,String>{crate::component_reference_custody_tests::fx(grant.retirement,|effect|cursor.step(text,grant,effect))}
#[test]
fn neutral_polygon_io_preserves_original_parser_on_short_grants(){
 let law:serde_json::Value=serde_json::from_str(crate::test_inputs::text("🚪️io/🧪️tests/🧫️fixtures/🔣️.json")).unwrap();
 let text=serde_json::to_string(&law["source"]).unwrap();let mut cursor=PolygonSourcePreparation::new();
 let empty=RetainedCloneGrant{maximum_items:0,maximum_copy_bytes:0,maximum_capacity_bytes:0,maximum_release_bytes:0,maximum_depth:0};
 let work=PolygonSourceGrant{maximum_units:4096,maximum_projection_bytes:4096,retirement:empty};
 let before=cursor.parser.as_ref().unwrap() as *const json::JsonParseCursor;let refused=step(&mut cursor,&text,work).unwrap();assert!(refused.source.is_none());assert_eq!(refused.retirement,RetainedCloneProgress::default());assert_eq!(cursor.stage,0);assert_eq!(cursor.parser.as_ref().unwrap() as *const json::JsonParseCursor,before);
 let mut normal_turns=0;while cursor.stage==0{let grant=cursor.cold_grant(&text,4096,8).unwrap();let result=step(&mut cursor,&text,grant).unwrap();assert!(result.retirement.fits(grant.retirement));assert!(result.retirement.copied_items<=1);normal_turns+=1;assert!(normal_turns<100000);}
 let original=cursor.parser.as_ref().unwrap() as *const json::JsonParseCursor;
 let demand=pack::value::retirement::owned_retirement_birth_bytes::<json::JsonParseCursor>();
 for retirement in [empty,RetainedCloneGrant{maximum_items:1,maximum_capacity_bytes:demand-1,maximum_depth:1,..empty}]{let result=step(&mut cursor,&text,PolygonSourceGrant{retirement,..work}).unwrap();assert_eq!(result.retirement,RetainedCloneProgress::default());assert_eq!(cursor.parser.as_ref().unwrap() as *const json::JsonParseCursor,original);assert!(cursor.retirement.is_none());}
 assert!(step(&mut cursor,&text,PolygonSourceGrant{retirement:RetainedCloneGrant{maximum_items:1,maximum_capacity_bytes:demand,..empty},..work}).is_err());assert_eq!(cursor.parser.as_ref().unwrap() as *const json::JsonParseCursor,original);
 let mut turns=0;let mut allocated=0;let mut released=0;let source=loop{let grant=cursor.cold_grant(&text,1,8).unwrap();let result=step(&mut cursor,&text,grant).unwrap();assert!(result.retirement.fits(grant.retirement));allocated+=result.retirement.retained_capacity_bytes;released+=result.retirement.released_bytes;turns+=1;assert!(turns<100000);if let Some(source)=result.source{break source;}};
 let encoded:serde_json::Value=serde_json::from_str(&encode_polygon_mesh_source(&source)).unwrap();assert_eq!(encoded["faces"],law["source"]["faces"]);for (actual,expected) in encoded["vertices"].as_array().unwrap().iter().zip(law["source"]["vertices"].as_array().unwrap()){for (actual,expected) in actual.as_array().unwrap().iter().zip(expected.as_array().unwrap()){assert_eq!(actual.as_f64(),expected.as_f64());}}
 assert!(cursor.retirement.is_none());assert!(cursor.parser.is_none());assert!(cursor.projection.is_none());assert!(allocated>0);assert!(released>0);
}


#[test]
fn authored_polygon_json_writes_exact_wire_under_funded_turns() {
    use semio_framework_value::{AuthoringWallet,ValueRefusalKind};
    let law:serde_json::Value=serde_json::from_str(include_str!("../../../🧪️tests/🧫️fixtures/🔣️.json")).unwrap();
    let case=&law["authoredJson"];
    let mut source=parse_polygon_mesh_source(&case["source"].to_string()).unwrap();
    let texture=source.textures.get_mut("tex").unwrap();
    texture.bytes=texture.bytes.repeat(case["textureRepeat"].as_u64().unwrap() as usize);
    let original=encode_polygon_mesh_source(&source);
    let budget=RetainedCloneGrant{maximum_items:100000,maximum_copy_bytes:1<<24,maximum_capacity_bytes:1<<24,
        maximum_release_bytes:1<<24,maximum_depth:256};
    let turn=|cursor:&mut PolygonJsonExport,wallet:&mut AuthoringWallet|{
        let d=cursor.demand(256).unwrap();
        let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:d.copy_bytes,
            maximum_capacity_bytes:d.capacity_bytes,maximum_release_bytes:d.release_bytes,maximum_depth:d.depth};
        let result=wallet.run(grant,32,|effect|cursor.step(effect)).unwrap().unwrap().output;
        if let Ok((_,receipt))=&result{assert!(receipt.fits(grant));wallet.post(*receipt).unwrap();}
        result
    };
    let mut wallet=AuthoringWallet::new(budget,1<<28);
    let mut writer=PolygonJsonExport::new(source.clone());
    let short=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:256,maximum_capacity_bytes:0,maximum_release_bytes:0,maximum_depth:256};
    let (output,receipt)=wallet.run(short,32,|effect|writer.step(effect)).unwrap().unwrap().output.unwrap();
    assert!(output.is_none());
    assert_eq!(receipt,Default::default());

    let mut turns=0;
    let mut reserved_output=false;
    let written=loop {
        let (output,receipt)=turn(&mut writer,&mut wallet).unwrap();
        turns+=1;assert!(turns<100000);
        if receipt.retained_capacity_bytes==original.len() {reserved_output=true;}
        if let Some(output)=output{break output;}
    };
    if written!=original {
        let offset=written.bytes().zip(original.bytes()).position(|(a,b)|a!=b).unwrap_or(written.len().min(original.len()));
        panic!("wire differs at byte {offset}: streamed {:?}, original {:?}",
            &written.as_bytes()[offset..(offset+32).min(written.len())],&original.as_bytes()[offset..(offset+32).min(original.len())]);
    }
    assert!(reserved_output && turns>source.textures.get("tex").unwrap().bytes.len(),"reserved={reserved_output} turns={turns}");
    let independent:serde_json::Value=serde_json::from_str(&written).unwrap();
    assert_eq!(independent["vertices"][0][0],case["coordinate"]);
    for key in ["faces","attributes","materials","surface"] {
        if key=="surface" {
            for (row,expected) in independent[key].as_array().unwrap().iter().zip(case["source"][key].as_array().unwrap()){
                for field in ["kind","index"] {assert_eq!(row[field],expected[field]);}
            }
        }else{assert_eq!(independent[key],case["source"][key]);}
    }
    assert_eq!(independent["textures"]["tex"]["bytes"].as_array().unwrap().len(),1536);

    let mut scheduled=PolygonJsonExport::new(source.clone());
    let operation=semio_framework_job::allocate_operation_id();
    let cancel=semio_framework_job::root_cancel_token();
    let scheduled_text=loop {
        let mut spent=0;
        let mut retained=Default::default();
        let mut cx=semio_framework_job::StepContext::new(operation,semio_framework_job::Generation(1),
            semio_framework_job::StepBudget::new(1,u64::MAX,budget),cancel.clone(),||Some(0),&mut spent,&mut retained);
        let grant=cx.retained_grant();
        assert!(cx.admit_work_step(scheduled.demand(grant.maximum_copy_bytes).unwrap()).unwrap());
        let (result,work)=cx.with_original_effect(grant,|_,effect|scheduled.step(effect)).unwrap().unwrap();
        cx.consume_work(work);
        let (text,receipt)=result.unwrap();
        cx.consume_retained(receipt).unwrap();
        assert_eq!(work,1);
        if let Some(text)=text{break text;}
    };
    assert_eq!(scheduled_text,original);

    let mut writer=PolygonJsonExport::new(source);
    let mut wallet=AuthoringWallet::new(budget,1<<28);
    loop {
        let (output,receipt)=turn(&mut writer,&mut wallet).unwrap();
        assert!(output.is_none());
        if receipt.retained_capacity_bytes==original.len() {break;}
    }
    assert!(turn(&mut writer,&mut wallet).unwrap().0.is_none());
    writer.cancel();
    assert!(matches!(turn(&mut writer,&mut wallet),Err(error) if error.kind==ValueRefusalKind::Canceled));
}
