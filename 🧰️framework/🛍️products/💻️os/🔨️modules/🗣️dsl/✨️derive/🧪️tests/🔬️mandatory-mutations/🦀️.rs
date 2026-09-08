
use super::*;

#[test]
fn expanded_aggregate_matches_neutral_contract_and_syn_ast() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../🧬️mandatory-mutations/🔣️.json")).unwrap();
    let leaf_fixture = mutation_source_authority_tests::fixture();
    let (workspace, cwd, leaf) = mutation_source_authority_tests::materialize("valid", &leaf_fixture);
    let source = leaf.parent().unwrap().parent().unwrap().join("🦀️.rs");
    fs::write(&source, "pub enum Probe {}").unwrap();
    let authority = mutation_aggregate_source_authority(&source, &cwd).unwrap();
    for case in fixture["cases"].as_array().unwrap() {
        let input: DeriveInput = syn::parse_str(case["source"].as_str().unwrap()).unwrap();
        let result = expand_mutations(&input, &authority);
        assert_eq!(result.is_ok(), case["accepted"].as_bool().unwrap(), "{}", case["name"]);
        let Ok(tokens) = result else {
            continue;
        };
        let syntax: syn::File = syn::parse2(tokens.clone()).unwrap();
        let implementations: Vec<&syn::ItemImpl> = syntax.items.iter().filter_map(|item| if let syn::Item::Impl(item) = item { Some(item) } else { None }).collect();
        let mutation = implementations.iter().find(|item| item.trait_.as_ref().unwrap().1.segments.last().unwrap().ident == "Mutation").unwrap();
        let names: Vec<String> = mutation
            .items
            .iter()
            .filter_map(|item| match item {
                syn::ImplItem::Const(item) => Some(item.ident.to_string()),
                syn::ImplItem::Fn(item) => Some(item.sig.ident.to_string()),
                _ => None,
            })
            .collect();
        assert!(names.iter().any(|name| name == "DESCRIPTORS"));
        assert!(names.iter().any(|name| name == "descriptor"));
        assert!(names.iter().any(|name| name == "timestamp"));
        let conversions = implementations.iter().filter(|item| item.trait_.as_ref().unwrap().1.segments.last().unwrap().ident == "From").count();
        assert_eq!(conversions, case["leaves"].as_u64().unwrap() as usize);
        let timestamp = mutation
            .items
            .iter()
            .find_map(|item| match item {
                syn::ImplItem::Fn(item) if item.sig.ident == "timestamp" => Some(item),
                _ => None,
            })
            .unwrap();
        let Some(syn::Stmt::Expr(syn::Expr::Match(dispatch), _)) = timestamp.block.stmts.last() else { panic!("timestamp must directly delegate by leaf") };
        assert_eq!(dispatch.arms.len(), conversions);
        for arm in &dispatch.arms {
            let syn::Expr::Call(call) = arm.body.as_ref() else { panic!("timestamp must call the leaf hook") };
            assert_eq!(call.args.len(), 1);
            assert!(matches!(&call.func.as_ref(), syn::Expr::Path(path) if path.path.segments.last().unwrap().ident == "timestamp"));
            assert!(matches!(&call.args[0], syn::Expr::Path(path) if path.path.is_ident("payload")));
        }
        assert_eq!(!mutation.generics.params.is_empty(), case["generic"].as_bool().unwrap());
        if case["generic"] == true {
            assert!(mutation.generics.where_clause.is_some());
        }
        let expanded = tokens.to_string();
        assert_eq!(expanded.matches("validate_leaf").count(), conversions);
        assert_eq!(expanded.matches("validate_mutation_leaf_descriptor_roster_uniqueness").count(), 1);
        assert_eq!(expanded.matches("include_str !").count(), 3);
        assert!(!expanded.contains("include !"));
        assert!(expanded.contains("MutationLeaf > :: DESCRIPTOR"));
        assert!(expanded.contains("MutationLeaf > :: PROVENANCE"));
        let registration = syntax
            .items
            .iter()
            .find_map(|item| match item {
                syn::Item::Fn(item) if item.sig.ident.to_string().starts_with("register_") => Some(item),
                _ => None,
            })
            .unwrap();
        assert_eq!(registration.sig.inputs.len(), 1);
        assert!(matches!(&registration.sig.output, syn::ReturnType::Type(_, ty) if matches!(ty.as_ref(), Type::Path(path) if path.path.segments.last().unwrap().ident == "Result")));
        assert_eq!(registration.sig.generics, mutation.generics);
        let descriptors = registration
            .block
            .stmts
            .iter()
            .find_map(|statement| match statement {
                syn::Stmt::Local(local) if matches!(&local.pat, syn::Pat::Ident(name) if name.ident == "descriptors") => Some(local.init.as_ref().unwrap().expr.as_ref()),
                _ => None,
            })
            .unwrap();
        let syn::Expr::Array(descriptors) = descriptors else { panic!("registration must preconstruct the complete descriptor array") };
        assert_eq!(descriptors.elems.len(), conversions);
        for descriptor in &descriptors.elems {
            let syn::Expr::Try(checked) = descriptor else { panic!("descriptor errors must propagate") };
            let syn::Expr::Call(constructor) = checked.expr.as_ref() else { panic!("expected complete descriptor constructor") };
            assert_eq!(constructor.args.len(), 5);
        }
        assert_eq!(expanded.matches("register_mutation_descriptors").count(), 1);
        assert!(!expanded.contains("with_semantics"));
    }
    assert!(workspace.exists());
}
