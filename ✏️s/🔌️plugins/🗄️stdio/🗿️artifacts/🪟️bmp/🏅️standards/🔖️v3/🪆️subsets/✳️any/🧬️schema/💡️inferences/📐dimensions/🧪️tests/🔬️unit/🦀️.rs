use super::*;

#[test]
fn derives_dimensions_from_owned_fields() {
 let snapshot=BmpSnapshot {schema:crate::STDIO_BMP_DOCUMENT_SCHEMA.into(),image:crate::schema::snapshot::BmpImage {width:3,height:2,pixels:crate::schema::snapshot::BmpPixels::Direct {samples:vec![crate::schema::snapshot::BmpNativeSample::default();6]},..Default::default()}};
 assert_eq!(compute_bmp_dimensions(&snapshot),BmpDimensions {width:3,height:2,bit_depth:24,has_alpha:false,pixel_count:6});
}

#[test]
fn geometry_query_has_no_native_admission_dependency() {
 let snapshot=BmpSnapshot {schema:crate::STDIO_BMP_DOCUMENT_SCHEMA.into(),image:crate::schema::snapshot::BmpImage {width:u32::MAX,..Default::default()}};
 assert!(snapshot.validate().is_err());
 assert_eq!(compute_bmp_dimensions(&snapshot).pixel_count,u64::from(u32::MAX));
}
