//! 🧪️ Shared compositor vectors independently rendered by librsvg in the TypeScript suite.
use super::*;
use serde_json::Value;

fn fixture() -> Value { serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap() }
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
        images:value["images"].as_object().unwrap().iter().map(|(key,v)|(key.clone(),Arc::new(RasterImage {width:v["width"].as_u64().unwrap() as u32,height:v["height"].as_u64().unwrap() as u32,pixels:bytes(&v["pixels"])}))).collect(),
        layers:value["layers"].as_array().unwrap().iter().map(layer).collect(),
    }
}
fn complete(input: CompositeInput, grant: usize) -> RasterImage {
    let mut job = CompositeJob::new(input).unwrap();
    while !job.advance(grant).unwrap().done {}
    {let(mut retirement,output)=job.into_retirement();while !retirement.terminal_is_empty(){retirement.advance(1).unwrap();}output.unwrap()}
}
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
fn compositor_bounds_grants_and_discards_cancelled_candidates() {
    let mut source=input(&fixture()["cases"][0]["input"]);source.width=512;source.height=512;
    let mut job=CompositeJob::new(source).unwrap();
    let first=job.advance(17).unwrap();assert_eq!(first.completed,17);assert!(!first.done);
    assert_eq!(job.result(),Err(PixelEditError::Incomplete));job.cancel();
    assert_eq!(job.advance(1),Err(PixelEditError::Cancelled));assert_eq!(job.result(),Err(PixelEditError::Cancelled));
}
#[test]
fn compositor_validates_geometry_assets_and_depth() {
    let mut source=input(&fixture()["cases"][0]["input"]);source.layers[0].transform=[0.0;6];assert!(CompositeJob::new(source).is_err());
    let mut missing=input(&fixture()["blendInput"]);missing.images.remove("front");assert!(CompositeJob::new(missing).is_err());
    let mut nested=input(&fixture()["cases"][0]["input"]);
    for _ in 0..33 {let mut group=nested.layers[0].clone();group.content=CompositeContent::Group(nested.layers);nested.layers=vec![group];}
    assert!(CompositeJob::new(nested).is_err());
}

#[test]
fn compositor_actual_owner_retirement_neutral_interruptions(){
    let rows:Value=serde_json::from_str(include_str!("../🧫️fixtures/🧹️retirement/🔣️.json")).unwrap();
    let fixture=fixture();
    for row in rows.as_array().unwrap(){for grant in [1,7,4096]{
        let source=fixture["cases"].as_array().unwrap().iter().chain(fixture["spatialCases"].as_array().unwrap()).find(|v|v["name"]==row["source"]).unwrap();
        let parsed=input(&source["input"]);let shared=parsed.clone();let before:Vec<_>=shared.images.values().map(|v|v.pixels.clone()).collect();
        let mut job=CompositeJob::new(parsed).unwrap();let mode=row["mode"].as_str().unwrap();
        if matches!(mode,"complete"|"cancelledComplete"|"published"){while !job.advance(4096).unwrap().done{}}else{let steps=row["steps"].as_u64().unwrap() as usize;if steps>0{job.advance(steps).unwrap();}}
        let commands=job.commands.len();let buffers=job.buffers.len();let candidate=job.candidate.as_ref().unwrap().pixels.as_ptr();
        let published=(mode=="published").then(||job.result().unwrap().pixels.clone());
        if mode.starts_with("cancelled")||published.is_some(){job.cancel();assert_eq!(job.commands.len(),commands);assert_eq!(job.buffers.len(),buffers);assert_eq!(job.candidate.as_ref().unwrap().pixels.as_ptr(),candidate);}
        let(mut retired,output)=job.into_retirement();assert_eq!(output.is_some(),mode=="complete");if let Some(image)=&output{assert_eq!(image.pixels.as_ptr(),candidate);}
        assert!(retired.advance(0).is_err());if usize::BITS>53{assert!(retired.advance(usize::MAX).is_err());}
        let mut work=0;
        while !retired.terminal_is_empty(){let owner=retired.job.as_ref().unwrap();let count=owner.commands.len()+owner.buffers.len();let p=retired.advance(grant).unwrap();assert!(p.work>work&&p.work-work<=grant as u64);if let Some(owner)=retired.job.as_ref(){assert!(count-owner.commands.len()-owner.buffers.len()<=grant);}work=p.work;}
        assert!(retired.job.is_none());assert_eq!(work,(commands+buffers+4) as u64);let p=retired.advance(1).unwrap();assert_eq!(p.work,work);assert!(p.done);assert_eq!(p.phase,"complete");
        for(image,bytes)in shared.images.values().zip(&before){assert_eq!(&image.pixels,bytes);assert_eq!(Arc::strong_count(image),1);}
        if let Some(actual)=output.map(|v|v.pixels).or(published){let expected=if source["expected"].is_array(){bytes(&source["expected"])}else{source["runs"].as_array().unwrap().iter().flat_map(|r|bytes(&r["pixel"]).repeat(r["count"].as_u64().unwrap() as usize)).collect()};assert_eq!(actual,expected);}
        eprintln!("[DEBUG] Actual native compositor retirement {}: grant={grant} work={work} terminal_empty=true commands={commands} buffers={buffers}",row["name"]);
    }}
}
