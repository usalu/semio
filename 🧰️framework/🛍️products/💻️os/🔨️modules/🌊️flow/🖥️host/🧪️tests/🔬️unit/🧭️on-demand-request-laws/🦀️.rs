/// 🧭️ On-demand request laws: the guest asks for one deferred extension per namespace of an unserved kind, with the same namespace rule and wire shape as the kernel's `nodeKindRequests` (`🧪️tests/🧭️node-kind-trigger` of the plugin registry pins the TypeScript twin on the same cases).

#[test]
fn unserved_kinds_become_one_sorted_request_per_namespace() {
    let unserved: Vec<String> = ["math.add", "brep.bool.cut", "brep.prim3d.box", "math.vector"].into_iter().map(String::from).collect();
    assert_eq!(
        on_demand_node_kind_requests(&unserved),
        vec![r#"{"kind":"node-kind","nodeKind":"brep.bool.cut"}"#.to_string(), r#"{"kind":"node-kind","nodeKind":"math.add"}"#.to_string()],
    );
}

#[test]
fn a_kind_without_a_usable_namespace_asks_nothing_and_a_bare_kind_is_its_own_namespace() {
    let unserved: Vec<String> = [".hidden", " padded.kind", "zzz", ""].into_iter().map(String::from).collect();
    assert_eq!(on_demand_node_kind_requests(&unserved), vec![r#"{"kind":"node-kind","nodeKind":"zzz"}"#.to_string()]);
    assert!(on_demand_node_kind_requests(&[]).is_empty());
}

#[test]
fn the_wire_text_is_valid_json_even_for_a_hostile_kind() {
    let requests = on_demand_node_kind_requests(&["a\"b\\c.d".to_string()]);
    assert_eq!(requests, vec![r#"{"kind":"node-kind","nodeKind":"a\"b\\c.d"}"#.to_string()]);
}

#[test]
fn the_topic_is_the_one_the_hosts_listen_on() {
    assert_eq!(ON_DEMAND_REQUEST_TOPIC, "os.on-demand.request");
}
