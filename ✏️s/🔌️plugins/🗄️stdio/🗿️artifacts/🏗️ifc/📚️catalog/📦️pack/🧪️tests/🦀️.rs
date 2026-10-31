//! 🧪️ Laws of the catalog resource reader: the decoded declarations equal the language-neutral JSON under third-party `serde_json`, and hostile resources are refused.
use super::pack::{decode_catalog_pack,CatalogPackError};
use super::{IfcCatalog,IfcTypeExpression};
use serde_json::{json,Map,Value};
fn expression(value:&IfcTypeExpression)->Value{match value.item.as_deref(){None=>json!({"kind":value.kind,"name":value.name,"width":value.width,"fixed":value.fixed}),Some(item)=>json!({"kind":value.kind,"aggregate":value.aggregate,"lower":value.lower,"upper":value.upper,"lowerExpression":value.lower_expression,"upperExpression":value.upper_expression,"unique":value.unique,"optional":value.optional,"item":expression(item)})}}
fn attribute(value:&super::IfcAttributeDeclaration)->Value{json!({"name":value.name,"typeExpr":expression(&value.type_expr),"optional":value.optional,"derived":value.derived,"declaredBy":value.declared_by})}
pub fn catalog_value(catalog:&IfcCatalog)->Value{
let entities:Map<String,Value>=catalog.entities.iter().map(|(key,e)|(key.clone(),json!({"name":e.name,"abstractEntity":e.abstract_entity,"supertype":e.supertype,"attributes":e.attributes.iter().map(attribute).collect::<Vec<_>>(),"allAttributes":e.all_attributes.iter().map(attribute).collect::<Vec<_>>(),"derived":e.derived.iter().map(|d|json!({"name":d.name,"typeExpr":expression(&d.type_expr),"redeclares":d.redeclares,"expression":d.expression})).collect::<Vec<_>>(),"inverse":e.inverse,"uniqueRules":e.unique_rules,"whereRules":e.where_rules,"supertypeExpression":e.supertype_expression}))).collect();
let types:Map<String,Value>=catalog.types.iter().map(|(key,t)|{let mut row=json!({"name":t.name,"kind":t.kind,"members":t.members,"whereRules":t.where_rules});if let(Some(expr),Some(map))=(&t.type_expr,row.as_object_mut()){map.insert("typeExpr".into(),expression(expr));}(key.clone(),row)}).collect();
json!({"schema":catalog.schema,"version":catalog.version,"source":{"url":catalog.source_url,"mirror":catalog.source_mirror,"sha256":catalog.source_sha256},"entities":entities,"types":types,"functions":catalog.functions,"rules":catalog.rules})
}
/// ⚖️ Assert the decoded catalog equals the retained-declaration JSON member for member, ignoring the generator-only `validation` facet.
pub fn assert_catalog_matches_json(catalog:&IfcCatalog,json_text:&str){
let mut expected:Value=serde_json::from_str(json_text).unwrap();
expected.as_object_mut().unwrap().remove("validation");
let actual=catalog_value(catalog);
for member in ["schema","version","source","functions","rules","types"]{assert_eq!(actual[member],expected[member],"{member}");}
let(actual_entities,expected_entities)=(actual["entities"].as_object().unwrap(),expected["entities"].as_object().unwrap());
assert_eq!(actual_entities.len(),expected_entities.len());
for(key,row)in expected_entities{assert_eq!(actual_entities.get(key),Some(row),"{key}");}
assert_eq!(actual,expected);
}
const MINIMAL_JSON:&str=include_str!("🧫️fixtures/minimal/🔣️.json");
const MINIMAL_PACK:&[u8]=include_bytes!("🧫️fixtures/minimal/catalog.ifcpack");
#[test]
fn minimal_resource_decodes_to_the_neutral_json(){assert_catalog_matches_json(&decode_catalog_pack(MINIMAL_PACK).unwrap(),MINIMAL_JSON);}
#[test]
fn minimal_resource_keeps_expression_domains(){let catalog=decode_catalog_pack(MINIMAL_PACK).unwrap();let points=&catalog.entities["IFCLEAF"].attributes[0].type_expr;assert_eq!((points.kind.as_str(),points.lower,points.upper,points.unique,points.optional),("aggregate",Some(2),None,true,true));assert_eq!(catalog.types["IFCKIND"].members,["A","ÄB"]);}
#[test]
fn truncated_streams_are_refused(){for cut in [0,1,8,MINIMAL_PACK.len()/2,MINIMAL_PACK.len()-1]{assert!(matches!(decode_catalog_pack(&MINIMAL_PACK[..cut]),Err(CatalogPackError::Inflate(_)|CatalogPackError::Truncated|CatalogPackError::Magic)),"{cut}");}}
#[test]
fn foreign_payloads_are_refused(){let stored=|payload:&[u8]|{let length=payload.len() as u16;let mut raw=vec![1u8];raw.extend(length.to_le_bytes());raw.extend((!length).to_le_bytes());raw.extend(payload);raw};
assert_eq!(decode_catalog_pack(&stored(b"NOPE\x01")),Err(CatalogPackError::Magic));
assert_eq!(decode_catalog_pack(&stored(b"IFCP\x01")),Err(CatalogPackError::Truncated));
assert_eq!(decode_catalog_pack(&stored(b"IFCP\x01\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\x01")),Err(CatalogPackError::Overflow));
let mut empty=b"IFCP\x01".to_vec();empty.extend([0,0,0]);
for _ in 0..5{empty.push(0);}
assert!(decode_catalog_pack(&stored(&empty)).is_err());}
