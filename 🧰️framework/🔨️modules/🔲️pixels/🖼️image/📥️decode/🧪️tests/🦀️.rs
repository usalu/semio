use super::*;
use base64::Engine;
use crate::retirement::tests::{drain,observed};
fn rows()->serde_json::Value{serde_json::from_str(crate::text("🖼️image/📥️decode/🧪️tests/🧫️fixtures/🔣️.json")).unwrap()}
fn input(value:&serde_json::Value)->ImageDecodeInput<'_>{ImageDecodeInput{mime:value["mime"].as_str().unwrap(),data:value["data"].as_str().unwrap(),max_source_bytes:value["maxSourceBytes"].as_u64().unwrap()as usize,max_bytes:value["maxBytes"].as_u64().unwrap()as usize,max_pixels:value["maxPixels"].as_u64().unwrap()as usize,max_chunks:value["maxChunks"].as_u64().unwrap()as usize}}
fn funded(job:&ImageDecodeJob)->RetainedCloneGrant{RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:job.next_copy_byte_demand().unwrap(),maximum_capacity_bytes:job.next_capacity_byte_demand().unwrap(),maximum_release_bytes:0,maximum_depth:1}}
fn finish(job:&mut ImageDecodeJob,source:&str,items:usize)->Result<(),ImageDecodeError>{let mut previous=job.progress();for _ in 0..2_000_000{let grant=RetainedCloneGrant{maximum_items:items,maximum_copy_bytes:items*131072,maximum_capacity_bytes:536870912,maximum_release_bytes:0,maximum_depth:1};let(progress,receipt)=job.advance(source,grant)?;assert!(receipt.fits(grant));assert!(progress.work-previous.work<=items as u64);assert!(progress.source_completed>=previous.source_completed&&progress.source_completed<=progress.source_total);assert!(progress.bytes>=previous.bytes&&progress.pixels>=previous.pixels);previous=progress;if progress.done{return Ok(());}}panic!("Image source did not finish")}
fn take(job:&mut ImageDecodeJob)->RasterImage{job.take_result(RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:size_of::<RasterImage>(),maximum_depth:1,..Default::default()}).unwrap().unwrap().0}
fn oracle(text:&str)->Vec<u8>{let bytes=if text.get(..5).is_some_and(|s|s.eq_ignore_ascii_case("data:")){let(header,body)=text.split_once(',').unwrap();let bytes=percent_encoding::percent_decode_str(body).collect::<Vec<_>>();if header.to_ascii_lowercase().ends_with(";base64"){base64::engine::general_purpose::STANDARD.decode(bytes).unwrap()}else{bytes}}else{base64::engine::general_purpose::STANDARD.decode(text).unwrap()};let mut decoder=png::Decoder::new(std::io::Cursor::new(bytes));decoder.set_transformations(png::Transformations::EXPAND|png::Transformations::STRIP_16);let mut reader=decoder.read_info().unwrap();let mut bytes=vec![0;reader.output_buffer_size()];let info=reader.next_frame(&mut bytes).unwrap();let mut rgba=Vec::new();for p in bytes[..info.buffer_size()].chunks(info.color_type.samples()){match info.color_type{png::ColorType::Grayscale=>rgba.extend_from_slice(&[p[0],p[0],p[0],255]),png::ColorType::Rgb=>rgba.extend_from_slice(&[p[0],p[1],p[2],255]),png::ColorType::GrayscaleAlpha=>rgba.extend_from_slice(&[p[0],p[0],p[0],p[1]]),png::ColorType::Rgba=>rgba.extend_from_slice(p),_=>panic!("Unexpanded oracle palette")}}rgba}
#[test]
fn image_sources_decode_shared_rfc_transport_and_all_png_forms(){
 let rows=rows();for row in rows.as_array().unwrap(){let expected:Vec<u8>=serde_json::from_value(row["expected"]["pixels"].clone()).unwrap();let source=input(&row["input"]);for items in [1,7,4096]{let mut job=ImageDecodeJob::new(source).unwrap();assert!(job.result().is_err());finish(&mut job,source.data,items).unwrap();let result=take(&mut job);assert_eq!(result.width,row["expected"]["width"].as_u64().unwrap()as u32);assert_eq!(result.height,row["expected"]["height"].as_u64().unwrap()as u32);assert_eq!(result.pixels,expected,"{} grant {items}",row["name"]);drain(job);drain(result);}assert_eq!(oracle(source.data),expected,"{} third-party oracle",row["name"]);}
}
#[test]
fn image_sources_reject_shared_malformed_and_unsupported_sources(){
 let rows:serde_json::Value=serde_json::from_str(crate::text("🖼️image/📥️decode/🧪️tests/🧫️fixtures/⚠️invalid/🔣️.json")).unwrap();for row in rows.as_array().unwrap(){if let Ok(mut job)=ImageDecodeJob::new(input(&row["input"])){assert!(finish(&mut job,row["input"]["data"].as_str().unwrap(),7).is_err(),"{}",row["name"]);assert!(job.result().is_err());drain(job);}}
}
#[test]
fn image_sources_exact_limits_and_live_phase_cancellation(){
 let rows=rows();let row=&rows[37];let mut limits=input(&row["input"]);limits.max_source_bytes=limits.data.len();limits.max_bytes=base64::engine::general_purpose::STANDARD.decode(limits.data.split_once(',').unwrap().1).unwrap().len();limits.max_pixels=row["expected"]["width"].as_u64().unwrap()as usize*row["expected"]["height"].as_u64().unwrap()as usize;
 let mut job=ImageDecodeJob::new(limits).unwrap();finish(&mut job,limits.data,1).unwrap();assert_eq!(job.result().unwrap().pixels,oracle(limits.data));drain(job);
 for limit in 0..3{let mut value=limits;match limit{0=>value.max_source_bytes-=1,1=>value.max_bytes-=1,_=>value.max_pixels-=1}if let Ok(mut job)=ImageDecodeJob::new(value){assert!(finish(&mut job,value.data,7).is_err());drain(job);}}
 let mut seen=Vec::new();let mut probe=ImageDecodeJob::new(limits).unwrap();let mut steps=0;loop{let p=probe.advance(limits.data,funded(&probe)).unwrap().0;steps+=1;if !p.done&&!seen.contains(&p.phase){seen.push(p.phase);let mut cancelled=ImageDecodeJob::new(limits).unwrap();for _ in 0..steps{cancelled.advance(limits.data,funded(&cancelled)).unwrap();}let old=cancelled.progress();cancelled.cancel();assert_eq!(cancelled.progress(),old);assert_eq!(cancelled.advance(limits.data,funded(&cancelled)),Err(ImageDecodeError::Cancelled));drain(cancelled);}if p.done{break;}}assert_eq!(seen,["header","validate","decode","source-handoff","decoder","png","image-handoff"]);drain(probe);
}
#[test]
fn image_sources_original_funding_and_jpeg_allocator_custody(){
 let fixture:serde_json::Value=serde_json::from_str(crate::text("🖼️image/📥️decode/🧪️tests/🧫️fixtures/🎟️funding.json")).unwrap();let png=rows();let source=input(&png[0]["input"]);
 for axis in fixture["axes"].as_array().unwrap(){let mut job=ImageDecodeJob::new(source).unwrap();let axis=axis.as_str().unwrap();if matches!(axis,"maximumCapacityBytes"|"releaseForCapacity"){while job.next_capacity_byte_demand().unwrap()==0{job.advance(source.data,funded(&job)).unwrap();}}let mut grant=funded(&job);match axis{"maximumItems"=>grant.maximum_items=0,"maximumCopyBytes"=>grant.maximum_copy_bytes-=1,"maximumCapacityBytes"=>grant.maximum_capacity_bytes-=1,"maximumDepth"=>grant.maximum_depth=0,_=>{grant.maximum_capacity_bytes=0;grant.maximum_release_bytes=536870912;}}let old=job.progress();let(step,physical)=observed(||job.advance(source.data,grant).unwrap());assert_eq!(physical,(0,0));assert_eq!(step,(old,Default::default()));let changed=format!("{}x",source.data);let(error,physical)=observed(||job.advance(&changed,funded(&job)));assert!(error.is_err());assert_eq!(physical,(0,0));drain(job);}
 let jpeg:serde_json::Value=serde_json::from_str(crate::text("📸️jpeg/📥️decode/🧪️tests/🧫️fixtures/🔣️.json")).unwrap();let mut phases=std::collections::BTreeSet::new();
 for index in fixture["jpegCases"].as_array().unwrap(){let row=&jpeg["cases"][index.as_u64().unwrap()as usize];let original=format!("data:image/jpeg;base64,{}",row["base64"].as_str().unwrap());let source=ImageDecodeInput{mime:"image/jpeg",data:&original,max_source_bytes:268439552,max_bytes:67108864,max_pixels:16777216,max_chunks:65536};let(mut job,physical)=observed(||ImageDecodeJob::new(source).unwrap());assert_eq!(physical,(0,0));let mut born=0;
  loop{let phase=job.progress().phase;if phases.insert(phase)&&phase!="complete"{let mut cancelled=ImageDecodeJob::new(source).unwrap();while cancelled.progress().phase!=phase{cancelled.advance(source.data,funded(&cancelled)).unwrap();}let before=cancelled.progress();let(_,physical)=observed(||cancelled.cancel());assert_eq!(physical,(0,0));assert_eq!(cancelled.progress(),before);drain(cancelled);}let grant=funded(&job);let(step,physical)=observed(||job.advance(source.data,grant).unwrap());assert_eq!(physical,(step.1.retained_capacity_bytes,step.1.released_bytes));assert!(step.1.fits(grant));born+=physical.0;if step.0.done{break;}}
  let decoded=image::load_from_memory_with_format(&base64::engine::general_purpose::STANDARD.decode(row["base64"].as_str().unwrap()).unwrap(),image::ImageFormat::Jpeg).unwrap().into_rgba8();let output=job.result().unwrap();assert_eq!((output.width,output.height),(row["width"].as_u64().unwrap()as u32,row["height"].as_u64().unwrap()as u32));let maximum=output.pixels.iter().zip(decoded.as_raw()).map(|(left,right)|left.abs_diff(*right)).max().unwrap();assert!(maximum<=row["oracleTolerance"].as_u64().unwrap()as u8);let denied=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:size_of::<RasterImage>()-1,maximum_depth:1,..Default::default()};assert!(job.take_result(denied).unwrap().is_none());let output=take(&mut job);let(a,r)=drain(job);let(oa,or)=drain(output);assert_eq!(born+a+oa,r+or);
 }
 for phase in fixture["interruptionPhases"].as_array().unwrap().iter().filter(|phase|phase.as_str()!=Some("png")){assert!(phases.contains(phase.as_str().unwrap()));}
}

//#region BlockingDecodeAndOrientation
use crate::{Orientation,apply_orientation,encoded_orientation,tiff_orientation};
fn encoded_bytes(text:&str)->Vec<u8>{if text.get(..5).is_some_and(|s|s.eq_ignore_ascii_case("data:")){let(header,body)=text.split_once(',').unwrap();let bytes=percent_encoding::percent_decode_str(body).collect::<Vec<_>>();if header.to_ascii_lowercase().ends_with(";base64"){base64::engine::general_purpose::STANDARD.decode(bytes).unwrap()}else{bytes}}else{base64::engine::general_purpose::STANDARD.decode(text).unwrap()}}
fn never()->bool{false}
fn limits()->EncodedDecodeLimits{EncodedDecodeLimits::MAXIMUM}
fn fixture_orientation()->serde_json::Value{serde_json::from_str(crate::text("🖼️image/📥️decode/🧪️tests/🧫️fixtures/🧭️orientation/🔣️.json")).unwrap()}
fn numbers(value:&serde_json::Value)->Vec<u8>{value.as_array().unwrap().iter().map(|n|n.as_u64().unwrap()as u8).collect()}
fn hex(text:&str)->Vec<u8>{(0..text.len()).step_by(2).map(|i|u8::from_str_radix(&text[i..i+2],16).unwrap()).collect()}
fn crc32(bytes:&[u8])->u32{let mut crc=0xFFFF_FFFFu32;for byte in bytes{crc^=*byte as u32;for _ in 0..8{crc=if crc&1==1{(crc>>1)^0xEDB8_8320}else{crc>>1};}}!crc}
fn gradient(width:u32,height:u32)->image::RgbaImage{image::RgbaImage::from_fn(width,height,|x,y|image::Rgba([(x*40+y*3)as u8,(y*50+x*2)as u8,(x*y*5)as u8,255]))}
fn png_bytes(image:&image::RgbaImage)->Vec<u8>{let mut out=Vec::new();image::DynamicImage::ImageRgba8(image.clone()).write_to(&mut std::io::Cursor::new(&mut out),image::ImageFormat::Png).unwrap();out}
fn jpeg_bytes(image:&image::RgbaImage)->Vec<u8>{let mut out=Vec::new();image::DynamicImage::ImageRgb8(image::DynamicImage::ImageRgba8(image.clone()).to_rgb8()).write_to(&mut std::io::Cursor::new(&mut out),image::ImageFormat::Jpeg).unwrap();out}
fn tiff_for(orientation:u16)->Vec<u8>{let mut tiff=b"II*\0".to_vec();tiff.extend_from_slice(&8u32.to_le_bytes());tiff.extend_from_slice(&1u16.to_le_bytes());for part in [0x0112u16,3]{tiff.extend_from_slice(&part.to_le_bytes());}tiff.extend_from_slice(&1u32.to_le_bytes());tiff.extend_from_slice(&orientation.to_le_bytes());tiff.extend_from_slice(&[0,0]);tiff}
fn jpeg_with_orientation(mut jpeg:Vec<u8>,orientation:u16)->Vec<u8>{let tiff=tiff_for(orientation);let mut segment=vec![0xff,0xe1];segment.extend_from_slice(&((tiff.len()+8)as u16).to_be_bytes());segment.extend_from_slice(b"Exif\0\0");segment.extend_from_slice(&tiff);jpeg.splice(2..2,segment);jpeg}
fn png_with_orientation(mut png:Vec<u8>,orientation:u16)->Vec<u8>{let tiff=tiff_for(orientation);let end=png.len()-12;let mut chunk=(tiff.len()as u32).to_be_bytes().to_vec();let mut body=b"eXIf".to_vec();body.extend_from_slice(&tiff);chunk.extend_from_slice(&body);chunk.extend_from_slice(&crc32(&body).to_be_bytes());png.splice(end..end,chunk);png}
#[test]
fn blocking_decode_matches_the_independent_png_oracle_for_every_shared_row(){
 let rows=rows();let mut checked=0;
 for row in rows.as_array().unwrap(){if !row["input"]["mime"].as_str().unwrap().eq_ignore_ascii_case("image/png"){continue;}
  let data=row["input"]["data"].as_str().unwrap();let image=decode_encoded_blocking(encoded_bytes(data),limits(),&never).unwrap().image;
  assert_eq!(image.pixels,oracle(data),"{}",row["name"]);assert_eq!((image.width as u64,image.height as u64),(row["expected"]["width"].as_u64().unwrap(),row["expected"]["height"].as_u64().unwrap()),"{}",row["name"]);checked+=1;
 }
 assert!(checked>=40);
}
#[test]
fn blocking_decode_reads_jpeg_within_the_oracle_tolerance_and_png_exactly(){
 let source=gradient(13,9);
 let png=decode_encoded_blocking(png_bytes(&source),limits(),&never).unwrap().image;assert_eq!(png.pixels,source.as_raw().to_vec());
 let jpeg=jpeg_bytes(&source);let reference=image::load_from_memory_with_format(&jpeg,image::ImageFormat::Jpeg).unwrap().into_rgba8();
 let ours=decode_encoded_blocking(jpeg,limits(),&never).unwrap().image;assert_eq!((ours.width,ours.height),reference.dimensions());
 for(actual,expected)in ours.pixels.iter().zip(reference.as_raw()){assert!(actual.abs_diff(*expected)<=4,"JPEG oracle difference {}",actual.abs_diff(*expected));}
}
#[test]
fn blocking_decode_refuses_unknown_formats_over_limit_sources_and_cancellation(){
 let source=png_bytes(&gradient(4,4));
 assert_eq!(decode_encoded_blocking(b"GIF89a\x01\0\x01\0\0\0\0;".to_vec(),limits(),&never).unwrap_err(),ImageDecodeError::UnsupportedMime);
 assert!(decode_encoded_blocking(source.clone(),EncodedDecodeLimits{max_pixels:15,..limits()},&never).is_err());
 assert!(decode_encoded_blocking(source.clone(),EncodedDecodeLimits{max_source_bytes:source.len()-1,..limits()},&never).is_err());
 assert_eq!(decode_encoded_blocking(source.clone(),limits(),&||true).unwrap_err(),ImageDecodeError::Cancelled);
 assert!(decode_encoded_blocking(source[..source.len()/2].to_vec(),limits(),&never).is_err());
}
#[test]
fn tiff_orientation_matches_the_shared_vectors(){
 for row in fixture_orientation()["tiff"].as_array().unwrap(){let expected=row["expected"].as_u64().map(|value|Orientation::from_exif(value as u16).unwrap());assert_eq!(tiff_orientation(&hex(row["hex"].as_str().unwrap())),expected,"{}",row["name"]);}
}
#[test]
fn apply_orientation_matches_the_shared_vectors_and_the_image_crate(){
 for row in fixture_orientation()["apply"].as_array().unwrap(){
  let exif=row["exif"].as_u64().unwrap()as u16;let orientation=Orientation::from_exif(exif).unwrap();
  let source=RasterImage{width:row["width"].as_u64().unwrap()as u32,height:row["height"].as_u64().unwrap()as u32,pixels:numbers(&row["pixels"])};
  let ours=apply_orientation(&source,orientation);
  assert_eq!((ours.width as u64,ours.height as u64,ours.pixels.clone()),(row["expected"]["width"].as_u64().unwrap(),row["expected"]["height"].as_u64().unwrap(),numbers(&row["expected"]["pixels"])),"exif {exif}");
  let mut oracle=image::DynamicImage::ImageRgba8(image::RgbaImage::from_raw(source.width,source.height,source.pixels.clone()).unwrap());oracle.apply_orientation(image::metadata::Orientation::from_exif(exif as u8).unwrap());
  assert_eq!((ours.width,ours.height,ours.pixels),(oracle.width(),oracle.height(),oracle.into_rgba8().into_raw()),"exif {exif} against image");
 }
}
#[test]
fn encoded_orientation_reads_jpeg_exif_and_png_exif_like_the_image_crate(){
 use image::ImageDecoder as _;
 let source=gradient(6,4);
 assert_eq!(encoded_orientation(&jpeg_bytes(&source)),Orientation::NoTransforms);assert_eq!(encoded_orientation(&png_bytes(&source)),Orientation::NoTransforms);assert_eq!(encoded_orientation(b"GIF89a"),Orientation::NoTransforms);assert_eq!(encoded_orientation(&[]),Orientation::NoTransforms);
 for exif in 1u16..=8{
  let jpeg=jpeg_with_orientation(jpeg_bytes(&source),exif);let png=png_with_orientation(png_bytes(&source),exif);
  let mut jpeg_decoder=image::codecs::jpeg::JpegDecoder::new(std::io::Cursor::new(&jpeg)).unwrap();let mut png_decoder=image::codecs::png::PngDecoder::new(std::io::Cursor::new(&png)).unwrap();
  let expected=Orientation::from_exif(exif).unwrap();assert_eq!(jpeg_decoder.orientation().unwrap().to_exif() as u16,exif);assert_eq!(png_decoder.orientation().unwrap().to_exif() as u16,exif);
  assert_eq!(encoded_orientation(&jpeg),expected,"jpeg {exif}");assert_eq!(encoded_orientation(&png),expected,"png {exif}");
  let decoded=decode_encoded_blocking(png,limits(),&never).unwrap();assert_eq!(decoded.orientation,expected);assert_eq!((decoded.image.width,decoded.image.height),source.dimensions());let upright=decoded.upright();let mut oracle=image::DynamicImage::ImageRgba8(source.clone());oracle.apply_orientation(image::metadata::Orientation::from_exif(exif as u8).unwrap());
  assert_eq!((upright.width,upright.height,upright.pixels),(oracle.width(),oracle.height(),oracle.into_rgba8().into_raw()),"png upright {exif}");
 }
 for truncated in [3usize,5,9,20]{assert_eq!(encoded_orientation(&jpeg_with_orientation(jpeg_bytes(&source),6)[..truncated]),Orientation::NoTransforms);}
}
//#endregion BlockingDecodeAndOrientation
