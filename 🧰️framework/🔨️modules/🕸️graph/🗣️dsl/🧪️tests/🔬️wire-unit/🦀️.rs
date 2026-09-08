mod tests {
    use super::*;

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
    fn wire_literal_roundtrip_simple() {
        block_on_test(async {
            // 🩹️ unified grammar: port names are `dsl_core` idents (must start with a letter or
            // `_`, never a digit) — the old port name `"3d"` is no longer lexable, renamed `"d3"`.
            let nodes = vec![WireNode { id: "p".into(), kind: "Puzzle3d".into(), port: None, properties: PropertyBag::new() }];
            let edges = vec![WireEdge { from: "p".into(), from_port: "d3".into(), to: "s".into(), to_port: "d3".into(), directed: true, properties: PropertyBag::new() }];
            let text = wire_literal_from_dag(&nodes, &edges);
            assert!(text.contains("p:Puzzle3d"));
            assert!(text.contains("p:Puzzle3d@d3->s:node@d3"));
            let parsed = dag_from_wire_literal(&text).unwrap();
            assert_eq!(parsed.1.len(), 1);
        });
    }

    #[test]
    fn wire_literal_undirected() {
        block_on_test(async {
            let edges = vec![WireEdge { from: "a".into(), from_port: "out".into(), to: "b".into(), to_port: "in".into(), directed: false, properties: PropertyBag::new() }];
            let text = wire_literal_from_dag(&[], &edges);
            assert!(text.contains('@'));
            assert!(text.contains('-'));
        });
    }

    #[test]
    fn wire_literal_with_properties() {
        block_on_test(async {
            let mut props = PropertyBag::new();
            props.insert("value".into(), PropertyValue::Number(3.0));
            let nodes = vec![WireNode { id: "n".into(), kind: "slider".into(), port: None, properties: props }];
            let text = wire_literal_from_dag(&nodes, &[]);
            // 🩹️ unified syntax: `key=value` (never `key: value`), space-padded braces when glued
            // onto a preceding atom, per `dsl_core::Writer`'s canonical spacing law.
            assert!(text.contains("{ value=3 }"), "expected unified {{ value=3 }} properties, got: {text}");
        });
    }

    #[test]
    fn wire_literal_nested_object_and_array_properties() {
        block_on_test(async {
            let mut inner = PropertyBag::new();
            inner.insert("y".into(), PropertyValue::Bool(true));
            let mut props = PropertyBag::new();
            props.insert("obj".into(), PropertyValue::Object(inner));
            props.insert("arr".into(), PropertyValue::Array(vec![PropertyValue::Number(1.0), PropertyValue::Null]));
            let nodes = vec![WireNode { id: "n".into(), kind: "slider".into(), port: None, properties: props }];
            let text = wire_literal_from_dag(&nodes, &[]);
            assert!(text.contains("obj={ y=true }"), "expected unified obj={{ y=true }}, got: {text}");
            assert!(text.contains("arr=[ 1 null ]"), "expected unified arr=[ 1 null ], got: {text}");
        });
    }

    #[test]
    fn wire_literal_from_dag_unknown_node_kind_defaults_to_node() {
        block_on_test(async {
            let edges = vec![WireEdge { from: "missing".into(), from_port: "out".into(), to: "also-missing".into(), to_port: "in".into(), directed: true, properties: PropertyBag::new() }];
            let text = wire_literal_from_dag(&[], &edges);
            assert_eq!(text, "missing:node@out->also-missing:node@in");
        });
    }

    #[test]
    fn dag_from_wire_literal_rejects_unterminated_string() {
        block_on_test(async {
            // 🩹️ unified syntax: double-quoted properties, `key="value"` (never `key: 'value'`).
            let err = dag_from_wire_literal("n:kind{prop=\"unterminated").unwrap_err();
            assert!(matches!(err, GraphDslError::Lex(_)));
            assert!(err.to_string().contains("unterminated string literal"), "got: {err}");
        });
    }

    #[test]
    fn dag_from_wire_literal_rejects_unexpected_char() {
        block_on_test(async {
            // 🩹️ `#` is now a legitimate comment starter (unified with the rest of the DSL engine),
            // so the "genuinely unrecognized character" trigger moved to `?`, which is outside
            // `dsl_core`'s alphabet in every mode.
            let err = dag_from_wire_literal("n:kind?bad").unwrap_err();
            assert!(matches!(err, GraphDslError::Lex(_)));
            assert!(err.to_string().contains("unexpected character '?'"), "got: {err}");
        });
    }

    #[test]
    fn dag_from_wire_literal_rejects_edge_missing_target_port() {
        block_on_test(async {
            let err = dag_from_wire_literal("a:kind@out->b:kind").unwrap_err();
            assert!(matches!(err, GraphDslError::EdgeTargetMissingPort));
        });
    }

    #[test]
    fn dag_from_wire_literal_rejects_edge_missing_source_port() {
        block_on_test(async {
            // 🆕️ the unified grammar itself leaves the source port optional (unlike the old
            // hand-rolled parser, which could never even reach an edge without one) — this
            // module's own DAG domain rule must now catch it explicitly.
            let err = dag_from_wire_literal("a:kind->b:kind@in").unwrap_err();
            assert!(matches!(err, GraphDslError::EdgeTargetMissingPort));
        });
    }

    #[test]
    fn dag_from_wire_literal_parses_bool_and_null_properties() {
        block_on_test(async {
            // 🩹️ unified syntax: space-separated `key=value` pairs, no commas, no colons.
            let (nodes, _) = dag_from_wire_literal("n:kind{on=true off=false empty=null}").unwrap();
            let props = &nodes[0].properties;
            assert_eq!(props.get("on"), Some(&PropertyValue::Bool(true)));
            assert_eq!(props.get("off"), Some(&PropertyValue::Bool(false)));
            assert_eq!(props.get("empty"), Some(&PropertyValue::Null));
        });
    }

    #[test]
    fn dag_from_wire_literal_parses_double_quoted_string_properties() {
        block_on_test(async {
            let (nodes, _) = dag_from_wire_literal("n:kind{label=\"hello world\"}").unwrap();
            assert_eq!(nodes[0].properties.get("label"), Some(&PropertyValue::String("hello world".to_string())));
        });
    }

    #[test]
    fn dag_from_wire_literal_rejects_malformed_properties() {
        block_on_test(async {
            let err = dag_from_wire_literal("n:kind{prop 1}").unwrap_err();
            assert!(matches!(err, GraphDslError::Lex(_)));
        });
    }

    #[test]
    fn dag_from_wire_literal_accepts_back_arrow_sugar_and_normalizes_direction() {
        block_on_test(async {
            // 🆕️ `<-` is accepted sugar, normalized to the same stored/parsed shape as `->` with
            // endpoints swapped — `dsl_core::parse_wire`'s law, inherited for free.
            let (_, edges) = dag_from_wire_literal("b:kind@in<-a:kind@out").unwrap();
            assert_eq!(edges.len(), 1);
            let edge = &edges[0];
            assert_eq!(edge.from, "a");
            assert_eq!(edge.from_port, "out");
            assert_eq!(edge.to, "b");
            assert_eq!(edge.to_port, "in");
            assert!(edge.directed);
        });
    }

    #[test]
    fn dag_from_wire_literal_parses_undirected_dash_dash_edge() {
        block_on_test(async {
            // 🆕️ unified undirected sigil is `--`, not the old single `-`.
            let (_, edges) = dag_from_wire_literal("a:x@out--b:y@in").unwrap();
            assert_eq!(edges.len(), 1);
            assert!(!edges[0].directed);
            assert_eq!(edges[0].from, "a");
            assert_eq!(edges[0].to, "b");
        });
    }

    #[test]
    fn wire_literal_from_dag_round_trips_through_unified_double_quoted_syntax() {
        block_on_test(async {
            let mut props = PropertyBag::new();
            props.insert("label".into(), PropertyValue::String("hi".into()));
            let nodes = vec![WireNode { id: "n".into(), kind: "slider".into(), port: None, properties: props }];
            let text = wire_literal_from_dag(&nodes, &[]);
            assert!(text.contains("\"hi\""), "properties must print double-quoted: {text}");
            let (parsed_nodes, _) = dag_from_wire_literal(&text).unwrap();
            assert_eq!(parsed_nodes[0].properties.get("label"), Some(&PropertyValue::String("hi".into())));
        });
    }
}
