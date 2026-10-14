use crate::snapshot::Puzzle3dSnapshot;
use crate::io::binary::snapshot::*;
use crate::io::text::snapshot as dsl;

#[test]
fn pack_round_trips_and_agrees_with_dsl() {
    let document = dsl::parse_dsl(&std::fs::read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../📚️examples/🌲️concrete-forest/🖼️assets/🌲️forest/🗣️.dsl.semio")).expect("committed example reads")).expect("parse concrete-forest example");
    semio_framework_os_kernel::os_store::test_support::assert_dsl_pack_equivalence(&document);
    let bytes = encode(&document);
    assert_eq!(decode(&bytes).expect("decode"), document);
}
