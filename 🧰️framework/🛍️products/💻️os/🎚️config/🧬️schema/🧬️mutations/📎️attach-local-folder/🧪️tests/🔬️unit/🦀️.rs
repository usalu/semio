use super::*;
use protocol::Mutation;

fn binding(document_id: &str, path: &str) -> LocalFolderBinding {
    LocalFolderBinding { document_id: document_id.to_string(), plugin_id: "puzzle2d".to_string(), app_id: "s.puzzle.puzzle2d@1/*#editor".to_string(), folder: LocalFolderRef::Path { path: path.to_string() } }
}

#[test]
fn attach_serializes_like_the_typescript_projection() {
    let json = serde_json::to_value(attach_local_folder(binding("board.ports.directed.v1", "/data/puzzles"))).expect("attach encodes");
    assert_eq!(json, serde_json::json!({ "mutation": "attachLocalFolder", "documentId": "board.ports.directed.v1", "pluginId": "puzzle2d", "appId": "s.puzzle.puzzle2d@1/*#editor", "folder": { "kind": "path", "path": "/data/puzzles" } }));
}

#[test]
fn attaching_a_new_document_inverts_to_its_detachment() {
    let base = LocalFolderBindings::default();
    assert_eq!(attach_local_folder(binding("board.ports.directed.v1", "/data/puzzles")).inverse(&base).expect("valid retained mutation inverse fixture"), vec![super::super::detach_local_folder("board.ports.directed.v1")]);
}

#[test]
fn reattaching_a_bound_document_replaces_it_in_place_and_inverts_to_the_prior_binding() {
    let base = LocalFolderBindings { bindings: vec![binding("a.fixture", "/data/a"), binding("b.fixture", "/data/b")] };
    let moved = attach_local_folder(binding("a.fixture", "/data/a-moved"));
    let mut snapshot = base.clone();
    apply_local_folders_config_mutation(&mut snapshot, &moved).expect("attach applies");
    assert_eq!(snapshot.bindings.iter().map(|entry| entry.folder.clone()).collect::<Vec<_>>(), vec![LocalFolderRef::Path { path: "/data/a-moved".into() }, LocalFolderRef::Path { path: "/data/b".into() }], "one binding per document, ordered by id");
    assert_eq!(moved.inverse(&base).expect("valid retained mutation inverse fixture"), vec![attach_local_folder(binding("a.fixture", "/data/a"))]);
}

#[test]
fn attaching_an_identical_binding_is_a_warned_no_op() {
    let base = LocalFolderBindings { bindings: vec![binding("a.fixture", "/data/a")] };
    let mut snapshot = base.clone();
    let raised = apply_local_folders_config_mutation_reporting(&mut snapshot, &attach_local_folder(binding("a.fixture", "/data/a")));
    assert_eq!(snapshot, base);
    assert_eq!(raised.iter().map(|(code, _)| code.as_str()).collect::<Vec<_>>(), vec!["mutation.no-op"]);
}

#[test]
fn the_bindings_round_trip_through_their_json_projection() {
    let bindings = LocalFolderBindings { bindings: vec![binding("a.fixture", "/data/a")] };
    assert_eq!(decode_local_folder_bindings_json(&encode_local_folder_bindings_json(&bindings)).expect("bindings decode"), bindings);
    assert_eq!(decode_local_folder_bindings_json("{\"bindings\":[]}").expect("empty bindings decode"), LocalFolderBindings::default());
    assert!(decode_local_folder_bindings_json("{\"bindings\":[{\"documentId\":\"a\",\"pluginId\":\"p\",\"appId\":\"q\",\"folder\":{\"kind\":\"handle\",\"handleId\":\"h\"}}]}").is_err(), "a folder reference names a path, nothing else");
}

#[test]
fn the_folder_is_edited_in_history_through_its_path_and_its_kind_is_no_input() {
    use protocol::MutationLeaf;
    let inputs = semio_framework::mutation_input_defs(AttachLocalFolder::PAYLOAD_SCHEMA, &|_: &str| None).expect("the payload schema derives every input");
    let folder = inputs.iter().find(|input| input.id == "/folder").expect("the folder input");
    let semio_framework::ArgSchema::Object { fields } = &folder.schema else { panic!("the folder is an object of fields: {:?}", folder.schema) };
    assert_eq!(fields.iter().map(|field| field.id.as_str()).collect::<Vec<_>>(), vec!["/path"], "the kind discriminates the reference and is never an input");
    assert_eq!(fields[0].control(), semio_framework::ActionArgControl::Text { placeholder: None }, "a path is retyped in a text field");
    let label = serde_json::to_string(&fields[0].label).expect("the label encodes");
    assert!(label.contains("\"Path\"") && label.contains("\"Pfad\""), "the path is labelled in English and German: {label}");
}
