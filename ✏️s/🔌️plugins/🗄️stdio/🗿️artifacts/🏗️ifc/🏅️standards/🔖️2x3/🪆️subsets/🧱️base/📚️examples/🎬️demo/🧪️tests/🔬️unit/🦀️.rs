use super::*;
#[semio_framework_async_macros::async_test]
async fn demo_source_nonempty() {
    use crate::standards::v2x3::subsets::base::schema::{demo_ifc2x3_snapshot,snapshot::Ifc2x3Snapshot};
    let demo=demo_ifc2x3_snapshot();
    assert_eq!(<Ifc2x3Snapshot as store::ArtifactDsl>::parse_dsl(PRIMARY_TEXT).expect("authored IFC2x3 example text"),demo);
    assert_eq!(store::ArtifactDsl::print_dsl(&demo),PRIMARY_TEXT);
    let pack=include_bytes!("../../🖼️assets/🎒️.pack.semio");
    assert_eq!(<Ifc2x3Snapshot as store::ArtifactPack>::decode_pack(pack).expect("authored IFC2x3 example pack"),demo);
    assert_eq!(store::ArtifactPack::encode_pack(&demo),pack);
    let _ = source();
}
