use super::*;

#[test]
fn generic_emission_matches_every_neutral_mode_under_independent_syntax_parsing() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/🧩️composition/🔣️.json")).unwrap();
    for row in fixture["cases"].as_array().unwrap() {
        let mode = row["mode"].as_str().unwrap();
        let declaration: DeriveInput = syn::parse_str(row["declaration"].as_str().unwrap_or_else(||if matches!(mode, "Scalar" | "Variants") { "enum Example { First, Second }" } else { "struct Example { name: String }" })).unwrap();
        let output = match mode {
            "Record" => emit_record(declaration),
            "Projection" => emit_projection(declaration, syn::parse_str(row["names"]["spec"].as_str().unwrap()).unwrap(), syn::parse_str(row["names"]["to"].as_str().unwrap()).unwrap(), syn::parse_str(row["names"]["from"].as_str().unwrap()).unwrap()),
            "Scalar" => emit_scalar(declaration),
            "Variants" => emit_enum(declaration),
            _ => panic!("unknown authored emission mode"),
        };
        let parsed: syn::File = syn::parse2(output).unwrap();
        let mut members = Vec::new();
        let mut traits = Vec::new();
        for item in parsed.items {
            if let syn::Item::Impl(item) = item {
                if let Some((_, trait_, _)) = item.trait_ { traits.push(trait_.segments.last().unwrap().ident.to_string()); }
                else { for member in item.items { if let syn::ImplItem::Fn(member) = member { members.push(member.sig.ident.to_string()); } } }
            }
        }
        let expected_members: Vec<_> = row["members"].as_array().unwrap().iter().map(|name| name.as_str().unwrap().to_owned()).collect();
        let expected_traits: Vec<_> = row["traits"].as_array().unwrap().iter().map(|name| name.as_str().unwrap().to_owned()).collect();
        assert_eq!(members, expected_members, "{}", row["id"]);
        assert_eq!(traits, expected_traits, "{}", row["id"]);
    }
}
