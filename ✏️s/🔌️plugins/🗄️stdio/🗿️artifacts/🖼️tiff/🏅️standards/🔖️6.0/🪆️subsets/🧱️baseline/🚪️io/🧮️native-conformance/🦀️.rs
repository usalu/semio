//! 🛡️ Native TIFF6 Baseline storage class, evaluated at IO.
use crate::standards::v6_0::subsets::document::io::native_observations::TiffNativeObservations;
pub fn classify_tiff_native_baseline(o:&TiffNativeObservations)->Vec<&'static str>{
 let mut codes=Vec::new();
 if !o.raster{codes.push("stdio.tiff.baseline.degenerate-raster")}
 if o.compression.as_ref().and_then(|v|v.first()).is_some_and(|v|![1,2,32773].contains(v)){codes.push("stdio.tiff.baseline.unsupported-compression")}
 if o.photometric.as_ref().and_then(|v|v.first()).is_some_and(|v|*v>3){codes.push("stdio.tiff.baseline.unsupported-photometric")}
 if o.bits_per_sample.as_ref().is_some_and(|v|v.iter().any(|v|![1,4,8].contains(v))){codes.push("stdio.tiff.baseline.unsupported-bits-per-sample")}
 if o.tile_width.is_some()||o.tile_length.is_some(){codes.push("stdio.tiff.baseline.tiled-not-baseline")}else if o.strip_offsets.is_none(){codes.push("stdio.tiff.baseline.missing-strip-offsets")}
 codes
}
