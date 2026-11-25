//! 🧫️ Imports initial JSON through the selected editor's declared native document owner.

use super::*;
use semio_framework::io_schema::IoPayload;
use semio_framework::{ArtifactDialect, IoEntryDirection};
use semio_framework_artifact_reference::io::text::artifact_reference::DialectCoordinateText as _;
use semio_framework_value::ToValue;
use semio_os_mcp_foundation::errors::GatewayErrorCode;
use semio_plugin_host::{PluginRegistration, call_plugin};

#[derive(ToValue)]
struct IoImport {
    source: String,
    target: String,
    payload: IoPayload,
}

impl PluginArtifactChannel {
    /// 🌱️ Loads the guest's native JSON import into its normal archive lane and returns the imported Pack for readback.
    pub fn import_initial_artifact(
        &mut self, instance: u32, artifact_id: &str, initial: &str, scope: &ActivationScope,
    ) -> Result<Vec<u8>, GatewayError> {
        let app = self.descriptor.manifest.apps.iter()
            .find(|app| app.id == self.app_ref.app_id)
            .ok_or_else(|| GatewayError::new(GatewayErrorCode::Internal, "selected artifact owner app is absent"))?;
        let source = ArtifactDialect::parse_coordinate("s.stdio.json@rfc8259/*").expect("authored JSON dialect");
        let json_import = self.descriptor.contributions.io_entries.iter().any(|route| {
            route.direction == IoEntryDirection::Import && route.counterpart == source && route.owner == app.dialect
        });
        if !json_import {
            return Err(GatewayError::new(GatewayErrorCode::InputInvalid, "selected artifact owner declares no JSON import for initial content"));
        }
        if initial.len() > self.host_driver_policy.root.max_patch_bytes as usize {
            return Err(GatewayError::new(GatewayErrorCode::InputInvalid, "initial content exceeds the configured JSON input bound"));
        }
        scope.enter(ActivationPhase::LoadingDocument).map_err(crate::compiled_components::activation_fault)?;
        let driver = self.initial_import_driver(scope)?;
        let request = semio_framework_pack_json::to_json_string(&IoImport {
            source: source.to_coordinate(), target: app.dialect.to_coordinate(), payload: IoPayload::Text(initial.to_owned()),
        });
        let origin = crate::compiled_components::native_instance_origin(self.compiled.package_hash)
            .map_err(|error| GatewayError::new(GatewayErrorCode::Internal, format!("initial import origin: {error:?}")))?;
        let actor = semio_framework_async::poll::resolve_ready(semio_framework_actor::ActorId::new(0xc71, 0, 1, 0));
        let imported = semio_framework_async::block_on(call_plugin(
            &PluginRegistration { actor, origin }, semio_plugin_guest_contract::job_kinds::JOB_KIND_IO_RUN, request.as_bytes(), driver,
        )).map_err(|error| GatewayError::new(GatewayErrorCode::SideEffectRejected, format!("owner JSON import refused: {error}")))?;
        let answer = std::str::from_utf8(&imported).map_err(|error| GatewayError::new(GatewayErrorCode::Internal, error.to_string()))?;
        let payload: IoPayload = semio_framework_pack_json::from_json_str(answer, semio_framework_pack_json::JsonMemberPolicy::Reject)
            .map_err(|error| GatewayError::new(GatewayErrorCode::Internal, error.to_string()))?;
        let IoPayload::Binary(pack) = payload else {
            return Err(GatewayError::new(GatewayErrorCode::PluginUnavailable, "owner JSON import did not supply a native Pack"));
        };
        let schema = &app.io.artifact_schema;
        let spr = semio_framework_async::block_on(async {
            let empty = store::os_store::text_format::empty_document_spr(artifact_id, schema).await;
            store::os_store::text_format::stamp_document_spr_identity(&empty, artifact_id, schema, &app.dialect, None).await
        }).map_err(|error| GatewayError::new(GatewayErrorCode::Internal, error.to_string()))?;
        scope.checkpoint().map_err(crate::compiled_components::activation_fault)?;
        self.load_document_archive(instance, &pack, &spr, scope).map_err(crate::compiled_components::activation_fault)?;
        Ok(pack)
    }

    fn initial_import_driver(&self, scope: &ActivationScope) -> Result<semio_plugin_host::GuestRelayWakeDriver, GatewayError> {
        let cancel = scope.cancel().clone();
        self.operation_caps().issue(implicit_editor_request(DOCUMENT_LOAD_WALL_BUDGET.as_micros() as u64), semio_framework_job::default_now_us, Box::new(move || cancel.is_cancelled_now()))
            .map_err(|error| GatewayError::new(GatewayErrorCode::Internal, format!("initial JSON import Source: {error}")))
    }
}
