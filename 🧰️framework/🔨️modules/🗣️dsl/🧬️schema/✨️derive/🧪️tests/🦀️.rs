use super::*;

#[test]
fn generic_emission_matches_every_neutral_mode_under_independent_syntax_parsing() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../🧪️tests/🧫️fixtures/✨️derive/🧩️composition/🔣️.json")).unwrap();
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

#[test]
fn required_inline_statement_emission_preserves_the_authored_native_owner() {
 let fixture:serde_json::Value=serde_json::from_str(include_str!("../../../🧪️tests/🧫️fixtures/✨️derive/🏷️required-inline/🔣️.json")).unwrap();
 for row in fixture["fieldKinds"].as_array().unwrap(){
  let declaration:DeriveInput=syn::parse_str(&format!("struct Required {{ #[dsl(statements)] statement: {} }}",row["type"].as_str().unwrap())).unwrap();
  let fields=match &declaration.data{Data::Struct(value)=>&value.fields,_=>unreachable!()};
  let plans=plan_fields(fields);let actual=match &plans[0].kind{FieldKind::RequiredInlineStatements(_)=>"RequiredInlineStatements",FieldKind::RequiredStatements(_)=>"RequiredStatements",FieldKind::OptionStatements(_)=>"OptionStatements",FieldKind::VecStatements(_)=>"VecStatements",_=>"wrong"};
  assert_eq!(actual,row["role"].as_str().unwrap());
  let output=emit_record(declaration);let _:syn::File=syn::parse2(output.clone()).unwrap();
  if actual=="RequiredInlineStatements"{let output=output.to_string();assert!(!output.contains("Box :: new"));assert!(!output.contains("self . statement . as_ref"));assert!(output.contains("DslVariants"));}
 }
 println!("[DEBUG] required inline, boxed, optional and repeated native variant roles parsed with independent syn");
}

#[test]
fn borrowed_tables_are_upper_snake_named_and_stay_distinct() {
    let table = |owner: &str, role: &str| borrowed_table_ident(&syn::Ident::new(owner, proc_macro2::Span::call_site()), role).to_string();
    assert_eq!(table("OpsAuthor", "FIELDS"), "__DSL_BORROWED_OPS_AUTHOR_FIELDS");
    assert_eq!(table("Port", "VARIANT_3_FIELDS"), "__DSL_BORROWED_PORT_VARIANT_3_FIELDS");
    assert_ne!(table("HTTPServer", "ENUM"), table("HttpServer", "ENUM"));
    for input in ["struct OpsAuthor { name: String }", "enum Kind { A, B }"] {
        let declaration: DeriveInput = syn::parse_str(input).unwrap();
        let output = match &declaration.data {
            Data::Struct(_) => borrowed_record_owner_codegen(&declaration, false),
            _ => borrowed_scalar_owner_codegen(&declaration),
        };
        let parsed: syn::File = syn::parse2(output).unwrap();
        let statics: Vec<_> = parsed.items.iter().filter_map(|item| if let syn::Item::Static(item) = item { Some(item.ident.to_string()) } else { None }).collect();
        assert_eq!(statics.len(), 1);
        assert!(statics.iter().all(|name| name.chars().all(|c| !c.is_lowercase())), "{statics:?}");
    }
}

#[test]
fn original_optional_custody_is_excluded_from_wire_but_retired_as_owned_native_field() {
 let fixture:serde_json::Value=serde_json::from_str(include_str!("../../../🧪️tests/🧫️fixtures/✨️derive/🫴️custody/🔣️.json")).unwrap();
 let declaration:DeriveInput=syn::parse_str(fixture["declaration"].as_str().unwrap()).unwrap();let Data::Struct(data)=&declaration.data else {unreachable!()};
 let (spec,to,from,fields)=record_codegen(&data.fields);let _:syn::File=syn::parse2(emit_record(declaration.clone())).unwrap();
 assert_eq!(spec.len(),fixture["wireIds"].as_array().unwrap().len());assert_eq!(fields.iter().map(ToString::to_string).collect::<Vec<_>>(),fixture["nativeFields"].as_array().unwrap().iter().map(|field|field.as_str().unwrap().to_owned()).collect::<Vec<_>>());
 for output in [spec,to,schema_record_codegen(&data.fields),encoding_record_codegen(&data.fields,false),borrowed_record_codegen(&data.fields,&declaration.ident)] {let text=output.into_iter().map(|tokens|tokens.to_string()).collect::<String>();assert!(!text.contains("source"),"custody source entered wire projection: {text}");}
 for output in [retained_record_codegen(&data.fields,false),retained_record_key_codegen(&data.fields,false)] {let text=output.to_string();assert!(!text.contains("self . source"));}
 let decode=from.into_iter().map(|tokens|tokens.to_string()).collect::<String>();assert!(decode.contains("None"));assert!(!decode.contains("missing field 'source'"));
 let controlled=controlled_record_codegen(&data.fields).into_iter().map(|tokens|tokens.to_string()).collect::<String>();assert!(controlled.contains("None"));assert!(!controlled.contains("record . get (1u16)"));
 assert!(record_retirement_codegen(&data.fields,None).to_string().contains("source"));
 let invalid:DeriveInput=syn::parse_str(fixture["invalidDeclaration"].as_str().unwrap()).unwrap();assert!(emit_record(invalid).to_string().contains(fixture["invalidMessage"].as_str().unwrap()));
 println!("[DEBUG] originalOptionalCustody=wireOmitted originalNativeField=retirementIncluded independentSynOutput=valid invalidNonOptional=refused");
}

/// 🕳️ Zero-variant borrowed projections eliminate the underlying uninhabited owner.
#[test]
fn empty_enum_borrowed_and_receiving_emission_matches_independent_syntax() {
 struct Matches { count:usize }
 impl<'ast> syn::visit::Visit<'ast> for Matches {
  fn visit_expr_match(&mut self,expression:&'ast syn::ExprMatch) {
   if expression.arms.is_empty() && matches!(&*expression.expr,syn::Expr::Unary(value) if matches!(value.op,syn::UnOp::Deref(_)) && matches!(&*value.expr,syn::Expr::Path(value) if value.path.is_ident("self"))) { self.count+=1; }
   syn::visit::visit_expr_match(self,expression);
  }
 }
 let fixture:serde_json::Value=serde_json::from_str(include_str!("../../../🧪️tests/🧫️fixtures/✨️derive/🕳️empty/🔣️.json")).unwrap();
 for row in fixture["cases"].as_array().unwrap() {
  let declaration:syn::DeriveInput=syn::parse_str(row["declaration"].as_str().unwrap()).unwrap();
  let output=match row["mode"].as_str().unwrap() { "Variants"=>emit_enum(declaration),"Scalar"=>emit_scalar(declaration),"Receiving"=>emit_receiving(declaration),_=>unreachable!() };
  let syntax:syn::File=syn::parse2(output).unwrap();let mut matches=Matches{count:0};syn::visit::Visit::visit_file(&mut matches,&syntax);
  assert_eq!(matches.count,row["borrowedEmptyMatches"].as_u64().unwrap() as usize,"{}",row["id"]);
 }
 println!("[DEBUG] Empty tagged/scalar/owned-receiving borrowed eliminations match six neutral cases under independent syn AST");
}

#[test]
fn original_empty_enums_match_dereferenced_uninhabited_values_in_all_borrowed_methods() {
    let law:serde_json::Value=serde_json::from_str(include_str!("🕳️empty/🔣️.json")).unwrap();
    for row in law["cases"].as_array().unwrap() {
        let input:DeriveInput=syn::parse_str(row["declaration"].as_str().unwrap()).unwrap();
        let generated=match row["mode"].as_str().unwrap(){"Variants"=>emit_enum(input),"Receiving"=>owned_receiving::emit_owned_receiving(&input),"Scalar"=>emit_scalar(input),_=>unreachable!()};
        let parsed:syn::File=syn::parse2(generated.clone()).unwrap();assert!(!parsed.items.is_empty());
        let text=generated.to_string();assert_eq!(text.matches("match * self { }").count(),row["borrowedMatches"].as_u64().unwrap() as usize);assert_eq!(text.matches("match self { }").count(),row["ownedMatches"].as_u64().unwrap() as usize);
    }
}
