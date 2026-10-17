//! 🗂️ Shipped family requests retain their actual immutable sources through funded close.
use super::*;
use semio_framework_2d_text_font::*;
use semio_framework_2d_text_font_testing::{close};
#[global_allocator]
static FONT_HEAP:semio_framework_trace::HeapWitness=semio_framework_trace::HeapWitness;
#[test]
fn font_catalog_shared_family_requests_preserve_actual_sources(){let policy:serde_json::Value=serde_json::from_str(include_str!("🧫️fixtures/🔣️.json")).unwrap();assert_eq!(policy["defaultFamily"],crate::DEFAULT_FAMILY);for family in policy["families"].as_array().unwrap(){let family=family.as_str().unwrap();let mut job=crate::FontCatalogAdmissionJob::new(family,400,FontOutlineLimits::default());while !job.advance(7).unwrap().done{}let output=job.take_result().unwrap();let sources:Vec<_>=crate::FONT_SOURCES.iter().filter(|row|row.family==family).chain(crate::FONT_SOURCES.iter().filter(|row|family!="Noto Emoji"&&row.family=="Noto Emoji")).collect();assert_eq!(sources.len(),output.len());assert_eq!(sources[0].family,family);for(font,source)in output.iter().zip(sources){assert_eq!(font.source.0.as_ptr(),source.bytes.0.as_ptr());let oracle=ttf_parser::Face::parse(source.bytes.0,0).unwrap();assert_eq!(font.face.glyphs,oracle.number_of_glyphs());assert_eq!(font.face.units_per_em,oracle.units_per_em());}close(job);close(output);}for(family,weight)in[("Unsupported Font",400),("Anta",700)]{let mut job=crate::FontCatalogAdmissionJob::new(family,weight,FontOutlineLimits::default());assert!(job.advance(1).is_err());assert!(job.result().is_err());close(job);}}
