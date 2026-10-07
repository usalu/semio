use super::*;
use crate::schema::operations::*;
use crate::schema::snapshot::{PngRegion,PngNativePaint};
use protocol::{Mutation, MutationDiff};
use std::{io::Write, process::{Command, Stdio}};

const PRECISION_16: &[u8] = include_bytes!("../../../../../../../🧫️fixtures/🧬️canonical-source/precision-16bit-gray.png");
const INDEXED_2: &[u8] = include_bytes!("../../../../../../../🧫️fixtures/🧬️canonical-source/indexed-2bit-duplicate-palette.png");
const MULTI_IDAT: &[u8] = include_bytes!("../../../../../../../🧫️fixtures/🧬️canonical-source/rgba8-multi-idat-private.png");
const ADAM7_RGBA8: &[u8] = include_bytes!("../../../../../../../🧫️fixtures/🧬️canonical-source/rgba8-adam7.png");
const GRAYSCALE_1: &[u8] = include_bytes!("../../../../../../../🧫️fixtures/🧬️canonical-source/grayscale-1bit.png");
const CONTROLLED_PAINT: &str = include_str!("../../../../../../../🧫️fixtures/🧬️native-paint-controlled/🔣️.json");
const STRUCTURE_CORPUS: &str = include_str!("../../🪶️sqlite/📸️snapshot/🧫️fixtures/🚦️audit/🔣️.json");

fn hex(text: &str) -> Vec<u8> {
    text.as_bytes().chunks_exact(2).map(|pair| u8::from_str_radix(std::str::from_utf8(pair).unwrap(), 16).unwrap()).collect()
}

fn fixture_png(input: &serde_json::Value) -> Vec<u8> {
    let mut output = PNG_SIGNATURE.to_vec();
    for (index, chunk) in input["chunks"].as_array().unwrap().iter().enumerate() {
        write_chunk(&mut output, chunk["tag"].as_str().unwrap().as_bytes().try_into().unwrap(), &hex(chunk["dataHex"].as_str().unwrap()));
        if input["corruptCrc"].as_u64() == Some(index as u64) {
            *output.last_mut().unwrap() ^= 1;
        }
    }
    output
}

fn chunk_names(bytes: &[u8]) -> Vec<String> {
    chunk_addresses(bytes).unwrap().iter().map(|chunk| String::from_utf8_lossy(&chunk.kind).into_owned()).collect()
}

fn native_fixture(width: u32, height: u32, bit_depth: u8, color_type: u8, interlace: u8, raw: &[u8], palette: Option<&[u8]>) -> Vec<u8> {
    let mut bytes = PNG_SIGNATURE.to_vec();
    let mut ihdr = Vec::new();
    ihdr.extend_from_slice(&width.to_be_bytes());
    ihdr.extend_from_slice(&height.to_be_bytes());
    ihdr.extend_from_slice(&[bit_depth, color_type, 0, 0, interlace]);
    write_chunk(&mut bytes, b"IHDR", &ihdr);
    if let Some(palette) = palette { write_chunk(&mut bytes, b"PLTE", palette); }
    write_chunk(&mut bytes, b"IDAT", &semio_s_artifact_stdio_deflate::standards::v_rfc1950::subsets::any::io::zlib_compress(raw).unwrap());
    write_chunk(&mut bytes, b"IEND", &[]);
    bytes
}

fn rgba16_adam7_fixture(width: u32, height: u32) -> Vec<u8> {
    let mut raw = Vec::new();
    for (pass, &(start_x, start_y, step_x, step_y)) in ADAM7.iter().enumerate() {
        let (pass_width, pass_height) = adam7_pass_dims(width, height, pass);
        for row in 0..pass_height {
            raw.push(0);
            for column in 0..pass_width {
                let x = start_x + column * step_x;
                let y = start_y + row * step_y;
                for sample in [x as u16 * 257, y as u16 * 257, 0x1234, 0xffff] { raw.extend_from_slice(&sample.to_be_bytes()); }
            }
        }
    }
    native_fixture(width, height, 16, 6, 1, &raw, None)
}

fn uniform_native_fixture(width: u32, height: u32, bit_depth: u8, color_type: u8, interlace: bool, samples: &[u16]) -> Vec<u8> {
    let mut raw = Vec::new();
    let passes = if interlace { ADAM7.to_vec() } else { vec![(0, 0, 1, 1)] };
    for (pass, &(start_x, start_y, step_x, step_y)) in passes.iter().enumerate() {
        let (pass_width, pass_height) = if interlace { adam7_pass_dims(width, height, pass) } else { (width, height) };
        if pass_width == 0 || pass_height == 0 { continue; }
        for _ in 0..pass_height {
            raw.push(0);
            let mut row = vec![0; packed_row_bytes(pass_width, color_type, bit_depth)];
            for pixel in 0..pass_width as usize {
                for (channel, sample) in samples.iter().enumerate() { write_native_sample(&mut row, pixel * samples.len() + channel, bit_depth, *sample); }
            }
            raw.extend_from_slice(&row);
        }
        let _ = (start_x, start_y, step_x, step_y);
    }
    let palette = (color_type == 3).then(|| (0..(1usize << bit_depth)).flat_map(|index| [index as u8, index as u8, index as u8]).collect::<Vec<_>>());
    native_fixture(width, height, bit_depth, color_type, u8::from(interlace), &raw, palette.as_deref())
}

fn independent_native_samples(bytes: &[u8]) -> (png::ColorType, png::BitDepth, Vec<u8>) {
    let mut decoder = png::Decoder::new(std::io::Cursor::new(bytes));
    decoder.set_transformations(png::Transformations::IDENTITY);
    let mut reader = decoder.read_info().unwrap();
    let mut samples = vec![0; reader.output_buffer_size().unwrap()];
    let frame = reader.next_frame(&mut samples).unwrap();
    samples.truncate(frame.buffer_size());
    (frame.color_type, frame.bit_depth, samples)
}

fn non_idat_chunks(bytes: &[u8]) -> Vec<([u8; 4], Vec<u8>)> {
    read_chunks(bytes,&mut semio_framework_value::NativeDecodeControl::new(1024*1024,&mut |_|true)).unwrap_or_else(|_|panic!("raw fixture chunk admission")).into_iter().filter(|(kind, _)| *kind != *b"IDAT").map(|(kind, data)| (kind, data.to_vec())).collect()
}

fn fixture_cases() -> [(&'static str, &'static [u8], (u32, u32, u8, PngColorType, bool)); 5] {
    [
        ("precision-16bit-gray", PRECISION_16, (2, 1, 16, PngColorType::Grayscale, false)),
        ("indexed-2bit-duplicate-palette", INDEXED_2, (4, 1, 2, PngColorType::Palette, false)),
        ("rgba8-multi-idat-private", MULTI_IDAT, (2, 1, 8, PngColorType::Rgba, false)),
        ("rgba8-adam7", ADAM7_RGBA8, (3, 3, 8, PngColorType::Rgba, true)),
        ("grayscale-1bit", GRAYSCALE_1, (8, 1, 1, PngColorType::Grayscale, false)),
    ]
}

#[test]
fn canonical_native_output_preserves_owned_samples_and_metadata() {
 for (id,bytes,expected) in fixture_cases() {
  let snapshot=decode_png(bytes).unwrap_or_else(|e|panic!("{id}: {e}"));
  let published=encode_png(&snapshot).unwrap();assert_eq!(decode_png(&published).unwrap(),snapshot,"{id}: owned native roundtrip");
  let layout=png_layout(&snapshot).unwrap();assert_eq!((layout.width,layout.height,layout.bit_depth,layout.color_type,layout.interlace),expected);
  assert_eq!(independent_native_samples(&published).2,independent_native_samples(bytes).2,"{id}: precise native samples");
 }
}

#[test]
fn exact_projection_handles_packed_precision_palette_and_adam7() {
    let indexed = project_png(INDEXED_2).unwrap();
    assert_eq!(indexed.plte.as_ref().unwrap()[0], indexed.plte.as_ref().unwrap()[1], "duplicate palette identities remain distinct source indices");
    assert_eq!(indexed.pixels, vec![255, 0, 0, 255, 255, 0, 0, 64, 0, 255, 0, 128, 0, 0, 255, 0]);
    assert_eq!(project_png(GRAYSCALE_1).unwrap().pixels, vec![255, 255, 255, 255, 0, 0, 0, 255, 255, 255, 255, 255, 0, 0, 0, 255, 255, 255, 255, 255, 0, 0, 0, 255, 255, 255, 255, 255, 0, 0, 0, 255]);
    let precision = project_png(PRECISION_16).unwrap();
    assert_eq!(precision.bit_depth, 16);
    assert_eq!(precision.pixels.len(), 8);
    let adam7 = project_png(ADAM7_RGBA8).unwrap();
    assert_eq!(adam7.pixels.len(), 3 * 3 * 4);
    assert_eq!(&adam7.pixels[..4], &[0, 0, 0, 255]);
    assert_eq!(&adam7.pixels[32..36], &[140, 140, 140, 255]);
}

#[test]
fn multi_idat_private_chunk_and_source_order_remain_addressable() {
    let addresses = chunk_addresses(MULTI_IDAT).unwrap();
    assert_eq!(addresses.iter().filter(|chunk| chunk.kind == *b"IDAT").count(), 2);
    assert!(addresses.iter().any(|chunk| chunk.kind == *b"prIV"));
    assert_eq!(addresses.first().unwrap().kind, *b"IHDR");
    assert_eq!(addresses.last().unwrap().kind, *b"IEND");
}

#[test]
fn gamma_edit_preserves_owned_samples_and_other_metadata() {
 let snapshot=decode_png(MULTI_IDAT).unwrap();let edited=set_gamma_chunk_controlled(&snapshot,&png_revision(&snapshot),Some(45455),&mut |_,_|true).unwrap();
 assert_eq!(edited.image.gamma,Some(45455));let mut expected=snapshot.clone();expected.image.gamma=Some(45455);assert_eq!(edited,expected);
 assert_eq!(decode_png(&encode_png(&edited).unwrap()).unwrap(),edited);
}

#[test]
fn indexed_gamma_edit_obeys_neutral_order_inverse_zero_and_pngjs_laws() {
    let corpus: serde_json::Value = serde_json::from_str(STRUCTURE_CORPUS).unwrap();
    let law = &corpus["gammaEdit"];
    let snapshot = decode_png(INDEXED_2).unwrap();
    let mutation = crate::PngMutation::ChangeGamma(crate::schema::mutations::ChangeGammaMutation {
        revision: png_revision(&snapshot),
        gama: Some(law["value"].as_u64().unwrap() as u32),
    });
    let outcome = mutation.diff(&snapshot);
    assert!(outcome.messages().is_empty(), "indexed gamma edit was refused: {:?}", outcome.messages());
    let mut edited = outcome.diff().apply(&snapshot).unwrap();
    assert_eq!(edited.image.gamma,Some(law["value"].as_u64().unwrap() as u32));
    let mut expected=snapshot.clone();expected.image.gamma=edited.image.gamma;assert_eq!(edited,expected);
    let mut oracle = Command::new("bun").args(["-e", "import {PNG} from 'pngjs';const bytes=Buffer.from(await Bun.stdin.arrayBuffer()),image=PNG.sync.read(bytes);if(Math.abs(image.gamma-0.45455)>1e-9)throw Error(`gamma ${image.gamma}`);console.log(JSON.stringify([...image.data]));"])
        .stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::piped()).spawn().unwrap();
    oracle.stdin.take().unwrap().write_all(&encode_png(&edited).unwrap()).unwrap();
    let output = oracle.wait_with_output().unwrap();
    assert!(output.status.success(), "pngjs refused edited indexed PNG: {}", String::from_utf8_lossy(&output.stderr));
    assert_eq!(serde_json::from_slice::<serde_json::Value>(&output.stdout).unwrap(), serde_json::json!(project_png(INDEXED_2).unwrap().pixels));
    for inverse in mutation.inverse(&snapshot).unwrap() {
        edited = inverse.diff(&edited).diff().apply(&edited).unwrap();
    }
    assert_eq!(edited, snapshot, "gamma inverse must restore the exact indexed source bytes");
    for value in law["rejectedValues"].as_array().unwrap() {
        let invalid = crate::PngMutation::ChangeGamma(crate::schema::mutations::ChangeGammaMutation { revision: png_revision(&snapshot), gama: Some(value.as_u64().unwrap() as u32) });
        assert!(!invalid.diff(&snapshot).messages().is_empty(), "gAMA value {value} must be refused");
    }
}

#[test]
fn checked_source_refuses_neutral_structural_and_profile_violations() {
    let corpus: serde_json::Value = serde_json::from_str(STRUCTURE_CORPUS).unwrap();
    for case in corpus["invalidPng"].as_array().unwrap() {
        assert!(decode_png(&fixture_png(case)).is_err(), "checked source admitted neutral case {}", case["id"]);
    }
    for chunk in corpus["singletonChunks"].as_array().unwrap() {
        let case = serde_json::json!({
            "chunks": [
                { "tag": "IHDR", "dataHex": chunk["ihdrHex"] },
                { "tag": chunk["tag"], "dataHex": chunk["dataHex"] },
                { "tag": chunk["tag"], "dataHex": chunk["dataHex"] },
                { "tag": "IDAT", "dataHex": "789c63606462fe0f0001140106" },
                { "tag": "IEND", "dataHex": "" }
            ]
        });
        assert!(decode_png(&fixture_png(&case)).is_err(), "checked source admitted duplicate modeled chunk {}", chunk["tag"]);
    }
}

#[test]
fn region_paint_is_exactly_profile_scoped_and_revision_guarded() {
    let snapshot = decode_png(MULTI_IDAT).unwrap();
    let revision = png_revision(&snapshot);
    let retained = non_idat_chunks(&encode_png(&snapshot).unwrap());
    let idat_count=1;
    let edited = paint_rgba8_region_controlled(&snapshot, &revision, PngRegion { x: 1, y: 0, width: 1, height: 1 }, [9, 8, 7, 6], &mut |_, _| true).unwrap();
    assert_eq!(&project_png(&encode_png(&edited).unwrap()).unwrap().pixels[4..8], &[9, 8, 7, 6]);
    assert_eq!(non_idat_chunks(&encode_png(&edited).unwrap()), retained);
    assert_eq!(chunk_addresses(&encode_png(&edited).unwrap()).unwrap().iter().filter(|chunk| chunk.kind == *b"IDAT").count(), idat_count);
    assert!(paint_rgba8_region_controlled(&snapshot, "stale", PngRegion { x: 0, y: 0, width: 1, height: 1 }, [0; 4], &mut |_, _| true).unwrap_err().contains("revision"));
    let precision = decode_png(PRECISION_16).unwrap();
    assert!(paint_rgba8_region_controlled(&precision, &png_revision(&precision), PngRegion { x: 0, y: 0, width: 1, height: 1 }, [0; 4], &mut |_, _| true).unwrap_err().contains("8-bit RGBA"));
    assert_eq!(decode_png(PRECISION_16).unwrap(),precision);
}

#[test]
fn native_paint_preserves_index_identity_packing_and_non_idat_bytes() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../✏️editor/🎭️modes/✏️edit/🎮️commands/🎨️paint-native-region/🧫️fixtures/🔣️.json")).unwrap();
    assert_eq!(fixture["cases"][0]["id"], "indexed-duplicate-identity");
    let snapshot = decode_png(INDEXED_2).unwrap();
    let before_non_idat = non_idat_chunks(&encode_png(&snapshot).unwrap());
    let before_count=1;
    let edited = paint_native_region_controlled(&snapshot, &png_revision(&snapshot), PngRegion { x: 0, y: 0, width: 1, height: 1 }, PngNativePaint::indexed(1), &mut |_, _| true).unwrap();
    assert_eq!(png_native_pixel(&edited, 0, 0).unwrap(), vec![1]);
    assert_eq!(non_idat_chunks(&encode_png(&edited).unwrap()), before_non_idat);
    assert_eq!(chunk_addresses(&encode_png(&edited).unwrap()).unwrap().iter().filter(|chunk| chunk.kind == *b"IDAT").count(), before_count);
    let (color, depth, samples) = independent_native_samples(&encode_png(&edited).unwrap());
    assert_eq!((color, depth), (png::ColorType::Indexed, png::BitDepth::Two));
    assert_eq!(samples, vec![0b01_01_10_11]);
}

#[test]
fn native_paint_preserves_packed_tail_bits_and_exact_16_bit_samples() {
    let packed = native_fixture(5, 1, 1, 0, 0, &[0, 0b10101_101], None);
    let packed = decode_png(&packed).unwrap();
    let packed_edit = paint_native_region_controlled(&packed, &png_revision(&packed), PngRegion { x: 1, y: 0, width: 2, height: 1 }, PngNativePaint::grayscale(1), &mut |_, _| true).unwrap();
    assert_eq!(png_native_pixel(&packed_edit, 1, 0).unwrap(), vec![1]);
    assert_eq!(independent_native_samples(&encode_png(&packed_edit).unwrap()).2, vec![0b11101_000], "unused packed tail bits are canonical zeros");

    let rgb16 = native_fixture(2, 1, 16, 2, 0, &[0, 0, 1, 0, 2, 0, 3, 0, 4, 0, 5, 0, 6], None);
    let rgb16 = decode_png(&rgb16).unwrap();
    let rgb16_edit = paint_native_region_controlled(&rgb16, &png_revision(&rgb16), PngRegion { x: 1, y: 0, width: 1, height: 1 }, PngNativePaint::rgb(0x1234, 0xabcd, 0x00ff), &mut |_, _| true).unwrap();
    assert_eq!(png_native_pixel(&rgb16_edit, 1, 0).unwrap(), vec![0x1234, 0xabcd, 0x00ff]);
    assert_eq!(&independent_native_samples(&crate::standards::v1_2::subsets::any::io::encode_png(&rgb16_edit).unwrap()).2[6..12], &[0x12, 0x34, 0xab, 0xcd, 0x00, 0xff]);

    let gray_alpha16 = native_fixture(1, 1, 16, 4, 0, &[0, 0, 1, 0, 2], None);
    let gray_alpha16 = decode_png(&gray_alpha16).unwrap();
    let gray_alpha16_edit = paint_native_region_controlled(&gray_alpha16, &png_revision(&gray_alpha16), PngRegion { x: 0, y: 0, width: 1, height: 1 }, PngNativePaint::grayscale_alpha(0x0102, 0xfedc), &mut |_, _| true).unwrap();
    assert_eq!(png_native_pixel(&gray_alpha16_edit, 0, 0).unwrap(), vec![0x0102, 0xfedc]);
    assert_eq!(independent_native_samples(&crate::standards::v1_2::subsets::any::io::encode_png(&gray_alpha16_edit).unwrap()).2, vec![0x01, 0x02, 0xfe, 0xdc]);
}

#[test]
fn native_paint_maps_adam7_coordinates_without_precision_loss() {
    let source = rgba16_adam7_fixture(5, 5);
    let snapshot = decode_png(&source).unwrap();
    let edited = paint_native_region_controlled(&snapshot, &png_revision(&snapshot), PngRegion { x: 3, y: 4, width: 1, height: 1 }, PngNativePaint::rgba(0x0102, 0x3456, 0x789a, 0xbcde), &mut |_, _| true).unwrap();
    assert_eq!(png_layout(&edited).unwrap().interlace, true);
    assert_eq!(png_native_pixel(&edited, 3, 4).unwrap(), vec![0x0102, 0x3456, 0x789a, 0xbcde]);
    let (color, depth, samples) = independent_native_samples(&encode_png(&edited).unwrap());
    assert_eq!((color, depth), (png::ColorType::Rgba, png::BitDepth::Sixteen));
    let offset = (4 * 5 + 3) * 8;
    assert_eq!(&samples[offset..offset + 8], &[0x01, 0x02, 0x34, 0x56, 0x78, 0x9a, 0xbc, 0xde]);
}

#[test]
fn native_paint_supports_every_png_sample_profile_with_and_without_adam7() {
    let profiles: &[(u8, &[u8])] = &[(0, &[1, 2, 4, 8, 16]), (2, &[8, 16]), (3, &[1, 2, 4, 8]), (4, &[8, 16]), (6, &[8, 16])];
    for &(color_type, depths) in profiles {
        for &bit_depth in depths {
            let maximum = if bit_depth == 16 { u16::MAX } else { ((1u32 << bit_depth) - 1) as u16 };
            let source_samples = match color_type { 0 | 3 => vec![0], 2 => vec![0, 1, 2], 4 => vec![0, maximum], 6 => vec![0, 1, 2, maximum], _ => unreachable!() };
            let paint = match color_type {
                0 => PngNativePaint::grayscale(maximum),
                2 => PngNativePaint::rgb(maximum, maximum.saturating_sub(1), maximum.saturating_sub(2)),
                3 => PngNativePaint::indexed(maximum),
                4 => PngNativePaint::grayscale_alpha(maximum, maximum.saturating_sub(1)),
                6 => PngNativePaint::rgba(maximum, maximum.saturating_sub(1), maximum.saturating_sub(2), maximum.saturating_sub(3)),
                _ => unreachable!(),
            };
            let expected = paint.samples()[..samples_per_pixel(color_type)].to_vec();
            for interlace in [false, true] {
                let source = uniform_native_fixture(5, 5, bit_depth, color_type, interlace, &source_samples);
                let snapshot = decode_png(&source).unwrap_or_else(|error| panic!("profile {color_type}/{bit_depth}/{interlace}: {error}"));
                let retained=non_idat_chunks(&encode_png(&snapshot).unwrap());
                let edited = paint_native_region_controlled(&snapshot, &png_revision(&snapshot), PngRegion { x: 3, y: 4, width: 1, height: 1 }, paint, &mut |_, _| true).unwrap();
                assert_eq!(png_native_pixel(&edited, 3, 4).unwrap(), expected, "profile {color_type}/{bit_depth}/{interlace}");
                assert_eq!(non_idat_chunks(&encode_png(&edited).unwrap()), retained, "profile {color_type}/{bit_depth}/{interlace}");
                let (oracle_color, oracle_depth, _) = independent_native_samples(&encode_png(&edited).unwrap());
                assert_eq!(oracle_color as u8, color_type, "png crate color profile");
                assert_eq!(oracle_depth as u8, bit_depth, "png crate sample precision");
            }
        }
    }
}

#[test]
fn native_paint_refuses_profile_range_revision_bounds_and_cancellation() {
    let snapshot = decode_png(PRECISION_16).unwrap();
    assert!(paint_native_region_controlled(&snapshot, "stale", PngRegion { x: 0, y: 0, width: 1, height: 1 }, PngNativePaint::grayscale(1), &mut |_, _| true).unwrap_err().contains("revision"));
    assert!(paint_native_region_controlled(&snapshot, &png_revision(&snapshot), PngRegion { x: 2, y: 0, width: 1, height: 1 }, PngNativePaint::grayscale(1), &mut |_, _| true).unwrap_err().contains("exceeds"));
    assert!(paint_native_region_controlled(&snapshot, &png_revision(&snapshot), PngRegion { x: 0, y: 0, width: 1, height: 1 }, PngNativePaint::rgb(1, 2, 3), &mut |_, _| true).unwrap_err().contains("profile"));
    let indexed = decode_png(INDEXED_2).unwrap();
    assert!(paint_native_region_controlled(&indexed, &png_revision(&indexed), PngRegion { x: 0, y: 0, width: 1, height: 1 }, PngNativePaint::indexed(4), &mut |_, _| true).unwrap_err().contains("palette index"));
    let mut calls = 0;
    assert!(paint_native_region_controlled(&snapshot, &png_revision(&snapshot), PngRegion { x: 0, y: 0, width: 1, height: 1 }, PngNativePaint::grayscale(0x1234), &mut |_, _| { calls += 1; calls < 2 }).unwrap_err().contains("cancelled"));
    assert_eq!(decode_png(PRECISION_16).unwrap(),snapshot);
}


#[test]
fn controlled_owned_paint_yields_and_cancels_during_copy_and_paint() {
 let snapshot=decode_png(MULTI_IDAT).unwrap();let region=PngRegion{x:0,y:0,width:2,height:1};let paint=PngNativePaint::rgba(9,8,7,6);
 for phase in [PngNativePaintPhase::Copy,PngNativePaintPhase::Paint] {
  let cancel=semio_framework_job::root_cancel_token();let mut operation=PngNativePaintWorkOperation::try_new(&snapshot,&png_revision(&snapshot),region,paint,MAXIMUM_NATIVE_PAINT_OWNED_BYTES).unwrap();let mut witnessed=false;
  for _ in 0..100000 {let mut sequence=0;let mut context=semio_framework_job::StepContext::new(semio_framework_job::allocate_operation_id(),semio_framework_job::Generation(1),semio_framework_job::StepBudget::new(1,u64::MAX),cancel.clone(),||Some(0),&mut sequence);
   match operation.advance(&mut context).unwrap() {PngNativePaintWorkStep::Yield(progress) if progress.phase==phase && progress.completed>0=>{witnessed=true;cancel.cancel_now();},PngNativePaintWorkStep::Cancelled=>break,PngNativePaintWorkStep::Yield(_)=>{},PngNativePaintWorkStep::Complete=>panic!("phase cancellation witness missing")}
  }
  assert!(witnessed);operation.begin_close();assert!(matches!(operation.close_step(1,1),semio_framework_job::InteractiveJobCloseStep::Pending{released_items,released_bytes}if released_items<=1&&released_bytes<=1));
  for _ in 0..100000 {if operation.terminal_is_empty(){break;}operation.close_step(4096,1024*1024);}assert!(operation.terminal_is_empty());
 }
 let mut operation=PngNativePaintWorkOperation::try_new(&snapshot,&png_revision(&snapshot),region,paint,MAXIMUM_NATIVE_PAINT_OWNED_BYTES).unwrap();let mut copies=0;let mut paints=0;
 for _ in 0..100000 {let mut sequence=0;let mut context=semio_framework_job::StepContext::new(semio_framework_job::allocate_operation_id(),semio_framework_job::Generation(1),semio_framework_job::StepBudget::new(1,u64::MAX),semio_framework_job::root_cancel_token(),||Some(0),&mut sequence);match operation.advance(&mut context).unwrap(){PngNativePaintWorkStep::Yield(p)=>match p.phase{PngNativePaintPhase::Copy=>copies+=1,PngNativePaintPhase::Paint=>paints+=1},PngNativePaintWorkStep::Complete=>break,PngNativePaintWorkStep::Cancelled=>panic!("uncancelled operation")}}
 let edited=operation.take_result().unwrap();assert!(copies>1&&paints>=1);assert_eq!(edited.image.samples,vec![9,8,7,6,9,8,7,6]);assert!(snapshot.image.same_metadata(&edited.image));assert_eq!(independent_native_samples(&encode_png(&edited).unwrap()).2,vec![9,8,7,6,9,8,7,6]);operation.begin_close();for _ in 0..100000{if operation.terminal_is_empty(){break;}operation.close_step(4096,1024*1024);}assert!(operation.terminal_is_empty());assert_eq!(snapshot,decode_png(MULTI_IDAT).unwrap());
 println!("[DEBUG] PNG owned copy and painting yielded, cancelled and retired within explicit close bounds");
}

#[test]
fn controlled_native_paint_preserves_all_owned_metadata() {
 let snapshot=decode_png(MULTI_IDAT).unwrap();let edited=paint_native_region_owned_controlled(&snapshot,&png_revision(&snapshot),PngRegion{x:1,y:0,width:1,height:1},PngNativePaint::rgba(9,8,7,6),MAXIMUM_NATIVE_PAINT_OWNED_BYTES,&mut |_|true).unwrap();
 assert!(snapshot.image.same_metadata(&edited.image));assert_eq!(png_native_pixel(&edited,1,0).unwrap(),vec![9,8,7,6]);assert_eq!(decode_png(&encode_png(&edited).unwrap()).unwrap(),edited);
}

#[test]
fn controlled_native_paint_refuses_cumulative_ownership_and_forged_completed_results() {
    use protocol::Mutation;
    let base = decode_png(MULTI_IDAT).unwrap();
    let revision = png_revision(&base);
    let region = PngRegion { x: 1, y: 0, width: 1, height: 1 };
    let paint = PngNativePaint::rgba(9, 8, 7, 6);
    let refusal = PngNativePaintWorkOperation::try_new(&base,&revision,region,paint,1).err().unwrap();
    assert!(refusal.contains("exceeds caller ownership limit"));
    let valid = paint_native_region_owned_controlled(
        &base,
        &revision,
        region,
        paint,
        MAXIMUM_NATIVE_PAINT_OWNED_BYTES,
        &mut |_| true,
    ).unwrap();
    let mut forged=valid;forged.image.ancillary_chunks[0].data.push(1);
    let mutation = crate::PngMutation::PaintNativeSamples(crate::schema::mutations::PaintNativeSamplesMutation { revision, region, paint, result: forged });
    let outcome = mutation.diff(&base);
    assert!(!outcome.messages().is_empty(), "a completed result that changes bytes outside the admitted IDAT domain must be refused without replaying codecs");
    assert_eq!(base,decode_png(MULTI_IDAT).unwrap());
}

#[test]
fn native_paint_mutation_roundtrips_and_inverse_restores_exact_source() {
    use protocol::{Mutation, MutationDiff, OpBinary, OpText};
    let base = decode_png(PRECISION_16).unwrap();
    let region = PngRegion { x: 1, y: 0, width: 1, height: 1 };
    let paint = PngNativePaint::grayscale(0x1234);
    let result = paint_native_region_owned_controlled(&base, &png_revision(&base), region, paint, MAXIMUM_NATIVE_PAINT_OWNED_BYTES, &mut |_| true).unwrap();
    let mutation = crate::PngMutation::PaintNativeSamples(crate::schema::mutations::PaintNativeSamplesMutation {
        revision: png_revision(&base),
        region,
        paint,
        result,
    });
    assert_eq!(crate::PngMutation::decode_op(&mutation.encode_op().unwrap()).unwrap(), mutation);
    assert_eq!(crate::PngMutation::parse_op(&mutation.print_op()).unwrap(), mutation);
    let edited = mutation.diff(&base).diff().apply(&base).unwrap();
    assert_eq!(png_native_pixel(&edited, 1, 0).unwrap(), vec![0x1234]);
    let inverse = mutation.inverse(&base).unwrap();
    assert_eq!(inverse[0].diff(&edited).diff().apply(&edited).unwrap(), base);
}

#[test]
fn preview_publishes_precise_owned_model_and_keeps_source_unchanged() {
 for (id,source,_) in fixture_cases(){let snapshot=decode_png(source).unwrap();let before=snapshot.clone();let preview=png_preview(&snapshot).unwrap();assert_eq!(decode_png(&preview.bytes).unwrap(),snapshot);assert_eq!(snapshot,before);assert_eq!((preview.width,preview.height),(snapshot.image.width,snapshot.image.height));assert_eq!(independent_native_samples(&preview.bytes).2,independent_native_samples(source).2);println!("[DEBUG] PNG preview {id} retained precise owned model");}
}

#[test]
fn corrupt_crc_and_non_png_sources_are_refused() {
    let mut bytes = MULTI_IDAT.to_vec();
    bytes[20] ^= 0x80;
    assert!(decode_png(&bytes).unwrap_err().contains("CRC"));
    assert!(decode_png(b"not a png").unwrap_err().contains("signature"));
}
