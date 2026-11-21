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
    use semio_plugin::{artifact_app_laws as laws, EditorApp, PluginApp};
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
    let cx = scope.cx();
    let fuel = laws::OriginalFixtureCaller::turn_fuel();
    let grant = cx.retained_grant();
    laws::ensure_fixture_cancellation_issuer(&mut app, cx).expect("mounted PDF instance is born");
    app.render_ctx().refresh_cache().await.expect("mounted PDF head is projected");
    let mut artifact = Some(artifact);
    let mut consumed = false;
    let mut idle = 0;
    for _ in 0..laws::OriginalFixtureCaller::maximum_turns() {
        laws::fixture_renew_turn(cx, fuel, grant).expect("mounted PDF import turn is funded");
        let before = (cx.original_work_units_spent(), cx.retained_progress());
        if PluginApp::consume_media(&mut app, semio_plugin::NATURAL_FILE_PORT, &mut artifact, cx)
            .await.expect("mounted PDF consumes original bytes").is_some()
        {
            consumed = true;
            break;
        }
        app.maintenance_step(cx, policy.close).expect("mounted PDF upkeep is funded");
        let after = (cx.original_work_units_spent(), cx.retained_progress());
        idle = if before == after { idle + 1 } else { 0 };
        if idle >= laws::OriginalFixtureCaller::maximum_idle_turns() { break }
    }
    assert!(consumed, "mounted PDF media intake stalled after {idle} idle turns: {:?}, {}", cx.retained_grant(), app.typed_operation_diagnostic());
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
    let revision = app.document_base_revision().expect("edited document revision");
    let document = app.document_identity().expect("edited document identity");
    app.bind_instance_id(meta.instance_id).await;
    let mut submitted = None;
    for _ in 0..laws::OriginalFixtureCaller::maximum_turns() {
        if app.submit_media_export_checked(semio_plugin::NATURAL_FILE_PORT, &document, revision, None, &mut submitted, scope.cx()).await.expect("checked PDF Save submission") { break }
        app.maintenance_step(scope.cx(), policy.close).expect("PDF Save admission maintenance");
    }
    let handle = submitted.expect("registered PDF Save handle");
    assert_eq!(handle.parent_document_id.as_str(), document.as_str());
    let mut completed = None;
    for _ in 0..laws::OriginalFixtureCaller::maximum_turns() {
        match laws::poll_fixture_media_export(&mut app, &handle, scope.cx()).await.expect("registered PDF Save poll") {
            semio_plugin::ArtifactMediaExportPoll::Running { .. } => {},
            semio_plugin::ArtifactMediaExportPoll::Complete(result) => { completed = Some(result); break },
            other => panic!("registered PDF Save did not produce its segmented output: {other:?}"),
        }
    }
    let result = completed.expect("registered PDF Save completes");
    assert_eq!(result.schema, codec.format_kind);
    assert_eq!(result.mime_type, codec.media_type);
    let status = laws::retain_fixture_media_export_result(&mut app, &handle, result, scope.cx()).await.expect("PDF Save publishes download status");
    assert_eq!(status.port, semio_plugin::NATURAL_FILE_PORT);
    let mut bytes = Vec::new();
    let mut terminal = false;
    type Output = semio_plugin::ArtifactOutputChunks;
    for _ in 0..=(Output::MAXIMUM_TOTAL_BYTES / Output::CHUNK_BYTES) {
        if let Some(chunk) = laws::take_fixture_media_export_chunk(&mut app, &handle, &mut terminal, scope.cx()).await.expect("PDF Save download page") {
            assert!(!chunk.is_empty() && chunk.len() <= Output::CHUNK_BYTES);
            bytes.extend_from_slice(&chunk);
        }
        if terminal { break }
    }
    assert!(terminal, "PDF Save downloads a terminal page sequence");
    assert_eq!(bytes.len() as u64, status.total_bytes);
    assert_eq!(app.document_base_revision().unwrap(), revision);
    assert_eq!(app.document_identity().as_deref(), Some(document.as_str()));
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
    pdf_fixture_file(rows)
}

fn pdf_fixture_file(rows: Vec<Vec<u8>>) -> Vec<u8> {
    use std::io::Write;
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

semio_plugin::history_edit_acceptance_law!("stdio", Pdf17Editor, || semio_plugin::App { definition: create_pdf17_editor(), examples: Vec::new() }, "../../../../../..");

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
    let snapshot = <Pdf17Editor as ArtifactEditor>::decode_natural_file(fixture_bytes!("../../../../../../🧪️tests/🧫️fixtures/🔀️move-page/⬅️before.pdf")).expect("natural PDF opens");
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
    let _arena = semio_plugin::artifact_app_laws::FixtureUiArena::admit();
    let panel = crate::editor::page::render_text_layer(&snapshot, "s.stdio.pdf@1.7/*#editor", Locale::De).unwrap();
    semio_plugin::artifact_app_laws::observe_and_retire_fixture_tree(semio_plugin::built_to_component_tree(panel), |panel| {
        let page_two = &panel.children[second];
        let binding = &page_two.bindings[0];
        assert_eq!(binding.action.scope.as_str(), "s.stdio.pdf@1.7/*#editor");
        assert_eq!(binding.action.name.as_str(), semio_plugin::INTERACTION_SELECT_ACTION_ID);
        let Some(semio_ui_contract::UiValue::Map(args)) = &binding.args else { panic!("accessible page selection") };
        assert!(args.iter().any(|(key, value)| key.as_str() == "targets" && matches!(value, semio_ui_contract::UiValue::Text(text) if text.as_str().contains("p1:paper"))));
    });
}

//#region 🧪️Controlled image natural file

fn sampled_fixture_bytes(spec: &serde_json::Value, mixed: bool) -> (Vec<u8>, Vec<(u32, Vec<u8>)>) {
    use serde_json::Value;
    let dimensions = |key| serde_json::from_value::<[usize; 2]>(spec[key].clone()).unwrap();
    let [width, height] = dimensions("imageSize");
    let [page_width, page_height] = dimensions("pageSize");
    let [x, y] = dimensions("imageOrigin");
    let images = spec["images"].as_array().unwrap();
    let samples = |image: &Value| {
        let quadrants: [Vec<u8>; 4] = serde_json::from_value(image["samples"].clone()).unwrap();
        let row = |index: usize| [quadrants[index].repeat(width / 2), quadrants[index + 1].repeat(width / 2)].concat();
        [row(0).repeat(height / 2), row(2).repeat(height / 2)].concat()
    };
    let source_samples: Vec<_> = images.iter().map(|image| (8, samples(image))).collect();
    let resources = |image: &Value| {
        let name = image["name"].as_str().unwrap();
        let object = image["object"].as_u64().unwrap();
        let font = if mixed { "/Font << /F1 15 0 R >> " } else { "" };
        format!("<< {font}/XObject << /{name} {object} 0 R >> >>")
    };
    let page = |contents, entry: &str| {
        format!("<< /Type /Page /Parent 2 0 R /MediaBox [0 0 {page_width} {page_height}] /Contents {contents} 0 R{entry} >>").into_bytes()
    };
    let stream = |fields: &str, body: &[u8]| {
        let mut stream = format!("<< /Length {}{fields} >>\nstream\n", body.len()).into_bytes();
        stream.extend_from_slice(body);
        stream.extend_from_slice(b"\nendstream");
        stream
    };
    let content = |image: &Value, index: usize| {
        let name = image["name"].as_str().unwrap();
        let mut body = format!("q {width} 0 0 {height} {x} {y} cm /{name} Do Q\n");
        if mixed {
            let text = spec["mixedText"][index].as_str().unwrap();
            body.push_str(&format!("BT /F1 12 Tf 20 102 Td ({text}) Tj ET\n"));
        }
        stream("", body.as_bytes())
    };
    let picture = |index: usize| {
        let image = &images[index];
        let space = image["space"].as_str().unwrap();
        let mut fields = format!(" /Type /XObject /Subtype /Image /Width {width} /Height {height} /ColorSpace /{space} /BitsPerComponent 8");
        let samples = &source_samples[index].1;
        let body = if let Some(filter) = image["filter"].as_str() {
            fields.push_str(&format!(" /Filter /{filter}"));
            fixture_stored_zlib(samples)
        } else { samples.clone() };
        stream(&fields, &body)
    };
    let mut rows = vec![
        b"<< /Type /Catalog /Pages 2 0 R >>".to_vec(),
        format!("<< /Type /Pages /Kids [3 0 R 6 0 R 9 0 R 13 0 R] /Count 4 /Resources {} >>", resources(&images[0])).into_bytes(),
        page(4, ""), content(&images[0], 0), picture(0),
        page(7, &format!(" /Resources {}", resources(&images[1]))), content(&images[1], 1), picture(1),
        page(12, " /Resources 10 0 R"), resources(&images[2]).into_bytes(), picture(2), content(&images[2], 2),
        page(14, ""), content(&images[0], 3),
    ];
    if mixed {
        rows.push(b"<< /Type /Font /Subtype /Type1 /BaseFont /Helvetica /Encoding /WinAnsiEncoding >>".to_vec());
    }
    (pdf_fixture_file(rows), source_samples)
}

fn fixture_stored_zlib(samples: &[u8]) -> Vec<u8> {
    let length = u16::try_from(samples.len()).unwrap();
    let mut out = vec![0x78, 0x01, 0x01];
    out.extend_from_slice(&length.to_le_bytes());
    out.extend_from_slice(&(!length).to_le_bytes());
    out.extend_from_slice(samples);
    let (a, b) = samples.iter().fold((1u32, 0u32), |(a, b), byte| {
        let a = (a + u32::from(*byte)) % 65_521;
        (a, (b + a) % 65_521)
    });
    out.extend_from_slice(&((b << 16) | a).to_be_bytes());
    out
}

#[semio_framework_async_macros::async_test]
async fn natural_file_route_opens_reexports_and_reopens_large_sampled_images() {
    use crate::standards::v1_7::subsets::base::{
        io::import::decode_pdf, schema::{snapshot::{ObjRef, PdfOp}, stream_roles::PdfStreamRoleValue},
    };
    use semio_plugin::{artifact_app_laws as laws, PluginApp};
    use semio_s_artifact_stdio_pdf_test_oracle::standards::v1_7::subsets::base::{project_pdf_1_7, reference_image_sample_streams};

    let spec: serde_json::Value = serde_json::from_str(fixture_text!("../../../../../../🧪️tests/🧫️fixtures/🎞️controlled-images/🔣️.json")).unwrap();
    for mixed in [false, true] {
        let (input, samples) = sampled_fixture_bytes(&spec, mixed);
        assert!(input.len() > spec["minimumByteLength"].as_u64().unwrap() as usize);
        assert_eq!(reference_image_sample_streams(&input).unwrap(), samples);
        let verify_text = |data: &[u8]| {
            if mixed {
                let projected: serde_json::Value = serde_json::from_str(&project_pdf_1_7(data).unwrap().to_string()).unwrap();
                for (page, text) in projected["pages"].as_array().unwrap().iter().zip(spec["mixedText"].as_array().unwrap()) {
                    assert_eq!(page["text"].as_array().unwrap().as_slice(), std::slice::from_ref(text));
                }
            }
        };
        verify_text(&input);
        let cold = decode_pdf(&input).unwrap();
        let pages = spec["pages"].as_array().unwrap();
        let mut scope = laws::OriginalFixtureScope::admit();
        let mut app = open_registered_pdf(input, pages.len(), &mut scope).await;
        let mounted = app.snapshot().unwrap();
        assert_eq!(mounted.pages, cold.pages);
        assert_eq!(mounted.images, cold.images);
        assert_eq!(mounted.fonts, cold.fonts);
        assert_eq!(mounted.objects, cold.objects);
        assert_eq!(mounted.trailer, cold.trailer);
        assert_eq!(mounted.admitted_stream_roles, cold.admitted_stream_roles);
        for (image, role) in mounted.images.iter().zip(&mounted.admitted_stream_roles) {
            let spec = spec["images"].as_array().unwrap().iter().find(|spec| spec["name"].as_str() == Some(image.id.as_str())).unwrap();
            assert_eq!(role.identity.owner, ObjRef { num: spec["object"].as_u64().unwrap() as u32, gen: 0 });
            assert!(role.identity.path.is_empty());
            assert_eq!(role.dependencies.as_slice(), std::slice::from_ref(&role.identity));
            assert!(matches!(&role.value, PdfStreamRoleValue::Image { image: owned } if owned == image));
        }
        for (page, expected) in mounted.pages.iter().zip(pages) {
            let image = expected["image"].as_u64().unwrap() as usize;
            let name = spec["images"][image]["name"].as_str().unwrap();
            assert!(matches!(&page.content[1..3], [PdfOp::Transform { matrix }, PdfOp::PaintXObject { name: actual }]
                if actual == name && *matrix == [72., 0., 0., 72., 20., 20.]));
        }

        let artifact = semio_s_artifact_stdio_contract::testing::with_encoding_async(async |owner| {
            PluginApp::produce_media(&mut app, semio_plugin::NATURAL_FILE_PORT, owner, scope.cx()).await
        }).await.expect("mounted PDF reexports").expect("natural PDF output");
        assert_eq!(reference_image_sample_streams(&artifact.data).unwrap(), samples);
        verify_text(&artifact.data);
        let policy = semio_s_artifact_stdio_contract::testing::mounted_policy();
        laws::close_registered_fixture_app(&mut app, policy, scope.cx()).expect("original image editor closes");
        let mut reopened = open_registered_pdf(artifact.data, pages.len(), &mut scope).await;
        assert_eq!(reopened.snapshot().unwrap().images, cold.images);
        assert_eq!(reopened.snapshot().unwrap().pages, cold.pages);
        laws::close_registered_fixture_app(&mut reopened, policy, scope.cx()).expect("reopened image editor closes");
    }
}

//#endregion 🧪️Controlled image natural file

#[semio_framework_async_macros::async_test]
async fn registered_pdf_font_change_preserves_encoded_text_and_the_next_line() {
    use crate::editor::page::{objects, ObjectKind};
    use crate::standards::v1_7::subsets::base::schema::snapshot::{PdfOp, PdfTextArrayItem};
    use semio_plugin::{artifact_app_laws as laws, PluginApp};
    use semio_framework_value::DslValue;
    use serde_json::json;

    let fixture: serde_json::Value = serde_json::from_str(fixture_text!("../../../../../../🧪️tests/🧫️fixtures/📄️natural-file-edit/🔣️.json")).unwrap();
    let case = &fixture["fontReassignment"];
    let size = &fixture["fontSize"];
    let byte = case["sourceCode"].as_u64().unwrap();
    let adjust = case["kerning"].as_f64().unwrap();
    let alternate = case["sourceAlternateCode"].as_u64().unwrap();
    let program = format!("BT /Fsrc {} Tf 10 50 Td 24 TL [<{byte:02X}> {adjust} <{byte:02X}>] TJ [<{}> -20] TJ T* ({}) Tj ET", size["before"], size["sameLineCode"].as_str().unwrap(), size["followingLine"].as_str().unwrap());
    let input = pdf_fixture_file(vec![
        b"<< /Type /Catalog /Pages 2 0 R >>".to_vec(),
        b"<< /Type /Pages /Kids [3 0 R] /Count 1 >>".to_vec(),
        b"<< /Type /Page /Parent 2 0 R /MediaBox [0 0 180 90] /Resources << /Font << /Fsrc 5 0 R /Fdst 6 0 R >> >> /Contents 4 0 R >>".to_vec(),
        pdf_fixture_stream(program.as_bytes()),
        format!("<< /Type /Font /Subtype /Type1 /BaseFont /Helvetica /Encoding << /BaseEncoding /WinAnsiEncoding /Differences [{alternate} /eacute] >> >>").into_bytes(),
        b"<< /Type /Font /Subtype /Type1 /BaseFont /Times-Roman /Encoding /MacRomanEncoding >>".to_vec(),
    ]);
    let mut scope = laws::OriginalFixtureScope::admit();
    let meta = laws::meta("local");
    let policy = semio_s_artifact_stdio_contract::testing::mounted_policy();
    let mut app = open_registered_pdf(input, 1, &mut scope).await;
    let text_objects = |snapshot: &PdfSnapshot| objects(snapshot).into_iter().filter(|object| object.kind == ObjectKind::Text).collect::<Vec<_>>();
    let before = text_objects(&app.snapshot().unwrap());
    assert_eq!(before[0].text.as_str(), case["firstLine"].as_str().unwrap());
    assert_eq!(before[1].text.as_str(), size["sameLine"].as_str().unwrap());
    assert_eq!(before[2].text.as_str(), size["followingLine"].as_str().unwrap());
    let first = before[0].id.as_str();
    let original_same_line = app.snapshot().unwrap().pages[0].content[before[1].end].clone();
    for (action, args) in [
        ("set-font", json!({"page": 0, "object": first, "text": case["targetFont"]})),
        ("set-font-size", json!({"page": 0, "object": first, "field": "size", "value": size["after"]})),
    ] {
        semio_plugin::with_authoring_identity!(|identity| laws::dispatch_fixture_action(
            &mut app, action, Some(DslValue::from(args.clone())), &meta, &mut identity, scope.cx(), policy.close
        ).await).unwrap();
        semio_plugin::with_authoring_identity!(|identity| laws::settle_registered_typed_operation(
            &mut app, meta.instance_id, policy, &mut identity, scope.cx()
        ).await).unwrap();
    }

    let verify = |snapshot: &PdfSnapshot, from_original: bool| {
        let after = text_objects(snapshot);
        assert_eq!(after[0].text, before[0].text);
        assert_eq!((after[0].x, after[0].y), (before[0].x, before[0].y));
        assert_eq!(after[0].font_size, size["after"].as_f64().unwrap());
        assert_eq!(after.len(), before.len());
        assert_eq!(snapshot.pages[0].content[after[1].end], original_same_line);
        for (after, before) in after[1..].iter().zip(&before[1..]) {
            assert_eq!((&after.text, after.font_size), (&before.text, before.font_size));
            assert!((after.x - before.x).abs() < 1e-5 && (after.y - before.y).abs() < 1e-5);
        }
        for (object, expected) in after.iter().zip(["targetFont", "sourceFont", "sourceFont"]) {
            assert_eq!(snapshot.font(&object.font).unwrap().base_font(), case[expected].as_str().unwrap());
        }
        let PdfOp::ShowTextArray { items } = &snapshot.pages[0].content[after[0].end] else { panic!("the edited text array retains its spacing"); };
        assert!(items.iter().any(|item| matches!(item, PdfTextArrayItem::Adjust { amount } if *amount == adjust)));
        if from_original {
            assert!(matches!(&items[0], PdfTextArrayItem::Text { text } if text == "é"));
        }
    };
    verify(&app.snapshot().unwrap(), true);
    let saved = semio_s_artifact_stdio_contract::testing::with_encoding_async(async |owner| {
        PluginApp::produce_media(&mut app, semio_plugin::NATURAL_FILE_PORT, owner, scope.cx()).await
    }).await.unwrap().unwrap();
    laws::close_registered_fixture_app(&mut app, policy, scope.cx()).unwrap();
    let mut reopened = open_registered_pdf(saved.data, 1, &mut scope).await;
    verify(&reopened.snapshot().unwrap(), false);
    laws::close_registered_fixture_app(&mut reopened, policy, scope.cx()).unwrap();
}

#[semio_framework_async_macros::async_test]
async fn registered_pdf_png_picker_publishes_an_atomic_revision_guarded_image_edit() {
    use crate::standards::v1_7::subsets::base::schema::snapshot::{PdfFormXObject, PdfImage, PdfOp, PdfPage};
    use semio_framework_value::DslValue;
    use semio_plugin::{artifact_app_laws as laws, EditorApp, Effect, PluginApp};

    let fixture: serde_json::Value = serde_json::from_str(fixture_text!("../../../../../../🧪️tests/🧫️fixtures/🖼️local-png/🔣️.json")).unwrap();
    let samples = |key: &str| fixture[key].as_array().unwrap().iter().map(|sample| u8::try_from(sample.as_u64().unwrap()).unwrap()).collect::<Vec<_>>();
    let matrix: [f64; 6] = std::array::from_fn(|index| fixture["existing"]["insertPlacement"][index].as_f64().unwrap());
    let source_id = fixture["existing"]["images"][0].as_str().unwrap();
    let form_id = fixture["existing"]["forms"][0].as_str().unwrap();
    let image_id = fixture["expected"]["image"].as_str().unwrap();
    let mask_id = fixture["expected"]["mask"].as_str().unwrap();

    let mut original = PdfSnapshot::default();
    let mut first = PdfPage::new(160.0, 120.0);
    first.content.push(PdfOp::PaintXObject { name: source_id.into() });
    original.pages = vec![first.clone(), first];
    original.images.push(PdfImage::gray8(source_id, 1, 1, vec![127]));
    original.forms.push(PdfFormXObject::new(form_id, [0.0, 0.0, 1.0, 1.0], Vec::new()));

    let mut scope = laws::OriginalFixtureScope::admit();
    let policy = semio_s_artifact_stdio_contract::testing::mounted_policy();
    let meta = laws::meta("local");
    let mut app = semio_plugin::with_authoring_identity!(|identity| laws::new_registered_app::<EditorApp<Pdf17Editor>, _>(
        async { semio_plugin::App { definition: create_pdf17_editor(), examples: Vec::new() } },
        semio_os_kernel::ActorId(semio_os_kernel::LOCAL_ACTOR_ID.into()), policy, &mut identity
    ).await);
    let Effect::LoadDocument { pack, spr } = semio_s_artifact_stdio_contract::load_example_effect(&original, STDIO_PDF17_DOCUMENT_SCHEMA) else {
        panic!("the seed PDF can produce a document load");
    };
    semio_plugin::with_authoring_identity!(|identity| laws::load_document(
        &mut app, &store::os_store::serialization::ArtifactPackFiles { pack, spr, ops: String::new() }, &mut identity, scope.cx()
    ).await).unwrap();
    let loaded = app.snapshot().unwrap().clone();
    let request = DslValue::object([
        ("page".into(), DslValue::float(0.0)),
        ("x".into(), DslValue::float(matrix[4])),
        ("y".into(), DslValue::float(matrix[5])),
        ("width".into(), DslValue::float(matrix[0])),
        ("height".into(), DslValue::float(matrix[3])),
    ]);
    semio_plugin::with_authoring_identity!(|identity| laws::dispatch_fixture_action(
        &mut app, "insert-image", Some(request), &meta, &mut identity, scope.cx(), policy.close
    ).await).unwrap();
    let picker = semio_plugin::with_authoring_identity!(|identity| laws::settle_registered_typed_operation(
        &mut app, meta.instance_id, policy, &mut identity, scope.cx()
    ).await).unwrap();
    let [Effect::RequestFileOpen { accept, read_as, import_action, multiple, args: Some(args), max_file_bytes, .. }] = picker.effects.as_slice() else {
        panic!("the registered action emits exactly one local file picker");
    };
    assert_eq!(accept, "image/png,.png");
    assert_eq!(read_as.as_deref(), Some("dataUrl"));
    assert_eq!(import_action, "import-local-image");
    assert!(!*multiple);
    assert_eq!(max_file_bytes.map(|bytes| bytes as u64), fixture["bounds"]["maxFileBytes"].as_u64());
    assert_eq!(args.get("page").and_then(DslValue::as_u64), Some(0));
    assert_eq!(args.get("operator").and_then(DslValue::as_u64), Some(u64::from(u32::MAX)));
    let revision = args.get("revision").and_then(DslValue::as_str).expect("exact picker revision");
    assert_eq!(revision.len(), 64);
    assert!(revision.bytes().all(|byte| byte.is_ascii_hexdigit()));
    assert_eq!(app.snapshot().unwrap(), loaded);

    let mut callback = args.as_object().unwrap().to_vec();
    callback.extend([
        ("name".into(), DslValue::String("sample.png".into())),
        ("chunk".into(), DslValue::uint(0)),
        ("chunkCount".into(), DslValue::uint(1)),
        ("payload".into(), DslValue::String(format!("data:application/octet-stream;base64,{}", fixture["png"].as_str().unwrap()))),
    ]);
    let callback = DslValue::object(callback);
    semio_plugin::with_authoring_identity!(|identity| laws::dispatch_fixture_action(
        &mut app, import_action, Some(callback.clone()), &meta, &mut identity, scope.cx(), policy.close
    ).await).unwrap();
    assert_eq!(app.snapshot().unwrap(), loaded, "dispatch leaves the retained import private");
    let receipt = semio_plugin::with_authoring_identity!(|identity| laws::settle_registered_typed_operation(
        &mut app, meta.instance_id, policy, &mut identity, scope.cx()
    ).await).unwrap();
    assert_eq!(receipt.completions, 1);

    let published = app.snapshot().unwrap().clone();
    let width = u32::try_from(fixture["width"].as_u64().unwrap()).unwrap();
    let height = u32::try_from(fixture["height"].as_u64().unwrap()).unwrap();
    let mut image = PdfImage::rgb8(image_id, width, height, samples("rgb"));
    image.soft_mask = Some(mask_id.into());
    let mask = PdfImage::gray8(mask_id, width, height, samples("softMask"));
    assert_eq!(published.images.len(), loaded.images.len() + 2);
    assert_eq!(published.image(image_id), Some(&image));
    assert_eq!(published.image(mask_id), Some(&mask));
    assert_eq!(published.image(source_id), loaded.image(source_id));
    assert_eq!(published.forms, loaded.forms);
    assert_eq!(published.pages[1], loaded.pages[1]);
    assert!(published.pages[0].content.starts_with(&loaded.pages[0].content));
    assert!(matches!(
        &published.pages[0].content[loaded.pages[0].content.len()..],
        [PdfOp::Save, PdfOp::Transform { matrix: applied }, PdfOp::PaintXObject { name }, PdfOp::Restore]
            if applied == &matrix && name == image_id
    ));

    semio_plugin::with_authoring_identity!(|identity| laws::dispatch_fixture_action(
        &mut app, import_action, Some(callback), &meta, &mut identity, scope.cx(), policy.close
    ).await).unwrap();
    let stale = semio_plugin::with_authoring_identity!(|identity| laws::settle_registered_typed_operation(
        &mut app, meta.instance_id, policy, &mut identity, scope.cx()
    ).await);
    assert!(format!("{:?}", stale.err().expect("the completed picker revision is stale")).contains("PDF changed while the image picker was open"));
    assert_eq!(app.snapshot().unwrap(), published);

    semio_plugin::with_authoring_identity!(|identity| laws::settle_history_verb(
        &mut app, "undo", meta.instance_id, policy, &mut identity, scope.cx()
    ).await);
    assert_eq!(app.snapshot().unwrap(), loaded);
    semio_plugin::with_authoring_identity!(|identity| laws::settle_history_verb(
        &mut app, "redo", meta.instance_id, policy, &mut identity, scope.cx()
    ).await);
    assert_eq!(app.snapshot().unwrap(), published);
    laws::close_registered_fixture_app(&mut app, policy, scope.cx()).expect("registered PNG fixture closes");
}

#[test]
fn controlled_pdf_terminal_publication_requires_both_owned_headers() {
    use crate::standards::v1_7::subsets::base::io::import::retained_graph::PreparedPdfFile;
    use semio_framework_value::{RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep, RetainedWorkOutcome, RetirementDemand, RetirementPollExit};
    use semio_plugin::artifact_app_laws::OriginalWorkFixture;
    use semio_plugin::{NaturalFileDecodeOwner, NaturalFileDecodeStep, ToolExecutionContract};
    use std::mem::{size_of, size_of_val};

    let input = fixture_bytes!("../../../../../../🧪️tests/🧫️fixtures/✏️edit-existing-pdf/2️⃣two-pages.pdf");
    let contract = ToolExecutionContract::resumable(input.len(), 16_384, 1, 16 << 20, 7_500, 1, 1);
    let quote = size_of::<PreparedPdfFile>() + size_of::<PdfSnapshot>();
    #[cfg(all(target_arch = "wasm32", target_os = "wasi"))]
    assert_eq!(quote, 964);
    let stock = OriginalWorkFixture::stock();
    let run = |constrained_at: Option<usize>| {
        let mut scope = OriginalWorkFixture::new();
        let mut source = Some(input.to_vec());
        let mut claimed = None;
        assert!(scope.turn(stock, |cx| Pdf17Editor::admit_natural_file_decode_owner(&mut source, contract, &mut claimed, cx)).unwrap());
        assert!(source.is_none(), "the exact original PDF bytes transferred");
        let Some(NaturalFileDecodeOwner::Controlled(decoder)) = claimed else {
            panic!("one-unit file contract must use the real controlled PDF decoder");
        };
        let mut decoder = Some(decoder);
        let mut complete = None;
        for turn in 0..200_000 {
            let owner = decoder.as_mut().unwrap();
            if constrained_at == Some(turn) {
                let short = RetainedCloneGrant { maximum_copy_bytes: quote - 1, ..stock };
                for _ in 0..2 {
                    scope.turn(short, |cx| {
                        let before = (cx.retained_progress(), cx.original_work_units_spent());
                        assert!(matches!(owner.advance(cx).unwrap(), NaturalFileDecodeStep::Yield));
                        assert_eq!((cx.retained_progress(), cx.original_work_units_spent()), before);
                        Ok(())
                    }).unwrap();
                }
            }
            let grant = RetainedCloneGrant { maximum_copy_bytes: if constrained_at == Some(turn) { quote } else { stock.maximum_copy_bytes }, ..stock };
            let (step, copied, items) = scope.turn(grant, |cx| {
                let before = cx.retained_progress();
                let step = owner.advance(cx).unwrap();
                let after = cx.retained_progress();
                Ok((step, after.copied_bytes - before.copied_bytes, after.copied_items - before.copied_items))
            }).unwrap();
            if let NaturalFileDecodeStep::Complete { snapshot, progress } = step {
                assert_eq!((copied, items), (quote, 1));
                if let Some(expected) = constrained_at { assert_eq!(turn, expected); }
                complete = Some((snapshot, progress, turn));
                break;
            }
            assert_ne!(Some(turn), constrained_at, "the exact quote must publish after the two refusals");
        }
        let (snapshot, progress, terminal_turn) = complete.expect("the original controlled PDF reached publication");
        let owner = decoder.as_mut().unwrap();
        owner.begin_close();
        for _ in 0..200_000 {
            if owner.terminal_is_empty() { break; }
            scope.turn(stock, |cx| {
                let demand = owner.retirement_demands(cx.retained_grant().maximum_copy_bytes)?;
                semio_plugin::__job::close_original_effect_step(cx, demand, |effect| {
                    let result = owner.close_step(effect).map(|step| RetainedCloneStep::Progress(step.progress().unwrap_or_default())).map_err(Into::into);
                    RetirementPollExit::settle(effect, result)
                })?;
                Ok(())
            }).unwrap();
        }
        assert!(owner.terminal_is_empty());
        let demand = RetirementDemand {
            release_bytes: size_of_val(&**owner),
            copy_bytes: size_of_val(&decoder),
            depth: 1,
            ..Default::default()
        };
        scope.turn(stock, |cx| {
            semio_plugin::__job::close_original_retained_step(cx, demand, |_| {
                drop(decoder.take());
                RetainedWorkOutcome {
                    work: 1,
                    result: Ok(RetainedCloneStep::Complete(RetainedCloneProgress {
                        copied_items: 1,
                        copied_bytes: demand.copy_bytes,
                        released_bytes: demand.release_bytes,
                        ..Default::default()
                    })),
                }
            })?;
            Ok(())
        }).unwrap();
        assert!(decoder.is_none());
        (snapshot, progress, terminal_turn, scope)
    };

    let (reference, reference_progress, terminal_turn, mut reference_scope) = run(None);
    let (published, progress, _, mut scope) = run(Some(terminal_turn));
    assert_eq!(published, reference);
    assert_eq!(progress, reference_progress);
    assert_eq!(published.pages.len(), 2);
    assert!(!published.objects.is_empty());
    assert!(!published.trailer.is_empty());
    scope.retire_owned(published, stock);
    scope.finish();
    reference_scope.retire_owned(reference, stock);
    reference_scope.finish();
}
