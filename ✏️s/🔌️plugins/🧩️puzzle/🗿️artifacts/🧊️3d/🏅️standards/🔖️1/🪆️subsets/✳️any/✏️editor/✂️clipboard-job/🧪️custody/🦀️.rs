//! ✂️ Clipboard selection preserves exact primary/fallback identities without a hover owner.
use super::*;
#[test]
fn clipboard_selection_matches_neutral_primary_fallback_and_empty_domains(){
 let law:semio_framework_value::serde_json::Value=semio_framework_value::serde_json::from_str(include_str!("fixture.json")).unwrap();for row in law["selectionCases"].as_array().unwrap(){let mut state=protocol::InteractionState::default();for(domain,value)in row["domains"].as_object().unwrap(){let target=state.selection.entry(domain.clone()).or_default();target.granularity=value["granularity"].as_str().unwrap().into();target.ids=value["ids"].as_array().unwrap().iter().map(|id|id.as_str().unwrap().into()).collect();}let selected=Puzzle3dInteractionSnapshot::from_selection_state(&state);assert_eq!(selected.granularity,row["expected"]["granularity"].as_str().unwrap());assert_eq!(selected.selected,row["expected"]["selected"].as_array().unwrap().iter().map(|id|id.as_str().unwrap().to_owned()).collect::<Vec<_>>());assert!(selected.hovered.is_empty()&&selected.referenced.is_empty());}
}
