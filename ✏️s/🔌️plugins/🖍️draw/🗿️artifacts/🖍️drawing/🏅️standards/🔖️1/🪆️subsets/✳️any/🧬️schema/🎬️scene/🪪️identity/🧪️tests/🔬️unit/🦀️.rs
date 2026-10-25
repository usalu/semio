#[allow(unused_imports)] use semio_s_artifact_draw_drawing_scene::{booleans::*,identity::*,paint::*,picking::*,placement::*,preparation::*,raster::*,retirement::*,text::*,trace::*,view::*};
#[allow(unused_imports)] use semio_s_artifact_draw_drawing_model::fill_rule::FillRule;
use semio_framework_test_fixtures::fixture_text;

#[allow(unused_imports)] use semio_s_artifact_draw_drawing_scene::identity::*;
use serde_json::Value;
fn decode(value:&Value)->Option<SceneIdentity>{let instance=u32::try_from(value["instance"].as_u64()?).ok().filter(|instance|*instance!=0)?;let base=parse_scene_identity_u64(value["base"].as_str()?)?;let generation=parse_scene_identity_u64(value["generation"].as_str()?)?;let bytes=value["revision"].as_array()?;if bytes.len()!=32{return None;}let mut revision=[0;32];for(index,value)in bytes.iter().enumerate(){revision[index]=u8::try_from(value.as_u64()?).ok()?;}Some(SceneIdentity{instance,base,generation,revision})}
#[test]
fn mounted_vector_geometry_requires_every_full_width_source_identity_field(){let rows:Value=serde_json::from_str(fixture_text!("../../../../../../🧪️tests/🧫️fixtures/🎬️scene/🪪️identity/🔣️.json")).unwrap();for row in rows.as_array().unwrap(){let captured=decode(&row["captured"]).unwrap();let live=decode(&row["live"]).unwrap();assert_eq!(captured.matches(live),row["matches"].as_bool().unwrap());}}
#[test]
fn mounted_vector_geometry_refuses_malformed_decimal_source_lanes_and_unsigned_widths(){let rows:Value=serde_json::from_str(fixture_text!("../../../../../../🧪️tests/🧫️fixtures/🎬️scene/🪪️identity/⚠️invalid/🔣️.json")).unwrap();let source:Value=serde_json::from_str(fixture_text!("../../../../../../🧪️tests/🧫️fixtures/🎬️scene/🪪️identity/🔣️.json")).unwrap();for row in rows.as_array().unwrap(){let mut value=source[0]["captured"].clone();for(key,entry)in row["patch"].as_object().unwrap(){value[key]=entry.clone();}assert!(decode(&value).is_none(),"{}",row["name"]);}let invalid=SceneIdentity{instance:0,base:0,generation:0,revision:[0;32]};assert!(!invalid.matches(invalid));}
