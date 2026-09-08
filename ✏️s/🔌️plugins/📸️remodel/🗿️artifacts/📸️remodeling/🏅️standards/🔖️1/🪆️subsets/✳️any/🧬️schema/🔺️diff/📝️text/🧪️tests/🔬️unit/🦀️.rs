
use super::*;
use crate::default_remodeling_scene;
use crate::schema::diff::RemodelingGcpList;

#[semio_framework_async_macros::async_test]
async fn empty_diff_is_identity_and_absorb_is_fieldwise_last_writer() {
    let scene = default_remodeling_scene();
    assert_eq!(RemodelingDiff::default().apply(&scene).expect("valid identity diff"), scene);

    let mut diff = RemodelingDiff::default();
    diff.absorb(RemodelingDiff { gcps: Some(RemodelingGcpList { values: Vec::new() }), ..Default::default() });
    assert!(diff.gcps.is_some());
    diff.absorb(RemodelingDiff::default());
    assert!(diff.gcps.is_some(), "absorbing empty never clobbers a real entry");
}
