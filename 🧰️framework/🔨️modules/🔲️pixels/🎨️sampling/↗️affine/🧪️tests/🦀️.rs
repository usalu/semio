use super::*;
#[test]
fn affine_retirement_preserves_real_private_owners_and_transfers_complete_pixels(){
 let sources:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();let cases:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/🧹️retirement/🔣️.json")).unwrap();assert_eq!(cases.as_array().unwrap().len(),16);
 for row in cases.as_array().unwrap(){for grant in [1,7,4096]{
  let source=sources.as_array().unwrap().iter().find(|v|v["name"]==row["source"]).unwrap();let mut v=input(&source["input"]);let mode=row["mode"].as_str().unwrap();if mode=="failure"{v.transform=[1e9,0.0,0.0,1.0,1e9,0.0];}
  let shared=Arc::clone(&v.source);let before=shared.pixels.clone();let mut job=AffineImageJob::new(v).unwrap();let phase=row["phase"].as_str().unwrap();
  if mode=="failure"{assert!(job.advance(100000).is_err());assert!(job.failed.is_some());assert!(job.coverage.is_some());assert!(job.advance(1).is_err());}else{let mut reached=false;for _ in 0..2000000{if job.phase==phase{reached=true;break;}job.advance(1).unwrap();}assert!(reached,"{}",row["name"]);let steps=row["steps"].as_u64().unwrap()as usize;if steps>0{job.advance(steps).unwrap();}}
  let candidate=job.output.pixels.as_ptr();let active=job.coverage.is_some();let closing=job.coverage_retirement.is_some();let published=if mode=="published"{Some(job.result().unwrap().pixels.clone())}else{None};
  if mode=="cancelled"||mode=="published"{job.cancel();assert_eq!(job.output.pixels.as_ptr(),candidate);assert_eq!(job.coverage.is_some(),active);assert_eq!(job.coverage_retirement.is_some(),closing);assert!(Arc::ptr_eq(job.source.as_ref().unwrap(),&shared));assert!(job.result().is_err());assert!(job.advance(1).is_err());}
  let(mut retired,output)=job.into_retirement();assert_eq!(output.is_some(),phase=="complete"&&mode=="live");if let Some(image)=&output{assert_eq!(image.pixels.as_ptr(),candidate);}
  let state=retired.job.as_ref().unwrap();assert!(state.coverage.is_none());assert_eq!(state.coverage_retirement.is_some(),active||closing);assert!(Arc::ptr_eq(state.source.as_ref().unwrap(),&shared));if output.is_none(){assert_eq!(state.output.pixels.as_ptr(),candidate);}
  if closing{let p=retired.job.as_mut().unwrap().coverage_retirement.as_mut().unwrap().advance(1).unwrap();assert_eq!(p.work,2,"closing child counter must continue");}
  assert!(retired.advance(0).is_err());if usize::BITS>53{assert!(retired.advance(usize::MAX).is_err());}
  let mut work=0;let mut done=false;
  for _ in 0..2000000{let p=retired.advance(grant).unwrap();assert!(p.work>work&&p.work-work<=grant as u64);work=p.work;if let Some(state)=&retired.job{if state.coverage_retirement.as_ref().is_some_and(|v|!v.terminal_is_empty()){assert!(Arc::ptr_eq(state.source.as_ref().unwrap(),&shared));}}if p.done{done=true;break;}}
  assert!(done&&retired.terminal_is_empty());assert!(retired.job.is_none());let terminal=retired.advance(1).unwrap();assert_eq!(terminal.phase,"complete");assert_eq!(terminal.work,work);assert!(terminal.done);assert_eq!(shared.pixels,before);assert_eq!(Arc::strong_count(&shared),1);
  if let Some(image)=output{let expected:Vec<u8>=serde_json::from_value(source["expected"].clone()).unwrap();assert_eq!(image.pixels,expected);}if let Some(pixels)=published{let expected:Vec<u8>=serde_json::from_value(source["expected"].clone()).unwrap();assert_eq!(pixels,expected);}
  eprintln!("[DEBUG] Actual native affine retirement {}: grant={grant} work={work} terminal_empty=true child_adopted={}",row["name"],active||closing);
 }}
}
#[test]
fn affine_sampling_waits_for_the_real_coverage_destructor_before_writing_any_pixel(){
 let sources:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();let mut job=AffineImageJob::new(input(&sources[9]["input"])).unwrap();let mut reached=false;
 for _ in 0..2000000{job.advance(1).unwrap();if job.phase=="coverageCleanup"{reached=true;break;}}assert!(reached);assert!(job.coverage.is_none());let mask=job.mask.as_ref().unwrap().coverage.as_ptr();let mut grants=0;
 while !job.coverage_retirement.as_ref().unwrap().terminal_is_empty(){job.advance(1).unwrap();grants+=1;assert_eq!(job.phase,"coverageCleanup");assert_eq!(job.at,0);assert!(job.output.pixels.iter().all(|v|*v==0));assert_eq!(job.mask.as_ref().unwrap().coverage.as_ptr(),mask);}
 assert!(grants>=14);job.advance(1).unwrap();assert!(job.coverage_retirement.is_none());assert_eq!(job.phase,"coverageCleanup");job.advance(1).unwrap();assert_eq!(job.phase,"sampling");while !job.advance(4096).unwrap().done{}let expected:Vec<u8>=serde_json::from_value(sources[9]["expected"].clone()).unwrap();assert_eq!(job.result().unwrap().pixels,expected);
 eprintln!("[DEBUG] Actual native affine coverage handoff: {grants} one-unit grants; mask pointer retained and pixels untouched until child terminal");
}
fn input(v:&serde_json::Value)->AffineImageInput{let source=&v["source"];AffineImageInput{source:Arc::new(RasterImage{width:source["width"].as_u64().unwrap()as u32,height:source["height"].as_u64().unwrap()as u32,pixels:serde_json::from_value(source["pixels"].clone()).unwrap()}),width:v["width"].as_u64().unwrap()as u32,height:v["height"].as_u64().unwrap()as u32,origin:serde_json::from_value(v["origin"].clone()).unwrap(),transform:serde_json::from_value(v["transform"].clone()).unwrap(),sampling:match v["sampling"].as_str().unwrap(){"nearest"=>AffineSampling::Nearest,"bilinear"=>AffineSampling::Bilinear,"area"=>AffineSampling::Area,"auto"=>AffineSampling::Auto,_=>panic!("unknown fixture filter")}}}
#[test]
fn affine_image_samples_match_every_shared_rgba_under_bounded_grants(){
 let cases:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();
 for row in cases.as_array().unwrap(){for grant in [1,7,4096]{let mut job=AffineImageJob::new(input(&row["input"])).unwrap();let mut work=0;let mut done=false;for _ in 0..2000000{let p=job.advance(grant).unwrap();assert!(p.work-work<=grant as u64);work=p.work;if p.done{done=true;break;}}assert!(done);let expected:Vec<u8>=serde_json::from_value(row["expected"].clone()).unwrap();assert_eq!(job.result().unwrap().pixels,expected,"{} grant {grant}",row["name"]);}}
 eprintln!("[DEBUG] All twenty-one native affine samples matched neutral RGBA under grants 1, 7 and 4096");
}
#[test]
fn affine_image_samples_refuse_partial_cancelled_and_failed_candidates(){
 let cases:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();
 for steps in [0,1,5,20,60]{let mut job=AffineImageJob::new(input(&cases[9]["input"])).unwrap();assert!(job.result().is_err());for _ in 0..steps{job.advance(1).unwrap();}job.cancel();assert!(job.advance(1).is_err());assert!(job.result().is_err());}
 assert!(AffineImageJob::new(input(&cases[0]["input"])).unwrap().advance(0).is_err());
 if usize::BITS>53{assert!(AffineImageJob::new(input(&cases[0]["input"])).unwrap().advance(usize::MAX).is_err());}
 let mut v=input(&cases[0]["input"]);v.width=0;assert!(AffineImageJob::new(v).is_err());
 let mut v=input(&cases[9]["input"]);v.transform=[1e9,0.0,0.0,1.0,1e9,0.0];let mut job=AffineImageJob::new(v).unwrap();assert!(job.advance(100000).is_err());assert!(job.result().is_err());assert!(job.advance(1).is_err());
}
