//! 🚪️ Syn independently validates generated diff contracts against the neutral representation fixture.
#[test]
fn diff_representations_generate_only_owned_methods() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🚪️diff-codecs/🔣️.json")).unwrap();
    let name: syn::Type = syn::parse_quote!(crate::semantic::ExampleDiff);
    for contract in fixture["representations"].as_array().unwrap() {
        let tokens = match contract["name"].as_str().unwrap() { "text" => super::diff_text_tokens(&name), "binary" => super::diff_binary_tokens(&name), _ => panic!("invalid representation") };
        let generated: syn::ItemImpl = syn::parse2(tokens).unwrap();
        assert_eq!(generated.trait_.unwrap().1.segments.last().unwrap().ident.to_string(), contract["contract"].as_str().unwrap());
        let methods: Vec<_> = generated.items.iter().filter_map(|item| match item { syn::ImplItem::Fn(method) => Some(method.sig.ident.to_string()), _ => None }).collect();
        let expected: Vec<_> = contract["methods"].as_array().unwrap().iter().map(|name|name.as_str().unwrap().to_owned()).collect();
        assert_eq!(methods, expected);
    }
}
