use crate::editor::{build_set_page_mutation, create_idml_editor, IdmlEditor, IDML_EDITOR_DIALECT};
use crate::viewer::{create_idml_viewer, IDML_VIEWER_DIALECT};
use semio_plugin::app::DocumentWindowKit;
use semio_plugin::{AppRole, ArtifactEditor};
use semio_s_artifact_stdio_idml_story::story_pages;
use semio_s_artifact_stdio_idml::{IdmlMutation, IdmlSnapshot};

fn birth() -> IdmlSnapshot {
    <IdmlEditor as ArtifactEditor>::reference_birth().expect("birth producer")().expect("birth snapshot")
}

fn apply(snapshot: &IdmlSnapshot, mutation: IdmlMutation) -> IdmlSnapshot {
    let IdmlMutation::ReplacePart { path, bytes } = mutation else { panic!("replace part expected") };
    let mut next = snapshot.clone();
    next.parts.iter_mut().find(|part| part.path == path).unwrap().bytes = bytes;
    next
}

#[test]
fn manifests_declare_the_idml_coordinate_and_roles() {
    let editor = create_idml_editor();
    let viewer = create_idml_viewer();
    assert_eq!(editor.role, AppRole::Editor);
    assert_eq!(viewer.role, AppRole::Viewer);
    assert_eq!(editor.dialect, IDML_EDITOR_DIALECT.into());
    assert_eq!(viewer.dialect, IDML_VIEWER_DIALECT.into());
    assert!(editor.window_kinds.iter().any(|window| window.id == DocumentWindowKit::KIND_ID));
}

#[test]
fn edit_roundtrips_through_natural_file_and_keeps_other_parts() {
    let base = birth();
    let run = &story_pages(&base)[0][0];
    let revision = DocumentWindowKit::text_revision(&run.text);
    let mutation = build_set_page_mutation(&base, 0, 0, &revision, "Grüße & mehr").unwrap().unwrap();
    let edited = apply(&base, mutation);
    let bytes = <IdmlEditor as ArtifactEditor>::encode_natural_file(&edited).unwrap();
    let reopened = <IdmlEditor as ArtifactEditor>::decode_natural_file(&bytes).unwrap();
    assert_eq!(reopened, edited);
    assert_eq!(story_pages(&reopened)[0][0].text, "Grüße & mehr");
    assert_eq!(reopened.parts[0], base.parts[0]);
    assert_eq!(reopened.parts[1], base.parts[1]);
}

#[test]
fn stale_and_missing_addresses_are_refused() {
    let base = birth();
    assert!(build_set_page_mutation(&base, 0, 0, "0000000000000000", "x").is_err());
    assert!(build_set_page_mutation(&base, 4, 0, "0000000000000000", "x").is_err());
}

#[test]
fn unchanged_text_emits_no_mutation() {
    let base = birth();
    let revision = DocumentWindowKit::text_revision(&story_pages(&base)[0][0].text);
    assert_eq!(build_set_page_mutation(&base, 0, 0, &revision, "").unwrap(), None);
}

const SAMPLE: &[u8] = include_bytes!("../../../../../📚️examples/🎬️demo/🖼️assets/🧪️example/📚️.idml");

#[test]
fn the_demo_package_opens_edits_saves_and_reopens() {
    let opened = <IdmlEditor as ArtifactEditor>::decode_natural_file(SAMPLE).expect("demo package admits");
    let pages = story_pages(&opened);
    let texts: Vec<_> = pages[0].iter().map(|run| run.text.as_str()).collect();
    assert_eq!(texts, ["Hello IDML", "Grüße & Salut"]);
    let revision = DocumentWindowKit::text_revision(texts[1]);
    let edited = apply(&opened, build_set_page_mutation(&opened, 0, 1, &revision, "Servus").unwrap().unwrap());
    let reopened = <IdmlEditor as ArtifactEditor>::decode_natural_file(&<IdmlEditor as ArtifactEditor>::encode_natural_file(&edited).unwrap()).unwrap();
    assert_eq!(story_pages(&reopened)[0][1].text, "Servus");
    assert_eq!(story_pages(&reopened)[0][0].text, "Hello IDML");
    assert_eq!(reopened.parts.iter().map(|part| part.path.as_str()).collect::<Vec<_>>(), opened.parts.iter().map(|part| part.path.as_str()).collect::<Vec<_>>());
}
