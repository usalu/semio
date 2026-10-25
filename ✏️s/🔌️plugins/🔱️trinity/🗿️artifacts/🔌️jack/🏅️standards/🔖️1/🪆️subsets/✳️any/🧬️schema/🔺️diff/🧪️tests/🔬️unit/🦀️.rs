#[allow(unused_imports)]
use crate::standards::v1::subsets::any::schema::graph::Camera;
#[allow(unused_imports)]
use crate::standards::v1::subsets::any::schema::content::JackContentChild;
#[allow(unused_imports)]
use ::semio_framework_schema::ArtifactSchema;
#[allow(unused_imports)]
use crate::standards::v1::subsets::any::schema::snapshot::JackSnapshot;
#[allow(unused_imports)]
use protocol::{DiffAlgebra, MutationDiff};
use protocol::ColdMutationDiff;
use crate::standards::v1::subsets::any::schema::diff::*;

#[semio_framework_async_macros::async_test]
async fn jack_diff_absorb_merges_content() {
    let mut diff = JackDiff::default();
    let node = crate::standards::v1::subsets::any::schema::graph::Node { id: "x".into(), kind: "Piece".into(), name: "x".into(), x: 0.0, y: 0.0, width: 1.0, height: 1.0, properties: Default::default(), ports: vec![] };
    let other = JackDiff { content: Some(<crate::test_support::FixtureComposition as crate::standards::v1::subsets::any::schema::content::JackContentFactory>::child_from_working(vec![node], vec![])), ..Default::default() };
    diff.absorb(other.clone());
    assert_eq!(diff.content, other.content);
}
