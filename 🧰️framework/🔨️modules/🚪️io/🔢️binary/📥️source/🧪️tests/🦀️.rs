use super::*;
use base64::Engine;
fn input(v:&serde_json::Value)->Result<BinarySourceInput,String>{
 let number=|key:&str|v[key].as_u64().ok_or_else(||format!("Invalid {key}"));
 Ok(BinarySourceInput{mime:v["mime"].as_str().ok_or("MIME")?.into(),data:Arc::new(v["data"].as_str().ok_or("Data")?.into()),min_bytes:number("minBytes")?as usize,max_bytes:number("maxBytes")?as usize,max_source_bytes:number("maxSourceBytes")?as usize,max_work:number("maxWork")?})
}
fn rows()->serde_json::Value{serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap()}
fn finish(job:&mut BinarySourceJob,grant:usize)->Result<BinarySourceProgress,BinarySourceError>{
 let(mut work,mut read,mut bytes)=(0,0,0);
 for _ in 0..2000000{let p=job.advance(grant)?;assert!(p.work-work<=grant as u64);assert!(p.source_completed>=read&&p.source_completed<=p.source_total);assert!(p.bytes>=bytes&&p.bytes<=p.total_bytes);work=p.work;read=p.source_completed;bytes=p.bytes;if p.done{return Ok(p);}}
 panic!("Source did not finish")
}
fn oracle(data:&str)->Vec<u8>{
 if data.get(..5).is_some_and(|v|v.eq_ignore_ascii_case("data:")){
  let(header,body)=data.split_once(',').unwrap();let bytes=percent_encoding::percent_decode_str(body).collect::<Vec<_>>();
  if header.to_ascii_lowercase().ends_with(";base64"){base64::engine::general_purpose::STANDARD.decode(bytes).unwrap()}else{bytes}
 }else{base64::engine::general_purpose::STANDARD.decode(data).unwrap()}
}
#[test]
fn binary_sources_match_neutral_font_image_bytes_and_independent_oracles(){
 for row in rows().as_array().unwrap(){
  let expected:Vec<u8>=serde_json::from_value(row["expected"].clone()).unwrap();
  for grant in [1,7,4096]{let mut job=BinarySourceJob::new(input(&row["input"]).unwrap()).unwrap();assert!(job.result().is_err());finish(&mut job,grant).unwrap();assert_eq!(job.into_result().unwrap(),expected,"{} grant {grant}",row["name"]);}
  assert_eq!(oracle(row["input"]["data"].as_str().unwrap()),expected,"{} independent oracle",row["name"]);
 }
 eprintln!("[DEBUG] Twelve native binary font/image fixtures match independent base64/percent bytes at grants 1/7/4096");
}
#[test]
fn binary_sources_refuse_malformed_contracts_and_private_failures(){
 let invalid:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/⚠️invalid/🔣️.json")).unwrap();
 for row in invalid.as_array().unwrap(){if let Ok(value)=input(&row["input"]){if let Ok(mut job)=BinarySourceJob::new(value){assert!(finish(&mut job,7).is_err(),"{}",row["name"]);let error=job.advance(1).unwrap_err();assert_eq!(job.advance(1).unwrap_err(),error);assert!(job.result().is_err());}}}
 eprintln!("[DEBUG] Native binary source malformed/cap refusals retain no published output");
}
#[test]
fn binary_sources_exact_admission_and_every_phase_cancellation(){
 let rows=rows();let limits=input(&rows[1]["input"]).unwrap();let mut baseline=BinarySourceJob::new(limits.clone()).unwrap();let p=finish(&mut baseline,1).unwrap();let expected=baseline.into_result().unwrap();
 let mut exact=limits.clone();exact.max_work=p.work;let mut job=BinarySourceJob::new(exact).unwrap();finish(&mut job,7).unwrap();assert_eq!(job.into_result().unwrap(),expected);
 for cap in 0..4{let mut value=limits.clone();match cap{0=>value.max_work=p.work-1,1=>value.max_bytes-=1,2=>value.max_source_bytes-=1,_=>{value.min_bytes=value.max_bytes+1;value.max_bytes+=1;}}if let Ok(mut job)=BinarySourceJob::new(value){assert!(finish(&mut job,7).is_err());assert!(job.result().is_err());}}
 let mut seen=Vec::new();let mut probe=BinarySourceJob::new(limits.clone()).unwrap();let mut steps=0;
 loop{let p=probe.advance(1).unwrap();steps+=1;if !seen.contains(&p.phase){seen.push(p.phase);let mut job=BinarySourceJob::new(limits.clone()).unwrap();job.advance(steps).unwrap();let published=if p.done{Some(std::mem::take(&mut job.output))}else{None};job.cancel();assert_eq!(job.advance(1),Err(BinarySourceError::Cancelled));assert_eq!(job.result().unwrap_err(),BinarySourceError::Cancelled);if let Some(value)=published{assert_eq!(value,expected);}assert!(job.source.is_none()&&job.binary.is_empty()&&job.output.is_empty());}if p.done{break;}}
 assert_eq!(seen,["header","validate","decode","complete"]);
 assert!(BinarySourceJob::new(limits.clone()).unwrap().advance(0).is_err());
 let mut job=BinarySourceJob::new(limits.clone()).unwrap();assert_eq!(Arc::strong_count(&limits.data),2);job.cancel();assert_eq!(Arc::strong_count(&limits.data),1);
 eprintln!("[DEBUG] Native exact source/byte/work limits and all phase cancellations release source ownership");
}

#[test]
fn binary_sources_load_actual_bundled_font_resources(){
 let rows:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/🔤️fonts/🔣️.json")).unwrap();
 let fonts:&[&[u8]]=&[include_bytes!("../../../../🖼️assets/🔤️fonts/😀️noto-emoji/🪉️supplement/⚖️medium/🔤️outline.ttf"),include_bytes!("../../../../📚️compiler/🌍️world/🔤️fonts/⌨️LibertinusMono-Regular.otf")];
 for (row,bytes) in rows.as_array().unwrap().iter().zip(fonts){
  assert_eq!(bytes.len(),row["bytes"].as_u64().unwrap()as usize);let mime=row["mime"].as_str().unwrap();let data=format!("data:{mime};name=Document%20Font;base64,{}",base64::engine::general_purpose::STANDARD.encode(bytes));
  let input=BinarySourceInput{mime:mime.into(),min_bytes:bytes.len(),max_bytes:bytes.len(),max_source_bytes:data.len(),max_work:data.len()as u64*2+2,data:Arc::new(data.clone())};let mut job=BinarySourceJob::new(input).unwrap();finish(&mut job,4096).unwrap();assert_eq!(job.into_result().unwrap(),*bytes);assert_eq!(oracle(&data),*bytes);
  eprintln!("[DEBUG] Actual bundled {mime} preserves all {} encoded-resource bytes",bytes.len());
 }
}
