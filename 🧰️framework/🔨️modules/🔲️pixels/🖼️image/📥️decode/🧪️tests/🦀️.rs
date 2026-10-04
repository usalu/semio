use super::*;
use base64::Engine;
fn input(v:&serde_json::Value)->ImageDecodeInput{ImageDecodeInput{mime:v["mime"].as_str().unwrap().into(),data:Arc::new(v["data"].as_str().unwrap().into()),max_source_bytes:v["maxSourceBytes"].as_u64().unwrap()as usize,max_bytes:v["maxBytes"].as_u64().unwrap()as usize,max_pixels:v["maxPixels"].as_u64().unwrap()as usize,max_chunks:v["maxChunks"].as_u64().unwrap()as usize}}
fn finish(job:&mut ImageDecodeJob,grant:usize)->Result<(),ImageDecodeError>{let(mut work,mut read,mut bytes,mut pixels)=(0,0,0,0);for _ in 0..2000000{let p=job.advance(grant)?;assert!(p.work-work<=grant as u64);assert!(p.source_completed>=read&&p.source_completed<=p.source_total);assert!(p.bytes>=bytes);assert!(p.pixels>=pixels);work=p.work;read=p.source_completed;bytes=p.bytes;pixels=p.pixels;if p.done{return Ok(());}}panic!("Image source did not finish")}
fn oracle(text:&str)->Vec<u8>{let bytes=if text.get(..5).is_some_and(|s|s.eq_ignore_ascii_case("data:")){let (header,body)=text.split_once(',').unwrap();let bytes=percent_encoding::percent_decode_str(body).collect::<Vec<_>>();if header.to_ascii_lowercase().ends_with(";base64"){base64::engine::general_purpose::STANDARD.decode(bytes).unwrap()}else{bytes}}else{base64::engine::general_purpose::STANDARD.decode(text).unwrap()};let mut decoder=png::Decoder::new(std::io::Cursor::new(bytes));decoder.set_transformations(png::Transformations::EXPAND|png::Transformations::STRIP_16);let mut reader=decoder.read_info().unwrap();let mut bytes=vec![0;reader.output_buffer_size()];let info=reader.next_frame(&mut bytes).unwrap();let mut rgba=Vec::new();for p in bytes[..info.buffer_size()].chunks(info.color_type.samples()){match info.color_type{png::ColorType::Grayscale=>rgba.extend_from_slice(&[p[0],p[0],p[0],255]),png::ColorType::Rgb=>rgba.extend_from_slice(&[p[0],p[1],p[2],255]),png::ColorType::GrayscaleAlpha=>rgba.extend_from_slice(&[p[0],p[0],p[0],p[1]]),png::ColorType::Rgba=>rgba.extend_from_slice(p),_=>panic!("Unexpanded oracle palette")}}rgba}
#[test]
fn image_sources_decode_shared_rfc_transport_and_all_png_forms(){
 let rows:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();
 for row in rows.as_array().unwrap(){let expected:Vec<u8>=serde_json::from_value(row["expected"]["pixels"].clone()).unwrap();for grant in [1,7,4096]{let mut job=ImageDecodeJob::new(input(&row["input"])).unwrap();assert!(job.result().is_err());finish(&mut job,grant).unwrap();let result=job.into_result().unwrap();assert_eq!(result.width,row["expected"]["width"].as_u64().unwrap()as u32);assert_eq!(result.height,row["expected"]["height"].as_u64().unwrap()as u32);assert_eq!(result.pixels,expected,"{} grant {grant}",row["name"]);}assert_eq!(oracle(row["input"]["data"].as_str().unwrap()),expected,"{} third-party oracle",row["name"]);}
 eprintln!("[DEBUG] All 47 native encoded sources matched independent base64/percent-encoding/png under grants 1, 7 and 4096");
}
#[test]
fn image_sources_reject_shared_malformed_and_unsupported_sources(){
 let rows:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/⚠️invalid/🔣️.json")).unwrap();
 for row in rows.as_array().unwrap(){if let Ok(mut job)=ImageDecodeJob::new(input(&row["input"])){assert!(finish(&mut job,7).is_err(),"{}",row["name"]);assert!(job.result().is_err());assert!(job.advance(1).is_err());}}
 eprintln!("[DEBUG] All {} malformed or unsupported native image sources refused publication",rows.as_array().unwrap().len());
}
#[test]
fn image_sources_cancel_and_drop_every_preparation_stage(){
 let rows:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();
 for steps in [0,1,10,50,200,1000,3000]{let mut job=ImageDecodeJob::new(input(&rows[45]["input"])).unwrap();for _ in 0..steps{if job.advance(1).unwrap().done{break;}}job.cancel();assert!(job.advance(1).is_err());assert!(job.result().is_err());}
 for steps in [1,100,300,500,1000]{let mut job=ImageDecodeJob::new(input(&rows[37]["input"])).unwrap();job.advance(steps).unwrap();drop(job);}
 assert!(ImageDecodeJob::new(input(&rows[0]["input"])).unwrap().advance(0).is_err());
}
#[test]
fn image_sources_exact_limits_and_live_phase_cancellation(){
 let rows:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();
 let row=&rows[37];let mut limits=input(&row["input"]);limits.max_source_bytes=limits.data.len();limits.max_bytes=base64::engine::general_purpose::STANDARD.decode(limits.data.split_once(',').unwrap().1).unwrap().len();limits.max_pixels=row["expected"]["width"].as_u64().unwrap()as usize*row["expected"]["height"].as_u64().unwrap()as usize;
 let mut job=ImageDecodeJob::new(limits.clone()).unwrap();finish(&mut job,1).unwrap();let expected:Vec<u8>=serde_json::from_value(row["expected"]["pixels"].clone()).unwrap();assert_eq!(job.into_result().unwrap().pixels,expected);
 for limit in 0..3{let mut value=limits.clone();match limit{0=>value.max_source_bytes-=1,1=>value.max_bytes-=1,_=>value.max_pixels-=1}if let Ok(mut job)=ImageDecodeJob::new(value){assert!(finish(&mut job,7).is_err());}}
 let mut seen=Vec::new();let mut probe=ImageDecodeJob::new(limits.clone()).unwrap();let mut steps=0;
 loop{let p=probe.advance(1).unwrap();steps+=1;if !p.done&&!seen.contains(&p.phase){seen.push(p.phase);let mut job=ImageDecodeJob::new(limits.clone()).unwrap();job.advance(steps).unwrap();job.cancel();assert_eq!(job.advance(1),Err(ImageDecodeError::Cancelled));assert_eq!(job.result().unwrap_err(),ImageDecodeError::Cancelled);}if p.done{break;}}
 assert_eq!(seen,["header","validate","decode","png"]);
 eprintln!("[DEBUG] Native exact source/binary/pixel caps and all live preparation phases verified");
}
