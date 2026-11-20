use crate::editor::pdf17::modes::edit;
use crate::editor::pdf17::modes::edit::windows::main;
use crate::standards::v1_7::subsets::base::schema::mutations::PdfMutation;
use crate::standards::v1_7::subsets::base::schema::snapshot::{PdfSnapshot, STDIO_PDF17_DOCUMENT_SCHEMA};
use semio_plugin::{ArtifactEditor, Editor};
use semio_framework_test_fixtures::{fixture_bytes, fixture_text};
#[allow(unused_imports)]
use crate::standards::v1_7::subsets::base::schema::PDF_ARTIFACT_SCHEMA_ID;
#[allow(unused_imports)]
use {semio_plugin::built_to_component_tree, semio_plugin::ArtifactView, semio_plugin::ComponentTree, semio_plugin::ConfigView, semio_framework_artifact_reference::Dialect, semio_plugin::DraftView, semio_plugin::Emit, semio_plugin::Fault, semio_plugin::NoConfig, semio_plugin::NoConfigMutation, semio_plugin::NoDraft, semio_plugin::NoDraftMutation, semio_plugin::NoPresence, semio_plugin::NoPresenceMutation, semio_plugin::NoTransient, semio_plugin::NoTransientMutation, semio_framework_artifact_reference::StandardId, semio_framework_artifact_reference::SubsetId};
#[allow(unused_imports)]
use semio_framework_2d_compute::EngineHandles;
use crate::editor::pdf17::*;

#[test]
fn natural_file_route_profile_checks_refuse_decrypted_security_state() {
    use crate::standards::v1_7::subsets::{a, e, vt, x};
    use crate::standards::v1_7::subsets::base::io;
    use crate::standards::v1_7::subsets::base::schema::snapshot::{PdfEncryption, PdfEncryptionAlgorithm};
    use semio_framework_diagnostic::Severity;
    let fixture: serde_json::Value = serde_json::from_str(fixture_text!("../../../../../../🧪️tests/🧫️fixtures/🔒️natural-file-profile-encryption/🔣️.json")).unwrap();
    let mut snapshot = io::import::decode_pdf(fixture_bytes!("../../../../../../🧪️tests/🧫️fixtures/✏️edit-existing-pdf/2️⃣two-pages.pdf")).unwrap();
    assert_eq!(fixture["algorithm"], "rc4-128");
    let password = fixture["password"].as_str().unwrap();
    snapshot.encryption = Some(PdfEncryption { algorithm: PdfEncryptionAlgorithm::Rc4_128, permissions: -1, user_password: password.into(), owner_password: None, encrypt_metadata: true });
    let encrypted = <Pdf17Editor as ArtifactEditor>::encode_natural_file(&snapshot).unwrap();
    assert_eq!(semio_s_artifact_stdio_pdf_test_oracle::standards::v1_7::subsets::base::encryption_present(&encrypted).unwrap(), fixture["encrypted"].as_bool().unwrap());
    assert!(<Pdf17Editor as ArtifactEditor>::decode_natural_file(&encrypted).is_err());
    let decoded = io::import::decode_pdf_with_password(&encrypted, password).unwrap();
    assert!(decoded.encryption.is_some());
    let checkers: [(&str, fn(&PdfSnapshot) -> Vec<semio_framework_diagnostic::Diagnostic>); 4] = [
        ("a", a::io::check_pdf_a_conformance),
        ("x", x::io::check_x_conformance),
        ("e", e::io::check_e_conformance),
        ("vt", vt::io::check_vt_conformance),
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

async fn open_registered_pdf(input: Vec<u8>, page_count: usize, scope: &mut semio_plugin::artifact_app_laws::OriginalFixtureScope) -> semio_plugin::VcsArtifactApp<semio_plugin::EditorApp<Pdf17Editor>> {
    use semio_plugin::{artifact_app_laws as laws, EditorApp};
    use semio_plugin::app::{MediaArtifact, MediaArtifactDescriptor, MediaWireFormat};
    let policy = semio_s_artifact_stdio_contract::testing::mounted_policy();
    let meta = laws::meta("local");
    let codec = <Pdf17Editor as ArtifactEditor>::natural_file_codec().unwrap();
    let mut app = semio_plugin::with_authoring_identity!(|identity| laws::new_registered_app::<EditorApp<Pdf17Editor>, _>(
        async { semio_plugin::App { definition: create_pdf17_editor(), examples: Vec::new() } },
        semio_os_kernel::ActorId(semio_os_kernel::LOCAL_ACTOR_ID.into()), policy, &mut identity
    ).await);
    let artifact = MediaArtifact {
        descriptor: MediaArtifactDescriptor {
            edge_id: None, port_id: Some(semio_plugin::NATURAL_FILE_PORT.into()),
            kind_id: Some(codec.format_kind.into()), media_type: None, blob_hash: None,
            wire: MediaWireFormat::Binary { format_kind: codec.format_kind.into() },
        },
        data: input,
    };
    laws::consume_media(&mut app, semio_plugin::NATURAL_FILE_PORT, artifact, scope.cx()).await.unwrap();
    let receipt = semio_plugin::with_authoring_identity!(|identity| laws::settle_registered_typed_operation(
        &mut app, meta.instance_id, policy, &mut identity, scope.cx()
    ).await).unwrap();
    assert_eq!(receipt.completions, 1, "the mounted PDF import settles once");
    assert!(matches!(receipt.ui_scope, Some(semio_plugin::kernel::UiDirtyScope::Full)));
    assert!(!receipt.effects.iter().any(|effect| matches!(effect, semio_plugin::Effect::LoadDocument { .. })));
    assert_eq!(app.snapshot().expect("the mounted PDF is ready").pages.len(), page_count);
    app
}

#[semio_framework_async_macros::async_test]
async fn natural_file_route_matches_independent_rotation_and_retains_other_pdf_objects() {
    use semio_s_artifact_stdio_pdf_test_oracle::standards::v1_7::subsets::base as oracle;
    let fixture: serde_json::Value = serde_json::from_str(fixture_text!("../../../../../../🧪️tests/🧫️fixtures/📄️natural-file-edit/🔣️.json")).unwrap();
    let input = fixture_bytes!("../../../../../../🧪️tests/🧫️fixtures/✏️edit-existing-pdf/2️⃣two-pages.pdf");
    let codec = <Pdf17Editor as ArtifactEditor>::natural_file_codec().expect("PDF editor declares paired natural IO");
    assert_eq!(codec.format_kind, fixture["codec"]["formatKind"].as_str().unwrap());
    assert_eq!(codec.extension, fixture["codec"]["extension"].as_str().unwrap());
    assert_eq!(codec.media_type, fixture["codec"]["mediaType"].as_str().unwrap());
    assert_eq!(codec.binary, fixture["codec"]["binary"].as_bool().unwrap());
    use semio_plugin::{artifact_app_laws as laws, PluginApp};
    let meta = laws::meta("local");
    let policy = semio_s_artifact_stdio_contract::testing::mounted_policy();
    let mut scope = laws::OriginalFixtureScope::admit();
    let mut app = open_registered_pdf(input.to_vec(), 2, &mut scope).await;
    let original = app.snapshot().unwrap().clone();
    let action = &fixture["action"];
    let args = semio_framework_value::DslValue::object([
        ("page".into(), semio_framework_value::DslValue::float(action["page"].as_f64().unwrap())),
        ("x".into(), semio_framework_value::DslValue::float(action["x"].as_f64().unwrap())),
    ]);
    semio_plugin::with_authoring_identity!(|identity| laws::dispatch_fixture_action(
        &mut app, action["id"].as_str().unwrap(), Some(args.clone()), &meta, &mut identity, scope.cx(), policy.close
    ).await).unwrap();
    semio_plugin::with_authoring_identity!(|identity| laws::settle_registered_typed_operation(
        &mut app, meta.instance_id, policy, &mut identity, scope.cx()
    ).await).unwrap();
    let snapshot = app.snapshot().unwrap();
    let index = action["page"].as_u64().unwrap() as usize;
    let rotation = action["x"].as_u64().unwrap() as u16;
    assert_eq!(snapshot.pages[0], original.pages[0], "an unedited page remains unchanged");
    let artifact = semio_s_artifact_stdio_contract::testing::with_encoding_async(async |owner| {
        PluginApp::produce_media(&mut app, semio_plugin::NATURAL_FILE_PORT, owner, scope.cx()).await
    }).await.expect("registered PDF editor saves").expect("natural PDF media is present");
    assert_eq!(artifact.descriptor.kind_id.as_deref(), Some(codec.format_kind));
    let bytes = artifact.data;
    let reference_spec = semio_repo_test_host::parse_json(&fixture["mutation"].to_string()).unwrap();
    let reference = oracle::oracle_apply_mutation(input, &reference_spec).expect("lopdf independently edits the input");
    let observed = oracle::project_pdf_1_7(&bytes).expect("lopdf reads natural output");
    assert_eq!(observed, oracle::project_pdf_1_7(&reference).unwrap());
    let observed: serde_json::Value = serde_json::from_str(&observed.to_string()).unwrap();
    assert_eq!(observed["pageCount"], fixture["expected"]["pageCount"]);
    assert_eq!(observed["pages"][index]["rotate"], fixture["expected"]["rotation"]);
    let mut reopened_app = open_registered_pdf(bytes, 2, &mut scope).await;
    let reopened = reopened_app.snapshot().unwrap();
    assert_eq!(reopened.pages[index].rotate, i32::from(rotation));
    assert_eq!(reopened.pages[0].text(), original.pages[0].text());
    assert_eq!(reopened.info, original.info);
    assert!(<Pdf17Editor as ArtifactEditor>::decode_natural_file(fixture["invalidText"].as_str().unwrap().as_bytes()).is_err());
    laws::close_registered_fixture_app(&mut reopened_app, policy, scope.cx()).expect("reopened editor closes");
    laws::close_registered_fixture_app(&mut app, policy, scope.cx()).expect("registered PDF editor closes");
}


fn large_pdf(spec: &serde_json::Value) -> Vec<u8> {
    use std::io::Write;
    let pages = spec["pageCount"].as_u64().unwrap() as usize;
    let lines = spec["segmentCount"].as_u64().unwrap() as usize;
    let font = spec.get("font");
    let first_page = if font.is_some() { 5 } else { 3 };
    let page_stride = if font.is_some() { 3 } else { 2 };
    let kids = (0..pages).map(|page| format!("{} 0 R", first_page + page * page_stride)).collect::<Vec<_>>().join(" ");
    let mut rows = vec![
        b"<< /Type /Catalog /Pages 2 0 R >>".to_vec(),
        format!("<< /Type /Pages /Kids [{kids}] /Count {pages} >>").into_bytes(),
    ];
    if let Some(font) = font {
        let face = font["face"].as_str().unwrap();
        let encoding = font["encoding"].as_str().unwrap();
        let name = font["name"].as_str().unwrap();
        rows.push(format!("<< /Type /Font /Subtype /Type1 /BaseFont /{face} /Encoding /{encoding} >>").into_bytes());
        rows.push(format!("<< /Font << /{name} 3 0 R >> >>").into_bytes());
    }
    for page in 0..pages {
        let mut content = b"q 0.2 0.3 0.4 RG 0.5 w\n".to_vec();
        for index in 0..lines {
            let at = 20 + index * 6;
            writeln!(content, "{at} 40 m {at} {} l", 400 + page).unwrap();
        }
        content.extend_from_slice(b"S Q\n");
        let contents = rows.len() + 2;
        let contents_and_resources = if font.is_some() {
            format!("/Resources 4 0 R /Contents [{contents} 0 R {} 0 R]", contents + 1)
        } else {
            format!("/Contents {contents} 0 R")
        };
        rows.push(format!("<< /Type /Page /Parent 2 0 R /MediaBox [0 0 360 480] {contents_and_resources} >>").into_bytes());
        if let Some(font) = font {
            let name = font["name"].as_str().unwrap();
            let label = pdf_page_label(spec, page);
            rows.push(pdf_fixture_stream(format!("BT /{name} 11 Tf 48 416 Td ({label}) Tj ET\n").as_bytes()));
        }
        rows.push(pdf_fixture_stream(&content));
    }
    let mut output = b"%PDF-1.7\n".to_vec();
    let positions: Vec<_> = rows.into_iter().enumerate().map(|(index, row)| {
        let position = output.len();
        writeln!(output, "{} 0 obj", index + 1).unwrap();
        output.extend(row);
        output.extend_from_slice(b"\nendobj\n");
        position
    }).collect();
    let offset = output.len();
    let size = positions.len() + 1;
    write!(output, "xref\n0 {size}\n0000000000 65535 f \n").unwrap();
    for position in positions { writeln!(output, "{position:010} 00000 n ").unwrap(); }
    write!(output, "trailer\n<< /Size {size} /Root 1 0 R >>\nstartxref\n{offset}\n%%EOF\n").unwrap();
    output
}

#[semio_framework_async_macros::async_test]
async fn natural_file_route_opens_large_retained_vector_document_and_reexports() {
    use crate::standards::v1_7::subsets::base::{io::import::decode_pdf, schema::snapshot::PdfOp};
    use semio_plugin::{artifact_app_laws as laws, PluginApp};
    use semio_s_artifact_stdio_pdf_test_oracle::standards::v1_7::subsets::base::project_pdf_1_7;
    let fixtures: serde_json::Value = serde_json::from_str(fixture_text!("../../../../../../🧪️tests/🧫️fixtures/📄️natural-file-edit/🔣️.json")).unwrap();
    let spec = &fixtures["largeVector"];
    let input = large_pdf(spec);
    assert_eq!(input.len(), spec["byteLength"].as_u64().unwrap() as usize);
    let pages = spec["pageCount"].as_u64().unwrap() as usize;
    let lines = spec["segmentCount"].as_u64().unwrap() as usize;
    let reference = decode_pdf(&input).expect("the cold PDF reader projects the fixture");
    let oracle: serde_json::Value = serde_json::from_str(&project_pdf_1_7(&input).unwrap().to_string()).unwrap();
    assert_eq!(oracle["pageCount"].as_u64().unwrap() as usize, pages);
    let mut scope = laws::OriginalFixtureScope::admit();
    let mut app = open_registered_pdf(input, pages, &mut scope).await;
    let snapshot = app.snapshot().unwrap();
    assert_eq!(snapshot.pages, reference.pages);
    assert_eq!(snapshot.objects, reference.objects);
    assert_eq!(snapshot.trailer, reference.trailer);
    assert_eq!(snapshot.admitted_stream_roles, reference.admitted_stream_roles);
    assert_eq!(snapshot.pages.last().unwrap().content.iter().filter(|operator| matches!(operator, PdfOp::LineTo { .. })).count(), lines);
    let artifact = semio_s_artifact_stdio_contract::testing::with_encoding_async(async |owner| {
        PluginApp::produce_media(&mut app, semio_plugin::NATURAL_FILE_PORT, owner, scope.cx()).await
    }).await.expect("controlled PDF reexports").expect("natural PDF output is present");
    let saved: serde_json::Value = serde_json::from_str(&project_pdf_1_7(&artifact.data).unwrap().to_string()).unwrap();
    assert_eq!(saved["pageCount"].as_u64().unwrap() as usize, pages);
    assert_eq!(decode_pdf(&artifact.data).unwrap().pages, reference.pages);
    laws::close_registered_fixture_app(&mut app, semio_s_artifact_stdio_contract::testing::mounted_policy(), scope.cx()).expect("large editor closes");
}

fn pdf_fixture_stream(content: &[u8]) -> Vec<u8> {
    let mut stream = format!("<< /Length {} >>\nstream\n", content.len()).into_bytes();
    stream.extend_from_slice(content);
    stream.extend_from_slice(b"endstream");
    stream
}

fn pdf_page_label(spec: &serde_json::Value, index: usize) -> String {
    let label = spec["labelPrefix"].as_str().unwrap();
    let pages = spec["pageCount"].as_u64().unwrap();
    format!("{label} {:02} of {pages}", index + 1)
}

#[semio_framework_async_macros::async_test]
async fn natural_file_route_opens_large_retained_type1_document_and_reexports() {
    use crate::standards::v1_7::subsets::base::{io::import::decode_pdf, schema::snapshot::PdfFontKind};
    use semio_plugin::{artifact_app_laws as laws, PluginApp};
    use semio_s_artifact_stdio_pdf_test_oracle::standards::v1_7::subsets::base::project_pdf_1_7;

    let corpus: serde_json::Value = serde_json::from_str(fixture_text!("../../../../../../🧪️tests/🧫️fixtures/📄️natural-file-edit/🔣️.json")).unwrap();
    let spec = &corpus["largeType1"];
    let pages = spec["pageCount"].as_u64().unwrap() as usize;
    let input = large_pdf(spec);
    assert!(input.len() > spec["minimumByteLength"].as_u64().unwrap() as usize);
    let reference = decode_pdf(&input).unwrap();
    let check_independent_text = |bytes: &[u8]| {
        let witness: serde_json::Value = serde_json::from_str(&project_pdf_1_7(bytes).unwrap().to_string()).unwrap();
        let actual = witness["pages"].as_array().unwrap();
        assert_eq!(actual.len(), pages);
        for (index, page) in actual.iter().enumerate() {
            assert_eq!(page["text"].as_array().unwrap().as_slice(), &[serde_json::Value::String(pdf_page_label(spec, index))]);
        }
    };
    check_independent_text(&input);
    let mut scope = laws::OriginalFixtureScope::admit();
    let mut app = open_registered_pdf(input, pages, &mut scope).await;
    let snapshot = app.snapshot().unwrap();
    assert_eq!(snapshot.pages, reference.pages);
    assert_eq!(snapshot.fonts, reference.fonts);
    assert_eq!(snapshot.objects, reference.objects);
    assert_eq!(snapshot.trailer, reference.trailer);
    assert_eq!(snapshot.admitted_stream_roles, reference.admitted_stream_roles);
    let [font] = snapshot.fonts.as_slice() else { panic!("expected the shared Type1 font"); };
    assert_eq!(font.id, spec["font"]["name"].as_str().unwrap());
    assert!(matches!(&font.kind, PdfFontKind::Type1 { base_font, .. } if base_font == spec["font"]["face"].as_str().unwrap()));
    for (index, page) in snapshot.pages.iter().enumerate() {
        assert_eq!(page.text().trim(), pdf_page_label(spec, index));
    }
    let artifact = semio_s_artifact_stdio_contract::testing::with_encoding_async(async |owner| {
        PluginApp::produce_media(&mut app, semio_plugin::NATURAL_FILE_PORT, owner, scope.cx()).await
    }).await.expect("controlled text PDF reexports").expect("natural PDF output is present");
    check_independent_text(&artifact.data);
    assert_eq!(decode_pdf(&artifact.data).unwrap().pages, reference.pages);
    laws::close_registered_fixture_app(&mut app, semio_s_artifact_stdio_contract::testing::mounted_policy(), scope.cx()).expect("large text editor closes");
}

#[semio_framework_async_macros::async_test]
async fn create_pdf17_editor_builds_a_definition_for_the_editor_role() {
    let def = create_pdf17_editor();
    assert_eq!(def.role, semio_plugin::AppRole::Editor);
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
    assert!(<Pdf17Editor as ArtifactEditor>::command_from_action("set-page", &mut None, &mut None).is_err());
}

#[semio_framework_async_macros::async_test]
async fn explicit_nonzero_page_payload_is_preserved() {
    let args = semio_framework_value::DslValue::Object(vec![
        ("page".into(), semio_framework_value::DslValue::float(3.0)),
        ("item".into(), semio_framework_value::DslValue::float(0.0)),
        ("revision".into(), semio_framework_value::DslValue::String("0123456789abcdef".into())),
        ("text".into(), semio_framework_value::DslValue::String("replacement".into())),
    ]);
    let command = <Pdf17Editor as ArtifactEditor>::command_from_action("set-page", &mut Some(args), &mut None).expect("typed payload");
    assert!(matches!(command, semio_s_artifact_stdio_contract::editing::SnapshotEditingCommand::Native(Pdf17EditorCommand::SetPage { page: 3, item: 0, revision, text }) if revision == "0123456789abcdef" && text == "replacement"));
}

#[semio_framework_async_macros::async_test]
async fn registered_pdf_draft_publishes_once_refuses_stale_and_undoes_redoes() {
    use crate::standards::v1_7::subsets::base::schema::snapshot::{PdfOp, PdfPage, PdfTextString};
    use semio_plugin::app::DocumentWindowKit;
    use semio_plugin::{artifact_app_laws, EditorApp, PluginApp};

    let mut original = PdfSnapshot::default();
    original.pages.push(PdfPage { content: vec![PdfOp::BeginText, PdfOp::ShowText { text: PdfTextString::text("before") }, PdfOp::EndText], ..Default::default() });
    let mut scope = semio_plugin::artifact_app_laws::OriginalFixtureScope::admit();
    let mut app = semio_plugin::with_authoring_identity!(|identity| artifact_app_laws::new_registered_app::<EditorApp<Pdf17Editor>, _>(async { semio_plugin::App { definition: create_pdf17_editor(), examples: Vec::new() } }, semio_os_kernel::ActorId(semio_os_kernel::LOCAL_ACTOR_ID.into()), semio_s_artifact_stdio_contract::testing::mounted_policy(), &mut identity).await);
    let semio_plugin::Effect::LoadDocument { pack, spr } = semio_s_artifact_stdio_contract::load_example_effect(&original, STDIO_PDF17_DOCUMENT_SCHEMA) else { panic!("PDF fixture produces a document load") };
    semio_plugin::with_authoring_identity!(|identity| semio_plugin::artifact_app_laws::load_document(&mut app, &store::os_store::serialization::ArtifactPackFiles { pack, spr, ops: String::new() }, &mut identity, scope.cx()).await).unwrap();
    let loaded = app.snapshot().unwrap().clone();
    assert_eq!(loaded.pages[0].text(), "before");
    let meta = artifact_app_laws::meta("local");
    let arguments = |revision: String, text: &str| {
        semio_framework_value::DslValue::object([("page".into(), semio_framework_value::DslValue::float(0.0)), ("item".into(), semio_framework_value::DslValue::float(0.0)), ("revision".into(), semio_framework_value::DslValue::String(revision)), ("text".into(), semio_framework_value::DslValue::String(text.into()))])
    };

    let edit = arguments(DocumentWindowKit::text_revision("before"), "after");
    semio_plugin::with_authoring_identity!(|identity| semio_plugin::artifact_app_laws::dispatch_fixture_action(&mut app, "set-page", Some(edit.clone()), &meta, &mut identity, scope.cx(), semio_s_artifact_stdio_contract::testing::mounted_policy().close).await).unwrap();
    semio_plugin::with_authoring_identity!(|identity| artifact_app_laws::settle_registered_typed_operation(&mut app, meta.instance_id, semio_s_artifact_stdio_contract::testing::mounted_policy(), &mut identity, scope.cx()).await).unwrap();
    let expected = app.snapshot().unwrap().clone();
    assert_eq!(expected.pages[0].text(), "after");

    let no_op = arguments(DocumentWindowKit::text_revision("after"), "after");
    semio_plugin::with_authoring_identity!(|identity| semio_plugin::artifact_app_laws::dispatch_fixture_action(&mut app, "set-page", Some(no_op.clone()), &meta, &mut identity, scope.cx(), semio_s_artifact_stdio_contract::testing::mounted_policy().close).await).unwrap();
    semio_plugin::with_authoring_identity!(|identity| artifact_app_laws::settle_registered_typed_operation(&mut app, meta.instance_id, semio_s_artifact_stdio_contract::testing::mounted_policy(), &mut identity, scope.cx()).await).unwrap();
    semio_plugin::with_authoring_identity!(|identity| artifact_app_laws::settle_history_verb(&mut app, "undo", meta.instance_id, semio_s_artifact_stdio_contract::testing::mounted_policy(), &mut identity, scope.cx()).await);
    assert_eq!(app.snapshot().unwrap(), loaded, "an identical Apply must not add a history entry");
    semio_plugin::with_authoring_identity!(|identity| artifact_app_laws::settle_history_verb(&mut app, "redo", meta.instance_id, semio_s_artifact_stdio_contract::testing::mounted_policy(), &mut identity, scope.cx()).await);
    assert_eq!(app.snapshot().unwrap(), expected);

    let stale = arguments(DocumentWindowKit::text_revision("before"), "refused draft");
    semio_plugin::with_authoring_identity!(|identity| semio_plugin::artifact_app_laws::dispatch_fixture_action(&mut app, "set-page", Some(stale.clone()), &meta, &mut identity, scope.cx(), semio_s_artifact_stdio_contract::testing::mounted_policy().close).await).unwrap();
    assert!(semio_plugin::with_authoring_identity!(|identity| artifact_app_laws::settle_registered_typed_operation(&mut app, meta.instance_id, semio_s_artifact_stdio_contract::testing::mounted_policy(), &mut identity, scope.cx()).await).is_err());
    assert_eq!(app.snapshot().unwrap(), expected, "a refused draft must preserve every PDF page operator");
    artifact_app_laws::close_registered_fixture_app(&mut app, semio_s_artifact_stdio_contract::testing::mounted_policy(), scope.cx()).expect("original caller paid app close");
}

#[semio_framework_async_macros::async_test]
async fn registered_pdf_page_edit_publishes_and_undoes_redoes() {
    use crate::standards::v1_7::subsets::base::schema::snapshot::PdfOp;
    use semio_plugin::{artifact_app_laws, EditorApp, PluginApp};

    let mut original = crate::standards::v1_7::subsets::base::io::export::demo_pdf17_snapshot();
    let mut scope = semio_plugin::artifact_app_laws::OriginalFixtureScope::admit();
    let mut app = semio_plugin::with_authoring_identity!(|identity| artifact_app_laws::new_registered_app::<EditorApp<Pdf17Editor>, _>(async { semio_plugin::App { definition: create_pdf17_editor(), examples: Vec::new() } }, semio_os_kernel::ActorId(semio_os_kernel::LOCAL_ACTOR_ID.into()), semio_s_artifact_stdio_contract::testing::mounted_policy(), &mut identity).await);
    let semio_plugin::Effect::LoadDocument { pack, spr } = semio_s_artifact_stdio_contract::load_example_effect(&original, STDIO_PDF17_DOCUMENT_SCHEMA) else { panic!("PDF fixture produces a document load") };
    semio_plugin::with_authoring_identity!(|identity| semio_plugin::artifact_app_laws::load_document(&mut app, &store::os_store::serialization::ArtifactPackFiles { pack, spr, ops: String::new() }, &mut identity, scope.cx()).await).unwrap();
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
    semio_plugin::with_authoring_identity!(|identity| semio_plugin::artifact_app_laws::dispatch_fixture_action(&mut app, "insert-rectangle", Some(insert.clone()), &meta, &mut identity, scope.cx(), semio_s_artifact_stdio_contract::testing::mounted_policy().close).await).unwrap();
    semio_plugin::with_authoring_identity!(|identity| artifact_app_laws::settle_registered_typed_operation(&mut app, meta.instance_id, semio_s_artifact_stdio_contract::testing::mounted_policy(), &mut identity, scope.cx()).await).unwrap();
    let edited = app.snapshot().unwrap().clone();
    assert!(edited.pages[0].content.iter().any(|op| matches!(op, PdfOp::Rectangle { x, y, width, height } if (*x - 12.0).abs() < 0.01 && (*y - 24.0).abs() < 0.01 && (*width - 30.0).abs() < 0.01 && (*height - 16.0).abs() < 0.01)));
    assert!(edited.pages[0].text().contains("Semio"));
    semio_plugin::with_authoring_identity!(|identity| artifact_app_laws::settle_history_verb(&mut app, "undo", meta.instance_id, semio_s_artifact_stdio_contract::testing::mounted_policy(), &mut identity, scope.cx()).await);
    assert_eq!(app.snapshot().unwrap(), loaded);
    semio_plugin::with_authoring_identity!(|identity| artifact_app_laws::settle_history_verb(&mut app, "redo", meta.instance_id, semio_s_artifact_stdio_contract::testing::mounted_policy(), &mut identity, scope.cx()).await);
    assert_eq!(app.snapshot().unwrap(), edited);
    artifact_app_laws::close_registered_fixture_app(&mut app, semio_s_artifact_stdio_contract::testing::mounted_policy(), scope.cx()).expect("original caller paid app close");
}

semio_plugin::history_edit_acceptance_law!("stdio", Pdf17Editor, || semio_plugin::App { definition: create_pdf17_editor(), examples: Vec::new() }, "../..");

fn qpdf_accepts(bytes: &[u8], name: &str) {
    let path = std::env::temp_dir().join(format!("semio-{name}-{}.pdf", std::process::id()));
    std::fs::write(&path, bytes).unwrap();
    let output = std::process::Command::new("qpdf").arg("--check").arg(&path).output();
    let _ = std::fs::remove_file(&path);
    match output {
        Ok(output) => assert!(output.status.success(), "qpdf --check refused the saved PDF: {}", String::from_utf8_lossy(&output.stderr)),
        Err(error) => assert_eq!(error.kind(), std::io::ErrorKind::NotFound, "qpdf could not run: {error}"),
    }
}

#[test]
fn every_pdf_17_subset_editor_opens_and_saves_the_same_natural_file() {
    use crate::editor::{pdf17a::Pdf17AEditor, pdf17e::Pdf17EEditor, pdf17h::Pdf17HEditor, pdf17ua::Pdf17UaEditor, pdf17vt::Pdf17VtEditor, pdf17x::Pdf17XEditor};
    let input = fixture_bytes!("../../../../../../🧪️tests/🧫️fixtures/✏️edit-existing-pdf/2️⃣two-pages.pdf");
    let base = <Pdf17Editor as ArtifactEditor>::decode_natural_file(input).expect("natural PDF opens");
    let expected = <Pdf17Editor as ArtifactEditor>::encode_natural_file(&base).unwrap();
    macro_rules! assert_subset {
        ($editor:ty, $kind:expr) => {{
            let codec = <$editor as ArtifactEditor>::natural_file_codec().expect("every subset editor declares the natural PDF file");
            assert_eq!((codec.format_kind, codec.extension, codec.media_type, codec.binary), ($kind, ".pdf", "application/pdf", true));
            let opened = <$editor as ArtifactEditor>::decode_natural_file(input).expect("natural PDF opens");
            let saved = <$editor as ArtifactEditor>::encode_natural_file(&opened).unwrap();
            assert_eq!(saved, expected);
            qpdf_accepts(&saved, stringify!($editor));
            assert!(<$editor as ArtifactEditor>::decode_natural_file(b"%PDF-1.7 truncated").is_err());
        }};
    }
    assert_subset!(Pdf17AEditor, "s.stdio.pdf@1.7/a");
    assert_subset!(Pdf17EEditor, "s.stdio.pdf@1.7/e");
    assert_subset!(Pdf17HEditor, "s.stdio.pdf@1.7/h");
    assert_subset!(Pdf17UaEditor, "s.stdio.pdf@1.7/ua");
    assert_subset!(Pdf17VtEditor, "s.stdio.pdf@1.7/vt");
    assert_subset!(Pdf17XEditor, "s.stdio.pdf@1.7/x");
}

#[test]
fn text_layer_lists_a_real_document_in_reading_order_with_localized_labels() {
    use semio_ui_locale::Locale;
    let snapshot = <Pdf17Editor as ArtifactEditor>::decode_natural_file(fixture_bytes!("../../../../../../🧪️tests/🧫️fixtures/✏️edit-existing-pdf/2️⃣two-pages.pdf")).expect("natural PDF opens");
    let english = crate::editor::page::text_layer_lines(&snapshot, Locale::En);
    let german = crate::editor::page::text_layer_lines(&snapshot, Locale::De);
    assert_eq!(english.len(), german.len());
    let texts = |lines: &[(String, String)]| lines.iter().map(|(_, line)| line.clone()).collect::<Vec<_>>();
    let english = texts(&english);
    let german = texts(&german);
    assert!(english.iter().any(|line| line.starts_with("Language: ")) && german.iter().any(|line| line.starts_with("Sprache: ")));
    assert!(english.iter().any(|line| line.starts_with("Tagged PDF: ")) && german.iter().any(|line| line.starts_with("Getaggtes PDF: ")));
    let first = english.iter().position(|line| line == "Page 1 / 2").expect("page one heading");
    let second = english.iter().position(|line| line == "Page 2 / 2").expect("page two heading");
    assert_eq!(german.iter().position(|line| line == "Seite 2 / 2"), Some(second));
    let page_one_text = snapshot.pages[0].text();
    let found = english.iter().position(|line| line.starts_with("Text: ") && page_one_text.contains(line.trim_start_matches("Text: "))).expect("page one text is listed");
    assert!(first < found && found < second, "page one text sits between the page headings");
    let panel = crate::editor::page::render_text_layer(&snapshot, "s.stdio.pdf@1.7/*#editor", Locale::De).unwrap();
    let page_two = &panel.children[second];
    let binding = &page_two.bindings[0];
    assert_eq!(binding.action.scope.as_str(), "s.stdio.pdf@1.7/*#editor");
    assert_eq!(binding.action.name.as_str(), semio_plugin::INTERACTION_SELECT_ACTION_ID);
    let Some(semio_ui_contract::UiValue::Map(args)) = &binding.args else { panic!("accessible page selection") };
    assert!(args.iter().any(|(key, value)| key.as_str() == "targets" && matches!(value, semio_ui_contract::UiValue::Text(text) if text.as_str().contains("p1:paper"))));
}
