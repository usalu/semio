//! 🧪️ Shared raster-stack vectors independently checked against librsvg in TypeScript.
use super::*;
use serde_json::Value;
fn fixture()->Value {serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap()}
fn transform(value:&Value)->RasterStackTransform {RasterStackTransform {x:value["x"].as_f64().unwrap(),y:value["y"].as_f64().unwrap(),a:value["a"].as_f64().unwrap(),b:value["b"].as_f64().unwrap(),c:value["c"].as_f64().unwrap(),d:value["d"].as_f64().unwrap()}}
fn layer(value:&Value)->RasterStackLayer {
    let extent=|v:&Value|v.as_u64().map(|n|n as u32);
    let mask=(!value["mask"].is_null()).then(||{let m=&value["mask"];RasterStackMask {enabled:m["enabled"].as_bool().unwrap(),linked:m["linked"].as_bool().unwrap(),invert:m["invert"].as_bool().unwrap(),transform:transform(&m["transform"]),width:extent(&m["width"]),height:extent(&m["height"]),image_key:m["imageKey"].as_str().map(str::to_owned)}});
    let content=match value["kind"].as_str().unwrap(){
        "pixel"=>RasterStackContent::Pixel {width:extent(&value["width"]),height:extent(&value["height"]),image_key:value["imageKey"].as_str().map(str::to_owned)},
        "group"=>RasterStackContent::Group(value["children"].as_array().unwrap().iter().map(layer).collect()),
        "adjustment"=>RasterStackContent::BrightnessContrast {brightness:value["params"]["brightness"].as_f64().unwrap_or(0.0),contrast:value["params"]["contrast"].as_f64().unwrap_or(0.0)},
        _=>panic!("Unknown fixture layer"),
    };
    RasterStackLayer {id:value["id"].as_str().unwrap().into(),visible:value["visible"].as_bool().unwrap(),opacity:value["opacity"].as_f64().unwrap(),blend_mode:value["blendMode"].as_str().unwrap().parse().unwrap(),transform:transform(&value["transform"]),mask,content}
}
fn input(case:&Value)->RasterStackInput {
    let data=fixture();
    RasterStackInput {layers:case["layers"].as_array().unwrap().iter().map(layer).collect(),images:data["images"].as_object().unwrap().iter().map(|(key,v)|(key.clone(),Arc::new(RasterImage {width:v["width"].as_u64().unwrap() as u32,height:v["height"].as_u64().unwrap() as u32,pixels:v["pixels"].as_array().unwrap().iter().map(|n|n.as_u64().unwrap() as u8).collect()}))).collect()}
}
#[test]
fn raster_stack_matches_shared_svg_mask_vectors(){
    for case in fixture()["cases"].as_array().unwrap(){for grant in [1,17,256]{
        let mut job=RasterStackJob::new(input(case)).unwrap();
        let mut previous=0;let mut total=None;
        loop {let p=job.advance(grant).unwrap();assert!(p.completed>=previous&&p.completed-previous<=grant);previous=p.completed;assert_eq!(*total.get_or_insert(p.total),p.total);if p.done{break;}}
        let result=job.into_result().unwrap();assert!(!result.empty);assert_eq!([result.image.width,result.image.height],[3,1]);
        assert_eq!(result.origin,[case["origin"][0].as_f64().unwrap(),case["origin"][1].as_f64().unwrap()]);
        assert_eq!(result.image.pixels.chunks_exact(4).map(|p|p[3]).collect::<Vec<_>>(),case["alpha"].as_array().unwrap().iter().map(|n|n.as_u64().unwrap() as u8).collect::<Vec<_>>());
    }}
}
#[test]
fn raster_stack_cancels_before_publishing_and_bounds_grants(){
    let source=&fixture()["cases"][0];
    assert!(RasterStackJob::new(input(source)).unwrap().into_result().is_err());
    let mut job=RasterStackJob::new(input(source)).unwrap();
    assert!(job.advance(0).is_err());assert!(job.advance(1048577).is_err());assert!(!job.advance(1).unwrap().done);
    job.cancel();assert_eq!(job.advance(1),Err(PixelEditError::Cancelled));assert!(job.into_result().is_err());
}
#[test]
fn raster_stack_deduplicates_mask_work_and_preserves_transparent_bounds(){
    let mut source=input(&fixture()["cases"][0]);let mut second=source.layers[0].clone();second.id="second".into();source.layers.push(second);
    let mut job=RasterStackJob::new(source).unwrap();
    for completed in 1..=12{assert_eq!(job.advance(1).unwrap(),PixelProgress {completed,total:12,done:false});}
    let mut source=input(&fixture()["cases"][0]);let mut blank=source.layers.pop().unwrap();blank.mask=None;
    if let RasterStackContent::Pixel {image_key,..}=&mut blank.content{*image_key=None;}
    let transform=RasterStackTransform {x:0.0,y:0.0,a:1.0,b:0.0,c:0.0,d:1.0};
    for depth in 0..32 {blank=RasterStackLayer {id:format!("group-{depth}"),visible:true,opacity:if depth==0{0.0}else{1.0},blend_mode:CompositeBlend::Normal,transform,mask:None,content:RasterStackContent::Group(vec![blank])};}
    let mut job=RasterStackJob::new(RasterStackInput {layers:vec![blank],images:BTreeMap::new()}).unwrap();
    loop{let p=job.advance(1).unwrap();if p.done{assert_eq!(p.completed,p.total);break;}}
    let result=job.into_result().unwrap();assert!(!result.empty);assert_eq!([result.image.width,result.image.height],[3,1]);assert_eq!(result.image.pixels,vec![0;12]);
    let mut empty=RasterStackJob::new(RasterStackInput {layers:vec![],images:BTreeMap::new()}).unwrap();assert!(!empty.advance(1).unwrap().done);while !empty.advance(1).unwrap().done{}
    let result=empty.into_result().unwrap();assert!(result.empty);assert_eq!(result.image.pixels,vec![0;4]);
}


#[test]
fn raster_stack_actual_compositor_retirement_before_publication(){
    for row in fixture()["cases"].as_array().unwrap(){for grant in [1,7,4096]{
        let mut job=RasterStackJob::new(input(row)).unwrap();let mut moved=None;let mut observed=false;let mut done=false;
        for _ in 0..10000{
            let p=job.advance(grant).unwrap();
            if let Some(child)=&job.composite_retirement{observed=true;assert!(job.composite.is_none());assert!(!p.done);assert!(!job.done);let image=job.output.as_ref().unwrap();if let Some(pointer)=moved{assert_eq!(image.pixels.as_ptr(),pointer);}else{moved=Some(image.pixels.as_ptr());}if let Some(owner)=&child.job{assert!(owner.candidate.is_none());}}
            if p.done{done=true;break;}
        }
        assert!(done);assert!(job.composite_retirement.is_none());assert!(job.composite.is_none());assert!(job.done);
        let pointer=job.output.as_ref().unwrap().pixels.as_ptr();let output=job.into_result().unwrap();assert_eq!(output.image.pixels.as_ptr(),pointer);if let Some(moved)=moved{assert_eq!(pointer,moved);}if grant==1{assert!(observed);}
        assert_eq!(output.image.pixels.chunks_exact(4).map(|p|p[3]).collect::<Vec<_>>(),row["alpha"].as_array().unwrap().iter().map(|n|n.as_u64().unwrap() as u8).collect::<Vec<_>>());
        eprintln!("[DEBUG] Actual native raster stack compositor handoff {}: grant={grant} moved_pixel_pointer=true",row["name"]);
    }}
}
