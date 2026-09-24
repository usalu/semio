use serde_json::Value;
use std::fs::read_to_string;
use std::path::{Path, PathBuf};
use syn::visit::{self, Visit};
use syn::{Expr, ExprCall, Item, ItemFn, Stmt};

#[derive(Default)]
struct CallNames(Vec<String>);

impl<'ast> Visit<'ast> for CallNames {
    fn visit_expr_call(&mut self, node: &'ast ExprCall) {
        if let Expr::Path(path) = node.func.as_ref() {
            if let Some(segment) = path.path.segments.last() {
                self.0.push(segment.ident.to_string());
            }
        }
        visit::visit_expr_call(self, node);
    }
}

fn repo_root() -> PathBuf {
    option_env!("SEMIO_REPO_ROOT").map(PathBuf::from).unwrap_or_else(|| PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../.."))
}

fn source(path: impl AsRef<Path>) -> String {
    read_to_string(path).expect("source must remain readable")
}

fn mains(source: &str) -> Vec<ItemFn> {
    syn::parse_file(source)
        .expect("Rust source must parse through syn")
        .items
        .into_iter()
        .filter_map(|item| match item {
            Item::Fn(function) if function.sig.ident == "main" => Some(function),
            _ => None,
        })
        .collect()
}

fn calls(function: &ItemFn) -> Vec<String> {
    let mut visitor = CallNames::default();
    visitor.visit_item_fn(function);
    visitor.0
}

fn ordered(calls: &[String], expected: &[&str]) -> bool {
    let mut previous = None;
    expected.iter().all(|expected| {
        let start = previous.map_or(0, |index| index + 1);
        let found = calls[start..].iter().position(|actual| actual == expected).map(|index| start + index);
        previous = found;
        found.is_some()
    })
}

fn schemas_preflight(function: &ItemFn) -> bool {
    let Some(Stmt::Expr(Expr::If(preflight), _)) = function.block.stmts.first() else {
        return false;
    };
    let condition_is_equality = matches!(preflight.cond.as_ref(), Expr::Binary(condition) if matches!(&condition.op, syn::BinOp::Eq(_)));
    let Some(Stmt::Macro(print)) = preflight.then_branch.stmts.first() else {
        return false;
    };
    let Some(Stmt::Expr(Expr::Return(_), _)) = preflight.then_branch.stmts.last() else {
        return false;
    };
    condition_is_equality && print.mac.path.is_ident("print") && print.mac.tokens.to_string().contains("schema_mirror_json") && preflight.then_branch.stmts.len() == 2
}

#[test]
fn hub_credential_source_order_syn_parity() {
    let root = repo_root();
    let fixture: Value = serde_json::from_str(&source(root.join("🌎️hub/🧫️fixtures/🧱️foundation-source/🔣️.json"))).expect("foundation fixture must parse");
    let hostile = fixture["sourceBoundary"]["rust"]["source"].as_str().expect("Rust hostile source");
    let expected = fixture["sourceBoundary"]["rust"]["expectedCalls"].as_array().expect("expected Rust calls").iter().map(|value| value.as_str().expect("call name")).collect::<Vec<_>>();
    let hostile_mains = mains(hostile);
    assert_eq!(hostile_mains.len(), fixture["sourceBoundary"]["rust"]["expectedBodies"].as_u64().expect("expected body count") as usize);
    let hostile_calls = calls(&hostile_mains[0]);
    assert!(ordered(&hostile_calls, &expected));
    assert_eq!(hostile_calls.iter().filter(|name| expected.contains(&name.as_str())).count(), expected.len());

    let mcp = source(root.join("🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🏗️bootstrap/🦀️.rs"));
    let mcp_mains = mains(&mcp);
    assert_eq!(mcp_mains.len(), 1);
    assert!(schemas_preflight(&mcp_mains[0]));
    assert!(ordered(&calls(&mcp_mains[0]), &["claim_inherited_local_hub_credential", "parse_args", "run_stdio"]));

    let native = source(root.join("🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/⌨️native-entrypoint/🦀️.rs"));
    let native_mains = mains(&native);
    assert_eq!(native_mains.len(), 2);
    let serving = native_mains.iter().filter(|main| calls(main).iter().any(|name| name == "claim_inherited_local_hub_credential")).collect::<Vec<_>>();
    assert_eq!(serving.len(), 1);
    assert!(ordered(&calls(serving[0]), &["claim_inherited_local_hub_credential", "arg_value", "run_native"]));
}
