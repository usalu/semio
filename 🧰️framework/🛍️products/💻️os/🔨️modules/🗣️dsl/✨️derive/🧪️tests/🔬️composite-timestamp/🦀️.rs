
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

#[test]
fn cold_reference_composite_is_explicit_and_separate_from_native_metadata(){
    for explicit in [false,true]{
        let attribute=if explicit{"#[composite(snapshot=Doc,op=Ops,reference=Wire)]"}else{"#[composite(snapshot=Doc,op=Ops)]"};
        let input:DeriveInput=syn::parse_str(&format!("{attribute} struct ApplyBatch;")).unwrap();
        let syntax:syn::File=syn::parse2(expand_composite_mutation(&input).unwrap()).unwrap();
        let implementations=syntax.items.iter().filter_map(|item|match item{syn::Item::Impl(item)=>Some(item),_=>None}).collect::<Vec<_>>();
        let native=implementations.iter().find(|item|item.trait_.as_ref().is_some_and(|(_,path,_)|path.segments.last().unwrap().ident=="MutationKind")).unwrap();
        assert!(native.items.iter().all(|item|!matches!(item,syn::ImplItem::Fn(method) if ["diff","inverse"].iter().any(|name|method.sig.ident==name))));
        let cold=implementations.iter().find(|item|item.trait_.as_ref().is_some_and(|(_,path,_)|path.segments.last().unwrap().ident=="ColdMutationKind"));
        assert_eq!(cold.is_some(),explicit);
        if let Some(cold)=cold{
            assert!(cold.items.iter().any(|item|matches!(item,syn::ImplItem::Fn(method) if method.sig.ident=="diff")));
            assert!(quote!(#cold).to_string().contains("ReferenceDiff"));
        }
    }
}
