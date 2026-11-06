//! 🧭️ Independent Rust syntax observations verify every generated owner root.
use semio_framework_test_fixtures::fixture_text;
use syn::visit::Visit;
struct Paths{roots:Vec<String>}
impl<'a> Visit<'a> for Paths{
 fn visit_path(&mut self,path:&'a syn::Path){if path.segments.iter().nth(1).is_some_and(|segment|matches!(segment.ident.to_string().as_str(),"__private"|"snapshot_projection"|"retained_clone"|"retirement"|"RetirementDemand"|"ValueError"|"ValueRefusalKind")){self.roots.push(path.segments.first().unwrap().ident.to_string());}syn::visit::visit_path(self,path);}
}
#[test]
fn snapshot_projection_expansion_uses_declared_owner_for_empty_and_field_records(){
 let cases:serde_json::Value=serde_json::from_str(fixture_text!("../../📽️projection/🧪️tests/🔣️.json")).unwrap();
 for row in cases["layouts"].as_array().unwrap(){let input:syn::DeriveInput=syn::parse_str(&format!("{} struct Record{{field:u64}}",row["attribute"].as_str().unwrap())).unwrap();let output=super::expand(&input);assert_eq!(output.is_ok(),row["accepted"].as_bool().unwrap(),"{}",row["attribute"]);if let Ok(tokens)=output{let _:syn::File=syn::parse2(tokens).unwrap();}}
 for row in cases["cases"].as_array().unwrap(){let shape=if row["shape"]=="empty"{"struct Record;"}else{"struct Record{field:u64}"};let input:syn::DeriveInput=syn::parse_str(&format!("{} {shape}",row["attribute"].as_str().unwrap())).unwrap();let output=super::expand(&input).unwrap();let file:syn::File=syn::parse2(output).unwrap();let mut paths=Paths{roots:Vec::new()};paths.visit_file(&file);assert!(!paths.roots.is_empty());assert!(paths.roots.iter().all(|root|root==row["root"].as_str().unwrap()),"{}: {:?}",row["id"],paths.roots);}
}

#[test]fn projection_record_emits_bounded_work_ports_with_explicit_child_allowance(){let input:syn::DeriveInput=syn::parse_str("struct Record{field:u64}").unwrap();let file:syn::File=syn::parse2(super::expand(&input).unwrap()).unwrap();let mut seen=0;for item in file.items{if let syn::Item::Impl(owner)=item{for item in owner.items{if let syn::ImplItem::Fn(method)=item{let name=method.sig.ident.to_string();let expected=match name.as_str(){"demands"|"advance_work_demand"=>4,"advance"=>5,_=>continue};assert_eq!(method.sig.inputs.len(),expected);assert!(method.sig.inputs.iter().any(|arg|matches!(arg,syn::FnArg::Typed(arg)if matches!(&*arg.pat,syn::Pat::Ident(p)if p.ident=="max_work"))));seen+=1;}}}}assert_eq!(seen,3);}
