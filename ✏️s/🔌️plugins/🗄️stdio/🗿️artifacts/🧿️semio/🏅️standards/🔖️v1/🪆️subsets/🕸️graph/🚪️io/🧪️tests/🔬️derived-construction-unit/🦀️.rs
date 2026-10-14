mod tests {
    use super::*;

    #[semio_framework_async_macros::async_test]
    async fn typed_constructors_build_a_populated_snapshot() {
        let snapshot = SemioGraphBuilderConstruction::new()
            .add_node("n1", "source", "Source", SemioPoint2 { x: 0.0, y: 0.0 }, vec![], vec![])
            .add_node("n2", "sink", "Sink", SemioPoint2 { x: 10.0, y: 10.0 }, vec![], vec![])
            .add_edge("e1", "n1", "n2", "flow", "Main")
            .build()
            .expect("build");
        assert_eq!(snapshot.nodes.len(), 2);
        assert_eq!(snapshot.edges.len(), 1);
    }
}
