#[allow(unused_imports)]
use crate::standards::v1::subsets::any::schema::content::JackContentChild;
#[allow(unused_imports)]
use ::semio_framework_schema::ArtifactSchema;
#[allow(unused_imports)]
use crate::standards::v1::subsets::any::schema::graph::Camera;
#[allow(unused_imports)]
use crate::standards::v1::subsets::any::schema::manifest::Manifest;
use crate::standards::v1::subsets::any::schema::*;

#[semio_framework_async_macros::async_test]
async fn empty_jack_document_has_no_nodes_or_edges() {
    let fixture = empty_jack_document::<crate::test_support::FixtureComposition>();
    assert!(fixture.nodes().expect("valid retained Jack child").is_empty());
    assert!(fixture.edges().expect("valid retained Jack child").is_empty());
}
