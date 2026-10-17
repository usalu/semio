/// 📜️ Editor scene document laws: the canonical scene corpus, the contract scene conversion and the typed scene the editor host takes.

//#region ✍️EditorSceneDocumentLaws
/// 🧫️ The editor engine's own canonical scene corpus (`framework.editor.canonical-scene/v1`).
static EDITOR_CANONICAL_SCENE_CORPUS: std::sync::LazyLock<&'static str> = std::sync::LazyLock::new(|| fixture_text!("../../../../../../../../../../🔨️modules/✍️editor/🧬️scene/🧪️tests/🧫️fixtures/🔣️.json"));

/// 🔁️ The UI contract scene that carries one canonical scene's members the way a producer publishes them — each as an
/// encoded `*Json` string — with the host's own settings members beside the engine's.
fn contract_scene_of(canonical: &Value) -> TextEditorScene {
    let text = |member: &str| canonical.get(member).map(Value::to_string);
    let mut scene = TextEditorScene::base(canonical.get("buffer").and_then(Value::as_str).unwrap_or_default().to_string(), None, text("selection"));
    scene.tokens_json = text("tokens");
    scene.diagnostics_json = text("diagnostics");
    scene.placeholders_json = text("placeholders");
    scene.occurrences_json = canonical.get("occurrences").map(|occurrences| json!({ "hover": occurrences.get("hover").map(Value::to_string), "selection": occurrences.get("selection").map(Value::to_string) }).to_string());
    scene.extra_carets_json = text("extraCarets");
    scene.selectable_spans_json = text("selectableSpans");
    scene.settings_json = canonical.get("settings").map(|settings| {
        let mut published = settings.clone();
        published["readOnly"] = json!(false);
        published["commit"] = json!("explicit");
        published.to_string()
    });
    scene.camera_json = canonical.get("camera").map(|camera| json!({ "x": 0, "y": camera["y"], "zoom": 1 }).to_string());
    scene.overlays_json = text("overlays");
    scene.hover_json = canonical.get("hover").map(|hover| if hover["kind"] == "clear" { "null".to_string() } else { json!({ "start": hover["start"], "end": hover["end"] }).to_string() });
    scene
}

/// ⚖️ Two JSON values that say the same thing: numbers by value (`18` is `18.0`), everything else exactly.
fn same_json(left: &Value, right: &Value) -> bool {
    match (left, right) {
        (Value::Number(left), Value::Number(right)) => left.as_f64() == right.as_f64(),
        (Value::Array(left), Value::Array(right)) => left.len() == right.len() && left.iter().zip(right).all(|(left, right)| same_json(left, right)),
        (Value::Object(left), Value::Object(right)) => left.len() == right.len() && left.iter().all(|(key, value)| right.get(key).is_some_and(|other| same_json(value, other))),
        _ => left == right,
    }
}

/// ⚖️ LAW (the editor engine's canonical scene corpus): for every scene the corpus accepts, the UI contract scene that
/// carries its members as `*Json` strings builds exactly that canonical document — the host's own settings members left
/// out — and the engine decodes it; a scene that only echoes the host's edit builds the same document without buffer and
/// selection.
#[test]
fn the_contract_scene_builds_the_editor_engines_canonical_scene() {
    let corpus: Value = serde_json::from_str((*EDITOR_CANONICAL_SCENE_CORPUS)).expect("the canonical scene corpus parses");
    let cases: Vec<&Value> = corpus["cases"].as_array().expect("cases").iter().filter(|case| case["accepted"] == true).collect();
    assert!(cases.len() >= 6, "the corpus accepts scenes with a buffer, a selection, adornments, a hover and settings");
    for case in cases {
        let id = case["id"].as_str().expect("case id");
        let scene = contract_scene_of(&case["input"]);
        let (document, faults) = text_editor_scene_document(&scene, true);
        assert_eq!(faults, Vec::<String>::new(), "{id}: every member is read");
        let mut expected = case["input"].clone();
        if expected.get("buffer").is_none() {
            expected["buffer"] = json!("");
        }
        let built: Value = serde_json::from_str(&document).expect("the document is JSON");
        assert!(same_json(&built, &expected), "{id}: built {built} for {expected}");
        let mut admitted = |_| true;
        let mut control = semio_framework_value::NativeDecodeControl::new(TEXT_EDITOR_SCENE_OWNED_BYTES, &mut admitted);
        assert!(framework_editor::scene::from_json(&document, &mut control).is_ok(), "{id}: the engine decodes the built document");
        let (echo, _) = text_editor_scene_document(&scene, false);
        let echo: Value = serde_json::from_str(&echo).expect("the echo document is JSON");
        let mut without_text = expected.clone();
        without_text.as_object_mut().expect("document object").retain(|member, _| member != "buffer" && member != "selection");
        assert!(same_json(&echo, &without_text), "{id}: an echo carries neither buffer nor selection: {echo}");
    }
}

/// ⚖️ LAW: the host commits a scene through the typed document — its buffer and selection reach the engine, an echo of
/// the host's own edit leaves the local draft and caret alone, and a member this host cannot read is named, left
/// unchanged and never stops the rest of the scene.
#[test]
fn the_editor_host_takes_the_typed_scene_and_names_what_it_cannot_read() {
    let mut host = EditorHost::default();
    let scene = TextEditorScene::base("alpha".into(), None, Some(json!({ "start": 1, "end": 3 }).to_string()));
    synchronize_text_editor_host(&mut host, &scene, true).expect("the scene is committed");
    assert_eq!((host.text(), host.anchor(), host.caret()), ("alpha", 1, 3), "buffer and selection reach the engine");
    let echo = TextEditorScene::base("omega".into(), None, Some(json!({ "start": 0, "end": 0 }).to_string()));
    synchronize_text_editor_host(&mut host, &echo, false).expect("an echo is committed");
    assert_eq!((host.text(), host.anchor(), host.caret()), ("alpha", 1, 3), "an echo leaves the local draft and its caret");
    let mut torn = TextEditorScene::base("beta".into(), None, None);
    torn.tokens_json = Some("{".into());
    let (document, faults) = text_editor_scene_document(&torn, true);
    assert_eq!(faults.len(), 1, "the unreadable member is named: {faults:?}");
    assert!(faults[0].starts_with("tokensJson: "), "by its contract name: {faults:?}");
    assert!(!document.contains("tokens"), "and left out of the document: {document}");
    synchronize_text_editor_host(&mut host, &torn, true).expect("the rest of the scene is committed");
    assert_eq!(host.text(), "beta");
    let mut cleared = TextEditorScene::base("beta".into(), None, None);
    cleared.hover_json = Some("null".into());
    assert!(text_editor_scene_document(&cleared, true).0.contains("\"hover\":{\"kind\":\"clear\"}"), "a null hover clears");
}
//#endregion ✍️EditorSceneDocumentLaws
