//! 🪆️ Owned image validity does not admit native byte carriers.
use super::*;
#[test]
fn owned_snapshot_validation_requires_precise_samples_and_canonical_schema() {
 let snapshot=PngSnapshot::default();assert!(snapshot.validate().is_ok());let mut invalid=snapshot.clone();invalid.image.samples.pop();assert!(invalid.validate().is_err());let mut invalid=snapshot;invalid.schema="other".into();assert!(invalid.validate().is_err());
}
