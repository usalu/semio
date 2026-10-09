//! 🧪️ Incremental PNG publication agrees with the independent png decoder.
use super::*;

#[test]
fn incremental_png_encodes_scanline_and_block_boundaries() {
    let image=RasterImage {width:513,height:3,pixels:(0..513*3*4).map(|i|(i*37%256) as u8).collect()};
    let expected=image.pixels.clone();
    let mut job=PngEncodeJob::new(image).unwrap();
    assert!(job.result().is_err());
    let progress=job.advance().unwrap();
    assert_eq!(progress.completed,4096);
    assert!(!progress.done);
    assert!(job.result().is_err());
    while !job.advance().unwrap().done {}
    let output=job.into_result().unwrap();
    let mut reader=png::Decoder::new(std::io::Cursor::new(&output.data)).read_info().unwrap();
    let mut decoded=vec![0;reader.output_buffer_size()];
    let info=reader.next_frame(&mut decoded).unwrap();
    assert_eq!((info.width,info.height),(513,3));
    assert_eq!(&decoded[..info.buffer_size()],expected.as_slice());
    assert_eq!(super::super::decode_png(&output.data).unwrap().pixels,expected);
}

#[test]
fn incremental_png_cancel_never_exposes_partial_bytes() {
    let mut job=PngEncodeJob::new(RasterImage::new(64,64)).unwrap();
    job.advance().unwrap();
    job.cancel();
    assert!(job.advance().is_err());
    assert!(job.result().is_err());
}

#[test]
fn png_encoding_refusal_returns_original_source_and_cancel_retains_candidate() {
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();
    let image=RasterImage {width:fixture["width"].as_u64().unwrap() as u32,height:fixture["height"].as_u64().unwrap() as u32,pixels:fixture["pixels"].as_array().unwrap().iter().map(|v|v.as_u64().unwrap() as u8).collect()};
    let source=image.pixels.as_ptr();
    let (error,image)=match PngEncodeJob::with_maximum_bytes(image,fixture["maximumBytes"].as_u64().unwrap() as usize){Err(pair)=>pair,Ok(_)=>panic!("neutral admission must refuse")};
    assert!(matches!(error,PixelEditError::Invalid(_)));
    assert_eq!(image.pixels.as_ptr(),source);
    assert_eq!(image.pixels,vec![255,0,0,255,0,0,255,128]);
    let mut job=PngEncodeJob::new(image).unwrap();
    job.advance().unwrap();
    let candidate=job.bytes.as_ptr();
    let capacity=job.bytes.capacity();
    job.cancel();
    assert!(job.result().is_err());
    assert_eq!(job.image.pixels.as_ptr(),source);
    assert_eq!(job.bytes.as_ptr(),candidate);
    assert_eq!(job.bytes.capacity(),capacity);
    let (allocated,released)=crate::retirement::tests::drain(job);
    assert!(released>allocated);
    println!("[DEBUG] PNG neutral refusal preserves original source pointer; cancellation preserves candidate until exact caller-funded physical close");
}
#[test]
fn png_encoding_shared_source_keeps_exact_raster_lease_through_refusal_and_close(){
 use std::sync::Arc;
 use crate::retirement::RasterLease;
 let fixture:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();assert_eq!(fixture["sourceAuthorities"],serde_json::json!(["owned","shared"]));
 let source=Arc::new(RasterImage {width:fixture["width"].as_u64().unwrap()as u32,height:fixture["height"].as_u64().unwrap()as u32,pixels:fixture["pixels"].as_array().unwrap().iter().map(|v|v.as_u64().unwrap()as u8).collect()});let pointer=source.pixels.as_ptr();
 let lease=RasterLease(source.clone());let(error,lease)=match PngEncodeJob::with_maximum_bytes(lease,fixture["maximumBytes"].as_u64().unwrap()as usize){Err(pair)=>pair,Ok(_)=>panic!("shared admission must refuse")};assert!(matches!(error,PixelEditError::Invalid(_)));assert_eq!(lease.pixels.as_ptr(),pointer);assert_eq!(Arc::strong_count(&source),2);
 let mut job=PngEncodeJob::with_maximum_bytes(lease,1_000_000).unwrap();assert_eq!(job.image.raster().pixels.as_ptr(),pointer);while !job.advance().unwrap().done{}let output=job.take_result().unwrap();let mut decoder=png::Decoder::new(std::io::Cursor::new(&output.data)).read_info().unwrap();let mut pixels=vec![0;decoder.output_buffer_size()];let info=decoder.next_frame(&mut pixels).unwrap();assert_eq!(&pixels[..info.buffer_size()],source.pixels.as_slice());drop(decoder);assert_eq!(Arc::strong_count(&source),2);let(allocated,released)=crate::retirement::tests::drain(job);assert!(released>=allocated);assert_eq!(Arc::strong_count(&source),1);assert_eq!(source.pixels.as_ptr(),pointer);crate::retirement::tests::drain(output);crate::retirement::tests::drain(RasterLease(source));println!("[DEBUG] Shared PNG encoding borrowed original raster pointer without copying pixels, independently decoded samples, and physically retired exact lease under caller grants");
}