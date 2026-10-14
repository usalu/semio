//! 🎬️ Actual DAG demo source owned by its artifact producer.
use semio_framework_os_infinite_directed_presentation::*;
use semio_framework_graph_manifest::PropertyBag;
/// 🎬️ Builds the original child-owned DAG demo presentation.
pub fn dag_demo_host_snapshot()->DagHostSnapshot {
        let port = |id: &str, label: &str| IoPortSpec { id: id.into(), label: label.into(), ..IoPortSpec::default() };
        let node = |id: &str, name: &str, icon: &str, x: f64, y: f64, width: f64, height: f64, kind| DagNodeSpec {
            id: id.into(), name: name.into(), abbreviation: name.into(), icon: icon.into(), x, y, width, height, operator_kind: None, properties: PropertyBag::new(), kind,
        };
        DagHostSnapshot {
            schema: "dag.hostDocument".into(), camera: DagCamera { x: 0.0, y: 0.0, zoom: 1.0 },
            nodes: vec![
                node("slider", "Amount", "emoji:🎚️", -400.0, -40.0, 70.0, 14.0, DagNodeKind::Slider { min: 0.0, max: 10.0, step: 0.5, value: 5.0, output: port("out", "value") }),
                node("mode", "Mode", "emoji:📋️", -400.0, 80.0, 56.0, 28.0, DagNodeKind::Select { selected: 0, options: vec!["Add".into(), "Multiply".into(), "Max".into()], output: port("out", "mode") }),
                node("scale", "Scale", "emoji:📐️", -120.0, -40.0, 104.0, 14.0, DagNodeKind::Computation { variadic_inputs: false, variadic_outputs: false, inputs: vec![port("in", "value")], outputs: vec![port("out", "scaled")] }),
                node("combine", "Combine", "emoji:🔀️", 120.0, 0.0, 104.0, 28.0, DagNodeKind::Computation { variadic_inputs: false, variadic_outputs: false, inputs: vec![port("a", "a"), port("b", "b")], outputs: vec![port("out", "merged")] }),
                node("screen", "Preview", "emoji:🖥️", 400.0, 0.0, 200.0, 140.0, DagNodeKind::Screen { media: Some(DagMedia {
                    kind: DagMediaKind::Svg,
                    src: "data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 100 60'%3E%3Crect fill='%233c78d8' width='100' height='60'/%3E%3Ctext x='50' y='35' text-anchor='middle' fill='white' font-size='12'%3EDAG%3C/text%3E%3C/svg%3E".into(),
                }), input: port("in", "result") }),
            ],
            edges: [("e1", "slider@out", "scale@in"), ("e2", "scale@out", "combine@a"), ("e3", "mode@out", "combine@b"), ("e4", "combine@out", "screen@in")].into_iter().map(|(id, source, target)| DagHostSnapshotEdge {
                id: id.into(), source: source.into(), target: target.into(), ..DagHostSnapshotEdge::default()
            }).collect(),
        }
    }
