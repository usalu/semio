use crate::viewer::pdf17vt::modes::view::windows::main;
use semio_plugin::{ArtifactViewer, Viewer};
#[allow(unused_imports)]
use crate::viewer::pdf17vt::modes::view;
#[allow(unused_imports)]
use crate::standards::v1_7::subsets::base::schema::mutations::PdfMutation;
#[allow(unused_imports)]
use crate::standards::v1_7::subsets::base::schema::snapshot::{PdfSnapshot, STDIO_PDF17_DOCUMENT_SCHEMA};
#[allow(unused_imports)]
use crate::standards::v1_7::subsets::base::schema::PDF_ARTIFACT_SCHEMA_ID;
#[allow(unused_imports)]
use {semio_plugin::built_to_component_tree, semio_plugin::ArtifactView, semio_plugin::ComponentTree, semio_plugin::ConfigView, semio_framework_artifact_reference::Dialect, semio_plugin::Fault, semio_plugin::NoConfig, semio_plugin::NoConfigMutation, semio_plugin::NoPresence, semio_plugin::NoPresenceMutation, semio_plugin::NoTransient, semio_plugin::NoTransientMutation, semio_framework_artifact_reference::StandardId, semio_framework_artifact_reference::SubsetId, semio_plugin::ViewEmit};
use crate::viewer::pdf17vt::*;

#[semio_framework_async_macros::async_test]
async fn create_pdf17_vt_viewer_builds_a_definition_for_the_viewer_role() {
    let def = create_pdf17_vt_viewer();
    assert_eq!(def.role, semio_plugin::AppRole::Viewer);
    assert_eq!(def.dialect, PDF17VT_DIALECT.into());
}

#[semio_framework_async_macros::async_test]
async fn viewer_dialect_matches_the_artifact_coordinate() {
    assert_eq!(<Pdf17VtViewer as ArtifactViewer>::DIALECT, PDF17VT_DIALECT);
}

#[semio_framework_async_macros::async_test]
async fn viewer_declares_the_main_window() {
    let def = create_pdf17_vt_viewer();
    assert!(def.window_kinds.iter().any(|w| w.id == main::WINDOW_KIND_ID));
}

#[semio_framework_async_macros::async_test]
async fn committed_vt_archive_restores_forward_and_inverse_with_registered_owners() {
    use crate::editor::pdf17vt::{Pdf17VtEditor, create_pdf17_vt_editor};
    use crate::standards::v1_7::subsets::base::schema::{
        mutations::{InsertObject, RemoveObject},
        snapshot::{ObjRef, PdfObject},
    };
    use protocol::{ActorId, OpBinary, os_store::store_schema::ArtifactCommand, os_spr::{DecodeOptions, decode_history}};
    use semio_plugin::{App, ArtifactEditor, EditorApp, PluginApp, ViewerApp, artifact_app_laws as laws};

    type Editor = EditorApp<Pdf17VtEditor>;
    type Viewer = ViewerApp<Pdf17VtViewer>;
    let policy = laws::fixture_mounted_policy();
    let actor = "pdf-vt-committed-replay";
    let editor_manifest = || App { definition: create_pdf17_vt_editor(), examples: Vec::new() };
    let viewer_manifest = || App { definition: create_pdf17_vt_viewer(), examples: Vec::new() };
    let mut caller = laws::OriginalFixtureCaller::admit().expect("original replay fixture stock");
    let mut context = caller.context();
    for _ in 0..laws::OriginalFixtureCaller::maximum_turns() {
        let mut scope = None;
        if context.with_native_receiving(&mut scope, |_, _| Ok(())).expect("original replay decoder receives") {
            let receipt = scope.expect("original decoder receipt");
            assert!(receipt.source_refusal.is_none() && receipt.call.is_ok());
            laws::fixture_end_turn(&mut context).expect("original replay decoder settles");
            break;
        }
    }
    assert!(context.native_receiving_identity().is_some());
    let mut editor = semio_plugin::with_authoring_identity!(|identity|
        laws::new_app_with_registry_and_members::<Editor, <Pdf17VtEditor as ArtifactEditor>::Members>(
            editor_manifest, ActorId(actor.into()), policy, &mut identity).await);
    let mut viewer = semio_plugin::with_authoring_identity!(|identity|
        laws::new_app_with_registry_and_members::<Viewer, <Pdf17VtViewer as ArtifactViewer>::Members>(
            viewer_manifest, ActorId(actor.into()), policy, &mut identity).await);
    let instance = laws::meta(actor).instance_id;
    editor.bind_instance_id(instance).await;
    viewer.bind_instance_id(instance).await;

    let result = async {
        let original_objects = editor.stores.store.snapshot_ref().objects.clone();
        let num = original_objects.iter().map(|object| object.id.num).max().unwrap_or(0)
            .checked_add(1).ok_or("the actual VT genesis has no free object number")?;
        let id = ObjRef { num, r#gen: 0 };
        let forward = PdfMutation::InsertObject(InsertObject {
            id, value: PdfObject::Null, index: None, admitted_stream_roles: None,
        });
        let inverse = PdfMutation::RemoveObject(RemoveObject { id, admitted_stream_roles: None });
        let forward_bytes = forward.encode_op().map_err(|error| error.to_string())?;
        let inverse_bytes = inverse.encode_op().map_err(|error| error.to_string())?;
        let applied = semio_plugin::with_authoring_identity!(|identity| editor.stores.store.dispatch(
            ArtifactCommand::Apply { mutations: vec![forward.clone()], transaction: None }, &mut identity).await)
            .map_err(|error| error.to_string())?;
        if editor.stores.store.snapshot_ref().objects.iter().all(|object| object.id != id) {
            return Err(format!("actual VT apply produced no object: {:?}", applied.messages));
        }
        semio_plugin::with_authoring_identity!(|identity| editor.stores.store.dispatch(
            ArtifactCommand::CommitCheckpoint { message: Some("Archive replay witness".into()), authors: Vec::new() }, &mut identity).await)
            .map_err(|error| error.to_string())?;
        let archive = PluginApp::document_archive(&editor).await.map_err(|error| format!("actual document archive: {error:?}"))?;
        let spr = decode_history(&archive.parent_spr, &DecodeOptions::default()).await.map_err(|error| error.to_string())?;
        let event = spr.edits.iter().find(|edit| edit.ops.iter().any(|op| op.binary.as_deref() == Some(forward_bytes.as_slice())))
            .ok_or("the mounted writer omitted the committed forward operation")?;
        if event.inverse.len() != 1 || event.inverse[0].binary.as_deref() != Some(inverse_bytes.as_slice()) {
            return Err("the committed SPR omitted or changed the inverse bytes".into());
        }
        let recorded_id = event.id.clone();
        laws::admit_document_archive_load(&mut viewer, 1, archive, &mut context)
            .map_err(|error| format!("registered viewer load admission: {error:?}"))?;
        semio_plugin::with_authoring_identity!(|identity|
            laws::settle_document_archive_load(&mut viewer, 1, &mut identity, &mut context).await)
            .map_err(|error| format!("registered viewer original replay: {error:?}"))?;
        if viewer.stores.store.snapshot_ref().objects != editor.stores.store.snapshot_ref().objects {
            return Err("the registered viewer reconstructed a different object lane".into());
        }
        let loaded = viewer.stores.store.envelope().vcs.edits.iter().find(|edit| edit.id == recorded_id)
            .ok_or("the exact committed edit was not hydrated")?;
        if !loaded.forwards.iter().eq([&forward]) || !loaded.inverse.iter().eq([&inverse]) {
            return Err("the hydrated forward or inverse changed".into());
        }
        let restored = loaded.inverse.iter().next().cloned().ok_or("the hydrated inverse cannot be applied")?;
        semio_plugin::with_authoring_identity!(|identity| editor.stores.store.dispatch(
            ArtifactCommand::Apply { mutations: vec![restored], transaction: None }, &mut identity).await)
            .map_err(|error| error.to_string())?;
        if editor.stores.store.snapshot_ref().objects != original_objects {
            return Err("applying the hydrated inverse did not restore the original editor object lane".into());
        }
        Ok::<(), String>(())
    }.await;

    let viewer_close = laws::close_registered_fixture_app(&mut viewer, policy, &mut context);
    let editor_close = laws::close_registered_fixture_app(&mut editor, policy, &mut context);
    let remaining = context.fuel_remaining();
    drop(context);
    let caller_close = caller.retire(remaining);
    result.expect("genuine committed PDF/VT forward and inverse replay");
    viewer_close.expect("registered viewer paid close");
    editor_close.expect("registered editor paid close");
    caller_close.expect("original caller paid close");
}
