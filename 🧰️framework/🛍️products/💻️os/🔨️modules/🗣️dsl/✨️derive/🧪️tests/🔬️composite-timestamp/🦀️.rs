
use super::*;

#[test]
fn composite_timestamp_expansion_preserves_the_authored_leaf_hook() {
    let input: DeriveInput = syn::parse_str("#[composite(snapshot = Doc, op = Ops)] struct ApplyBatch { clock: Option<Clock> }").unwrap();
    let syntax: syn::File = syn::parse2(expand_composite_mutation(&input).unwrap()).unwrap();
    let implementation = syntax
        .items
        .iter()
        .find_map(|item| match item {
            syn::Item::Impl(item) if item.trait_.as_ref().is_some_and(|(_, path, _)| path.segments.last().unwrap().ident == "MutationKind") => Some(item),
            _ => None,
        })
        .unwrap();
    let timestamp = implementation
        .items
        .iter()
        .find_map(|item| match item {
            syn::ImplItem::Fn(item) if item.sig.ident == "timestamp" => Some(item),
            _ => None,
        })
        .unwrap();
    assert_eq!(timestamp.block.stmts.len(), 1);
    let syn::Stmt::Expr(syn::Expr::Call(call), _) = &timestamp.block.stmts[0] else { panic!("composite timestamp must delegate directly") };
    let syn::Expr::Path(function) = call.func.as_ref() else { panic!("expected composite timestamp hook") };
    assert_eq!(function.path.segments.iter().map(|segment| segment.ident.to_string()).collect::<Vec<_>>(), ["semio_framework_os_kernel", "CompositeMutationKind", "timestamp"]);
    assert_eq!(call.args.len(), 1);
    assert!(matches!(&call.args[0], syn::Expr::Path(path) if path.path.is_ident("self")));
}
