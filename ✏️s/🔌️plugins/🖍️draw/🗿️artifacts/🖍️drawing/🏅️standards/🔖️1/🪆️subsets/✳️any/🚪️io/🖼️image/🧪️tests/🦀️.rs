//! 🧪️ Neutral PNG sources publish only complete logical RGBA assets under real work grants.
use super::*;
fn fixture_grant(items:usize)->RetainedCloneGrant{RetainedCloneGrant{maximum_items:items,maximum_copy_bytes:16777216,maximum_capacity_bytes:536870912,maximum_release_bytes:0,maximum_depth:128}}
fn fixture_close(job:DrawingImageAdmissionJob){let mut owner=semio_framework_value::retirement::controlled::ControlledRetirement::new(job).unwrap_or_else(|_|panic!("drawing image fixture owner unsupported"));for _ in 0..2000000{if owner.terminal_is_empty(){return;}let copy=owner.next_copy_byte_demand().unwrap();let release=owner.next_release_byte_demand().unwrap();let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:copy,maximum_capacity_bytes:owner.next_capacity_byte_demand(if copy>0{copy}else{release}).unwrap(),maximum_release_bytes:release,maximum_depth:owner.next_depth_demand().unwrap()};assert!(owner.step(grant).unwrap().progress().fits(grant));}panic!("drawing image fixture close stalled")}
fn fixture_admit(input:ImageDecodeInput<'_>,items:usize)->Result<DrawingImageAsset,ImageDecodeError>{let source=input.data;let mut job=DrawingImageAdmissionJob::new(input)?;let result=(||{while !job.advance(source,fixture_grant(items))?.0.done{}Ok(job.take_result(fixture_grant(1))?.unwrap().0)})();fixture_close(job);result}
#[test]
fn drawing_image_admission_publishes_neutral_samples_under_independent_grants(){
 let fixture:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();let mut witnesses=0;
 for row in fixture["cases"].as_array().unwrap(){let bytes=row["png"].as_array().unwrap().iter().map(|value|value.as_u64().unwrap()as u8).collect::<Vec<_>>();let encoded=base64_codec::base64_standard_encode(&bytes);
  for data in [encoded.clone(),format!("data:image/png;base64,{encoded}")]{for items in [1,7,4096]{let asset=fixture_admit(ImageDecodeInput{mime:"image/png",data:&data,max_source_bytes:4096,max_bytes:4096,max_pixels:4096,max_chunks:64},items).unwrap();let expected:DrawingImageAsset=serde_json::from_value(row["expected"].clone()).unwrap();assert_eq!(asset,expected);witnesses+=1;}
   let mut job=DrawingImageAdmissionJob::new(ImageDecodeInput{mime:"image/png",data:&data,max_source_bytes:4096,max_bytes:4096,max_pixels:4096,max_chunks:64}).unwrap();assert_eq!(job.advance(&data,Default::default()).unwrap().1,Default::default());job.advance(&data,fixture_grant(1)).unwrap();job.cancel();assert!(matches!(job.advance(&data,fixture_grant(1)),Err(ImageDecodeError::Cancelled)));assert!(job.result().is_err());fixture_close(job);
  }
 }
 eprintln!("[DEBUG] Draw native logical RGBA admission actual neutral/grant witnesses={witnesses}");
}
#[test]
fn drawing_image_emission_preserves_original_samples_through_progress_cancellation_and_refusal(){
 let fixture:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();let mut witnesses=0;
 for row in fixture["cases"].as_array().unwrap(){let asset:DrawingImageAsset=serde_json::from_value(row["expected"].clone()).unwrap();let original=asset.clone();let pointer=asset.samples.get(0).unwrap()as*const[u8;4];
  for grant in [1,7,4096]{let mut job=DrawingImageEmissionJob::new(&asset,4096,4096).unwrap();assert!(job.result().is_err());let mut previous=0;loop{let progress=job.advance(grant).unwrap();assert!(progress.work-previous<=grant as u64);previous=progress.work;if progress.done{break;}}let decoded=semio_framework_pixels::decode_png(job.result().unwrap()).unwrap();assert_eq!(decoded.pixels,asset.samples.iter().flat_map(|sample|sample.iter().copied()).collect::<Vec<_>>());assert_eq!(asset,original);assert_eq!(asset.samples.get(0).unwrap()as*const[u8;4],pointer);witnesses+=1;}
  for stop in [0,1,asset.samples.len()+1]{let mut job=DrawingImageEmissionJob::new(&asset,4096,4096).unwrap();for _ in 0..stop{if job.advance(1).unwrap().done{break;}}job.cancel();assert!(job.result().is_err());assert!(job.advance(1).is_err());assert_eq!(asset,original);witnesses+=1;}
  assert!(DrawingImageEmissionJob::new(&asset,4096,8).is_err());assert_eq!(asset,original);assert_eq!(asset.samples.get(0).unwrap()as*const[u8;4],pointer);
 }
 eprintln!("[DEBUG] Draw native emission retained original sample identity under independent grant/cancellation witnesses={witnesses}");
}

#[test]
fn drawing_image_native_refusal_pipeline_retains_all_original_physical_vectors(){
 use crate::schema::scene_raster::*;
 let fixture:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/⚠️invalid/🔣️.json")).unwrap();let mut witnesses=0;
 for row in fixture.as_array().unwrap(){for grant in [1,7,4096]{let result=(||->Result<(),String>{let input=&row["input"];let number=|key:&str|input[key].as_u64().unwrap()as usize;let mut assets=Vec::new();let mut source_bytes=0;
  for source in input["assets"].as_array().unwrap(){let data=source["data"].as_str().unwrap();source_bytes+=data.len();if source_bytes>number("maxSourceBytes"){return Err("Encoded source byte allowance exceeded".into());}
   let asset=fixture_admit(ImageDecodeInput{mime:source["mime"].as_str().unwrap(),data,max_source_bytes:number("maxSourceBytes"),max_bytes:number("maxBytes"),max_pixels:number("maxPixels"),max_chunks:number("maxChunks")},grant).map_err(|error|error.to_string())?;let image=crate::schema::drawing_image_samples(&asset).ok_or("Invalid admitted sample extent")?;assets.push(RasterSceneAsset{id:source["id"].as_str().unwrap().into(),image:std::sync::Arc::new(image)});
  }
  let nodes=input["nodes"].as_array().unwrap().iter().map(|node|{let content=&node["content"];RasterSceneNode{id:node["id"].as_str().unwrap().into(),groups:Vec::new(),transform:serde_json::from_value(node["transform"].clone()).unwrap(),opacity:node["opacity"].as_f64().unwrap(),blend_mode:node["blendMode"].as_str().unwrap().into(),visible:node["visible"].as_bool().unwrap(),content:RasterSceneContent::Image{asset:content["asset"].as_str().unwrap().into(),width:content["width"].as_f64().unwrap(),height:content["height"].as_f64().unwrap()}}}).collect();
  let mut job=RasterSceneJob::new(RasterSceneInput{width:number("width")as u32,height:number("height")as u32,origin:serde_json::from_value(input["origin"].clone()).unwrap(),tolerance:input["tolerance"].as_f64().unwrap(),max_pixels:number("maxPixels"),max_source_bytes:number("maxSourceBytes"),assets,nodes}).map_err(|error|error.to_string())?;while !job.advance(grant).map_err(|error|error.to_string())?.done{}job.result().map_err(|error|error.to_string())?;Ok(())})();assert!(result.is_err(),"{} grant={grant}",row["name"]);witnesses+=1;}}
 assert_eq!(witnesses,129);eprintln!("[DEBUG] Draw native canonical IO physical refusal vectors=43 independent grant witnesses={witnesses}");
}

#[test]
fn drawing_image_data_uri_controls_png_and_base64_while_preserving_original_owner(){
 let fixture:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();let mut witnesses=0;
 for row in fixture["cases"].as_array().unwrap(){let asset:DrawingImageAsset=serde_json::from_value(row["expected"].clone()).unwrap();let original=asset.clone();let pointer=asset.samples.get(0).unwrap()as*const[u8;4];let mut work=0;let mut finished=false;let uri=drawing_image_data_uri_controlled(&asset,&mut |state|{assert!(state.work>work);work=state.work;finished|=state.done;true}).unwrap();assert!(finished);let encoded=uri.strip_prefix("data:image/png;base64,").unwrap();let image=semio_framework_pixels::decode_png(&base64_codec::base64_standard_decode(encoded).unwrap()).unwrap();assert_eq!(image.pixels,asset.samples.iter().flat_map(|sample|sample.iter().copied()).collect::<Vec<_>>());witnesses+=1;
  for phase in ["encoding","base64","done"]{assert!(drawing_image_data_uri_controlled(&asset,&mut |state|if phase=="done"{!state.done}else{state.phase!=phase}).is_err());assert_eq!(asset,original);assert_eq!(asset.samples.get(0).unwrap()as*const[u8;4],pointer);witnesses+=1;}
 }
 eprintln!("[DEBUG] Draw native PNG/base64 original-owner controlled output witnesses={witnesses}");
}
