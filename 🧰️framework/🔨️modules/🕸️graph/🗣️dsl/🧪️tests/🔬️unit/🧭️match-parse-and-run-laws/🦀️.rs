/// 🧭️ Match parse and run laws of the graph DSL: the synchronous test executor, the match, edge and port parses and the DAG fixture and port-filtered query runs.

// 🚫️async: E5-class executor bridge, sanctioned per R4 clause 5 — `#[test]` cannot run
// an `async fn` directly (std has no executor for it), so every async test body in this
// module runs through this instead. Sound because this crate performs no real I/O: every
// future here resolves on its first poll, so a single poll (never a spin-park loop) is
// enough — panics loudly if that invariant is ever violated rather than hanging.
fn block_on_test<F: std::future::Future>(fut: F) -> F::Output {
    use std::task::{Context, Poll, RawWaker, RawWakerVTable, Waker};
    fn noop(_: *const ()) {}
    fn clone_raw(_: *const ()) -> RawWaker {
        RawWaker::new(std::ptr::null(), &VTABLE)
    }
    static VTABLE: RawWakerVTable = RawWakerVTable::new(clone_raw, noop, noop, noop);
    let raw = RawWaker::new(std::ptr::null(), &VTABLE);
    let waker = unsafe { Waker::from_raw(raw) };
    let mut cx = Context::from_waker(&waker);
    let mut fut = Box::pin(fut);
    match fut.as_mut().poll(&mut cx) {
        Poll::Ready(v) => v,
        Poll::Pending => panic!("block_on_test: future did not complete synchronously"),
    }
}

#[test]
fn parse_match_return() {
    block_on_test(async {
        let q = parse("MATCH (a:computation) RETURN a.name").unwrap();
        assert_eq!(q.clauses.len(), 2);
    });
}

#[test]
fn idiom_hooks_canonicalize_and_classify_through_the_dsl_registry_seam() {
    block_on_test(async {
        let hooks = idiom_hooks();
        assert_eq!(hooks.lang, "jack");
        let canonical = (hooks.canonicalize)("MATCH   (a:computation)   RETURN   a.name").expect("canonicalize");
        assert_eq!(canonical, format("MATCH (a:computation) RETURN a.name").unwrap());
        assert!((hooks.canonicalize)("not jack at all $$$").is_err() || (hooks.canonicalize)("not jack at all $$$").is_ok(), "canonicalize must not panic on malformed input");
        let classes = (hooks.classify)("MATCH (a:computation) RETURN a.name");
        assert!(!classes.is_empty());
        assert!(classes.iter().any(|(class, _)| *class == semio_framework_dsl::TokenClass::Keyword), "MATCH/RETURN must classify as keywords");
        semio_framework_dsl::register_idiom(hooks);
        let resolved = semio_framework_dsl::idiom("jack").expect("jack must be resolvable by lang id after registration");
        assert_eq!((resolved.canonicalize)("MATCH (a:computation) RETURN a.name").unwrap(), format("MATCH (a:computation) RETURN a.name").unwrap());
    });
}

#[test]
fn run_dag_fixture_query() {
    block_on_test(async {
        // 🩹️ Was `include_str!` of the dag technology's example fixture; that technology migrated its
        // fixture to a handcrafted DSL (`pack::snapshot::ArtifactDsl`) — inlined the same dag-fixture JSON this
        // test actually parses (`from_dag_host_snapshot_json`), decoupled from its document format.
        let fixture = r#"{
  "schema": "dag.host_snapshot",
  "camera": { "x": 0, "y": 0, "zoom": 1 },
  "nodes": [
    {
      "id": "slider",
      "name": "Amount",
      "abbreviation": "Amount",
      "icon": "emoji:🎚️",
      "kind": "slider",
      "x": -400,
      "y": -40,
      "width": 70,
      "height": 14,
      "min": 0,
      "max": 10,
      "step": 0.5,
      "value": 5,
      "output": { "id": "out", "label": "value", "cardinality": "!" }
    },
    {
      "id": "mode",
      "name": "Mode",
      "abbreviation": "Mode",
      "icon": "emoji:📋️",
      "kind": "select",
      "x": -400,
      "y": 80,
      "width": 56,
      "height": 28,
      "options": ["Add", "Multiply", "Max"],
      "selected": 0,
      "output": { "id": "out", "label": "mode", "cardinality": "!" }
    },
    {
      "id": "scale",
      "name": "Scale",
      "abbreviation": "Scale",
      "icon": "emoji:📐️",
      "kind": "computation",
      "x": -120,
      "y": -40,
      "width": 104,
      "height": 14,
      "inputs": [{ "id": "in", "label": "value", "cardinality": "!" }],
      "outputs": [{ "id": "out", "label": "scaled", "cardinality": "!" }]
    },
    {
      "id": "combine",
      "name": "Combine",
      "abbreviation": "Combine",
      "icon": "emoji:🔀️",
      "kind": "computation",
      "x": 120,
      "y": 0,
      "width": 104,
      "height": 28,
      "inputs": [
        { "id": "a", "label": "a", "cardinality": "!" },
        { "id": "b", "label": "b", "cardinality": "!" }
      ],
      "outputs": [{ "id": "out", "label": "merged", "cardinality": "!" }]
    },
    {
      "id": "screen",
      "name": "Preview",
      "abbreviation": "Preview",
      "icon": "emoji:🖥️",
      "kind": "screen",
      "x": 400,
      "y": 0,
      "width": 200,
      "height": 140,
      "media": {
        "kind": "svg",
        "src": "data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 100 60'%3E%3Crect fill='%233c78d8' width='100' height='60'/%3E%3Ctext x='50' y='35' text-anchor='middle' fill='white' font-size='12'%3EDAG%3C/text%3E%3C/svg%3E"
      },
      "input": { "id": "in", "label": "result", "cardinality": "!" }
    }
  ],
  "edges": [
    { "id": "e1", "source": "slider:out", "target": "scale:in" },
    { "id": "e2", "source": "scale:out", "target": "combine:a" },
    { "id": "e3", "source": "mode:out", "target": "combine:b" },
    { "id": "e4", "source": "combine:out", "target": "screen:in" }
  ]
}
"#;
        let graph = BoardQueryableGraph::from_host_snapshot_json(fixture, None).unwrap();
        let result = run_query(&graph, "MATCH (n:computation) RETURN n.name").unwrap();
        assert!(!result.rows.is_empty());
    });
}

#[test]
fn parse_match_with_port() {
    block_on_test(async {
        let q = parse("MATCH (a:computation@out) RETURN a.name").unwrap();
        let Clause::Match(patterns) = &q.clauses[0] else { panic!("expected match") };
        assert_eq!(patterns[0].nodes[0].port.as_deref(), Some("out"));
    });
}

#[test]
fn parse_undirected_edge() {
    block_on_test(async {
        // 🩹️ unified undirected sigil is `--`, not the old bare `-` (not even lexable in the
        // shared `dsl_core` alphabet, which has no standalone dash token).
        let q = parse("MATCH (a:computation)--(b:slider) RETURN a.name").unwrap();
        let Clause::Match(patterns) = &q.clauses[0] else { panic!("expected match") };
        let edge = patterns[0].edge.as_ref().expect("edge");
        assert!(!edge.directed);
    });
}

#[test]
fn parse_back_arrow_edge_swaps_left_and_right() {
    block_on_test(async {
        let q = parse("MATCH (a:computation)<-(b:slider) RETURN a.name").unwrap();
        let Clause::Match(patterns) = &q.clauses[0] else { panic!("expected match") };
        // `<-` means the edge points INTO the parenthesized-first node — represented by swapping
        // which parsed node plays "left" so `edge.right` is always the forward-direction target.
        assert_eq!(patterns[0].nodes[0].kind, "slider");
        let edge = patterns[0].edge.as_ref().expect("edge");
        assert!(edge.directed);
        assert_eq!(edge.right.kind, "computation");
    });
}

#[test]
fn parse_labeled_directed_and_undirected_edges_use_double_dash_connector() {
    block_on_test(async {
        let forward = parse("MATCH (a:computation)--[r:wire]->(b:slider) RETURN a.name").unwrap();
        let Clause::Match(patterns) = &forward.clauses[0] else { panic!("expected match") };
        let edge = patterns[0].edge.as_ref().expect("edge");
        assert!(edge.directed);
        assert_eq!(edge.var.as_deref(), Some("r"));
        assert_eq!(edge.kind.as_deref(), Some("wire"));

        let undirected = parse("MATCH (a:computation)--[:wire]--(b:slider) RETURN a.name").unwrap();
        let Clause::Match(patterns) = &undirected.clauses[0] else { panic!("expected match") };
        let edge = patterns[0].edge.as_ref().expect("edge");
        assert!(!edge.directed);
    });
}

#[test]
fn run_port_filtered_query() {
    block_on_test(async {
        // 🩹️ Was `include_str!` of the dag technology's example fixture; that technology migrated its
        // fixture to a handcrafted DSL (`pack::snapshot::ArtifactDsl`) — inlined the same dag-fixture JSON this
        // test actually parses (`from_dag_host_snapshot_json`), decoupled from its document format.
        let fixture = r#"{
  "schema": "dag.host_snapshot",
  "camera": { "x": 0, "y": 0, "zoom": 1 },
  "nodes": [
    {
      "id": "slider",
      "name": "Amount",
      "abbreviation": "Amount",
      "icon": "emoji:🎚️",
      "kind": "slider",
      "x": -400,
      "y": -40,
      "width": 70,
      "height": 14,
      "min": 0,
      "max": 10,
      "step": 0.5,
      "value": 5,
      "output": { "id": "out", "label": "value", "cardinality": "!" }
    },
    {
      "id": "mode",
      "name": "Mode",
      "abbreviation": "Mode",
      "icon": "emoji:📋️",
      "kind": "select",
      "x": -400,
      "y": 80,
      "width": 56,
      "height": 28,
      "options": ["Add", "Multiply", "Max"],
      "selected": 0,
      "output": { "id": "out", "label": "mode", "cardinality": "!" }
    },
    {
      "id": "scale",
      "name": "Scale",
      "abbreviation": "Scale",
      "icon": "emoji:📐️",
      "kind": "computation",
      "x": -120,
      "y": -40,
      "width": 104,
      "height": 14,
      "inputs": [{ "id": "in", "label": "value", "cardinality": "!" }],
      "outputs": [{ "id": "out", "label": "scaled", "cardinality": "!" }]
    },
    {
      "id": "combine",
      "name": "Combine",
      "abbreviation": "Combine",
      "icon": "emoji:🔀️",
      "kind": "computation",
      "x": 120,
      "y": 0,
      "width": 104,
      "height": 28,
      "inputs": [
        { "id": "a", "label": "a", "cardinality": "!" },
        { "id": "b", "label": "b", "cardinality": "!" }
      ],
      "outputs": [{ "id": "out", "label": "merged", "cardinality": "!" }]
    },
    {
      "id": "screen",
      "name": "Preview",
      "abbreviation": "Preview",
      "icon": "emoji:🖥️",
      "kind": "screen",
      "x": 400,
      "y": 0,
      "width": 200,
      "height": 140,
      "media": {
        "kind": "svg",
        "src": "data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 100 60'%3E%3Crect fill='%233c78d8' width='100' height='60'/%3E%3Ctext x='50' y='35' text-anchor='middle' fill='white' font-size='12'%3EDAG%3C/text%3E%3C/svg%3E"
      },
      "input": { "id": "in", "label": "result", "cardinality": "!" }
    }
  ],
  "edges": [
    { "id": "e1", "source": "slider:out", "target": "scale:in" },
    { "id": "e2", "source": "scale:out", "target": "combine:a" },
    { "id": "e3", "source": "mode:out", "target": "combine:b" },
    { "id": "e4", "source": "combine:out", "target": "screen:in" }
  ]
}
"#;
        let graph = BoardQueryableGraph::from_host_snapshot_json(fixture, None).unwrap();
        let result = run_query(&graph, "MATCH (n:computation@out)--[:wire]->(m:slider) RETURN n.name, m.name");
        assert!(result.is_ok());
    });
}
