use super::*;

const SCAN: &[u8] = include_bytes!("../../../🧫️fixtures/🧪️abbau-aufbau-masterarbeit-grundriss/🖼️.tiff");

/// 🖼️ The real scan read by the `tiff` crate sits inside the Baseline class, strip-organized.
#[test]
fn the_real_scan_reads_inside_the_baseline_class() {
    let axes = read_axes(SCAN).expect("the tiff reader decodes the committed scan");
    assert!(axes.raster, "the scan's raster decodes");
    assert!(axes.strip_offsets.is_some() && axes.tile_width.is_none(), "the scan is strip-organized");
    assert_eq!(verdict(&axes), Vec::<&str>::new(), "the committed scan is a Baseline TIFF");
}

/// 🛡️ Every kind moves its own axis and raises exactly the code the specification table names.
#[test]
fn native_baseline_every_profile_moves_its_axis_and_the_verdict_the_table_names() {
    let axes = read_axes(SCAN).expect("reads");
    let json = |text: &str| semio_repo_test_host::parse_json(text).expect("params");
    for (kind, params, code) in [
        ("set-compression", r#"{"compression": 5}"#, Some("stdio.tiff.baseline.unsupported-compression")),
        ("set-photometric-interpretation", r#"{"photometric": 6}"#, Some("stdio.tiff.baseline.unsupported-photometric")),
        ("set-bits-per-sample", r#"{"bits": [16, 16, 16]}"#, Some("stdio.tiff.baseline.unsupported-bits-per-sample")),
        ("insert-tile-tags", r#"{"tileWidth": 256, "tileLength": 256}"#, Some("stdio.tiff.baseline.tiled-not-baseline")),
        ("remove-strip-offsets", "{}", Some("stdio.tiff.baseline.missing-strip-offsets")),
        ("set-strip-offsets", r#"{"offsets": [8, 65536]}"#, None),
    ] {
        let next = profile(&axes, kind, &json(params)).expect("applies");
        assert_ne!(project(&next), project(&axes), "{kind} must move the projection");
        assert_eq!(verdict(&next).first().copied(), code, "{kind}");
    }
    assert!(profile(&axes, "set-byte-order", &json("{}")).is_err(), "a kind outside the vocabulary is refused");
}

/// 🧮️ Neutral profiles have the independently specified exact ordered code lists.
#[test]
fn native_baseline_neutral_profiles_match_reference_tables(){
 let fixture=semio_repo_test_host::parse_json(include_str!("../../../🚪️io/🧪️tests/🧮️native-conformance/🔣️.json")).unwrap();
 let base=fixture.get("base").unwrap();
 let values=|o:&Json,key:&str|match o.get(key){Some(Json::Array(_))=>Some(numbers(o,key)),_=>None};
 for case in fixture.array("cases"){
  let patch=case.get("patch").unwrap();let value=|key|patch.get(key).or_else(||base.get(key));
  let observations=Json::Object(["compression","photometric","bitsPerSample","tileWidth","tileLength","stripOffsets"].into_iter().map(|key|(key.into(),value(key).unwrap().clone())).collect());
  let axes=Axes{ifd_count:1,raster:matches!(value("raster"),Some(Json::Bool(true))),compression:values(&observations,"compression"),photometric:values(&observations,"photometric"),bits_per_sample:values(&observations,"bitsPerSample"),tile_width:values(&observations,"tileWidth"),tile_length:values(&observations,"tileLength"),strip_offsets:values(&observations,"stripOffsets")};
  let expected=case.array("codes").iter().map(|code|match code{Json::String(code)=>format!("stdio.tiff.baseline.{code}"),_=>panic!("code")}).collect::<Vec<_>>();
  assert_eq!(verdict(&axes),expected,"{}",case.str("id"));
 }
 let before=read_axes(include_bytes!("../../../../🧾️document/🧫️fixtures/🎨️paint-region-applied/⬅️before.tiff")).unwrap();
 eprintln!("[DEBUG] independent native TIFF observations {}",project(&before).to_string());
}
