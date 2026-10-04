use super::*;
fn input(row:&serde_json::Value)->PngDecodeInput{PngDecodeInput{data:Arc::new(serde_json::from_value(row["data"].clone()).unwrap()),max_pixels:row["maxPixels"].as_u64().unwrap()as usize,max_bytes:row["maxBytes"].as_u64().unwrap()as usize,max_chunks:row["maxChunks"].as_u64().unwrap()as usize}}
fn finish(job:&mut PngDecodeJob,grant:usize)->Result<(),PngDecodeError>{let(mut work,mut bytes,mut pixels)=(0,0,0);for _ in 0..2000000{let p=job.advance(grant)?;assert!(p.work-work<=grant as u64);assert!(p.bytes>=bytes);assert!(p.pixels>=pixels);work=p.work;bytes=p.bytes;pixels=p.pixels;if p.done{return Ok(());}}panic!("PNG job did not finish")}
fn oracle(data:&[u8])->Vec<u8>{let mut decoder=png::Decoder::new(std::io::Cursor::new(data));decoder.set_transformations(png::Transformations::EXPAND|png::Transformations::STRIP_16);let mut reader=decoder.read_info().unwrap();let mut bytes=vec![0;reader.output_buffer_size()];let info=reader.next_frame(&mut bytes).unwrap();let mut out=Vec::new();for p in bytes[..info.buffer_size()].chunks(info.color_type.samples()){match info.color_type{png::ColorType::Grayscale=>out.extend_from_slice(&[p[0],p[0],p[0],255]),png::ColorType::Rgb=>out.extend_from_slice(&[p[0],p[1],p[2],255]),png::ColorType::GrayscaleAlpha=>out.extend_from_slice(&[p[0],p[0],p[0],p[1]]),png::ColorType::Rgba=>out.extend_from_slice(p),_=>panic!("oracle did not expand palette")}}out}
#[test]
fn png_decode_shared_all_depths_filters_interlace_and_huffman_under_grants(){
 let rows:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();
 for row in rows.as_array().unwrap(){let expected:Vec<u8>=serde_json::from_value(row["expected"]["pixels"].clone()).unwrap();let source=input(&row["input"]);let before=source.data.clone();for grant in [1,7,4096]{let mut job=PngDecodeJob::new(input(&row["input"])).unwrap();assert!(job.result().is_err());finish(&mut job,grant).unwrap();let result=job.into_result().unwrap();assert_eq!(result.width,row["expected"]["width"].as_u64().unwrap()as u32);assert_eq!(result.height,row["expected"]["height"].as_u64().unwrap()as u32);assert_eq!(result.pixels,expected,"{} grant {grant}",row["name"]);}assert_eq!(*source.data,*before);assert_eq!(oracle(&source.data),expected,"{} independent png oracle",row["name"]);}
 eprintln!("[DEBUG] All 32 native neutral PNG fixtures matched independent png under grants 1, 7 and 4096");
}
#[test]
fn png_decode_shared_malformed_files_have_no_publishable_candidate(){
 let rows:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/⚠️invalid/🔣️.json")).unwrap();
 for row in rows.as_array().unwrap(){let source=input(&row["input"]);if let Ok(mut job)=PngDecodeJob::new(source.clone()){assert!(finish(&mut job,7).is_err(),"{}",row["name"]);assert!(job.result().is_err());assert!(job.advance(1).is_err());}let name=row["name"].as_str().unwrap();if name.starts_with("incomplete dynamic")||name=="oversubscribed dynamic Huffman tree"{let decoder=png::Decoder::new(std::io::Cursor::new(source.data.as_slice()));if let Ok(mut reader)=decoder.read_info(){let mut bytes=vec![0;reader.output_buffer_size()];assert!(reader.next_frame(&mut bytes).is_err());}}}
 eprintln!("[DEBUG] All 38 malformed native PNG cases refused publication");
}
#[test]
fn png_decode_cancel_and_invalid_grants_purge_private_candidates(){
 let rows:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();
 for steps in [0,1,10,100,300,600]{let mut job=PngDecodeJob::new(input(&rows[24]["input"])).unwrap();for _ in 0..steps{if job.advance(1).unwrap().done{break;}}job.cancel();assert!(job.advance(1).is_err());assert!(job.result().is_err());}
 let mut job=PngDecodeJob::new(input(&rows[0]["input"])).unwrap();assert!(job.advance(0).is_err());assert!(job.result().is_err());
}
#[test]
fn png_decode_independent_encoder_wraps_history_and_drops_unfinished_jobs(){
 let (width,height)=(257u32,41u32);let mut expected=Vec::new();let mut seed=0x762ba5u32;
 for y in 0..height{for x in 0..width*4{seed=seed.wrapping_mul(1664525).wrapping_add(1013904223);expected.push(if x%9==0{(seed>>24)as u8}else{((x+y)%32)as u8});}}
 let mut bytes=Vec::new();{let mut encoder=png::Encoder::new(&mut bytes,width,height);encoder.set_color(png::ColorType::Rgba);encoder.set_depth(png::BitDepth::Eight);encoder.write_header().unwrap().write_image_data(&expected).unwrap();}
 let make=||PngDecodeInput{data:Arc::new(bytes.clone()),max_pixels:16384,max_bytes:1048576,max_chunks:4096};
 for grant in [1,97,4096]{let mut job=PngDecodeJob::new(make()).unwrap();finish(&mut job,grant).unwrap();assert_eq!(job.into_result().unwrap().pixels,expected);}
 for stop in [0,1,10,100,1000,20000,100000]{let mut job=PngDecodeJob::new(make()).unwrap();job.advance(stop+1).unwrap();drop(job);}
 assert_eq!(oracle(&bytes),expected);eprintln!("[DEBUG] Native PNG decoded independently encoded pixels beyond 32 KiB and released unfinished jobs");
}
#[test]
fn png_decode_public_entry_point_enforces_whole_image_contract(){
 let rows:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();
 for row in rows.as_array().unwrap(){let source=input(&row["input"]);let expected:Vec<u8>=serde_json::from_value(row["expected"]["pixels"].clone()).unwrap();assert_eq!(super::super::decode_png(&source.data).unwrap().pixels,expected,"{}",row["name"]);}
 let rows:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/⚠️invalid/🔣️.json")).unwrap();
 for row in rows.as_array().unwrap(){if !matches!(row["name"].as_str().unwrap(),"pixel limit"|"chunk limit"|"byte limit"){assert!(super::super::decode_png(&input(&row["input"]).data).is_err(),"{}",row["name"]);}}
 eprintln!("[DEBUG] Public native PNG entry point matches candidate reconstruction and rejects malformed files");
}
#[test]
fn png_decode_truncated_fixture_prefixes_never_publish(){
 let rows:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();let mut prefixes=0;
 for row in rows.as_array().unwrap(){let source=input(&row["input"]);for length in 0..source.data.len(){let mut truncated=source.clone();truncated.data=Arc::new(source.data[..length].to_vec());if let Ok(mut job)=PngDecodeJob::new(truncated){assert!(finish(&mut job,4096).is_err(),"{} prefix {length}",row["name"]);assert!(job.result().is_err());}prefixes+=1;}}
 eprintln!("[DEBUG] Native PNG refused all {prefixes} truncated fixture prefixes");
}
