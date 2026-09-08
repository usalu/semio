use super::*;

fn node(id: &str) -> EdgeNode {
    EdgeNode { id: id.to_string(), kind: None, port: None }
}

#[semio_framework_async_macros::async_test]
async fn parses_a_bare_node_with_no_link() {
    let value = parse_edge_text("branch-root").expect("parse_edge_text");
    assert_eq!(value, EdgeValue { from: node("branch-root"), link: None });
}

#[semio_framework_async_macros::async_test]
async fn parses_plain_directed_arrow() {
    let value = parse_edge_text("a->b").expect("parse_edge_text");
    assert!(value.link.as_ref().unwrap().directed);
    assert!(value.link.as_ref().unwrap().label.is_empty());
    assert_eq!(print_edge(&value), "a->b");
}

#[semio_framework_async_macros::async_test]
async fn parses_plain_undirected_dash() {
    let value = parse_edge_text("a--b").expect("parse_edge_text");
    assert!(!value.link.as_ref().unwrap().directed);
    assert_eq!(print_edge(&value), "a--b");
}

#[semio_framework_async_macros::async_test]
async fn back_arrow_is_sugar_normalized_by_endpoint_swap() {
    let value = parse_edge_text("b<-a").expect("parse_edge_text");
    assert_eq!(value.from, node("a"));
    assert_eq!(value.link.as_ref().unwrap().to, node("b"));
    assert!(value.link.as_ref().unwrap().directed);
    // Canonical print never re-emits `<-`.
    assert_eq!(print_edge(&value), "a->b");
}

#[semio_framework_async_macros::async_test]
async fn parses_labeled_directed_edge_with_id_and_kind() {
    let value = parse_edge_text("a -e1:Connection>b").expect("parse_edge_text");
    let link = value.link.expect("link");
    assert!(link.directed);
    assert_eq!(link.label, EdgeLabel { id: Some("e1".to_string()), kind: Some("Connection".to_string()) });
    assert_eq!(link.to, node("b"));
    assert_eq!(print_edge(&EdgeValue { from: node("a"), link: Some(link) }), "a -e1:Connection>b");
}

#[semio_framework_async_macros::async_test]
async fn parses_labeled_undirected_edge_id_only() {
    let value = parse_edge_text("a -e1-b").expect("parse_edge_text");
    let link = value.link.expect("link");
    assert!(!link.directed);
    assert_eq!(link.label, EdgeLabel { id: Some("e1".to_string()), kind: None });
    assert_eq!(print_edge(&EdgeValue { from: node("a"), link: Some(link) }), "a -e1-b");
}

#[semio_framework_async_macros::async_test]
async fn parses_labeled_edge_kind_only() {
    let value = parse_edge_text("a -:Connection>b").expect("parse_edge_text");
    let link = value.link.expect("link");
    assert_eq!(link.label, EdgeLabel { id: None, kind: Some("Connection".to_string()) });
    assert_eq!(print_edge(&EdgeValue { from: node("a"), link: Some(link) }), "a -:Connection>b");
}

#[semio_framework_async_macros::async_test]
async fn bracket_labeled_edge_still_parses() {
    let value = parse_edge_text("a -[e1:Connection]->b").expect("parse_edge_text");
    let link = value.link.expect("link");
    assert_eq!(link.label.id.as_deref(), Some("e1"));
}

#[semio_framework_async_macros::async_test]
async fn labeled_back_arrow_is_sugar_normalized_by_endpoint_swap() {
    let value = parse_edge_text("b<-[e1:Connection]-a").expect("parse_edge_text");
    assert_eq!(value.from, node("a"));
    let printed = print_edge(&value);
    let link = value.link.expect("link");
    assert_eq!(link.to, node("b"));
    assert!(link.directed);
    assert_eq!(link.label, EdgeLabel { id: Some("e1".to_string()), kind: Some("Connection".to_string()) });
    assert_eq!(printed, "a -e1:Connection>b");
}

#[semio_framework_async_macros::async_test]
async fn endpoints_carry_kind_and_port() {
    let value = parse_edge_text("capsule@in-a -c1:Connection>tower@out-b").expect("parse_edge_text");
    assert_eq!(value.from, EdgeNode { id: "capsule".to_string(), kind: None, port: Some("in-a".to_string()) });
    let link = value.link.expect("link");
    assert_eq!(link.to, EdgeNode { id: "tower".to_string(), kind: None, port: Some("out-b".to_string()) });
}

#[semio_framework_async_macros::async_test]
async fn node_kind_and_port_round_trip() {
    let value = parse_edge_text("v1:Vertex@p0->v2:Vertex@p1").expect("parse_edge_text");
    assert_eq!(print_edge(&value), "v1:Vertex@p0->v2:Vertex@p1");
}

#[semio_framework_async_macros::async_test]
async fn empty_label_is_rejected() {
    let err = parse_edge_text("a -[]->b").unwrap_err();
    assert!(err.message.contains("must name an id"), "unexpected message: {}", err.message);
}

#[semio_framework_async_macros::async_test]
async fn round_trip_matrix_over_representative_values() {
    let cases = vec![
        EdgeValue { from: node("a"), link: None },
        EdgeValue { from: node("a"), link: Some(EdgeLink { directed: true, label: EdgeLabel::default(), to: node("b") }) },
        EdgeValue { from: node("a"), link: Some(EdgeLink { directed: false, label: EdgeLabel::default(), to: node("b") }) },
        EdgeValue { from: node("a"), link: Some(EdgeLink { directed: true, label: EdgeLabel { id: Some("e1".to_string()), kind: Some("Connection".to_string()) }, to: node("b") }) },
    ];
    for case in cases {
        let printed = print_edge(&case);
        let reparsed = parse_edge_text(&printed).unwrap_or_else(|e| panic!("reparse of {printed:?} failed: {e:?}"));
        assert_eq!(reparsed, case, "round trip mismatch for {printed:?}");
    }
}

#[semio_framework_async_macros::async_test]
async fn parses_quantity_in_native_unit() {
    let gpa = crate::os_dsl::unit_by_symbol("GPa").unwrap();
    let value = parse_quantity_text("210GPa", gpa).expect("parse_quantity_text");
    assert!((value - 210.0).abs() < 1e-9);
    assert_eq!(print_quantity(value, gpa), "210GPa");
}

#[semio_framework_async_macros::async_test]
async fn converts_a_compatible_alien_unit_into_native_scale() {
    let gpa = crate::os_dsl::unit_by_symbol("GPa").unwrap();
    // 210000 MPa == 210 GPa
    let value = parse_quantity_text("210000MPa", gpa).expect("parse_quantity_text");
    assert!((value - 210.0).abs() < 1e-6, "got {value}");
}

#[semio_framework_async_macros::async_test]
async fn bare_number_with_no_suffix_is_read_in_native_unit() {
    let gpa = crate::os_dsl::unit_by_symbol("GPa").unwrap();
    let value = parse_quantity_text("210", gpa).expect("parse_quantity_text");
    assert!((value - 210.0).abs() < 1e-9);
}

#[semio_framework_async_macros::async_test]
async fn rejects_a_dimensionally_incompatible_unit() {
    let gpa = crate::os_dsl::unit_by_symbol("GPa").unwrap();
    let err = parse_quantity_text("210m", gpa).unwrap_err();
    assert!(err.message.contains("not compatible"), "unexpected message: {}", err.message);
}

#[semio_framework_async_macros::async_test]
async fn rejects_an_unknown_unit_symbol() {
    let gpa = crate::os_dsl::unit_by_symbol("GPa").unwrap();
    let err = parse_quantity_text("210Zorkels", gpa).unwrap_err();
    assert!(err.message.contains("unknown unit"), "unexpected message: {}", err.message);
}

#[semio_framework_async_macros::async_test]
async fn parses_and_prints_angles_in_degrees() {
    let value = parse_angle_text("45").expect("parse_angle_text");
    assert!((value - 45.0).abs() < 1e-9);
    assert_eq!(print_angle(value), "45°");
}

#[semio_framework_async_macros::async_test]
async fn angle_accepts_radians_and_converts_to_degrees() {
    let value = parse_angle_text("3.14159265358979rad").expect("parse_angle_text");
    assert!((value - 180.0).abs() < 1e-6, "got {value}");
}
