use semio_framework_test_fixtures::fixture_path;
use std::collections::BTreeSet;

fn identifiers(tokens: proc_macro2::TokenStream, output: &mut BTreeSet<String>) {
    for token in tokens {
        match token {
            proc_macro2::TokenTree::Ident(ident) => { output.insert(ident.to_string()); }
            proc_macro2::TokenTree::Group(group) => identifiers(group.stream(), output),
            _ => {}
        }
    }
}

fn owner_items(root: impl AsRef<std::path::Path>) -> Vec<syn::Item> {
    let mut items = Vec::new();
    let mut files = vec![root.as_ref().to_path_buf()];
    while let Some(path) = files.pop() {
        let file = syn::parse_file(std::fs::read_to_string(&path).expect("fixture").as_str()).unwrap();
        for item in file.items {
            if let syn::Item::Mod(module) = &item {
                for attribute in module.attrs.iter().filter(|attribute| attribute.path().is_ident("path")) {
                    if let syn::Meta::NameValue(pair) = &attribute.meta {
                        if let syn::Expr::Lit(literal) = &pair.value {
                            if let syn::Lit::Str(text) = &literal.lit { files.push(path.parent().unwrap().join(text.value())); }
                        }
                    }
                }
            }
            items.push(item);
        }
    }
    items
}

#[test]
fn product_expanders_own_envelopes_and_diff_transport_only() {
    use quote::ToTokens;
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/🪆️record-owner/🔣️.json")).unwrap();
    let items = owner_items(fixture_path!("../../../../🦀️.rs"));
    for contract in fixture["functions"].as_array().unwrap() {
        let functions: Vec<_> = items.iter().filter_map(|item| match item { syn::Item::Fn(function) if function.sig.ident == contract["name"].as_str().unwrap() => Some(function), _ => None }).collect();
        assert_eq!(functions.len(), 1);
        let mut names = BTreeSet::new();
        identifiers(functions[0].block.to_token_stream(), &mut names);
        for name in contract["required"].as_array().unwrap() { assert!(names.contains(name.as_str().unwrap()), "missing {name}"); }
        for name in contract["forbidden"].as_array().unwrap() { assert!(!names.contains(name.as_str().unwrap()), "duplicate generic identity {name}"); }
    }
}

#[test]
fn product_macro_owner_has_no_generic_record_generator() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/🪆️record-owner/🔣️.json")).unwrap();
    let items = owner_items(fixture_path!("../../../../🦀️.rs"));
    let functions: BTreeSet<_> = items.iter().filter_map(|item| match item { syn::Item::Fn(function) => Some(function.sig.ident.to_string()), _ => None }).collect();
    for name in fixture["retired"].as_array().unwrap() { assert!(!functions.contains(name.as_str().unwrap()), "retired {name}"); }
    assert!(functions.contains("expand_derive_composite_mutation"));
    assert!(functions.contains("expand_mutation_leaf"));
    assert!(functions.contains("expand_derive_mutations"));
}
