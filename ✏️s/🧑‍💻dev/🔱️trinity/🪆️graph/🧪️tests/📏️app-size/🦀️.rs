//! 📏️ The mounted Jack app stays a small handle: every owner above the budget lives in its own heap allocation, so no thread stack or future holds a megabyte.
use semio_plugin::{EditorApp,VcsArtifactApp};
use std::mem::size_of;
use crate::SemioJackComposition;
use crate::jack::editor::jack::TrinityJackPlayApp;
type Editor=EditorApp<TrinityJackPlayApp<SemioJackComposition>>;
type Members=semio_s_artifact_stdio_semio::SemioMembers;

/// 📏️ Every measured size is declared in the shared language-agnostic budget fixture.
#[test]
fn mounted_jack_app_stays_within_the_recorded_size_budget(){
 let fixture:serde_json::Value=serde_json::from_str(include_str!("../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🧫️fixtures/📏️mounted-app-size/🔣️.json")).expect("size budget fixture");
 let ceiling=fixture["stackCeilingBytes"].as_u64().expect("stack ceiling") as usize;
 for(id,actual) in [("trinityJackEditorVcsApp",size_of::<VcsArtifactApp<Editor,Members>>()),("trinityJackEditorApp",size_of::<Editor>())]{
  let budget=fixture["budgets"].as_array().expect("budgets").iter().find(|row|row["id"]==id).unwrap_or_else(||panic!("{id} has no recorded budget"))["maxBytes"].as_u64().expect("budget bytes") as usize;
  assert!(budget<=ceiling,"{id} budget {budget} exceeds the {ceiling} byte stack ceiling");
  assert!(actual<=budget,"{id} grew to {actual} bytes, budget {budget}");
 }
}
