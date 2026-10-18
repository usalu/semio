/// 🔭️ Kind infos and test fixture hosts: the port side law, the kind infos JSON, the fixture and bridge hosts and the slider track point.

/// 🧪️ Installs first-party light (+brep) flow extension manifests and real in-process ops for fixture tests.


/// 🌿️ All 9 first-party flow extensions install into the shared registry and each contributes
/// at least one evaluable neuron kind — procedural3d's flow graph can reach every extension.
/// 🔌️ The catalogue-wide port-side law: one operator's INPUT ids and OUTPUT ids are disjoint,
/// because `"{nodeId}@{portId}"` is the only public name a wire endpoint has. Driven over the LIVE
/// first-party catalogue — every operator every shipped extension registers — so a new operator
/// cannot reintroduce the ambiguity that made a press on `extrusion-axis`'s output `z` begin a wire
/// from its INPUT `z`. Ticket 26/09/09/PROCEDURAL-3D-END-TO-END.
///
/// @see `✏️s/🧑‍💻dev/🌊️flow/🧫️fixtures/🔌️port-sides/🔣️.json`







/// 🧹️ Encodes one law-local operator catalogue and RETIRES it.
///
/// `ChannelSpec::default` carries a `Value`, and `Value`/`Dictionary` fail closed on a bare drop
/// (`🧠️neural/⚙️engine/🦀️.rs`'s `Drop` — "final Dictionary ownership must be explicitly retired
/// or owned by a cold boundary"), so a temporary `Vec<OperatorInfo>` may not simply fall off the
/// end of a law (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
fn kind_infos_json(kind_infos: Vec<NeuronKindInfo>) -> String {
    let kind_infos = neural::ColdOwner::new(kind_infos);
    semio_framework_pack_json::to_json_string(&*kind_infos)
}

/// 🧪️ The two-operator catalogue every host law in this file indexes.
///
/// 🧹️ The catalogue is RETIRED, not dropped: `InputSpec::number_default` puts a `Value` in
/// `ChannelSpec::default`, and `Value`/`Dictionary` fail closed on a bare drop
/// (`🧠️neural/⚙️engine/🦀️.rs`'s `Drop` — "final Dictionary ownership must be explicitly retired
/// or owned by a cold boundary"). Letting the temporary `Vec<OperatorInfo>` fall off the end of
/// this helper aborted every law that builds a host through it
/// (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
fn test_kind_infos_json() -> String {
    kind_infos_json(vec![
        NeuronKindInfo {
            id: "math.add".into(),
            extension: "math".into(),
            name: "Add".into(),
            abbreviation: "Add".into(),
            icon: "emoji:➕️".into(),
            summary: "Sums two numbers".into(),
            inputs: vec![InputSpec::number("a", NUMBER_OPS), InputSpec::number_default("b", 0.0, NUMBER_OPS)],
            outputs: vec![InputSpec::named("S", "Sum", "sum", "Sum")],
            ..Default::default()
        },
        NeuronKindInfo {
            id: "math.passThrough".into(),
            extension: "math".into(),
            name: "PassThrough".into(),
            abbreviation: "Pass".into(),
            icon: "emoji:➡️".into(),
            summary: "Forwards a number".into(),
            inputs: vec![InputSpec::number_default("number", 0.0, NUMBER_OPS)],
            outputs: vec![InputSpec::named("N", "Num", "number", "Number")],
            ..Default::default()
        },
    ])
}

fn host_with_test_fixture() -> FlowHost {
    let mut host = FlowHost::default();
    host.set_neuron_kind_infos_json(&test_kind_infos_json());
    host.set_host_catalogue_json(
        &serde_json::to_string(&[CatalogueSection {
            id: "math".into(),
            title: "Math".into(),
            groups: vec![],
            items: vec![
                CatalogueItem { kind: "neuron".into(), neuron_kind: Some("math.add".into()), action: None, format: None, name: "Add".into(), abbreviation: "Add".into(), icon: "emoji:➕️".into(), summary: "Sums two numbers".into() },
                CatalogueItem {
                    kind: "neuron".into(), neuron_kind: Some("math.passThrough".into()), action: None, format: None, name: "PassThrough".into(), abbreviation: "Pass".into(), icon: "emoji:➡️".into(), summary: "Forwards a number".into()
                },
            ],
        }])
        .unwrap(),
    );
    host
}

#[cfg(not(target_arch = "wasm32"))]
fn host_with_test_bridge() -> FixtureRegistryHost {
    let mut host = host_with_test_fixture();
    let mut host=FixtureRegistryHost::new(host,FixtureOperatorFamily::Math);
    host.evaluate_internal();
    host
}

fn widget_slider_track_screen_point(host: &FlowHost, widget_id: &str) -> (f64, f64) {
    let node = host.dag.graph.host_snapshot.nodes.iter().find(|n| n.id == widget_id).expect("node");
    let (wx, wy) = dag::slider_track_center(node).expect("slider track");
    let cam = Camera { x: host.host_snapshot.camera.x, y: host.host_snapshot.camera.y, zoom: host.host_snapshot.camera.zoom };
    let viewport = Viewport { width: host.viewport_w, height: host.viewport_h, dpr: host.viewport_dpr };
    let screen = world_to_screen(&cam, &viewport, Point::new(wx, wy));
    (screen.x, screen.y)
}
