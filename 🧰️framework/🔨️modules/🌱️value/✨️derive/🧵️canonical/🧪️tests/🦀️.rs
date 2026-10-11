//! 🧪️ Field role classification admits only roles whose canonical projection equals the declared value wire.
use super::*;
use syn::parse::Parser;

fn role(field:&str)->syn::Result<Wire> {
    let field=syn::Field::parse_named.parse_str(field).unwrap();
    let declared=canonical_field_role(&field.attrs)?;
    wire(&parse_field_attrs(&field.attrs)?,declared)
}
fn predicate(field:&str)->syn::Result<String> {
    let field=syn::Field::parse_named.parse_str(field).unwrap();
    presence(&parse_field_attrs(&field.attrs)?,&quote!(&self.field)).map(|tokens|tokens.to_string())
}

#[test]
fn octet_roles_accept_only_the_intrinsic_array_projection() {
    for field in [
        r#"#[value(with = "pack::value::bytes")] x: Vec<u8>"#,
        r#"#[value(with = "pack::value::bytes", serialize_controlled_with = "pack::value::bytes::to_value_controlled")] x: Vec<u8>"#,
        r#"#[value(serialize_with = "pack::value::bytes::to_value", serialize_controlled_with = "pack::value::bytes::to_value_controlled")] x: Vec<u8>"#,
        r#"#[value(with = "::semio_framework_value::bytes")] x: Vec<u8>"#,
        r#"#[value(with = "pack::value::bytes::optional", serialize_controlled_with = "pack::value::bytes::optional::to_value_controlled")] x: Option<Vec<u8>>"#,
        r#"#[value(default, deserialize_with = "elsewhere")] x: Vec<u8>"#,
        "x: Vec<u8>",
    ] {
        assert!(matches!(role(field),Ok(Wire::Direct)),"{field}");
    }
    for field in [
        r#"#[value(with = "pack::value::bytes::optional", serialize_controlled_with = "pack::value::bytes::to_value_controlled")] x: Option<Vec<u8>>"#,
        r#"#[value(with = "pack::value::bytes", serialize_controlled_with = "other::to_value_controlled")] x: Vec<u8>"#,
        r#"#[value(with = "other::bytes")] x: Vec<u8>"#,
        r#"#[value(serialize_with = "pack::value::bytes::to_value_other")] x: Vec<u8>"#,
        r#"#[value(serialize_with = "position::to_value")] x: u64"#,
        r#"#[value(serialize_controlled_with = "pack::value::bytes::to_value_controlled")] x: Vec<u8>"#,
    ] {
        assert!(role(field).is_err(),"{field}");
    }
}

#[test]
fn decimal_string_role_is_explicit_and_requires_the_value_serializer() {
    assert!(matches!(role(r#"#[value(serialize_with = "position::to_value")] #[canonical_json(decimal_string)] x: u64"#),Ok(Wire::Decimal)));
    assert!(matches!(role(r#"#[value(with = "position")] #[canonical_json(decimal_string)] x: u64"#),Ok(Wire::Decimal)));
    assert!(role("#[canonical_json(decimal_string)] x: u64").is_err());
    assert!(role(r#"#[value(serialize_controlled_with = "position::to_value_controlled")] #[canonical_json(decimal_string)] x: u64"#).is_err());
    assert!(role(r#"#[canonical_json(unknown)] x: u64"#).is_err());
}

#[test]
fn skip_predicates_accept_any_path_and_keep_native_empty_owner_forms() {
    assert_eq!(predicate(r#"#[value(skip_serializing_if = "is_zero_byte")] x: u8"#).unwrap(),quote!(!is_zero_byte(&self.field)).to_string());
    assert_eq!(predicate(r#"#[value(skip_serializing_if = "Mp4CodecFormat::is_default")] x: u8"#).unwrap(),quote!(!Mp4CodecFormat::is_default(&self.field)).to_string());
    assert_eq!(predicate(r#"#[value(skip_serializing_if = "crate::util::is_zero")] x: u64"#).unwrap(),quote!(!crate::util::is_zero(&self.field)).to_string());
    assert_eq!(predicate(r#"#[value(skip_serializing_if = "Option::is_none")] x: Option<u8>"#).unwrap(),quote!(!(&self.field).is_none()).to_string());
    assert_eq!(predicate("x: u8").unwrap(),quote!(true).to_string());
    assert!(predicate(r#"#[value(skip_serializing_if = "not a path")] x: u8"#).is_err());
}

fn hex_reference(field:&str)->syn::Result<String> {
    let field=syn::Field::parse_named.parse_str(field).unwrap();
    hex_word_reference(&field.ty,&quote!(&self.field),&syn::parse_quote!(owner)).map(|tokens|tokens.to_string())
}

#[test]
fn hex_word_role_is_explicit_and_independent_of_the_value_serializer() {
    assert!(matches!(role("#[canonical_json(hex_word)] x: f32"),Ok(Wire::HexWord)));
    assert!(matches!(role(r#"#[value(with = "words")] #[canonical_json(hex_word)] x: [f32; 3]"#),Ok(Wire::HexWord)));
    assert!(role("#[canonical_json(hex_word)] #[canonical_json(decimal_string)] x: f32").is_err());
    assert!(role("#[canonical_json(hex_word, decimal_string)] x: f32").is_err());
    assert!(role(r#"#[value(serialize_with = "words::to_value")] x: [f32; 3]"#).is_err());
}

#[test]
fn hex_word_references_cover_scalar_array_list_and_optional_owners() {
    assert_eq!(hex_reference("x: f32").unwrap(),quote!(owner::ArtifactCanonicalHexWordF32::from_ref(&self.field)).to_string());
    assert_eq!(hex_reference("x: [f32; 3]").unwrap(),quote!(owner::ArtifactCanonicalHexWordArray::from_ref(&self.field)).to_string());
    assert_eq!(hex_reference("x: Vec<f32>").unwrap(),quote!(owner::ArtifactCanonicalHexWordList::from_ref(&self.field)).to_string());
    assert!(hex_reference("x: Option<[f32; 3]>").unwrap().contains("ArtifactCanonicalHexWordArray"));
    assert!(hex_reference("x: Option<f32>").unwrap().contains("None"));
    for ty in ["f64","[f64; 3]","Vec<u8>","Option<u32>","Vec<Vec<f32>>","[u8; 3]","String"] {assert!(hex_reference(&format!("x: {ty}")).is_err(),"{ty}");}
}

#[test]
fn tree_role_projects_through_the_field_tree_beside_any_value_serializer() {
    assert!(matches!(role("#[canonical_json(tree)] x: State"),Ok(Wire::Direct)));
    assert!(matches!(role(r#"#[value(with = "managed_mesh::json")] #[canonical_json(tree)] x: Option<State>"#),Ok(Wire::Direct)));
    assert!(matches!(role(r#"#[value(serialize_with = "a::to_value", serialize_controlled_with = "a::to_value_controlled")] #[canonical_json(tree)] x: State"#),Ok(Wire::Direct)));
    assert!(role(r#"#[value(with = "managed_mesh::json")] x: Option<State>"#).is_err());
    assert!(role("#[canonical_json(tree, hex_word)] x: f32").is_err());
    assert!(role("#[canonical_json(tree)] #[canonical_json(decimal_string)] x: u64").is_err());
}

#[test]
fn flatten_role_requires_the_field_tree_and_no_presence_predicate() {
    assert!(matches!(role("#[value(flatten)] x: Header"),Ok(Wire::Flatten)));
    assert!(matches!(role("#[value(flatten)] x: Option<Header>"),Ok(Wire::Flatten)));
    assert!(matches!(role(r#"#[value(flatten, with = "header_json")] #[canonical_json(tree)] x: Header"#),Ok(Wire::Flatten)));
    assert!(role(r#"#[value(flatten, with = "header_json")] x: Header"#).is_err());
    assert!(role("#[value(flatten)] #[canonical_json(hex_word)] x: f32").is_err());
    assert!(predicate(r#"#[value(flatten, skip_serializing_if = "Option::is_none")] x: Option<Header>"#).is_err());
    assert!(predicate("#[value(flatten)] x: Header").is_ok());
}
