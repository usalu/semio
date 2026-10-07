use super::*;
#[test]
fn derives_geometry_from_owned_image() {let mut snapshot=crate::PngSnapshot::default();snapshot.image.width=3;snapshot.image.height=3;snapshot.image.samples=vec![255;36];assert_eq!(compute_png_dimensions(&snapshot),PngDimensions{width:3,height:3,bit_depth:8,has_alpha:true,pixel_count:9});}
#[test]
fn grayscale_16_has_no_alpha() {let mut snapshot=crate::PngSnapshot::default();snapshot.image.width=2;snapshot.image.bit_depth=16;snapshot.image.color_type=crate::schema::snapshot::PngColorType::Grayscale;snapshot.image.samples=vec![257,32769];assert_eq!(compute_png_dimensions(&snapshot),PngDimensions{width:2,height:1,bit_depth:16,has_alpha:false,pixel_count:2});}
