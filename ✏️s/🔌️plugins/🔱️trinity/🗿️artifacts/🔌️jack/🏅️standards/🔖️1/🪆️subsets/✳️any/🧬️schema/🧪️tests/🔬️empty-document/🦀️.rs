
use super::*;

#[semio_framework_async_macros::async_test]
async fn empty_jack_document_has_no_nodes_or_edges() {
    let fixture = empty_jack_document();
    assert!(fixture.nodes().is_empty());
    assert!(fixture.edges().is_empty());
}
