#[allow(unused_imports)]
use crate::standards::v1_7::subsets::base::modules::colour::{lift_colour_space_inline, lower_colour_space_inline};
#[allow(unused_imports)]
use crate::standards::v1_7::subsets::base::modules::filters::{decode_stream, encode_stream};
#[allow(unused_imports)]
use crate::standards::v1_7::subsets::base::modules::fonts::FontCodec;
#[allow(unused_imports)]
use crate::standards::v1_7::subsets::base::modules::lexer::{is_ws, number_text, write_object, Lexer, Token};
#[allow(unused_imports)]
use crate::standards::v1_7::subsets::base::io::error::{PResult, PdfEngineError};
#[allow(unused_imports)]
use crate::standards::v1_7::subsets::base::schema::snapshot::{PdfDictEntry, PdfInlineImage, PdfLineCap, PdfLineJoin, PdfObject, PdfOp, PdfPropertyList, PdfTextArrayItem, PdfTextString};
#[allow(unused_imports)]
use std::collections::HashMap;
use crate::standards::v1_7::subsets::base::modules::content::*;


#[test]
fn inline_image_framing_matches_pdf_sample_fixture() {
    use crate::standards::v1_7::subsets::base::io::foreign_artifacts::NativePdfArtifactResources;

    let fixture: serde_json::Value = serde_json::from_str(include_str!("🧪️inline-images.json")).unwrap();
    for case in fixture.as_array().unwrap() {
        let field = |key: &str| case[key].as_str().unwrap();
        let mut stream = format!("q\n120 0 0 40 0 0 cm\nBI {} ID{}", field("dict"), field("separator")).into_bytes();
        stream.extend(case["raw"].as_array().unwrap().iter().map(|byte| byte.as_u64().unwrap() as u8));
        stream.extend_from_slice(field("tail").as_bytes());
        stream.extend_from_slice(b"Q\n");
        let mut resources = NativePdfArtifactResources::from_objects(&[]);
        let result = parse_content(&stream, &NoFonts, &mut resources);
        if let Some(problem) = case["error"].as_str() {
            let error = result.unwrap_err().to_string();
            assert!(error.contains(problem), "{}: {error}", field("name"));
            continue;
        }
        let ops = result.unwrap_or_else(|error| panic!("{}: {error}", field("name")));
        let [PdfOp::Save, _, PdfOp::InlineImage { image }, PdfOp::Restore] = ops.as_slice() else { panic!("{}", field("name")) };
        let Some(Ok(samples)) = image.packed_samples() else { panic!("{}", field("name")) };
        let expected: Vec<_> = case["samples"].as_array().unwrap().iter().map(|sample| sample.as_u64().unwrap() as u32).collect();
        assert_eq!(samples.iter().collect::<Vec<_>>(), expected, "{}", field("name"));
        let exported = print_content(&ops, &NoFonts).unwrap();
        assert_eq!(parse_content(&exported, &NoFonts, &mut resources).unwrap(), ops);
    }
}

/// 🎭️ LAW: an inline image mask is one bit per sample whether or not `/BPC` is spelled, and the writer may omit it, so a re-read of what the writer printed equals the first read (the thesis' Type 3 glyph programs read `/BPC 1`, the grafted copy printed none and read back 0).
#[test]
fn inline_image_masks_are_one_bit_per_sample_and_survive_the_writer() {
    use crate::standards::v1_7::subsets::base::io::foreign_artifacts::NativePdfArtifactResources;

    let fixture: serde_json::Value = serde_json::from_str(include_str!("🧪️inline-image-masks.json")).unwrap();
    for case in fixture.as_array().unwrap() {
        let name = case["name"].as_str().unwrap();
        let mut stream = format!("BI {} ID ", case["dict"].as_str().unwrap()).into_bytes();
        stream.extend(case["raw"].as_array().unwrap().iter().map(|byte| byte.as_u64().unwrap() as u8));
        stream.extend_from_slice(b"\nEI\n");
        let mut resources = NativePdfArtifactResources::from_objects(&[]);
        let ops = parse_content(&stream, &NoFonts, &mut resources).unwrap_or_else(|error| panic!("{name}: {error}"));
        let [PdfOp::InlineImage { image }] = ops.as_slice() else { panic!("{name}") };
        assert_eq!(u64::from(image.bits_per_component), case["bitsPerComponent"].as_u64().unwrap(), "{name}");
        let Some(Ok(samples)) = image.packed_samples() else { panic!("{name}") };
        let expected: Vec<_> = case["samples"].as_array().unwrap().iter().map(|sample| sample.as_u64().unwrap() as u32).collect();
        assert_eq!(samples.iter().collect::<Vec<_>>(), expected, "{name}");
        let exported = print_content(&ops, &NoFonts).unwrap();
        assert_eq!(parse_content(&exported, &NoFonts, &mut resources).unwrap(), ops, "{name}: the printed mask reads back equal");
        for (what, spelled) in [("authored", &stream), ("printed", &exported)] {
            let operators = semio_s_artifact_stdio_pdf_test_oracle::standards::v1_7::subsets::base::reference_inline_image_operators(spelled).unwrap_or_else(|error| panic!("{name}: lopdf reads the {what} mask: {error}"));
            assert_eq!(operators.iter().filter(|operator| operator.as_str() == "BI").count(), 1, "{name}: lopdf frames one {what} inline image ({operators:?})");
        }
    }
}

#[path = "🧪️content-program.rs"]
mod content_program;
