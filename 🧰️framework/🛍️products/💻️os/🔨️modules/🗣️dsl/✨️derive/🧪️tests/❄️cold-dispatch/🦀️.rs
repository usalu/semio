//! ❄️ Aggregate cold dispatch agrees with the portable contract and the independent syn syntax tree.
use super::*;

#[test]
fn aggregate_cold_dispatch_matches_portable_contract() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/❄️cold-dispatch/🔣️.json")).unwrap();
    let (workspace, cwd, leaf) = mutation_source_authority_tests::materialize("valid", &mutation_source_authority_tests::fixture());
    let source = leaf.parent().unwrap().parent().unwrap().join("🦀️.rs");
    fs::write(&source, "pub enum Probe {}").unwrap();
    let authority = mutation_aggregate_source_authority(&source, &cwd).unwrap();
    for case in fixture["cases"].as_array().unwrap() {
        let snapshot = case["snapshot"].as_str().unwrap();
        let diff = case["diff"].as_str().unwrap();
        let reference = case["reference"].as_str().map(|reference| format!(", reference = {reference}, reference_diff = {}", case["referenceDiff"].as_str().unwrap())).unwrap_or_default();
        let variants = case["variants"].as_array().unwrap();
        let leaves = variants.iter().map(|leaf| format!("{}({})", leaf["variant"].as_str().unwrap(), leaf["payload"].as_str().unwrap())).collect::<Vec<_>>().join(",");
        let input: DeriveInput = syn::parse_str(&format!(r#"#[mutations(snapshot = {snapshot}, diff = {diff}, schema = "probe"{reference})] enum Ops {{ {leaves} }}"#)).unwrap();
        let tokens = expand_mutations(&input, &authority).unwrap();
        let syntax: syn::File = syn::parse2(tokens.clone()).unwrap();
        let implementations = syntax.items.iter().filter_map(|item| match item { syn::Item::Impl(item) if item.trait_.as_ref().is_some_and(|(_, path, _)| path.segments.last().unwrap().ident == "ColdMutation") => Some(item), _ => None }).collect::<Vec<_>>();
        assert_eq!(implementations.len(), case["expected"]["implementations"].as_u64().unwrap() as usize, "{}", case["name"]);
        let implementation = implementations[0];
        let trait_path = &implementation.trait_.as_ref().unwrap().1;
        let syn::PathArguments::AngleBracketed(arguments) = &trait_path.segments.last().unwrap().arguments else { panic!("cold trait requires a reference type") };
        assert_eq!(quote!(#arguments).to_string().replace(' ', ""), format!("<{}>", case["expected"]["reference"].as_str().unwrap()));
        let associated = implementation.items.iter().find_map(|item| match item { syn::ImplItem::Type(item) if item.ident == "ReferenceDiff" => Some(&item.ty), _ => None }).unwrap();
        assert_eq!(quote!(#associated).to_string(), case["expected"]["referenceDiff"].as_str().unwrap());
        for method in case["expected"]["dispatchMethods"].as_array().unwrap() {
            let method = method.as_str().unwrap();
            let function = implementation.items.iter().find_map(|item| match item { syn::ImplItem::Fn(item) if item.sig.ident == method => Some(item), _ => None }).unwrap();
            let Some(syn::Stmt::Expr(syn::Expr::Match(dispatch), _)) = function.block.stmts.last() else { panic!("cold hook must dispatch by leaf") };
            assert_eq!(dispatch.arms.len(), variants.len(), "{} {method}", case["name"]);
            for (arm, leaf) in dispatch.arms.iter().zip(variants) {
                let syn::Pat::TupleStruct(pattern) = &arm.pat else { panic!("expected wrapped leaf") };
                assert_eq!(pattern.path.segments.last().unwrap().ident, leaf["variant"].as_str().unwrap());
                let syn::Expr::Call(call) = arm.body.as_ref() else { panic!("expected direct cold hook") };
                let syn::Expr::Path(callee) = call.func.as_ref() else { panic!("expected qualified cold kind") };
                let expected: syn::ExprPath = syn::parse_str(&format!("<{} as ::semio_framework_os_command::ColdMutationKind<{}, Ops>>::{method}", leaf["payload"].as_str().unwrap(), case["expected"]["reference"].as_str().unwrap())).unwrap();
                assert_eq!(callee, &expected);
                assert_eq!(call.args.len(), if method == "foreign_step_source" { 3 } else { 2 });
            }
            if method == "diff" {
                assert!(quote!(#function).to_string().contains("Self :: ReferenceDiff"));
                assert!(!quote!(#function).to_string().contains("Self :: Diff"));
            }
        }
        let expanded = tokens.to_string();
        assert_eq!(expanded.matches("admit_inverse").count(), case["expected"]["admissionCalls"].as_u64().unwrap() as usize);
        let expected_law = format!("mutation_inverse_rows_failures :: < {snapshot} , {} , Ops >", case["expected"]["reference"].as_str().unwrap());
        assert!(expanded.contains(&expected_law), "generated fold law must distinguish native and wire snapshots: {}", case["name"]);
    }
    fs::remove_dir_all(workspace).unwrap();
}
