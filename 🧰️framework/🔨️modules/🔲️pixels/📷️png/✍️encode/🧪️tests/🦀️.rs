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
