use super::*;

#[test]
fn natural_file_route_profile_checks_refuse_decrypted_security_state() {
    use crate::standards::v1_7::subsets::{a, e, vt, x};
    use crate::standards::v1_7::subsets::base::{io, schema::snapshot::{PdfEncryption, PdfEncryptionAlgorithm}};
    use semio_framework_diagnostic::Severity;
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../🧫️fixtures/🔒️natural-file-profile-encryption/🔣️.json")).unwrap();
    let mut snapshot = io::decode_pdf(include_bytes!("../../../🧫️fixtures/✏️edit-existing-pdf/2️⃣two-pages.pdf")).unwrap();
    assert_eq!(fixture["algorithm"], "rc4-128");
    let password = fixture["password"].as_str().unwrap();
    snapshot.encryption = Some(PdfEncryption { algorithm: PdfEncryptionAlgorithm::Rc4_128, permissions: -1, user_password: password.into(), owner_password: None, encrypt_metadata: true });
    let encrypted = <Pdf17Editor as ArtifactEditor>::encode_natural_file(&snapshot).unwrap();
    assert_eq!(semio_s_artifact_stdio_pdf_test_oracle::standards::v1_7::subsets::base::encryption_present(&encrypted).unwrap(), fixture["encrypted"].as_bool().unwrap());
    assert!(<Pdf17Editor as ArtifactEditor>::decode_natural_file(&encrypted).is_err());
    let decoded = io::decode_pdf_with_password(&encrypted, password).unwrap();
    assert!(decoded.encryption.is_some());
    let checkers: [(&str, fn(&PdfSnapshot) -> Vec<semio_framework_diagnostic::Diagnostic>); 4] = [
        ("a", a::schema::check_pdf_a_conformance),
        ("x", x::schema::check_x_conformance),
        ("e", e::schema::check_e_conformance),
        ("vt", vt::schema::check_vt_conformance),
    ];
    for profile in fixture["profiles"].as_array().unwrap() {
        let id = profile["id"].as_str().unwrap();
        let code = profile["code"].as_str().unwrap();
        let checker = checkers.iter().find(|(candidate, _)| *candidate == id).unwrap().1;
        assert!(checker(&decoded).iter().any(|issue| issue.code.0 == code && matches!(issue.severity, Severity::Error | Severity::Fatal)), "{id} must refuse actual encrypted input after the security dictionary is lifted into typed state");
        let mut plain = decoded.clone();
        plain.encryption = None;
        assert!(!checker(&plain).iter().any(|issue| issue.code.0 == code), "{id} must not invent encryption for a plain retained document");
    }
}

#[test]
fn natural_file_route_matches_independent_rotation_and_retains_other_pdf_objects() {
    use semio_s_artifact_stdio_pdf_test_oracle::standards::v1_7::subsets::base as oracle;
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../🧫️fixtures/📄️natural-file-edit/🔣️.json")).unwrap();
    let input = include_bytes!("../../../🧫️fixtures/✏️edit-existing-pdf/2️⃣two-pages.pdf");
    let codec = <Pdf17Editor as ArtifactEditor>::natural_file_codec().expect("PDF editor declares paired natural IO");
    assert_eq!(codec.format_kind, fixture["codec"]["formatKind"].as_str().unwrap());
    assert_eq!(codec.extension, fixture["codec"]["extension"].as_str().unwrap());
    assert_eq!(codec.media_type, fixture["codec"]["mediaType"].as_str().unwrap());
    assert_eq!(codec.binary, fixture["codec"]["binary"].as_bool().unwrap());
    let mut snapshot = <Pdf17Editor as ArtifactEditor>::decode_natural_file(input).expect("natural PDF opens");
    let original = snapshot.clone();
    let index = fixture["mutation"]["params"]["index"].as_u64().unwrap() as usize;
    let rotation = fixture["mutation"]["params"]["rotation"].as_u64().unwrap() as u16;
    let mutation = PdfMutation::SetPageRotation(crate::schema::mutations::SetPageRotation { index, rotation });
    crate::schema::mutations::apply_pdf_mutation(&mut snapshot, &mutation);
    let bytes = <Pdf17Editor as ArtifactEditor>::encode_natural_file(&snapshot).expect("natural PDF saves");
    let reference_spec = semio_repo_test_host::parse_json(&fixture["mutation"].to_string()).unwrap();
    let reference = oracle::oracle_apply_mutation(input, &reference_spec).expect("lopdf independently edits the input");
    let observed = oracle::project_pdf_1_7(&bytes).expect("lopdf reads natural output");
    assert_eq!(observed, oracle::project_pdf_1_7(&reference).unwrap());
    let observed: serde_json::Value = serde_json::from_str(&observed.to_string()).unwrap();
    assert_eq!(observed["pageCount"], fixture["expected"]["pageCount"]);
    assert_eq!(observed["pages"][index]["rotate"], fixture["expected"]["rotation"]);
    let reopened = <Pdf17Editor as ArtifactEditor>::decode_natural_file(&bytes).expect("saved PDF reopens");
    assert_eq!(reopened.pages[index].rotate, i32::from(rotation));
    assert_eq!(reopened.pages[0].text(), original.pages[0].text());
    assert_eq!(reopened.info, original.info);
    let operation = <Pdf17Editor as ArtifactEditor>::whole_document_operation(reopened.clone()).expect("natural Open publishes one mutation");
    let mut fresh = <Pdf17Editor as ArtifactEditor>::initial_snapshot();
    crate::schema::mutations::apply_pdf_mutation(&mut fresh, &operation);
    assert_eq!(fresh, reopened);
    assert_eq!(snapshot.pages[0], original.pages[0], "an unedited page remains unchanged");
    assert!(<Pdf17Editor as ArtifactEditor>::decode_natural_file(fixture["invalidText"].as_str().unwrap().as_bytes()).is_err());
}

#[semio_framework_async_macros::async_test]
async fn natural_file_route_uses_registered_media_and_isolates_reopened_history() {
    use semio_framework_plugin::{artifact_app_laws, app::{MediaArtifact, MediaArtifactDescriptor}, EditorApp, MediaWireFormat, PluginApp, NATURAL_FILE_PORT};
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../🧫️fixtures/📄️natural-file-edit/🔣️.json")).unwrap();
    let input = include_bytes!("../../../🧫️fixtures/✏️edit-existing-pdf/2️⃣two-pages.pdf");
    let artifact = |data: Vec<u8>| MediaArtifact {
        descriptor: MediaArtifactDescriptor { edge_id: None, port_id: Some(NATURAL_FILE_PORT.into()), kind_id: Some("s.stdio.pdf@1.7".into()), media_type: None, wire: MediaWireFormat::Binary { format_kind: "s.stdio.pdf@1.7".into() }, blob_hash: None },
        data,
    };
    let mut app = artifact_app_laws::new_registered_app::<EditorApp<Pdf17Editor>, _>(async { semio_framework_plugin::App { definition: create_pdf17_editor(), examples: Vec::new() } }).await;
    app.consume_media(NATURAL_FILE_PORT, artifact(input.to_vec())).await.expect("registered PDF natural import");
    artifact_app_laws::settle_registered_typed_operation(&mut app, 1).await.unwrap();
    let opened = app.snapshot().unwrap().clone();
    let refused = match app.consume_media(NATURAL_FILE_PORT, artifact(fixture["invalidText"].as_str().unwrap().as_bytes().to_vec())).await {
        Err(_) => true,
        Ok(_) => artifact_app_laws::settle_registered_typed_operation(&mut app, 1).await.is_err(),
    };
    assert!(refused, "malformed natural input must be refused before publication");
    assert_eq!(app.snapshot().unwrap(), opened, "a refused natural import must preserve the document and history");
    let index = fixture["mutation"]["params"]["index"].as_u64().unwrap() as usize;
    let rotation = fixture["expected"]["rotation"].as_f64().unwrap();
    let args = semio_framework_value::DslValue::object([("page".into(), semio_framework_value::DslValue::float(index as f64)), ("x".into(), semio_framework_value::DslValue::float(rotation))]);
    app.handle_action("set-page-rotation", Some(&args), &artifact_app_laws::meta("local")).await.unwrap();
    artifact_app_laws::settle_registered_typed_operation(&mut app, 1).await.unwrap();
    let edited = app.snapshot().unwrap().clone();
    assert_eq!(edited.pages[index].rotate, rotation as i32);
    let saved = app.produce_media(NATURAL_FILE_PORT).await.expect("registered PDF natural save");
    assert!(saved.data.starts_with(b"%PDF-"));
    assert_eq!(saved.descriptor.port_id.as_deref(), Some(NATURAL_FILE_PORT));
    let projected = semio_s_artifact_stdio_pdf_test_oracle::standards::v1_7::subsets::base::project_pdf_1_7(&saved.data).unwrap();
    assert_eq!(serde_json::from_str::<serde_json::Value>(&projected.to_string()).unwrap()["pages"][index]["rotate"], fixture["expected"]["rotation"]);
    let mut reopened = artifact_app_laws::new_registered_app::<EditorApp<Pdf17Editor>, _>(async { semio_framework_plugin::App { definition: create_pdf17_editor(), examples: Vec::new() } }).await;
    reopened.bind_instance_id(2).await;
    reopened.consume_media(NATURAL_FILE_PORT, saved).await.expect("fresh owner imports exported bytes");
    artifact_app_laws::settle_registered_typed_operation(&mut reopened, 2).await.unwrap();
    let reopened_snapshot = reopened.snapshot().unwrap().clone();
    assert_eq!(reopened_snapshot.pages[index].rotate, rotation as i32);
    artifact_app_laws::settle_history_verb(&mut app, "undo", 1).await;
    assert_eq!(app.snapshot().unwrap(), opened);
    assert_eq!(reopened.snapshot().unwrap(), reopened_snapshot, "source Undo cannot change the reopened owner");
    artifact_app_laws::settle_history_verb(&mut app, "redo", 1).await;
    assert_eq!(app.snapshot().unwrap(), edited);
    artifact_app_laws::settle_history_verb(&mut reopened, "undo", 2).await;
    assert_eq!(app.snapshot().unwrap(), edited, "reopened Undo cannot change the source owner");
    artifact_app_laws::settle_history_verb(&mut reopened, "redo", 2).await;
    assert_eq!(reopened.snapshot().unwrap(), reopened_snapshot);
    artifact_app_laws::close_registered_fixture_app(&mut reopened);
    artifact_app_laws::close_registered_fixture_app(&mut app);
}

#[semio_framework_async_macros::async_test]
async fn create_pdf17_editor_builds_a_definition_for_the_editor_role() {
    let def = create_pdf17_editor();
    assert_eq!(def.role, semio_framework_plugin::AppRole::Editor);
    assert_eq!(def.dialect, PDF17_DIALECT.into());
}

#[semio_framework_async_macros::async_test]
async fn editor_dialect_matches_the_artifact_coordinate() {
    assert_eq!(<Pdf17Editor as ArtifactEditor>::DIALECT, PDF17_DIALECT);
}

#[semio_framework_async_macros::async_test]
async fn editor_declares_the_main_window() {
    let def = create_pdf17_editor();
    assert!(def.window_kinds.iter().any(|w| w.id == main::WINDOW_KIND_ID));
}

#[semio_framework_async_macros::async_test]
async fn missing_set_page_payload_is_rejected() {
    assert!(<Pdf17Editor as ArtifactEditor>::command_from_action("set-page", None).is_err());
}

#[semio_framework_async_macros::async_test]
async fn explicit_nonzero_page_payload_is_preserved() {
    let args = semio_framework_value::DslValue::Object(vec![
        ("page".into(), semio_framework_value::DslValue::float(3.0)),
        ("item".into(), semio_framework_value::DslValue::float(0.0)),
        ("revision".into(), semio_framework_value::DslValue::String("0123456789abcdef".into())),
        ("text".into(), semio_framework_value::DslValue::String("replacement".into())),
    ]);
    let command = <Pdf17Editor as ArtifactEditor>::command_from_action("set-page", Some(&args)).expect("typed payload");
    assert!(matches!(command, semio_s_artifact_stdio_contract::editing::SnapshotEditingCommand::Native(Pdf17EditorCommand::SetPage { page: 3, item: 0, revision, text }) if revision == "0123456789abcdef" && text == "replacement"));
}

#[semio_framework_async_macros::async_test]
async fn registered_pdf_draft_publishes_once_refuses_stale_and_undoes_redoes() {
    use crate::schema::snapshot::{PdfOp, PdfPage, PdfTextString};
    use semio_framework_plugin::app::DocumentWindowKit;
    use semio_framework_plugin::{artifact_app_laws, EditorApp, PluginApp};

    let mut original = PdfSnapshot::default();
    original.pages.push(PdfPage { content: vec![PdfOp::BeginText, PdfOp::ShowText { text: PdfTextString::text("before") }, PdfOp::EndText], ..Default::default() });
    let mut app = artifact_app_laws::new_registered_app::<EditorApp<Pdf17Editor>, _>(async { semio_framework_plugin::App { definition: create_pdf17_editor(), examples: Vec::new() } }).await;
    let semio_framework_plugin::Effect::LoadDocument { pack, spr } = semio_s_artifact_stdio_contract::load_example_effect(&original, STDIO_PDF17_DOCUMENT_SCHEMA) else { panic!("PDF fixture produces a document load") };
    semio_framework_plugin::artifact_app_laws::load_document(&mut app, &store::ArtifactPackFiles { pack, spr, ops: String::new() }).await.unwrap();
    let loaded = app.snapshot().unwrap().clone();
    assert_eq!(loaded.pages[0].text(), "before");
    let meta = artifact_app_laws::meta("local");
    let arguments = |revision: String, text: &str| {
        semio_framework_value::DslValue::object([("page".into(), semio_framework_value::DslValue::float(0.0)), ("item".into(), semio_framework_value::DslValue::float(0.0)), ("revision".into(), semio_framework_value::DslValue::String(revision)), ("text".into(), semio_framework_value::DslValue::String(text.into()))])
    };

    let edit = arguments(DocumentWindowKit::text_revision("before"), "after");
    app.handle_action("set-page", Some(&edit), &meta).await.unwrap();
    artifact_app_laws::settle_registered_typed_operation(&mut app, meta.instance_id).await.unwrap();
    let expected = app.snapshot().unwrap().clone();
    assert_eq!(expected.pages[0].text(), "after");

    let no_op = arguments(DocumentWindowKit::text_revision("after"), "after");
    app.handle_action("set-page", Some(&no_op), &meta).await.unwrap();
    artifact_app_laws::settle_registered_typed_operation(&mut app, meta.instance_id).await.unwrap();
    artifact_app_laws::settle_history_verb(&mut app, "undo", meta.instance_id).await;
    assert_eq!(app.snapshot().unwrap(), loaded, "an identical Apply must not add a history entry");
    artifact_app_laws::settle_history_verb(&mut app, "redo", meta.instance_id).await;
    assert_eq!(app.snapshot().unwrap(), expected);

    let stale = arguments(DocumentWindowKit::text_revision("before"), "refused draft");
    app.handle_action("set-page", Some(&stale), &meta).await.unwrap();
    assert!(artifact_app_laws::settle_registered_typed_operation(&mut app, meta.instance_id).await.is_err());
    assert_eq!(app.snapshot().unwrap(), expected, "a refused draft must preserve every PDF page operator");
    artifact_app_laws::close_registered_fixture_app(&mut app);
}

#[semio_framework_async_macros::async_test]
async fn registered_pdf_page_edit_publishes_and_undoes_redoes() {
    use crate::schema::snapshot::PdfOp;
    use semio_framework_plugin::{artifact_app_laws, EditorApp, PluginApp};

    let mut original = crate::standards::v1_7::subsets::base::schema::snapshot::demo_pdf17_snapshot();
    let mut app = artifact_app_laws::new_registered_app::<EditorApp<Pdf17Editor>, _>(async { semio_framework_plugin::App { definition: create_pdf17_editor(), examples: Vec::new() } }).await;
    let semio_framework_plugin::Effect::LoadDocument { pack, spr } = semio_s_artifact_stdio_contract::load_example_effect(&original, STDIO_PDF17_DOCUMENT_SCHEMA) else { panic!("PDF fixture produces a document load") };
    semio_framework_plugin::artifact_app_laws::load_document(&mut app, &store::ArtifactPackFiles { pack, spr, ops: String::new() }).await.unwrap();
    let loaded = app.snapshot().unwrap().clone();
    let meta = artifact_app_laws::meta("local");
    let insert = semio_framework_value::DslValue::object([
        ("page".into(), semio_framework_value::DslValue::float(0.0)),
        ("x".into(), semio_framework_value::DslValue::float(12.0)),
        ("y".into(), semio_framework_value::DslValue::float(24.0)),
        ("width".into(), semio_framework_value::DslValue::float(30.0)),
        ("height".into(), semio_framework_value::DslValue::float(16.0)),
        ("red".into(), semio_framework_value::DslValue::float(0.2)),
        ("green".into(), semio_framework_value::DslValue::float(0.4)),
        ("blue".into(), semio_framework_value::DslValue::float(0.6)),
    ]);
    app.handle_action("insert-rectangle", Some(&insert), &meta).await.unwrap();
    artifact_app_laws::settle_registered_typed_operation(&mut app, meta.instance_id).await.unwrap();
    let edited = app.snapshot().unwrap().clone();
    assert!(edited.pages[0].content.iter().any(|op| matches!(op, PdfOp::Rectangle { x, y, width, height } if (*x - 12.0).abs() < 0.01 && (*y - 24.0).abs() < 0.01 && (*width - 30.0).abs() < 0.01 && (*height - 16.0).abs() < 0.01)));
    assert!(edited.pages[0].text().contains("Semio"));
    artifact_app_laws::settle_history_verb(&mut app, "undo", meta.instance_id).await;
    assert_eq!(app.snapshot().unwrap(), loaded);
    artifact_app_laws::settle_history_verb(&mut app, "redo", meta.instance_id).await;
    assert_eq!(app.snapshot().unwrap(), edited);
    artifact_app_laws::close_registered_fixture_app(&mut app);
}

semio_framework_plugin::history_edit_acceptance_law!("stdio", Pdf17Editor, || semio_framework_plugin::App { definition: create_pdf17_editor(), examples: Vec::new() }, "../..");
