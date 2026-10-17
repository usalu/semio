#[allow(unused_imports)]
use crate::Puzzle5dSnapshot;
#[allow(unused_imports)]
use crate::standards::v1::subsets::any::schema::kind_catalogs::Puzzle5dKindCatalogsExtra;
#[allow(unused_imports)]
use crate::standards::v1::subsets::any::schema::document_records::Puzzle5dTargetVolume;
#[allow(unused_imports)]
use ::semio_framework_schema::ArtifactSchema;
#[allow(unused_imports)]
use semio_s_artifact_stdio_semio_kit::schema::snapshot::SemioKitSnapshot;
#[allow(unused_imports)]
use std::collections::HashSet;

use crate::standards::v1::subsets::any::schema::*;

#[test]
fn puzzle5d_grip_kinds_compatible_reads_manifest_rows() {
    assert!(puzzle5d_grip_kinds_compatible("port", "port"));
    assert!(puzzle5d_grip_kinds_compatible("vortex", "vortex"));
    assert!(!puzzle5d_grip_kinds_compatible("port", "vortex"));
    assert!(!puzzle5d_grip_kinds_compatible("unknown-kind", "port"));
}
