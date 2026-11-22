//! 🧪️ Codec vectors from the neutral fixture: CCITT and JPEG 2000 against Pillow (libtiff, OpenJPEG), JBIG2 against MuPDF, PostScript functions analytically.

use serde_json::Value;

use crate::{ccitt, jbig2, jpx, postscript, RasterError};

fn fixture() -> Value {
    let path = concat!(env!("CARGO_MANIFEST_DIR"), "/../../🧪️tests/🔬️codecs/🧪️codecs.json");
    serde_json::from_str(&std::fs::read_to_string(path).unwrap()).unwrap()
}

fn unhex(text: &str) -> Vec<u8> {
    (0..text.len() / 2).map(|index| u8::from_str_radix(&text[2 * index..2 * index + 2], 16).unwrap()).collect()
}

#[test]
fn ccitt_matches_libtiff_vectors() {
    for case in fixture()["ccitt"].as_array().unwrap() {
        let parameters = ccitt::CcittParameters { k: case["k"].as_i64().unwrap() as i32, columns: case["columns"].as_u64().unwrap() as u32, rows: case["rows"].as_u64().unwrap() as u32, black_is_1: false, byte_align: false };
        let decoded = ccitt::decode(&unhex(case["dataHex"].as_str().unwrap()), &parameters, &mut |_| true).unwrap();
        assert_eq!(decoded, unhex(case["expectedHex"].as_str().unwrap()), "{}", case["name"]);
    }
}

#[test]
fn jpx_matches_openjpeg_vectors() {
    for case in fixture()["jpx"].as_array().unwrap() {
        let image = jpx::decode(&unhex(case["dataHex"].as_str().unwrap()), &mut |_| true).unwrap();
        let expected = unhex(case["expectedHex"].as_str().unwrap());
        assert_eq!((image.width as u64, image.height as u64, image.components as u64), (case["width"].as_u64().unwrap(), case["height"].as_u64().unwrap(), case["components"].as_u64().unwrap()));
        let tolerance = case["tolerance"].as_i64().unwrap() as i32;
        assert!(image.samples.iter().zip(&expected).all(|(a, b)| (i32::from(*a) - i32::from(*b)).abs() <= tolerance), "{}", case["name"]);
    }
}

#[test]
fn jbig2_matches_mupdf_rasters() {
    for case in fixture()["jbig2"].as_array().unwrap() {
        let page = jbig2::decode(None, &unhex(case["dataHex"].as_str().unwrap()), &mut |_| true).unwrap();
        let expected: Vec<u8> = case["expectedBits"].as_str().unwrap().bytes().map(|bit| bit - b'0').collect();
        assert_eq!(page.pixels, expected, "{}", case["name"]);
    }
}

#[test]
fn postscript_functions_evaluate_analytically() {
    for case in fixture()["postscript"].as_array().unwrap() {
        let inputs: Vec<f64> = case["inputs"].as_array().unwrap().iter().map(|value| value.as_f64().unwrap()).collect();
        let output = postscript::PostScriptProgram::parse(case["code"].as_str().unwrap()).unwrap().run(&inputs, case["outputs"].as_u64().unwrap() as usize).unwrap();
        for (actual, expected) in output.iter().zip(case["expected"].as_array().unwrap()) {
            assert!((actual - expected.as_f64().unwrap()).abs() < 1e-9, "{}", case["code"]);
        }
    }
}

#[test]
fn every_codec_stops_when_cancelled() {
    let fixture = fixture();
    let jpx_data = unhex(fixture["jpx"][0]["dataHex"].as_str().unwrap());
    assert_eq!(jpx::decode(&jpx_data, &mut |_| false), Err(RasterError::Cancelled));
    let jbig2_data = unhex(fixture["jbig2"][0]["dataHex"].as_str().unwrap());
    assert_eq!(jbig2::decode(None, &jbig2_data, &mut |_| false), Err(RasterError::Cancelled));
    let case = &fixture["ccitt"][0];
    let parameters = ccitt::CcittParameters { k: -1, columns: 37, rows: 11, black_is_1: false, byte_align: false };
    assert_eq!(ccitt::decode(&unhex(case["dataHex"].as_str().unwrap()), &parameters, &mut |_| false), Err(RasterError::Cancelled));
}
