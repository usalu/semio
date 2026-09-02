//! 🧪️ Third-party Syn parses the crate root and the facade against one language-neutral export
//! roster. `#[proc_macro_derive]` can only be tagged at the proc-macro crate root (rustc requires
//! it), so the owner `🦀️.rs` — pure implementation, no crate — must register none.
use std::collections::BTreeSet;

fn registered(source: &str) -> BTreeSet<String> {
    let file = syn::parse_file(source).unwrap();
    file.items.iter().flat_map(|item| match item { syn::Item::Fn(item) => item.attrs.as_slice(), _ => &[] }).filter_map(|attribute| {
        if !attribute.path().is_ident("proc_macro_derive") { return None; }
        let args = attribute.parse_args_with(syn::punctuated::Punctuated::<syn::Meta, syn::Token![,]>::parse_terminated).unwrap();
        match args.first().unwrap() { syn::Meta::Path(path) => Some(path.get_ident().unwrap().to_string()), _ => panic!("derive name is not an identifier") }
    }).collect()
}

#[test]
fn facade_exports_match_registered_macros() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("🧪️fixture/🔣️.json")).unwrap();
    let expected = |field: &str| fixture[field].as_array().unwrap().iter().map(|name| name.as_str().unwrap().to_owned()).collect::<BTreeSet<_>>();
    let owner = registered(include_str!("../../🦀️.rs"));
    let compiled = registered(include_str!("../../📦️packages/🦀️rust/🦀️.rs"));
    assert_eq!(owner, BTreeSet::new(), "owner component.rs must hold implementation only, no crate-root derive tags");
    assert_eq!(compiled, expected("registeredDerives"));
    let facade = syn::parse_file(include_str!("../../../🦀️.rs")).unwrap();
    let exports: BTreeSet<_> = facade.items.iter().filter_map(|item| match item { syn::Item::Use(item) => Some(&item.tree), _ => None }).filter_map(|tree| match tree { syn::UseTree::Path(path) if path.ident == "dsl_derive" => Some(path.tree.as_ref()), _ => None }).flat_map(|tree| match tree { syn::UseTree::Group(group) => group.items.iter().collect::<Vec<_>>(), _ => panic!("facade derive exports must be explicit") }).map(|tree| match tree { syn::UseTree::Name(name) => name.ident.to_string(), _ => panic!("facade derive exports must be named") }).collect();
    assert_eq!(exports, expected("facadeExports"));
    assert!(exports.is_subset(&compiled));
    assert!(expected("traitOnly").is_disjoint(&compiled));
}
