use super::*;
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
 let mut v=input(&cases[0]["input"]);v.width=0;assert!(AffineImageJob::new(v).is_err());
 let mut v=input(&cases[9]["input"]);v.transform=[1e9,0.0,0.0,1.0,1e9,0.0];let mut job=AffineImageJob::new(v).unwrap();assert!(job.advance(100000).is_err());assert!(job.result().is_err());assert!(job.advance(1).is_err());
}
