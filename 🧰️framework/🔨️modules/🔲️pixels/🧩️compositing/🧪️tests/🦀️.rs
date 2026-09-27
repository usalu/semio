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
    job.into_result().unwrap()
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
    assert_eq!(job.advance(1),Err(PixelEditError::Cancelled));assert_eq!(job.into_result(),Err(PixelEditError::Cancelled));
}
#[test]
fn compositor_validates_geometry_assets_and_depth() {
    let mut source=input(&fixture()["cases"][0]["input"]);source.layers[0].transform=[0.0;6];assert!(CompositeJob::new(source).is_err());
    let mut missing=input(&fixture()["blendInput"]);missing.images.remove("front");assert!(CompositeJob::new(missing).is_err());
    let mut nested=input(&fixture()["cases"][0]["input"]);
    for _ in 0..33 {let mut group=nested.layers[0].clone();group.content=CompositeContent::Group(nested.layers);nested.layers=vec![group];}
    assert!(CompositeJob::new(nested).is_err());
}
