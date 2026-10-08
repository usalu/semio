/// 🧪️ Baseline diagnostics inspect authored sample interpretation.
use super::*;
use crate::schema::snapshot::TiffValues;
fn snapshot(width:u32,height:u32,present:bool)->TiffSnapshot{let mut value=crate::schema::blank_tiff_snapshot();for tag in &mut value.ifds[0].entries{if tag.tag==256{tag.values=TiffValues::Long(vec![width]);}if tag.tag==257{tag.values=TiffValues::Long(vec![height]);}}if !present{value.ifds[0].blocks.clear();}value}
#[test]fn no_ifd_is_flagged_soft(){let findings=check_tiff_baseline_conformance(&TiffSnapshot::default());assert_eq!(findings.len(),1);assert_eq!(findings[0].code.0,CODE_NO_IFD);}
#[test]fn degenerate_zero_dimensions_are_flagged_soft(){assert!(check_tiff_baseline_conformance(&snapshot(0,0,false)).iter().any(|d|d.code.0==CODE_DEGENERATE_RASTER));}
#[test]fn missing_authored_sample_block_is_flagged_soft(){assert!(check_tiff_baseline_conformance(&snapshot(4,4,false)).iter().any(|d|d.code.0==CODE_DEGENERATE_RASTER));}
#[test]fn well_formed_raster_has_no_findings(){assert!(check_tiff_baseline_conformance(&crate::schema::blank_tiff_snapshot()).is_empty());}
#[test]fn unsupported_photometric_is_flagged_soft(){let mut value=crate::schema::blank_tiff_snapshot();value.ifds[0].entries.iter_mut().find(|t|t.tag==262).unwrap().values=TiffValues::Short(vec![6]);assert!(check_tiff_baseline_conformance(&value).iter().any(|d|d.code.0==CODE_UNSUPPORTED_PHOTOMETRIC));}
#[test]fn unsupported_bits_per_sample_is_flagged_soft(){let mut value=crate::schema::blank_tiff_snapshot();value.ifds[0].entries.iter_mut().find(|t|t.tag==258).unwrap().values=TiffValues::Short(vec![16]);assert!(check_tiff_baseline_conformance(&value).iter().any(|d|d.code.0==CODE_UNSUPPORTED_BITS_PER_SAMPLE));}
