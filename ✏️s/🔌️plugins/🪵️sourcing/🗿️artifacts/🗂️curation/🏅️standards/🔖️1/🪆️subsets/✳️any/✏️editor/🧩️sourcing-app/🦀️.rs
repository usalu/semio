//! 🧩️ Sourcing app: the `ArtifactEditor` implementation of `SourcingCurationApp` with its dispatch, render, media and composition bodies.

use super::*;
use semio_framework_2d_compute::EngineHandles;
use crate::editor::sourcing::editor_vocabulary::{SOURCING_CURATION_CONFIG_CONTRIBUTIONS_BYTES, SOURCING_ROWS_DOMAIN, reset_document_effect, sourcing_curation_io};

impl ArtifactEditor for SourcingCurationApp {
    /// 🧩️ Composes `s.stdio.semio@v1/*` children, so every bundle of this surface opens them through the same roster.
    type Members = semio_s_artifact_stdio_semio::SemioMembers;
    type Snapshot = CurationSnapshot;
    type Mutation = SourcingMutation;
    type Config = SourcingCurationConfig;
    type ConfigMutation = SourcingCurationConfigMutation;
    type Draft = NoDraft;
    type DraftMutation = NoDraftMutation;
    type Presence = SourcingCurationPresence;
    type PresenceMutation = SourcingCurationPresenceMutation;
    type Transient = semio_framework_plugin::NoTransient;
    type TransientMutation = semio_framework_plugin::NoTransientMutation;

    type Command = SourcingCurationCommand;

    fn build_artifact_store_one_item_preparation_factory() -> Option<std::sync::Arc<dyn store::os_store::one_item_publication::ArtifactStoreOneItemPreparationFactory<Self::Snapshot, Self::Mutation>>> {
        Some(std::sync::Arc::new(SourcingCurationArtifactPreparationFactory))
    }

    fn build_config_store_one_item_preparation_factory() -> Option<std::sync::Arc<dyn store::os_store::one_item_publication::ArtifactStoreOneItemPreparationFactory<Self::Config, Self::ConfigMutation>>> {
        Some(std::sync::Arc::new(SourcingCurationConfigPreparationFactory))
    }

    const DIALECT: Dialect = crate::SOURCING_DIALECT;
    const DOCUMENT_SCHEMA: &'static str = SOURCING_CURATION_SCHEMA;

    fn child_restore_projection(snapshot: &Self::Snapshot) -> Result<semio_framework_composition_admission::ChildRestoreProjection<'_>, Fault> {
        crate::curation_child_restore_projection(snapshot)
    }

    fn genesis_child_pack(snapshot: &Self::Snapshot, slot: &str, child_id: &str) -> Result<Option<Vec<u8>>,semio_framework_value::ValueError> {
 Ok((||{
        crate::genesis_catalog_pack(snapshot, slot, child_id)
    
})())
}

    semio_framework_plugin::bounded_first_step_tool_proofs! {
        owner: EditorApp<SourcingCurationApp>,
        owner_file: "✏️s/🔌️plugins/🪵️sourcing/🗿️artifacts/🗂️curation/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs",
        controller: "s.sourcing.curation@1/*#editor",
        artifact_schema: "sourcing.curation/v1",
        factory: "SourcingCurationBoundedCommandJobFactory",
        factory_type: SourcingCurationBoundedCommandJobFactory,
        tools: {
            "setActiveExample" => sourcing_curation_bounded_contract(),
            "loadDocumentJson" => sourcing_curation_bounded_contract(),
            "stockFromCatalogue" => sourcing_curation_bounded_contract(),
            "curationAdd" => sourcing_curation_bounded_contract(),
            "curationSetCount" => sourcing_curation_bounded_contract(),
            "curationRemove" => sourcing_curation_bounded_contract(),
            "dropOnPool" => sourcing_curation_bounded_contract(),
            "dropOnCurated" => sourcing_curation_bounded_contract(),
            "setFilterQuery" => sourcing_curation_bounded_contract(),
            "setFilterModule" => sourcing_curation_bounded_contract(),
            "setFilterTypology" => sourcing_curation_bounded_contract(),
            "setFilterMinAvailability" => sourcing_curation_bounded_contract(),
            "sortTable" => sourcing_curation_bounded_contract(),
            "setContributions" => sourcing_curation_bounded_contract(),
            "setGridInstanceDisplay" => sourcing_curation_bounded_contract(),
        }
    }

    fn register_window_config_owners(registry: &mut semio_framework_plugin::WindowConfigOwnerRegistry) -> Result<(), Fault> {
        grid_config::register(registry)
    }

    fn register_tool_job_factories(registry: &mut ArtifactToolFactoryRegistry<'_, EditorApp<Self>>) -> Result<(), Fault> {
        let controller = registry.controller_id().to_string();
        registry.register(SourcingCurationBoundedCommandJobFactory::new(&controller))
    }

    fn build_tool_job(request: &mut Option<ArtifactOwnedToolJobRequest<EditorApp<Self>>>, output: &mut Option<semio_framework_plugin::ErasedToolJob>, cx: &mut semio_framework_plugin::__job::StepContext<'_>) -> Result<bool, semio_framework_plugin::__job::OriginalTurnFailure> {
        let original = request.as_ref().ok_or_else(|| semio_framework_plugin::__value::ValueError::literal(semio_framework_plugin::__value::ValueRefusalKind::InvariantViolated, "original retained producer request is absent"))?;
        let bounded = SOURCING_CURATION_BOUNDED_TOOL_IDS.contains(&original.tool_id.as_str());
        if !bounded {
            return Ok(false);
        }
        if original.command.command_id() != original.tool_id {
            return Err(semio_framework_plugin::__value::ValueError::literal(semio_framework_plugin::__value::ValueRefusalKind::InvariantViolated, "sourcing-curation-command-tool-mismatch").into());
        }
        let tool_id = original.command.command_id();
        semio_framework_plugin::retained_command::producer::admit_retained_tool_producer(request, output, cx, SourcingCurationCommand::command_id, sourcing_curation_retained_raw_bytes(tool_id), SOURCING_CURATION_RETAINED_WORK_ITEMS, false, |request| BoundedArtifactCommandWork::new(tool_id, sourcing_curation_retained_reduce, sourcing_curation_bounded_extent))
    }

    fn app_schema() -> Option<::semio_framework_schema_registry::AppSchemaDescriptor> {
        Some(crate::editor::sourcing::config::schema::app_schema_descriptor())
    }

    fn initial_snapshot() -> CurationSnapshot {
        crate::standards::v1::subsets::any::io::text::snapshot::default_document()
    }

    fn io() -> Option<semio_framework_plugin::AppIo> {
        Some(sourcing_curation_io())
    }

    /// 🎞️ `catalog:out` (see `crate::schema::inferences::sourcing_catalog_fragment`)
    /// plus the inherited `document:out` default (the pack of `doc.snapshot`, replicated inline —
    /// overriding `export_media` shadows the trait's provided body for every port on this app, not just
    /// the new one).
    fn export_media(port: &str, doc: &ArtifactView<'_, CurationSnapshot>) -> Result<Media, MediaError> {
        match port {
            "catalog:out" => Ok(Media {
                media_type: MediaType { class: MediaClass::Kit, form: MediaForm::Type },
                payload: MediaPayload::Structured { schema: "kit.catalog".into(), json: semio_framework_pack_json::to_json_string(&crate::schema::inferences::sourcing_catalog_fragment(doc.snapshot)) },
            }),
            "artifact:out" => {
                let media_type = Self::io().map_or(MediaType { class: MediaClass::Data, form: MediaForm::Value }, |io| io.artifact_media_type);
                let bytes = doc.snapshot.encode_pack();
                Ok(Media { media_type, payload: MediaPayload::Structured { schema: Self::DOCUMENT_SCHEMA.to_string(), json: store::pack_rt::pack_value_to_base64(&bytes) } })
            }
            _ => Err(MediaError::NotImplemented),
        }
    }

    /// 🧬️ Whole-document replace has no mutation: `import_media` builds a `Effect::LoadDocument` via `reset_document_effect`,
    /// outside undo history.
    fn import_media(port: &str, media: &Media, _doc: &ArtifactView<'_, CurationSnapshot>) -> Result<Emit<SourcingMutation, SourcingCurationConfigMutation, Self::DraftMutation>, MediaError> {
        if port != "artifact:in" {
            return Err(MediaError::NotImplemented);
        }
        let MediaPayload::Structured { json, .. } = &media.payload else {
            return Err(MediaError::Payload(port.to_string(), "document:in importer only accepts a Structured (base64 pack) payload".into()));
        };
        let bytes = store::pack_rt::pack_value_from_base64(json).map_err(|error| MediaError::Payload(port.to_string(), error.to_string()))?;
        let snapshot = <CurationSnapshot as ArtifactPack>::decode_pack(&bytes).map_err(|error| MediaError::Payload(port.to_string(), error.to_string()))?;
        Ok(Emit { effects: vec![reset_document_effect(&snapshot)], ..Default::default() })
    }

    /// 🏷️ The manifest action id each command was declared under — supplied wholesale by
    /// not a user-facing action).
    fn command_id(command: &SourcingCurationCommand) -> &'static str {
        command.command_id()
    }

    /// 🎯️ Production action bridge — see `sourcing_curation_command_from_action`. Overriding this is
    /// mandatory for any app that declares its own actions: the trait default only admits the
    /// framework-reserved ids and rejects everything else.
    fn command_from_action(action: &str, args: &mut Option<semio_framework_value::DslValue>, original_fault: &mut Option<Fault>) -> Result<SourcingCurationCommand, Fault> {
        if let Some(fault) = original_fault.take() { return Err(fault); }
        let args = args.as_ref();
        sourcing_curation_command_from_action(action, args)
    }

    fn host_configuration_mutation(action: &str, args: Option<&semio_framework_value::DslValue>) -> Result<Option<Self::ConfigMutation>, Fault> {
        Ok((action == "setContributions").then(|| SourcingCurationConfigMutation::SetContributions(SetContributionsEdit { json: crate::standards::v1::subsets::any::io::text::snapshot::installable_contributions(args.and_then(|value| value.get("json")).and_then(semio_framework_value::DslValue::as_str).unwrap_or("[]"), SOURCING_CURATION_CONFIG_CONTRIBUTIONS_BYTES), })))
    }

    fn handle(
        command: &SourcingCurationCommand,
        doc: &ArtifactView<'_, CurationSnapshot>,
        cfg: &ConfigView<'_, SourcingCurationConfig>,
        _interaction: &InteractionView<'_>,
        view_state: Option<&semio_framework_plugin::ViewModel>,
        _draft: &DraftView<'_, Self::Draft>,
        _engines: &EngineHandles,
    ) -> Result<Emit<SourcingMutation, SourcingCurationConfigMutation, Self::DraftMutation>, Fault> {
        let emit = command.dispatch(doc, cfg)?;
        sourcing_curation_finish_emit(command, emit, cfg.window, view_state)
    }

    fn window_measures(_doc: &ArtifactView<'_, CurationSnapshot>, cfg: &ConfigView<'_, SourcingCurationConfig>, view_state: &semio_framework_plugin::ViewModel) -> HashMap<String, Vec<WindowMeasure>> {
        let grid_cfg = grid_config::current(cfg);
        HashMap::from([(grid::SOURCING_CURATION_WINDOW_GRID.into(), grid::window_measures(&grid_cfg, sourcing_curation_labels(view_state)))])
    }

    fn render(body_key: &str, doc: &ArtifactView<'_, CurationSnapshot>, cfg: &ConfigView<'_, SourcingCurationConfig>, view_state: &semio_framework_plugin::ViewModel) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::ComponentTree> {
        let snapshot = doc.snapshot;
        let config = cfg.snapshot;
        let labels = sourcing_curation_labels(view_state);
        let grid_window = grid_config::current(cfg);
        match body_key {
            pool::SOURCING_CURATION_BODY_POOL => pool::render(snapshot, config, labels).map(semio_framework_plugin::built_to_component_tree),
            curated::SOURCING_CURATION_BODY_CURATED => curated::render(snapshot, config, labels).map(semio_framework_plugin::built_to_component_tree),
            preview::SOURCING_CURATION_BODY_PREVIEW => preview::render(snapshot, &[], labels).map(semio_framework_plugin::built_to_component_tree),
            grid::SOURCING_CURATION_BODY_GRID => grid::render(snapshot, config, &grid_window).map(semio_framework_plugin::built_to_component_tree),
            _ => semio_framework_plugin::built_text_to_component_tree(Label::data("")),
        }
    }

    /// 🕹️ The Preview window follows the framework-owned `rows` selection a table row or grid pick makes;
    /// every other body renders exactly as [`Self::render`].
    fn render_with_request_context(
        _owner: &semio_framework_plugin::ArtifactInstanceOperationOwnerHandle,
        body_key: &str,
        doc: &ArtifactView<'_, CurationSnapshot>,
        cfg: &ConfigView<'_, SourcingCurationConfig>,
        view_state: &semio_framework_plugin::ViewModel,
        _transient: &semio_framework_plugin::TransientView<'_, semio_framework_plugin::NoTransient>,
        interaction: &InteractionView<'_>,
        cx:&mut semio_framework_plugin::StepContext<'_>,
    ) -> semio_framework_plugin::OriginalRenderResult<semio_framework_plugin::ComponentTree> {
        let rendered:semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::ComponentTree>=(||{
        if body_key == preview::SOURCING_CURATION_BODY_PREVIEW {
            let selected_ids = interaction.selection(SOURCING_ROWS_DOMAIN).ids.clone();
            return preview::render(doc.snapshot, &selected_ids, sourcing_curation_labels(view_state)).map(semio_framework_plugin::built_to_component_tree);
        }
        Self::render(body_key, doc, cfg, view_state)

        })();
        rendered.map(Some).map_err(Into::into)
        }
}
