//! 🧪️ Shared compositor vectors independently rendered by librsvg in the TypeScript suite.
use super::*;
use serde_json::Value;
use crate::editing::PixelProgress;
use crate::retirement::tests::{Wallet,physical_wallet,funded_turn,funded_close_owned};
fn turn(job:&mut CompositeJob,bank:&mut Wallet,items:usize)->Result<PixelProgress,PixelEditError>{funded_turn(bank,items,|work|job.advance(items,work))}
fn completed(job:&mut CompositeJob,bank:&mut Wallet,target:usize){for _ in 0..2000000{if job.completed>=target||job.progress().done{return;}turn(job,bank,1).unwrap();}panic!("finite compositor observation bound exhausted");}

fn fixture() -> Value { serde_json::from_str(crate::text("🧩️compositing/🧪️tests/🧫️fixtures/🔣️.json")).unwrap() }
fn bytes(value: &Value) -> Vec<u8> { value.as_array().unwrap().iter().map(|v| v.as_u64().unwrap() as u8).collect() }
fn affine(value: &Value) -> CompositeAffine { std::array::from_fn(|i| value[i].as_f64().unwrap()) }
fn layer(value: &Value) -> CompositeLayer {
    let mask = (!value["mask"].is_null()).then(|| {
        let m = &value["mask"];
        CompositeMask {width:m["width"].as_u64().unwrap() as u32,height:m["height"].as_u64().unwrap() as u32,coverage:bytes(&m["coverage"]).into(),transform:affine(&m["transform"]),invert:m["invert"].as_bool().unwrap()}
    });
    let content = match value["kind"].as_str().unwrap() {
        "pixels" => CompositeContent::Pixels(value["image"].as_str().unwrap().to_owned()),
        "group" => CompositeContent::Group(value["children"].as_array().unwrap().iter().map(layer).collect()),
        "adjustment" => CompositeContent::Adjustment {brightness:value["brightness"].as_f64().unwrap(),contrast:value["contrast"].as_f64().unwrap()},
        _ => unreachable!(),
    };
    CompositeLayer {opacity:value["opacity"].as_f64().unwrap(),blend:value["blend"].as_str().unwrap().parse().unwrap(),visible:value["visible"].as_bool().unwrap(),transform:affine(&value["transform"]),mask,content}
}
fn input(value: &Value) -> CompositeInput {
    CompositeInput {
        width:value["width"].as_u64().unwrap() as u32,height:value["height"].as_u64().unwrap() as u32,
        origin:[value["origin"][0].as_f64().unwrap(),value["origin"][1].as_f64().unwrap()],
        images:value["images"].as_object().unwrap().iter().map(|(key,v)|(key.clone(),RasterLease(Arc::new(RasterImage {width:v["width"].as_u64().unwrap() as u32,height:v["height"].as_u64().unwrap() as u32,pixels:bytes(&v["pixels"])})))).collect(),
        layers:value["layers"].as_array().unwrap().iter().map(layer).collect(),
    }
}
fn complete(input:CompositeInput,grant:usize)->RasterImage{let mut job=CompositeJob::new(input);let mut bank=physical_wallet();while !turn(&mut job,&mut bank,grant).unwrap().done{}let output=job.result().unwrap().clone();funded_close_owned(job.into_retirement(),&mut bank);output}
#[test]
fn compositor_language_neutral_cases() {
    for case in fixture()["cases"].as_array().unwrap() {
        assert_eq!(complete(input(&case["input"]),1).pixels,bytes(&case["expected"]),"{}",case["name"]);
    }
}
#[test]
fn compositor_translucent_blends_match_librsvg() {
    let fixture=fixture();
    for (index,mode) in fixture["blendModes"].as_array().unwrap().iter().enumerate() {
        let mut source=input(&fixture["blendInput"]);source.layers[1].blend=mode.as_str().unwrap().parse().unwrap();
        let result=complete(source,2);
        for (actual,expected) in result.pixels.iter().zip(bytes(&fixture["blendExpected"][index])) {
            assert!(actual.abs_diff(expected)<=fixture["oracleTolerance"].as_u64().unwrap() as u8,"{mode}: {}",actual.abs_diff(expected));
        }
    }
}
#[test]
fn compositor_transformed_masks_and_adjustments_cross_tile_boundaries() {
    for case in fixture()["spatialCases"].as_array().unwrap() {
        let expected:Vec<u8>=case["runs"].as_array().unwrap().iter().flat_map(|run|bytes(&run["pixel"]).repeat(run["count"].as_u64().unwrap() as usize)).collect();
        for grant in [1,17,256,65536] {assert_eq!(complete(input(&case["input"]),grant).pixels,expected,"{}",case["name"]);}
    }
}
#[test]
fn compositor_bounds_grants_and_discards_cancelled_candidates(){let mut source=input(&fixture()["cases"][0]["input"]);source.width=512;source.height=512;let mut job=CompositeJob::new(source);let mut bank=physical_wallet();completed(&mut job,&mut bank,17);assert_eq!(job.completed,17);assert!(!job.progress().done);assert_eq!(job.result(),Err(PixelEditError::Incomplete));job.cancel();assert_eq!(turn(&mut job,&mut bank,1),Err(PixelEditError::Cancelled));assert_eq!(job.result(),Err(PixelEditError::Cancelled));funded_close_owned(job.into_retirement(),&mut bank);}
#[test]
fn compositor_validates_geometry_assets_and_depth(){let mut invalid=Vec::new();let mut source=input(&fixture()["cases"][0]["input"]);source.layers[0].transform=[0.0;6];invalid.push(source);let mut missing=input(&fixture()["blendInput"]);missing.images.remove("front");invalid.push(missing);let mut nested=input(&fixture()["cases"][0]["input"]);for _ in 0..33{let mut group=nested.layers[0].clone();group.content=CompositeContent::Group(nested.layers);nested.layers=vec![group];}invalid.push(nested);for source in invalid{let mut job=CompositeJob::new(source);let mut bank=physical_wallet();loop{match turn(&mut job,&mut bank,1){Err(error)=>{assert_eq!(turn(&mut job,&mut bank,1).unwrap_err(),error);break;},Ok(p)=>assert!(!p.done)}}funded_close_owned(job.into_retirement(),&mut bank);}}
#[test]
fn compositor_actual_owner_retirement_neutral_interruptions(){
    let rows:Value=serde_json::from_str(crate::text("🧩️compositing/🧪️tests/🧫️fixtures/🧹️retirement/🔣️.json")).unwrap();
    let fixture=fixture();
    for row in rows.as_array().unwrap(){for grant in [1,7,4096]{
        let source=fixture["cases"].as_array().unwrap().iter().chain(fixture["spatialCases"].as_array().unwrap()).find(|v|v["name"]==row["source"]).unwrap();
        let parsed=input(&source["input"]);let shared=parsed.clone();let before:Vec<_>=shared.images.values().map(|v|v.pixels.clone()).collect();
        let mut job=CompositeJob::new(parsed);let mut bank=physical_wallet();let mode=row["mode"].as_str().unwrap();
        if matches!(mode,"complete"|"cancelledComplete"|"published"){while !turn(&mut job,&mut bank,4096).unwrap().done{}}else{let steps=row["steps"].as_u64().unwrap() as usize;if steps>0{completed(&mut job,&mut bank,steps);}}
        let commands=job.commands.len();let buffers=job.buffers.len();let candidate=job.candidate.as_ref().unwrap().pixels.as_ptr();
        let published=(mode=="published").then(||job.result().unwrap().pixels.clone());
        if mode.starts_with("cancelled")||published.is_some(){job.cancel();assert_eq!(job.commands.len(),commands);assert_eq!(job.buffers.len(),buffers);assert_eq!(job.candidate.as_ref().unwrap().pixels.as_ptr(),candidate);}
        let(retired,output)=job.into_retirement();assert_eq!(output.is_some(),mode=="complete");if let Some(image)=&output{assert_eq!(image.pixels.as_ptr(),candidate);}
        let actual=output.as_ref().map(|v|v.pixels.clone()).or(published);funded_close_owned((retired,output),&mut bank);
        for(image,bytes)in shared.images.values().zip(&before){assert_eq!(&image.pixels,bytes);assert_eq!(Arc::strong_count(&image.0),1);}
        if let Some(actual)=actual{let expected=if source["expected"].is_array(){bytes(&source["expected"])}else{source["runs"].as_array().unwrap().iter().flat_map(|r|bytes(&r["pixel"]).repeat(r["count"].as_u64().unwrap() as usize)).collect()};assert_eq!(actual,expected);}
    }}
}

#[test]fn compositor_real_backing_and_fixed_traversal_obey_same_stock(){let law:Value=serde_json::from_str(include_str!("../🎟️work/📐️.json")).unwrap();assert_eq!(law["tile"],256);let cases=fixture();let row=&cases["cases"][0];let mut job=CompositeJob::new(input(&row["input"]));let mut bank=crate::retirement::tests::physical_wallet();let mut denied=[false;4];loop{let d=job.next_work_demand();for axis in 0..4{let n=match axis{0=>1,1=>d.copy_bytes,2=>d.capacity_bytes,_=>d.depth};if n==0||denied[axis]{continue;}let mut grant=bank.remaining();*match axis{0=>&mut grant.maximum_items,1=>&mut grant.maximum_copy_bytes,2=>&mut grant.maximum_capacity_bytes,_=>&mut grant.maximum_depth}=n-1;let before=job.completed;let((p,receipt),heap)=crate::retirement::tests::observed(||{let mut work=semio_framework_value::work::RetainedWorkLedger::new(grant);let p=job.advance(1,&mut work).unwrap();(p,work.receipt())});assert!(!p.done);assert_eq!(heap,(0,0));assert_eq!(receipt,Default::default());assert_eq!(job.completed,before);assert_eq!(job.last_turn_work(),0);denied[axis]=true;}if crate::retirement::tests::funded_turn(&mut bank,1,|work|job.advance(1,work)).unwrap().done{break;}}assert!(denied.into_iter().all(|v|v));assert_eq!(job.result().unwrap().pixels,bytes(&row["expected"]));crate::retirement::tests::funded_close_owned(job.into_retirement(),&mut bank);}
