use super::*;
use crate::app::{ArtifactApp, ArtifactInstanceOperationOwner, ArtifactInstanceOperationOwnerHandle, HistoryView, MediaError, NoTransient, TransientView, ViewerApp};

pub(super) struct MediaOwner {
    label: String,
    closed: bool,
}

impl ArtifactInstanceOperationOwner for MediaOwner {
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
    fn retirement_demands(&self, _body: usize) -> Result<crate::app::RetirementDemand, crate::app::ValueError> {
        Ok(crate::app::RetirementDemand { release_bytes: if self.closed { 0 } else { self.label.capacity() }, depth: usize::from(!self.closed), ..Default::default() })
    }
    fn maintenance_step(&mut self, _grant: crate::app::RetainedCloneGrant) -> Result<crate::app::PluginLifecycleStep, Fault> {
        Ok(crate::app::PluginLifecycleStep::Complete(Default::default()))
    }
    fn close_step(&mut self, grant: crate::app::RetainedCloneGrant) -> Result<crate::app::PluginLifecycleStep, Fault> {
        if !self.closed && (grant.maximum_items == 0 || grant.maximum_release_bytes < self.label.capacity()) {
            return Ok(crate::app::PluginLifecycleStep::Progress(Default::default()));
        }
        let released_bytes = self.label.capacity();
        self.label = String::new();
        self.closed = true;
        Ok(crate::app::PluginLifecycleStep::Complete(crate::app::RetainedCloneProgress { copied_items: 1, released_bytes, ..Default::default() }))
    }
    fn terminal_is_empty(&self) -> bool {
        self.closed && self.label.is_empty()
    }
}

pub(super) fn owner(label: &str) -> Box<dyn ArtifactInstanceOperationOwner> {
    Box::new(MediaOwner { label: label.into(), closed: false })
}

pub(super) fn export(owner: &ArtifactInstanceOperationOwnerHandle, port: &str, doc: &ArtifactView<'_, SurfaceSnapshot>) -> Result<Media, MediaError> {
    owner
        .with_mut::<MediaOwner, _>(|owner| {
            if owner.closed {
                return Err(Fault::new(FaultOrigin::Framework, semio_framework::FaultCode::new("media.owner-closed"), "media owner is closed"));
            }
            let json = serde_json::json!({"owner": owner.label, "count": doc.snapshot.count}).to_string();
            Ok(Media { media_type: MediaType { class: MediaClass::Data, form: MediaForm::Value }, payload: MediaPayload::Structured { schema: "semio.test.media-owner-context/v1".into(), json } })
        })
        .map_err(|error| MediaError::Payload(port.into(), error.message))
}

fn payload(media: Media) -> serde_json::Value {
    let MediaPayload::Structured { schema, json } = media.payload else {
        panic!("structured fixture media");
    };
    assert_eq!(schema, "semio.test.media-owner-context/v1");
    serde_json::from_str(&json).unwrap()
}

/// 🎞️ Editor, viewer, and live VCS exports consume the supplied owner and preserve document defaults.
#[semio_framework_async_macros::async_test]
async fn media_export_request_context_preserves_exact_supplied_owner() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🎞️media-owner-context.json")).unwrap();
    let validator = semio_framework_schema::OwnedJsonSchemaValidator::compile(include_str!("../../🧪️testing/🎞️media-owner-context/🧬️schema/🔣️.json")).unwrap();
    let history = HistoryView::empty();
    let snapshot = SurfaceSnapshot { count: 7 };
    let doc = ArtifactView::new(&snapshot, &history);
    let transient = NoTransient {};
    let transient = TransientView { snapshot: &transient, window: None };
    let context_port = fixture["contextPort"].as_str().unwrap();
    for case in fixture["cases"].as_array().unwrap() {
        let supplied = ArtifactInstanceOperationOwnerHandle::new(owner(case["owner"].as_str().unwrap()));
        assert!(matches!(supplied.close_step(0, 0).unwrap(), PluginCloseStep::Pending { released_items: 0, released_bytes: 0 }));
        assert!(!supplied.terminal_is_empty().unwrap());
        let editor = payload(<EditorApp<SurfaceEditorFixture> as ArtifactApp>::export_media_with_request_context(&supplied, context_port, &doc, &transient).await.unwrap());
        let viewer = payload(<ViewerApp<SurfaceViewerFixture> as ArtifactApp>::export_media_with_request_context(&supplied, context_port, &doc, &transient).await.unwrap());
        assert_eq!(editor, case["expected"]);
        assert_eq!(viewer, case["expected"]);
        validator.validate_json(&editor.to_string()).unwrap();
        validator.validate_json(&viewer.to_string()).unwrap();
        supplied.close_step(1, 4096).unwrap();
        assert!(supplied.terminal_is_empty().unwrap());
        assert!(<EditorApp<SurfaceEditorFixture> as ArtifactApp>::export_media_with_request_context(&supplied, context_port, &doc, &transient).await.unwrap_err().to_string().contains("closed"));
        assert!(<ViewerApp<SurfaceViewerFixture> as ArtifactApp>::export_media_with_request_context(&supplied, context_port, &doc, &transient).await.unwrap_err().to_string().contains("closed"));
    }
    for invalid in fixture["invalid"].as_array().unwrap() {
        assert!(validator.validate_json(&invalid.to_string()).is_err());
    }
    let foreign = ArtifactInstanceOperationOwnerHandle::new(Box::new(crate::app::EmptyArtifactInstanceOperationOwner));
    assert!(<EditorApp<SurfaceEditorFixture> as ArtifactApp>::export_media_with_request_context(&foreign, context_port, &doc, &transient).await.unwrap_err().to_string().contains("type"));
    assert!(<ViewerApp<SurfaceViewerFixture> as ArtifactApp>::export_media_with_request_context(&foreign, context_port, &doc, &transient).await.unwrap_err().to_string().contains("type"));
    foreign.close_step(1, 4096).unwrap();
    let default_port = fixture["defaultPort"].as_str().unwrap();
    let supplied = ArtifactInstanceOperationOwnerHandle::new(owner("defaults"));
    let default_editor = <EditorApp<SurfaceEditorFixture> as ArtifactApp>::export_media_with_request_context(&supplied, default_port, &doc, &transient).await.unwrap();
    let default_viewer = <ViewerApp<SurfaceViewerFixture> as ArtifactApp>::export_media_with_request_context(&supplied, default_port, &doc, &transient).await.unwrap();
    let expected_pack = store::pack_rt::pack_value_to_base64(&store::ArtifactPack::encode_pack(&snapshot));
    for media in [default_editor, default_viewer] {
        assert_eq!(media.media_type, MediaType { class: MediaClass::Data, form: MediaForm::Value });
        let MediaPayload::Structured { schema, json } = media.payload else {
            panic!("default document pack");
        };
        assert_eq!(schema, "semio.testkit-surface/v1");
        assert_eq!(json, expected_pack);
    }
    assert!(matches!(<EditorApp<SurfaceEditorFixture> as ArtifactApp>::export_media_with_request_context(&supplied, "unknown:out", &doc, &transient).await, Err(MediaError::NotImplemented)));
    assert!(matches!(<ViewerApp<SurfaceViewerFixture> as ArtifactApp>::export_media_with_request_context(&supplied, "unknown:out", &doc, &transient).await, Err(MediaError::NotImplemented)));
    supplied.close_step(1, 4096).unwrap();
    let mut editor = new_app::<EditorApp<SurfaceEditorFixture>>(protocol::ActorId(crate::app::LOCAL_ACTOR_ID.into())).await;
    let mut viewer = new_viewer::<SurfaceViewerFixture>(protocol::ActorId(crate::app::LOCAL_ACTOR_ID.into())).await;
    editor
        .instance_operation_owner
        .with_mut::<MediaOwner, _>(|owner| {
            owner.label = "vcs-editor".into();
            Ok(())
        })
        .unwrap();
    viewer
        .instance_operation_owner
        .with_mut::<MediaOwner, _>(|owner| {
            owner.label = "vcs-viewer".into();
            Ok(())
        })
        .unwrap();
    assert_eq!(payload(editor.export_media(context_port).await.unwrap()), serde_json::json!({"owner":"vcs-editor","count":0}));
    assert_eq!(payload(viewer.export_media(context_port).await.unwrap()), serde_json::json!({"owner":"vcs-viewer","count":0}));
    assert_eq!(editor.snapshot().unwrap().count, 0);
    assert_eq!(viewer.snapshot().unwrap().count, 0);
    assert!(editor.export_media(default_port).await.is_ok());
    assert!(viewer.export_media(default_port).await.is_ok());
    assert!(matches!(editor.export_media("unknown:out").await, Err(MediaError::NotImplemented)));
    assert!(matches!(viewer.export_media("unknown:out").await, Err(MediaError::NotImplemented)));
    close_registered_fixture_app(&mut editor);
    close_registered_fixture_app(&mut viewer);
}
