
use super::*;

#[test]
fn every_authored_recipe_id_resolves() {for id in RECIPE_IDS{assert!(recipe(id).is_some(),"recipe {id} must resolve");}}

#[test]
fn encode_decode_round_trips_a_two_ifd_document() {
    let ifds = vec![IfdSpec { width: 4, height: 3, pixels: fill(4, 3, 0), description: Some("hi") }, IfdSpec { width: 2, height: 2, pixels: fill(2, 2, 9), description: None }];
    let bytes = write_doc(&ifds);
    let dir=std::path::PathBuf::from(std::env::var("SEMIO_TEST_ARTIFACT_DIR").expect("registered test artifact directory"));fs::create_dir_all(&dir).unwrap();let path=dir.join("two-ifd.tiff");fs::write(&path,&bytes).unwrap();
    let json = project(path.to_str().unwrap()).expect("project the just-written file");
    assert!(json.contains("\"ifdCount\":2"));
    assert!(!json.contains("byteOrder"));
    assert!(json.contains("\"kind\":\"ascii\",\"value\":\"hi\""));
}

#[test]
fn authored_recipe_catalog_matches_neutral_contract() {
 let fixture:serde_json::Value=serde_json::from_str(include_str!("../../../🧪️tests/🧬️owned-recipes/🔣️.json")).unwrap();let expected=fixture["recipes"].as_array().unwrap().iter().map(|value|value.as_str().unwrap()).collect::<Vec<_>>();assert_eq!(RECIPE_IDS,expected);for id in expected{assert!(recipe(id).is_some(),"authored recipe {id}");}
}

#[test]
fn independent_decoder_witnesses_exact_paint_and_sample_edits(){
 let fixture:serde_json::Value=serde_json::from_str(include_str!("../../../🧪️tests/🧬️owned-recipes/🔣️.json")).unwrap();for(id,offset,expected)in[("paint-region-applied",15usize,&fixture["raster"]["paint"]["rgb"]),("replace-samples-applied",0usize,&fixture["raster"]["replacement"]["samples"]) ]{let(before,after)=recipe(id).unwrap();let read=|spec:&[IfdSpec]|{let mut decoder=Decoder::new(Cursor::new(write_doc(spec))).unwrap();let tiff::decoder::DecodingResult::U8(samples)=decoder.read_image().unwrap()else{panic!("independent RGB8 samples")};samples};let before=read(&before);let after=read(&after);let expected=expected.as_array().unwrap().iter().map(|value|value.as_u64().unwrap()as u8).collect::<Vec<_>>();assert_eq!(&after[offset..offset+3],expected);assert_eq!(&after[..offset],&before[..offset]);assert_eq!(&after[offset+3..],&before[offset+3..]);assert_ne!(before,after);}
 eprintln!("[DEBUG] independent tiff decoder validates exact authored PaintRegion and ReplaceSamples words");
}
