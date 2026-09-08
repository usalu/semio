#[test]
    fn workflow_media_contract_projection_matches_neutral_document_binary_and_conversion_cases() {
        let fixture: Value = serde_json::from_str(include_str!("../🕸️media-projection/🧪️fixture/🔣️.json")).expect("neutral media presentation fixture");
        let cases = fixture["cases"].as_array().expect("four vectors");
        assert_eq!(cases.len(), 4);
        for row in cases {
            let input = &row["input"];
            let contract = MediaContract {
                kind_id: input["kindId"].as_str().unwrap().to_string(),
                media_type: serde_json::from_value(input["mediaType"].clone()).expect("typed media type"),
                wire: serde_json::from_value(input["wire"].clone()).expect("typed document or binary wire"),
                conversion: serde_json::from_value(input["conversion"].clone()).expect("typed optional conversion"),
            };
            let graph = Workflow { schema: WORKFLOW_SCHEMA.into(), nodes: Vec::new(), edges: vec![WorkflowEdge {
                id: row["id"].as_str().unwrap().to_string(), source_node_id: "source".into(), source_port_id: "out".into(),
                target_node_id: "target".into(), target_port_id: "in".into(), contract,
            }] };
            let payload = os_workflow_to_node_graph_payload(&graph);
            let edges: Value = serde_json::from_str(&payload.edges_json).expect("actual scene edge payload");
            assert_eq!(edges.as_array().unwrap().len(), 1);
            assert_eq!(edges[0]["contract"], row["expected"], "{}", row["id"]);
            assert_eq!(edges[0]["isConversion"], row["isConversion"]);
        }
    }
