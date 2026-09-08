
use super::*;

#[test]
fn puzzle5d_grip_kinds_compatible_reads_manifest_rows() {
    assert!(puzzle5d_grip_kinds_compatible("port", "port"));
    assert!(puzzle5d_grip_kinds_compatible("vortex", "vortex"));
    assert!(!puzzle5d_grip_kinds_compatible("port", "vortex"));
    assert!(!puzzle5d_grip_kinds_compatible("unknown-kind", "port"));
}
