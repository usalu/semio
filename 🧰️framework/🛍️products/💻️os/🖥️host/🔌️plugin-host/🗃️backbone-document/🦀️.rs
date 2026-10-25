//! 🗃️ Backbone document of the plugin host: the backbone document and its value impls, the space and workflow document aliases, the envelope helpers and the pack, text and payload codecs. See [`super`].

use super::*;

// #region host
//#region 🔖️BackboneDocument
/// 🧬️ Generic backbone-document envelope — mirrors the dissolved `OsDocument`'s exact shape
/// (schema/id/name/vcs/transitions/outcomes/conflicts/backbone), parametrized over any `<P, Op>` pair
/// `store::os_store::document_materialize::create_document_envelope`/`materialize_document_snapshot`/`print_document_pack`/
/// `parse_document_pack` already support generically — nothing OS-specific left to hardcode. See
/// `## The inversion` in the plan: `OsSnapshot`/`OsMutation`/`OsDocument` dissolve into the three
/// type aliases below instead of one bespoke studio-only document type.
/// 🌱️ `Serialize`/`Deserialize` dropped outright (not dual-derived): every field already
/// carries `ToValue`/`FromValue` (`ArtifactVcs`, `protocol::MutationEnvelope`, `protocol::
/// EditMessages`/`Conflict`, `ArtifactBackboneRef`), and no caller anywhere in this crate
/// requires `BackboneDocument: Serialize` as a trait bound or calls `serde_json` on one directly
/// — confirmed by reading every `serde_json::to_*`/`from_*` call site in this file before
/// removing the derive. See `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️01/
/// RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS/🔍️research/
/// 📓️os-kernel-serde-final.md` — this was `🌿️vcs`'s own last blocker.
#[derive(Clone, Debug, PartialEq)]
pub struct BackboneDocument<P, Op> {
    pub schema: String,
    pub id: String,
    pub name: String,
    pub vcs: ArtifactVcs<protocol::os_vcs::io::binary::genesis::AdmittedArtifactGenesis<P>, Op>,
    /// 🔀️ The document's history transitions; with `vcs.edits` the complete event log
    /// every position (applied/redo/checkpoint/alternative) is folded from.
    pub transitions: Vec<protocol::MutationEnvelope>,
    pub edit_messages: Vec<protocol::EditMessages>,
    pub conflicts: Vec<protocol::Conflict>,
    pub backbone: Option<ArtifactBackboneRef>,
}

/// 🧬️ `ToValue` twin of the dropped `Serialize` derive — camelCase object shape mirrors the
/// old `#[serde(rename_all = "camelCase")]`; `backbone` is omitted when `None`, matching the
/// old `skip_serializing_if`.
impl<P: ToValue, Op: ToValue> ToValue for BackboneDocument<P, Op> {
    fn to_value(&self) -> DslValue {
        let mut fields = vec![
            ("schema".to_string(), self.schema.to_value()),
            ("id".to_string(), self.id.to_value()),
            ("name".to_string(), self.name.to_value()),
            ("vcs".to_string(), protocol::os_vcs::io::text::vcs::native_vcs_value(&self.vcs)),
            ("transitions".to_string(), self.transitions.to_value()),
            ("editMessages".to_string(), self.edit_messages.to_value()),
            ("conflicts".to_string(), self.conflicts.to_value()),
        ];
        if let Some(backbone) = &self.backbone {
            fields.push(("backbone".to_string(), backbone.to_value()));
        }
        DslValue::Object(fields)
    }
}

/// 🧬️ `FromValue` twin — missing `backbone` decodes to `None`, missing `editMessages`/
/// `conflicts` decode to empty (matching the old `Deserialize`'s implicit `Vec::default()`
/// for an absent, non-`Option` field with no `#[serde(default)]` — those two were never
/// actually optional on the wire in practice, but this preserves exact prior leniency).
impl<P: FromValue + protocol::os_store::ArtifactPack, Op: FromValue> FromValue for BackboneDocument<P, Op> {
    fn from_value(value: DslValue) -> Result<Self, ValueError> {
        let DslValue::Object(fields) = value else {
            return Err(ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("expected an object for BackboneDocument, found {value:?}")));
        };
        let mut schema = None;
        let mut id = None;
        let mut name = None;
        let mut vcs = None;
        let mut transitions = None;
        let mut edit_messages = None;
        let mut conflicts = None;
        let mut backbone = None;
        for (key, entry) in fields {
            match key.as_str() {
                "schema" => schema = Some(String::from_value(entry).map_err(|e| e.under("schema"))?),
                "id" => id = Some(String::from_value(entry).map_err(|e| e.under("id"))?),
                "name" => name = Some(String::from_value(entry).map_err(|e| e.under("name"))?),
                "vcs" => vcs = Some(protocol::os_vcs::io::text::vcs::native_vcs_from_value(entry,&protocol::os_vcs::io::binary::genesis::ArtifactPackGenesisProducer).map_err(|e| e.under("vcs"))?),
                "transitions" => transitions = Some(Vec::from_value(entry).map_err(|e| e.under("transitions"))?),
                "editMessages" => edit_messages = Some(Vec::from_value(entry).map_err(|e| e.under("editMessages"))?),
                "conflicts" => conflicts = Some(Vec::from_value(entry).map_err(|e| e.under("conflicts"))?),
                "backbone" => backbone = Some(ArtifactBackboneRef::from_value(entry).map_err(|e| e.under("backbone"))?),
                _ => {}
            }
        }
        Ok(BackboneDocument {
            schema: schema.ok_or_else(|| ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "BackboneDocument missing schema"))?,
            id: id.ok_or_else(|| ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "BackboneDocument missing id"))?,
            name: name.ok_or_else(|| ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "BackboneDocument missing name"))?,
            vcs: vcs.ok_or_else(|| ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "BackboneDocument missing vcs"))?,
            transitions: transitions.unwrap_or_default(),
            edit_messages: edit_messages.unwrap_or_default(),
            conflicts: conflicts.unwrap_or_default(),
            backbone,
        })
    }
}

impl<P, Op: Mutation<P>> BackboneDocument<P, Op> {
    /// 🧮️ The edits the event log leaves applied, in fold order — what a projection replays.
    pub fn applied_edit_ids(&self) -> Result<Vec<String>, VcsError> {
        let head = protocol::ViewerHead::canonical_trunk(&protocol::ArtifactId(self.id.clone()));
        store::os_store::ledger::fold_event_log::<P, Op>(&self.id, &self.vcs.edits.iter().collect::<Vec<_>>(), &self.transitions, &self.conflicts, &head).map(|fold| fold.applied)
    }
}

/// 🏠️ A space's manifest document — the space-catalog half of the dissolved `OsSnapshot`.
pub type OsSpaceDocument = BackboneDocument<space::SpaceSnapshot, space::SpaceMutation>;

/// 🗂️ One collection's folder/entry tree document.
pub type OsCollectionDocument = BackboneDocument<space::CollectionSnapshot, space::CollectionMutation>;

/// 🕸️ One `s.workflow` artifact document — the workflow-graph half of the dissolved `OsSnapshot`
/// (see the kernel `workflow` crate's `WorkflowSnapshot`).
pub type OsWorkflowArtifactDocument = BackboneDocument<workflow::WorkflowSnapshot, workflow::WorkflowMutation>;

/// 🌉️ Live `ArtifactStore` handle for a space-manifest session — no bespoke wrapper needed (unlike
/// `OsWorkflowStore` below, whose only extra logic is workflow-specific node/parameter id-minting);
/// every generic `ArtifactStore` method (`snapshot`/`dispatch`/`attach_backbone`/...) already
/// applies directly. `OsWorkflowStore::add_workflow_node` dispatches into one of these to install
/// the spawned app's plugin into the owning space's `programs` list.
pub type OsSpaceStore = ArtifactStore<space::SpaceSnapshot, space::SpaceMutation>;

/// 🌱️ Mints a fresh backbone document wrapping `initial_snapshot` with empty edit history.
pub fn create_backbone_document<P, Op>(schema: &str, id: &str, name: &str, initial_snapshot: P) -> BackboneDocument<P, Op>
where
    P: Clone + protocol::os_store::ArtifactPack,
    Op: Clone,
{
    BackboneDocument {
        schema: schema.into(),
        id: id.into(),
        name: name.into(),
        vcs: create_document_envelope::<P, Op>(schema, id, initial_snapshot, None, &::semio_framework_os_kernel::os_vcs::io::binary::genesis::ArtifactPackGenesisProducer).into_owners().vcs,
        transitions: Vec::new(),
        edit_messages: Vec::new(),
        conflicts: Vec::new(),
        backbone: None,
    }
}

/// 🌉️ Builds the authoritative `ArtifactEnvelope` a `BackboneDocument` wraps, dropping only
/// the app-level `name` and preserving its complete event log.
fn backbone_envelope_of<P, Op>(document: &BackboneDocument<P, Op>) -> ArtifactEnvelope<P, Op>
where
    P: Clone,
    Op: Clone,
{
    ArtifactEnvelope::from_owners(store::os_store::store_schema::ArtifactEnvelopeOwners {
        schema: document.schema.clone(),
        id: document.id.clone(),
        vcs: document.vcs.clone(),
        backbone: document.backbone.clone(),
        active_alternative_id: None,
        cursor: None,
        dialect: None,
        migrated_from: None,
        owner: None,
        lanes: protocol::causal::HistoryFoldIndex::new(),
        viewer_checkpoint_id: None,
        edit_messages: store::os_store::ledger::ArtifactEditMessageLedger::from_preflighted_entries(document.edit_messages.clone()),
        conflicts: document.conflicts.clone(),
        transitions: document.transitions.clone(),
        history_shape: protocol::HistoryShape::Document,
        open_transaction: None,
    })
}

/// 🧺️ Moves every backbone owner into the canonical envelope shell.
pub(super) fn take_backbone_envelope<P, Op>(document: BackboneDocument<P, Op>) -> (String, ArtifactEnvelope<P, Op>) {
    let envelope = ArtifactEnvelope::from_owners(store::os_store::store_schema::ArtifactEnvelopeOwners {
        schema: document.schema,
        id: document.id,
        vcs: document.vcs,
        backbone: document.backbone,
        active_alternative_id: None,
        cursor: None,
        dialect: None,
        migrated_from: None,
        owner: None,
        lanes: protocol::causal::HistoryFoldIndex::new(),
        viewer_checkpoint_id: None,
        edit_messages: store::os_store::ledger::ArtifactEditMessageLedger::from_preflighted_entries(document.edit_messages),
        conflicts: document.conflicts,
        transitions: document.transitions,
        history_shape: protocol::HistoryShape::Document,
        open_transaction: None,
    });
    (document.name, envelope)
}

/// 🧺️ Retires a copied backbone envelope through its exact bounded owner catalogue.
pub(super) fn close_backbone_envelope<P, Op>(envelope: ArtifactEnvelope<P, Op>) -> Result<(), VcsError>
where
    P: Clone + ToValue + FromValue + pack::snapshot::ArtifactPack + ::semio_framework_os_kernel::os_store::serialization::ArtifactStoreSnapshot + semio_framework_value::retirement::RetireOwned + Send + Sync + 'static,
    Op: Clone + ToValue + FromValue + Mutation<P> + protocol::OpText + protocol::OpBinary + semio_framework_value::retirement::RetireOwned + Send + store::ArtifactCanonicalJsonTree + 'static,
{
    use store::ErasedSnapshotRetirement as _;
    fn grant_of(demand: semio_framework_value::RetirementDemand) -> semio_framework_value::retained_clone::RetainedCloneGrant {
        semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: demand.copy_bytes, maximum_capacity_bytes: demand.capacity_bytes, maximum_release_bytes: demand.release_bytes, maximum_depth: demand.depth.max(1) }
    }
    let fault = |error: ValueError| VcsError::ValidationFailed(error.to_string());
    let owners = store::os_store::document_store_owners::funded_bounded_artifact_store_owners::<P, Op>().map_err(fault)?;
    let birth = grant_of(owners.uninstalled_envelope_retirement_demands(&envelope));
    let (mut retirement, _) = owners.retire_envelope_uninstalled(envelope, birth).map_err(|(error, _, _)| fault(error))?;
    loop {
        let demand = retirement.next_demand(4096).map_err(fault)?;
        match retirement.close_step(grant_of(demand)).map_err(fault)? {
            semio_framework_value::retained_clone::RetainedCloneStep::Complete(_) => {
                assert!(retirement.terminal_is_empty(), "backbone envelope retirement reports its exact terminal");
                return Ok(());
            }
            semio_framework_value::retained_clone::RetainedCloneStep::Progress(_) => std::thread::yield_now(),
        }
    }
}

/// 🧺️ Lends a copied authoritative envelope, then retires every exact nested owner.
fn with_backbone_envelope<P, Op, R>(document: &BackboneDocument<P, Op>, read: impl FnOnce(&ArtifactEnvelope<P, Op>) -> Result<R, VcsError>) -> Result<R, VcsError>
where
    P: Clone + ToValue + FromValue + pack::snapshot::ArtifactPack + ::semio_framework_os_kernel::os_store::serialization::ArtifactStoreSnapshot + semio_framework_value::retirement::RetireOwned + Send + Sync + 'static,
    Op: Clone + ToValue + FromValue + Mutation<P> + protocol::OpText + protocol::OpBinary + semio_framework_value::retirement::RetireOwned + Send + store::ArtifactCanonicalJsonTree + 'static,
{
    let envelope = backbone_envelope_of(document);
    let result = read(&envelope);
    close_backbone_envelope(envelope)?;
    result
}

pub fn materialize_backbone_snapshot<P, Op>(document: &BackboneDocument<P, Op>, applied_edit_ids: &[String]) -> Result<P, VcsError>
where
    P: Clone + ToValue + FromValue + pack::snapshot::ArtifactPack + ::semio_framework_os_kernel::os_store::serialization::ArtifactStoreSnapshot + semio_framework_value::retirement::RetireOwned + Send + Sync + 'static,
    Op: Clone + ToValue + FromValue + Mutation<P> + store::ColdMutation<P> + protocol::OpText + protocol::OpBinary + semio_framework_value::retirement::RetireOwned + Send + store::ArtifactCanonicalJsonTree + 'static,
{
    with_backbone_envelope(document, |envelope| resolve_kernel_future(materialize_document_snapshot(envelope, applied_edit_ids)))
}

/// 📤️ Exports an already-loaded backbone document as pack bytes + ops text.
pub fn export_backbone_pack<P, Op>(document: &BackboneDocument<P, Op>) -> Result<store::os_store::serialization::ArtifactPackFiles, VcsError>
where
    P: Clone + ToValue + FromValue + pack::snapshot::ArtifactPack + ::semio_framework_os_kernel::os_store::serialization::ArtifactStoreSnapshot + semio_framework_value::retirement::RetireOwned + Send + Sync + 'static,
    Op: Clone + ToValue + FromValue + Mutation<P> + protocol::OpText + protocol::OpBinary + semio_framework_value::retirement::RetireOwned + Send + store::ArtifactCanonicalJsonTree + 'static,
{
    with_backbone_envelope(document, |envelope| resolve_kernel_future(store::os_store::text_format::print_document_pack(envelope)))
}

/// 📤️ DSL-text counterpart of `export_backbone_pack`.
pub fn export_backbone_dsl<P, Op>(document: &BackboneDocument<P, Op>) -> Result<store::os_store::text_format::ArtifactTextFiles, VcsError>
where
    P: Clone + ToValue + FromValue + pack::snapshot::ArtifactPack + ::semio_framework_os_kernel::os_store::serialization::ArtifactStoreSnapshot + semio_framework_value::retirement::RetireOwned + Send + Sync + 'static + pack::snapshot::ArtifactDsl,
    Op: Clone + ToValue + FromValue + Mutation<P> + protocol::OpText + protocol::OpBinary + semio_framework_value::retirement::RetireOwned + Send + store::ArtifactCanonicalJsonTree + 'static,
{
    with_backbone_envelope(document, |envelope| resolve_kernel_future(store::os_store::text_format::print_document_text(envelope)))
}

/// 📦️ Binary pack+spr payload for the whole `BackboneDocument` (name + genesis + event log)
/// — the persisted/synced form. `name` rides as a `store::os_store::serialization::encode_document_pack_bytes`-framed blob
/// wrapping a nested `pack`+`spr` pair; `spr` carries the events, whose fold restores the exact
/// undo/redo/checkpoint position.
pub fn encode_backbone_payload<P, Op>(document: &BackboneDocument<P, Op>) -> Result<Vec<u8>, VcsError>
where
    P: Clone + ToValue + FromValue + pack::snapshot::ArtifactPack + ::semio_framework_os_kernel::os_store::serialization::ArtifactStoreSnapshot + semio_framework_value::retirement::RetireOwned + Send + Sync + 'static,
    Op: Clone + ToValue + FromValue + Mutation<P> + protocol::OpText + protocol::OpBinary + semio_framework_value::retirement::RetireOwned + Send + store::ArtifactCanonicalJsonTree + 'static,
{
    let files = with_backbone_envelope(document, |envelope| resolve_kernel_future(store::os_store::text_format::print_document_pack(envelope)))?;
    let inner = resolve_kernel_future(store::os_store::serialization::encode_document_pack_bytes(&files.pack, &files.spr));
    Ok(resolve_kernel_future(store::os_store::serialization::encode_document_pack_bytes(document.name.as_bytes(), &inner)))
}

/// 📥️ Inverse of `encode_backbone_payload` — `expected_schema` guards against decoding one
/// document kind's bytes as another.
pub fn decode_backbone_payload<P, Op>(bytes: &[u8], expected_schema: &str) -> Result<BackboneDocument<P, Op>, VcsError>
where
    P: Clone + pack::snapshot::ArtifactPack + ::semio_framework_os_kernel::os_store::serialization::ArtifactStoreSnapshot + 'static,
    Op: Clone + protocol::OpText + protocol::OpBinary + Mutation<P> + store::ColdMutation<P> + 'static,
{
    let (name_bytes, inner) = resolve_kernel_future(store::os_store::serialization::decode_document_pack_bytes(bytes))?;
    let name = String::from_utf8(name_bytes).map_err(|error| VcsError::Deserialize(error.to_string()))?;
    let (pack, spr) = resolve_kernel_future(store::os_store::serialization::decode_document_pack_bytes(&inner))?;
    let parsed: store::os_store::text_format::ParsedDocumentText<P, Op> = resolve_kernel_future(store::os_store::text_format::parse_document_pack(&pack, &spr, &::semio_framework_os_kernel::os_vcs::io::binary::genesis::ArtifactPackGenesisProducer)).map_err(|error| VcsError::Deserialize(error.to_string()))?;
    let envelope = parsed.into_envelope();
    if envelope.schema != expected_schema {
        envelope.retire_unadopted();
        return Err(VcsError::Deserialize(format!("expected schema {expected_schema}")));
    }
    let mut owners = envelope.into_owners();
    let mut edit_messages = Vec::with_capacity(owners.edit_messages.len());
    while let Some(entry) = owners.edit_messages.pop() { edit_messages.push(entry); }
    edit_messages.reverse();
    Ok(BackboneDocument { schema: owners.schema, id: owners.id, name, vcs: owners.vcs, transitions: owners.transitions, edit_messages, conflicts: owners.conflicts, backbone: owners.backbone })
}
//#endregion 🔖️BackboneDocument
// #endregion host
