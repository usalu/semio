use super::*;

fn pseudo_random(len: usize, seed: u32) -> Vec<u8> {
    let mut state = seed;
    (0..len)
        .map(|index| {
            state = state.wrapping_mul(1664525).wrapping_add(1013904223);
            if index % 7 == 0 { (state >> 24) as u8 } else { b'a' + ((state >> 16) % 4) as u8 }
        })
        .collect()
}

#[test]
fn ascii_filters_round_trip() {
    let data = pseudo_random(1000, 1);
    assert_eq!(ascii_hex_decode(&ascii_hex_encode(&data)), data);
    assert_eq!(ascii85_decode(&ascii85_encode(&data)).unwrap(), data);
    assert_eq!(ascii85_decode(b"87cURD]i,\"Ebo80~>").unwrap(), b"Hello World!");
}

#[test]
fn run_length_round_trips_runs_and_literals() {
    let mut data = vec![7u8; 300];
    data.extend(pseudo_random(400, 2));
    data.extend(vec![0u8; 5]);
    assert_eq!(run_length_decode(&run_length_encode(&data)), data);
}

#[test]
fn lzw_round_trips_both_early_change_settings() {
    let data = pseudo_random(20_000, 3);
    for early_change in [true, false] {
        assert_eq!(lzw_decode(&lzw_encode(&data, early_change), early_change).unwrap(), data);
    }
    // 🔤 The spec's own example (§7.4.4.2 Table 8 / Example): input 45 45 45 45 45 65 45 45 45 66.
    let spec = [45u8, 45, 45, 45, 45, 65, 45, 45, 45, 66];
    let encoded = lzw_encode(&spec, true);
    assert_eq!(encoded, vec![0x80, 0x0B, 0x60, 0x50, 0x22, 0x0C, 0x0C, 0x85, 0x01]);
    assert_eq!(lzw_decode(&encoded, true).unwrap(), spec);
}

#[test]
fn predictors_round_trip() {
    let predictor = PdfPredictor { predictor: 12, colors: 3, bits_per_component: 8, columns: 10 };
    let data = pseudo_random(30 * 4, 4);
    assert_eq!(png_predictor_decode(&predictor_encode(&data, &predictor), 10, 3, 8).unwrap(), data);
    let tiff = PdfPredictor { predictor: 2, colors: 3, bits_per_component: 8, columns: 10 };
    assert_eq!(tiff_predictor2_decode(&predictor_encode(&data, &tiff), 10, 3), data);
}

#[test]
fn stream_pipeline_round_trips_and_retains_image_codecs() {
    let data = pseudo_random(5000, 5);
    let filters = vec![PdfStreamFilter::Ascii85, PdfStreamFilter::Flate { predictor: Some(PdfPredictor { predictor: 15, colors: 1, bits_per_component: 8, columns: 50 }) }];
    let (encoded, entries) = encode_stream(&data, &filters);
    let (decoded, pipeline) = decode_stream(&entries, &encoded).unwrap();
    assert_eq!(decoded, data);
    assert_eq!(pipeline, filters);

    let jpeg = vec![0xFF, 0xD8, 0xFF, 0xE0, 1, 2, 3];
    let dict = vec![PdfDictEntry::new("Filter", PdfObject::name("DCTDecode"))];
    let (retained, pipeline) = decode_stream(&dict, &jpeg).unwrap();
    assert_eq!(retained, jpeg);
    assert_eq!(pipeline, vec![PdfStreamFilter::Dct { color_transform: None }]);
    let (again, entries) = encode_stream(&retained, &pipeline);
    assert_eq!(again, jpeg);
    assert_eq!(entries, dict);
}
