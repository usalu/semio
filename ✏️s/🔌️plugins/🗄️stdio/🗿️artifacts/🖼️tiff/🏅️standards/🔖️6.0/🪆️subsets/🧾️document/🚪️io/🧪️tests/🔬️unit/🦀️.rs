//! 🧪️ Owned TIFF laws and independent native image witnesses.
use crate::schema::mutations::paint_region::samples::*;
use super::*;
/// 🧪️ The painted page for the witnesses: the sparse runs of the paint written into a copy of the page, the way the central applier would.
pub fn paint_tiff_region_controlled(snapshot:&TiffSnapshot,revision:&str,ifd_index:usize,region:TiffRegion,color:[u8;4],progress:&mut dyn FnMut(usize,usize)->bool)->Result<TiffSnapshot,String>{
 let runs=paint_tiff_region_runs(snapshot,revision,ifd_index,region,color,progress)?;let mut next=snapshot.clone();
 for run in &runs{next.ifds[ifd_index].blocks[run.block].samples[run.offset..run.offset+run.samples.len()].copy_from_slice(&run.samples);}
 Ok(next)
}
const OWNED:&str=include_str!("../../../🧫️fixtures/🧬️owned-native-samples/🔣️.json");
const TILED:&str=include_str!("../../../🧫️fixtures/🧬️tiled-8bit/🔣️.json");
fn fixture()->serde_json::Value{serde_json::from_str(OWNED).unwrap()}
fn short(tag:u16,value:Vec<u16>)->TiffTag{TiffTag{tag,values:TiffValues::Short(value)}}
fn long(tag:u16,value:u32)->TiffTag{TiffTag{tag,values:TiffValues::Long(vec![value])}}
fn page(width:u32,height:u32,channels:u16,photo:u16,pixels:Vec<u8>)->TiffIfd{TiffIfd{entries:vec![long(256,width),long(257,height),short(258,vec![8;channels as usize]),short(262,vec![photo]),short(277,vec![channels])],blocks:vec![TiffSampleBlock{x:0,y:0,width,height,channels,samples:pixels.into_iter().map(|v|TiffWord64::from_word(v.into())).collect()}]}}
fn rgb()->TiffSnapshot{TiffSnapshot{schema:STDIO_TIFF_DOCUMENT_SCHEMA.into(),ifds:vec![page(2,1,3,2,vec![255,0,0,0,255,0])]}}
fn tiled()->TiffSnapshot{
 let f:serde_json::Value=serde_json::from_str(TILED).unwrap();let width=f["width"].as_u64().unwrap()as u32;let height=f["height"].as_u64().unwrap()as u32;
 let mut pixels=vec![0;width as usize*height as usize*3];
 for y in 0..height{for x in 0..width{let index=(y/16*2+x/16)as usize;let color=f["tiles"][index]["rgb"].as_array().unwrap();let at=((y*width+x)*3)as usize;for lane in 0..3{pixels[at+lane]=color[lane].as_u64().unwrap()as u8;}}}
 let mut first=page(width,height,3,2,pixels);first.entries.push(TiffTag{tag:65000,values:TiffValues::Undefined(vec![4,2,4,2])});
 TiffSnapshot{schema:STDIO_TIFF_DOCUMENT_SCHEMA.into(),ifds:vec![first,rgb().ifds.remove(0)]}
}
fn options(order:TiffByteOrder,compression:TiffCompression,layout:TiffNativeLayout)->TiffNativeOptions{TiffNativeOptions{byte_order:order,compression,layout}}
#[test]fn exact_owned_words_survive_native_order_compression_and_layout(){
 for case in fixture()["cases"].as_array().unwrap(){let snapshot:TiffSnapshot=semio_framework_pack_json::from_json_str(&case["snapshot"].to_string(),semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();snapshot.validate().unwrap();
 for order in [TiffByteOrder::LittleEndian,TiffByteOrder::BigEndian]{for compression in [TiffCompression::None,TiffCompression::PackBits,TiffCompression::Lzw,TiffCompression::Deflate]{for layout in [TiffNativeLayout::SingleStrip,TiffNativeLayout::Strips{rows:1},TiffNativeLayout::Tiles{width:16,height:16}]{let bytes=encode_tiff_with(&snapshot,options(order,compression,layout)).unwrap();assert_eq!(decode_tiff(&bytes).unwrap(),snapshot);}}}}
}
#[test]fn retired_native_fields_have_no_semantic_admission(){for value in fixture()["reject"].as_array().unwrap(){assert!(semio_framework_pack_json::from_json_str::<TiffSnapshot>(&value.to_string(),semio_framework_pack_json::JsonMemberPolicy::Reject).is_err());}}
#[test]fn all_grayscale_ieee16_paints_match_independent_neutral_words(){let fixture:serde_json::Value=serde_json::from_str(include_str!("../../../🧫️fixtures/🧮️ieee16-paint/🔣️.json")).unwrap();let snapshot:TiffSnapshot=semio_framework_pack_json::from_json_str(&fixture["snapshot"].to_string(),semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();for case in fixture["cases"].as_array().unwrap(){let component=case["component"].as_u64().unwrap()as u8;let next=paint_tiff_region_controlled(&snapshot,&tiff_revision(&snapshot),0,TiffRegion{x:0,y:0,width:1,height:1},[component,component,component,255],&mut|_,_|true).unwrap();assert_eq!(next.ifds[0].blocks[0].samples[0],semio_framework_pack_json::from_json_str::<TiffWord64>(&case["expected"].to_string(),semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap());let native=encode_tiff(&next).unwrap();let mut independent=tiff::decoder::Decoder::new(std::io::Cursor::new(native)).unwrap();let tiff::decoder::DecodingResult::F16(samples)=independent.read_image().unwrap()else{panic!("independent IEEE16 paint raster")};assert_eq!(samples[0].to_bits(),u16::try_from(next.ifds[0].blocks[0].samples[0].lo).unwrap());}eprintln!("[DEBUG] all 256 IEEE16 grayscale paints match neutral Python struct words and independent TIFF decoder");}
#[test]fn png_projection_is_ephemeral_and_pixel_exact(){let snapshot=rgb();let before=snapshot.clone();let projected=decode_tiff_page_rgba(&snapshot,0).unwrap();assert_eq!(projected.pixels,vec![255,0,0,255,0,255,0,255]);let png=encode_tiff_page_png(&snapshot,0).unwrap();assert_eq!(image::load_from_memory_with_format(&png.bytes,image::ImageFormat::Png).unwrap().to_rgba8().into_raw(),projected.pixels);assert_eq!(snapshot,before);}
#[test]fn row_strips_preserve_logical_authored_order(){let snapshot=TiffSnapshot{schema:STDIO_TIFF_DOCUMENT_SCHEMA.into(),ifds:vec![page(1,2,3,2,vec![255,0,0,0,255,0])]};let bytes=encode_tiff_with(&snapshot,options(TiffByteOrder::BigEndian,TiffCompression::None,TiffNativeLayout::Strips{rows:1})).unwrap();assert_eq!(decode_tiff(&bytes).unwrap(),snapshot);}
#[test]fn malformed_owned_cardinality_is_refused_without_rewriting_source(){let mut snapshot=rgb();snapshot.ifds[0].blocks[0].samples.pop();let before=snapshot.clone();assert!(encode_tiff(&snapshot).is_err());assert_eq!(snapshot,before);}
#[test]fn neutral_tile_padding_is_not_owned_sample_authority(){let snapshot=tiled();let before=snapshot.clone();let bytes=encode_tiff_with(&snapshot,options(TiffByteOrder::LittleEndian,TiffCompression::None,TiffNativeLayout::Tiles{width:16,height:16})).unwrap();assert_eq!(decode_tiff(&bytes).unwrap(),snapshot);let projection=decode_tiff_page_rgba(&snapshot,0).unwrap();for (x,y,color)in[(0,0,[255,0,0,255]),(16,0,[0,255,0,255]),(0,16,[0,0,255,255]),(16,16,[255,255,0,255])]{let at=(y*17+x)*4;assert_eq!(&projection.pixels[at..at+4],&color);}assert_eq!(snapshot,before);}
#[test]fn region_paint_is_revision_guarded_cancellable_and_preserves_other_authority(){let before=tiled();let region=TiffRegion{x:15,y:15,width:2,height:2};let revision=tiff_revision(&before);let mut progress=vec![];let after=paint_tiff_region_controlled(&before,&revision,0,region,[9,8,7,255],&mut|done,total|{progress.push((done,total));true}).unwrap();assert_eq!(after.ifds[0].entries,before.ifds[0].entries);assert_eq!(after.ifds[1],before.ifds[1]);assert_eq!(progress.last(),Some(&(2,2)));let rgba=decode_tiff_page_rgba(&after,0).unwrap();for y in 15..17{for x in 15..17{let at=(y*17+x)*4;assert_eq!(&rgba.pixels[at..at+4],&[9,8,7,255]);}}assert!(paint_tiff_region_controlled(&before,"stale",0,region,[9,8,7,255],&mut|_,_|true).is_err());assert!(paint_tiff_region_controlled(&before,&revision,0,region,[9,8,7,255],&mut|_,_|false).is_err());assert_eq!(before,tiled());}
#[test]fn image_rs_independently_decodes_native_tiled_edits(){let before=tiled();let after=paint_tiff_region_controlled(&before,&tiff_revision(&before),0,TiffRegion{x:16,y:16,width:1,height:1},[9,8,7,255],&mut|_,_|true).unwrap();for compression in [TiffCompression::None,TiffCompression::PackBits,TiffCompression::Lzw,TiffCompression::Deflate]{let bytes=encode_tiff_with(&after,options(TiffByteOrder::LittleEndian,compression,TiffNativeLayout::Tiles{width:16,height:16})).unwrap();let image=image::load_from_memory_with_format(&bytes,image::ImageFormat::Tiff).unwrap().to_rgba8();assert_eq!(image.dimensions(),(17,17));assert_eq!(image.get_pixel(0,0).0,[255,0,0,255]);assert_eq!(image.get_pixel(16,16).0,[9,8,7,255]);}}
#[test]fn white_is_zero_and_unassociated_alpha_paint_exact_owned_samples(){for(channels,photo,pixel,color)in[(1,0,vec![0],[10,10,10,255]),(4,2,vec![1,2,3,4],[5,6,7,8])]{let mut ifd=page(1,1,channels,photo,pixel);if channels==4{ifd.entries.push(short(338,vec![2]));}let before=TiffSnapshot{schema:STDIO_TIFF_DOCUMENT_SCHEMA.into(),ifds:vec![ifd]};let after=paint_tiff_region_controlled(&before,&tiff_revision(&before),0,TiffRegion{x:0,y:0,width:1,height:1},color,&mut|_,_|true).unwrap();assert_eq!(decode_tiff_page_rgba(&after,0).unwrap().pixels,color);assert_eq!(after.ifds[0].blocks[0].samples[0].lo,if photo==0{245}else{5});}}
#[test]fn neutral_ccitt_authored_modes_decode_exact_bilevel_rows(){
 let fixture:serde_json::Value=serde_json::from_str(include_str!("../../💾️binary/📸️snapshot/📠️fax/🧫️fixtures/🔣️.json")).unwrap();
 for case in fixture["cases"].as_array().unwrap(){let input:Vec<u8>=serde_json::from_value(case["compressed"].clone()).unwrap();let expected:Vec<u8>=serde_json::from_value(case["expected"].clone()).unwrap();let mut progress=|_|true;let mut control=semio_framework_value::NativeDecodeControl::new(1_000_000,&mut progress);let output=fax::decode(&input,case["compression"].as_u64().unwrap()as u32,case["width"].as_u64().unwrap()as usize,case["height"].as_u64().unwrap()as usize,case["fill"].as_u64().unwrap()as u32,case["options"].as_u64().unwrap()as u32,&mut control).unwrap();assert_eq!(output,expected);}
}
#[test]fn fax_independently_decodes_ccitt_native_emission(){
 let mut ifd=page(8,2,1,1,vec![0,0,0,0,1,1,1,1,1,1,1,1,0,0,0,0]);ifd.entries[2].values=TiffValues::Short(vec![1]);let snapshot=TiffSnapshot{schema:STDIO_TIFF_DOCUMENT_SCHEMA.into(),ifds:vec![ifd]};
 for compression in [TiffCompression::ModifiedHuffman,TiffCompression::Group3,TiffCompression::Group4]{
  let bytes=encode_tiff_with(&snapshot,options(TiffByteOrder::LittleEndian,compression,TiffNativeLayout::Strips{rows:1})).unwrap();assert_eq!(decode_tiff(&bytes).unwrap(),snapshot);
  let mut metadata=tiff::decoder::Decoder::new(std::io::Cursor::new(&bytes)).unwrap();let offsets=metadata.get_tag_u32_vec(tiff::tags::Tag::StripOffsets).unwrap();let counts=metadata.get_tag_u32_vec(tiff::tags::Tag::StripByteCounts).unwrap();let mut pixels=vec![];
  for(offset,count)in offsets.into_iter().zip(counts){let strip=&bytes[offset as usize..(offset+count)as usize];let mut transitions=vec![];
   if compression==TiffCompression::Group4{::fax::decoder::decode_g4(strip.iter().copied(),8,Some(1),|row|transitions.extend_from_slice(row)).expect("independent Group4 raster");}
   else{let source=if compression==TiffCompression::ModifiedHuffman{let mut bits=vec![0u8;11];bits.push(1);for byte in strip{for shift in(0..8).rev(){bits.push((byte>>shift)&1);}}for _ in 0..6{bits.extend_from_slice(&[0,0,0,0,0,0,0,0,0,0,0,1]);}bits.chunks(8).map(|chunk|chunk.iter().enumerate().fold(0,|byte,(at,bit)|byte|bit<<(7-at))).collect::<Vec<_>>()}else{strip.to_vec()};let mut decoder=::fax::decoder::Group3Decoder::new(source.into_iter().map(Ok::<_,std::convert::Infallible>)).unwrap();decoder.advance().unwrap();transitions.extend_from_slice(decoder.transitions());}
   pixels.extend(::fax::decoder::pels(&transitions,8).map(|color|if color==::fax::Color::Black{1}else{0}));
  }
  assert_eq!(pixels,snapshot.ifds[0].blocks[0].samples.iter().map(|word|word.lo as u8).collect::<Vec<_>>());
 }eprintln!("[DEBUG] independent fax decoder validates native CCITT Modified Huffman, Group3 and Group4 payloads; third-party TIFF reads strip spans");
}
#[test]fn neutral_native_jpeg_predictor_and_fill_profiles_admit_exact_owned_words(){let cases:serde_json::Value=serde_json::from_str(include_str!("../../💾️binary/📸️snapshot/🧫️fixtures/🧮️native-profiles/🔣️.json")).unwrap();for case in cases["cases"].as_array().unwrap(){let hex=case["nativeHex"].as_str().unwrap();let bytes:Vec<u8>=(0..hex.len()).step_by(2).map(|at|u8::from_str_radix(&hex[at..at+2],16).unwrap()).collect();let expected:TiffSnapshot=semio_framework_pack_json::from_json_str(&case["snapshot"].to_string(),semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();assert_eq!(decode_tiff(&bytes).unwrap(),expected,"{}",case["name"]);if case["name"].as_str().unwrap().starts_with("floating predictor3"){let mut decoder=tiff::decoder::Decoder::new(std::io::Cursor::new(&bytes)).unwrap();let samples=match decoder.read_image().unwrap(){tiff::decoder::DecodingResult::F16(samples)=>samples.into_iter().map(|sample|u32::from(sample.to_bits())).collect::<Vec<_>>(),tiff::decoder::DecodingResult::F32(samples)=>samples.into_iter().map(f32::to_bits).collect::<Vec<_>>(),_=>panic!("independent IEEE16/32 raster")};assert_eq!(samples,expected.ifds[0].blocks[0].samples.iter().map(|sample|sample.lo).collect::<Vec<_>>());}if case["name"].as_str().unwrap().starts_with("JPEG7")&&case["name"].as_str().unwrap().contains("YCbCr"){let mut independent=tiff::decoder::Decoder::new(std::io::Cursor::new(&bytes)).unwrap();let tiff::decoder::DecodingResult::U8(samples)=independent.read_image().unwrap()else{panic!("independent native YCbCr component words")};assert_eq!(samples,expected.ifds[0].blocks[0].samples.iter().map(|word|word.lo as u8).collect::<Vec<_>>());}}eprintln!("[DEBUG] native TIFF profiles retain exact JPEG6/7, YCbCr, IEEE16/32 predictor and FillOrder words with independent IEEE/YCbCr decoders");}
#[test]fn image_rs_independently_decodes_old_jpeg_neutral_sample_words(){
 let fixture:serde_json::Value=serde_json::from_str(include_str!("../../💾️binary/📸️snapshot/🧫️fixtures/🧮️native-profiles/🔣️.json")).unwrap();let mut witnessed=0;
 for case in fixture["cases"].as_array().unwrap(){if let Some(hex)=case["independentJpegHex"].as_str(){let bytes:Vec<u8>=(0..hex.len()).step_by(2).map(|at|u8::from_str_radix(&hex[at..at+2],16).unwrap()).collect();let independent=image::load_from_memory_with_format(&bytes,image::ImageFormat::Jpeg).unwrap().to_luma8();let snapshot:TiffSnapshot=semio_framework_pack_json::from_json_str(&case["snapshot"].to_string(),semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();let block=&snapshot.ifds[0].blocks[0];assert_eq!(independent.dimensions(),(block.width,block.height));assert_eq!(independent.into_raw(),block.samples.iter().map(|sample|u8::try_from(sample.lo).unwrap()).collect::<Vec<_>>());witnessed+=1;}}
 assert_eq!(witnessed,2);eprintln!("[DEBUG] old JPEG separate-table neutral words independently witnessed by image JPEG decoder; native component identities=0,1");
}

#[test]
fn native_baseline_observations_match_independent_tiff_fields(){
 use crate::standards::v6_0::subsets::baseline::io::native_conformance::classify_tiff_native_baseline;
 use super::controlled_decoding::{inspect_tiff_native,inspect_tiff_native_controlled};
 let input=include_bytes!("../../../🧫️fixtures/🎨️paint-region-applied/⬅️before.tiff");
 let observed=inspect_tiff_native(input).unwrap();
 let mut independent=tiff::decoder::Decoder::new(std::io::Cursor::new(input)).unwrap();
 assert_eq!(observed.ifd_count,1);assert!(observed.raster);
 for (values,tag)in[(&observed.compression,tiff::tags::Tag::Compression),(&observed.photometric,tiff::tags::Tag::PhotometricInterpretation),(&observed.bits_per_sample,tiff::tags::Tag::BitsPerSample),(&observed.tile_width,tiff::tags::Tag::TileWidth),(&observed.tile_length,tiff::tags::Tag::TileLength),(&observed.strip_offsets,tiff::tags::Tag::StripOffsets)]{
  assert_eq!(*values,independent.find_tag(tag).unwrap().map(|value|value.into_u32_vec().unwrap()));
 }
 assert!(independent.read_image().is_ok());assert!(classify_tiff_native_baseline(&observed).is_empty());
 let fixture:serde_json::Value=serde_json::from_str(include_str!("../../../../🧱️baseline/🚪️io/🧪️tests/🧮️native-conformance/🔣️.json")).unwrap();
 let base=&fixture["base"];
 for case in fixture["cases"].as_array().unwrap(){
  let value=|key:&str|case["patch"].get(key).unwrap_or(&base[key]);
  let words=|key:&str|value(key).as_array().map(|v|v.iter().map(|v|v.as_u64().unwrap()as u32).collect());
  let profile=super::native_observations::TiffNativeObservations{ifd_count:1,raster:value("raster").as_bool().unwrap(),compression:words("compression"),photometric:words("photometric"),bits_per_sample:words("bitsPerSample"),tile_width:words("tileWidth"),tile_length:words("tileLength"),strip_offsets:words("stripOffsets")};
  let expected=case["codes"].as_array().unwrap().iter().map(|v|format!("stdio.tiff.baseline.{}",v.as_str().unwrap())).collect::<Vec<_>>();
  assert_eq!(classify_tiff_native_baseline(&profile),expected);
 }
 let mut progress=|_|true;assert!(inspect_tiff_native_controlled(input,&mut semio_framework_value::NativeDecodeControl::new(1,&mut progress),usize::MAX).is_err());
 let mut checkpoints=0;let mut cancel=|_|{checkpoints+=1;checkpoints<3};assert!(inspect_tiff_native_controlled(input,&mut semio_framework_value::NativeDecodeControl::new(1_000_000,&mut cancel),usize::MAX).is_err());assert_eq!(checkpoints,3);
 eprintln!("[DEBUG] paid native TIFF observations match all six independently read fields, actual raster and seven exact neutral class profiles");
}
