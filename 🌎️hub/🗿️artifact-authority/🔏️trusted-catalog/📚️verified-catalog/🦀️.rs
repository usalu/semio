//! 📚️ The process-lifetime verified catalog: packages, codecs, open targets, plugin modules, creation selection and document-open resolution.

use super::*;

/// 🗂️ Process-lifetime snapshot produced only after complete bundle verification and codec activation.
pub struct VerifiedTrustedCatalog {
    pub residency: Arc<GuestResidencyLedgerV1<semio_framework_plugin_interpreter_runtime::OwnedCompiledGuest>>,
    pub(super) guests: Box<[Arc<GuestArtifactComponent>]>,
    pub progress: Arc<TrustedCatalogLoadProgressCellV1>,
    pub packages: Box<[VerifiedTrustedPackage]>,
    pub codecs: Box<[VerifiedNativeArtifactCodec]>,
    pub(super) artifact_kind_count: usize,
    pub open_targets: Box<[VerifiedDocumentOpenSelectionV1]>,
    pub generation_id: String,
}

impl VerifiedTrustedCatalog {
    /// 📦️ Returns selected packages in deterministic dependency-first order.
    pub fn packages(&self) -> &[VerifiedTrustedPackage] {
        &self.packages
    }

    /// 📏️ What the compiled-guest residency holds now and has done since the catalog loaded.
    pub fn guest_residency(&self) -> TrustedCatalogGuestResidencyStateV1 {
        self.residency.state()
    }

    /// 🎚️ Applies the operator's residency budget (`OS_HUB_GUEST_RESIDENCY_BYTES`), releasing least recently
    /// used unheld guests until the resident charge fits it.
    pub fn configure_guest_residency(&self, residency: TrustedCatalogGuestResidencyV1) {
        self.residency.configure(residency.resident_component_bytes);
    }

    /// 📈️ How far this catalog's load and codec-row verification have come.
    pub fn load_progress(&self) -> TrustedCatalogLoadProgressV1 {
        self.progress.snapshot()
    }

    /// 🔐️ Pins every package's rows that still wait for their component's answer, smallest component first, so the
    /// most packages are ready soonest; a codec call that reaches a package first verifies it itself and this
    /// pass waits for it. A refused package is recorded in [`Self::load_progress`] and the pass moves on; the
    /// pass stops at cancellation or a stall of `context`.
    pub async fn verify_pending_guests(&self, context: &OperationContext<'_>) -> Result<(), AuthorityError> {
        let mut pending: Vec<&Arc<GuestArtifactComponent>> = self.guests.iter().filter(|guest| !guest.rows.is_empty() && guest.verified.get().is_none()).collect();
        pending.sort_by_key(|guest| guest.component.byte_length());
        for guest in pending {
            context.checkpoint()?;
            match guest.verified(context).await {
                Ok(()) | Err(AuthorityError::Catalog(_)) => {}
                Err(error) => return Err(error),
            }
        }
        Ok(())
    }

    /// 🧪️ Returns the exact number of activated artifact identities.
    pub fn codec_count(&self) -> usize {
        self.codecs.len()
    }

    /// 🧮️ Returns the distinct artifact kinds across all activated schema-qualified identities.
    pub fn artifact_kind_count(&self) -> usize {
        self.artifact_kind_count
    }

    /// 🪪 Returns the number of exact catalog-backed open choices in this immutable generation.
    pub fn open_target_count(&self) -> usize {
        self.open_targets.len()
    }

    /// 🧬 Returns the neutral SHA-256 identity of the sorted immutable open-target projection.
    pub fn generation_id(&self) -> &str {
        &self.generation_id
    }

    /// 📇️ Every plugin module of this generation in ascending plugin order: what
    /// `GET /trusted-catalog/plugin-modules` answers.
    pub fn plugin_module_index(&self) -> TrustedPluginModuleIndexV1 {
        let mut modules = self
            .packages
            .iter()
            .map(|package| TrustedPluginModuleIndexEntryV1 {
                plugin_id: package.plugin_id.clone(),
                package_id: package.package.package.0.clone(),
                version: package.version.clone(),
                component_sha256: hex_lower(&package.component_sha256),
                descriptor_byte_sha256: hex_lower(&package.descriptor_sha256),
                receipt_sha256: hex_lower(&Sha256::digest(&package.receipt.encode_canonical())),
                dependencies: package.dependencies.clone(),
                dialect_artifact_kinds: package.descriptor.manifest.apps.iter().map(|app| app.dialect.artifact_kind.clone()).collect::<BTreeSet<_>>().into_iter().collect(),
                extends_plugin_id: (package.descriptor.role == semio_framework::PackageRole::Extension).then(|| package.descriptor.manifest.dependencies.first().map(|dependency| dependency.plugin_id.clone())).flatten(),
                bundle_sha256: package.plugin_module.bundle_sha256.clone(),
                bundle_byte_length: package.plugin_module.manifest_bytes.len() as u64,
                entry: package.plugin_module.bundle.entry.clone(),
            })
            .collect::<Vec<_>>();
        modules.sort_by(|left, right| left.plugin_id.as_bytes().cmp(right.plugin_id.as_bytes()));
        TrustedPluginModuleIndexV1 { schema: TRUSTED_PLUGIN_MODULE_INDEX_SCHEMA.into(), generation_id: self.generation_id.clone(), modules }
    }

    /// 🧩️ The verified plugin module whose manifest has this SHA-256, when this generation carries it.
    pub fn plugin_module(&self, bundle_sha256: &str) -> Option<&VerifiedPluginModule> {
        self.packages.iter().map(|package| &package.plugin_module).find(|module| module.bundle_sha256 == bundle_sha256)
    }

    /// 🎯 Returns the profile's sole completely verified document-open choice without reconstructing it from public plan bytes.
    pub fn selected_document_open(&self) -> Option<&VerifiedDocumentOpenSelectionV1> {
        (self.open_targets.len() == 1).then(|| &self.open_targets[0])
    }

    /// 🌱️ Creation resolves one writable target in this exact admitted generation, OWNER-PREFERRED ([`owner_preferred_creation`]):
    /// the editors of packages that declare the kind decide; only a kind no owner's editor creates falls to the editors of packages
    /// that HOST it (`PluginManifest::hosted_artifact_kinds`). Hosting opens documents, it never re-owns their creation
    /// (demonstrator's embedded cad/gis/procedural/process editors made every creation of those kinds ambiguous, hub 7800 p34,
    /// 2026-09-29). Within the deciding tier the most general dialect creates ([`most_general_dialect`]): a kind whose subset
    /// editors share it with their whole-standard editor (stdio json, xml; the families' jpg, svg, tiff, step, docx, zip, ifc)
    /// is created by the whole-standard editor — the same app the guest answers `codec.genesis` with.
    pub fn artifact_creation_selection(&self, kind_id: &str) -> Option<&VerifiedDocumentOpenSelectionV1> {
        let (hosted, owned): (Vec<_>, Vec<_>) = self.open_targets.iter().filter(|selection| selection.artifact.kind == kind_id && selection.surface.role == DocumentOpenSurfaceRoleV1::Editor).partition(|selection| self.hosted_row(selection).is_some());
        owner_preferred_creation(owned, hosted, |selection| &selection.parent_dialect).one()
    }

    /// 📦️ The verified package `selection` opens its documents in.
    fn selection_package(&self, selection: &VerifiedDocumentOpenSelectionV1) -> Option<&VerifiedTrustedPackage> {
        self.packages.iter().find(|package| {
            package.plugin_id == selection.package.plugin_id
                && package.package.package.0 == selection.package.package_id
                && package.version == selection.package.version
                && hex_lower(&package.component_sha256) == selection.package.component_sha256
        })
    }

    /// 🏠️ The row by which `selection`'s own package HOSTS its kind (`PluginManifest::hosted_artifact_kinds`); `None` for a
    /// kind the package declares itself.
    fn hosted_row(&self, selection: &VerifiedDocumentOpenSelectionV1) -> Option<&semio_framework::HostedArtifactKind> {
        self.selection_package(selection)?.descriptor.manifest.hosted_artifact_kinds.iter().find(|kind| kind.id == selection.artifact.kind && kind.schema == selection.artifact.schema)
    }

    /// 🏠️ The package that OWNS `selection`'s hosted kind: the hosted row's explicit `owner`, a declared dependency the load
    /// verified before its host.
    fn hosted_kind_owner(&self, selection: &VerifiedDocumentOpenSelectionV1) -> Option<&VerifiedTrustedPackage> {
        let hosted = self.hosted_row(selection)?;
        self.packages.iter().find(|package| package.plugin_id == hosted.owner)
    }

    /// 🗣️ Projects only unambiguous factory-backed choices from retained compiled descriptors. A hosted kind is labelled by its
    /// owner's declaration (a host carries only `{ id, schema, owner }`); a kind no declaration labels is not offered.
    pub fn artifact_creation_catalog(&self, space_id: &str) -> Option<directory::os_directory::schema::space_artifact_creation::SpaceArtifactCreationCatalogV1> {
        use directory::os_directory::schema::space_artifact_creation::{
            SpaceArtifactCreationCatalogV1, SpaceArtifactCreationDialectV1, SpaceArtifactCreationKindV1, SpaceArtifactCreationLabelV1,
        };
        let kind_ids = self.open_targets.iter().map(|selection| selection.artifact.kind.as_str()).collect::<BTreeSet<_>>();
        let mut kinds = Vec::new();
        for kind_id in kind_ids {
            let Some(selection) = self.artifact_creation_selection(kind_id) else { continue };
            let retained = self.selection_package(selection)?;
            let app = retained.descriptor.manifest.apps.iter().find(|app| app.id == selection.surface.app_id && app.dialect == selection.parent_dialect)?;
            let owner = || self.hosted_kind_owner(selection).and_then(|owner| declared_kind(owner.descriptor.manifest.artifact_kinds.iter().chain(owner.descriptor.manifest.apps.iter().flat_map(|app| app.artifact_kinds.iter())), selection));
            let Some(kind) = declared_kind(app.artifact_kinds.iter().chain(retained.descriptor.manifest.artifact_kinds.iter()), selection).or_else(owner) else { continue };
            kinds.push(SpaceArtifactCreationKindV1 {
                kind_id: selection.artifact.kind.clone(),
                schema: selection.artifact.schema.clone(),
                dialect: SpaceArtifactCreationDialectV1 {
                    artifact_kind: selection.parent_dialect.artifact_kind.clone(),
                    standard: selection.parent_dialect.standard.clone(),
                    subset: selection.parent_dialect.subset.clone(),
                },
                label: SpaceArtifactCreationLabelV1 {
                    en: kind.label.resolve(semio_framework::Terminology::Native, semio_framework::Locale::En).to_string(),
                    de: kind.label.resolve(semio_framework::Terminology::Native, semio_framework::Locale::De).to_string(),
                },
            });
        }
        kinds.sort_by(|left, right| left.kind_id.cmp(&right.kind_id));
        let catalog = SpaceArtifactCreationCatalogV1 { schema: "semio.hub.space-artifact-creation-catalog/v1".into(), space_id: space_id.into(), catalog_generation_id: self.generation_id.clone(), kinds };
        catalog.validate().then_some(catalog)
    }

    /// 🧱 Returns the verified bytes of the current selection only. It is deliberately not a package
    /// lookup: it accepts no package id, digest, path or generation selector from a caller, resolves
    /// the selection from the durable descriptor and subject role alone, and answers only while the
    /// caller-observed generation is still this immutable catalog's own.
    pub fn assets_for_current_selection(&self, descriptor: &DocumentDescriptor, requested_surface_id: Option<&str>, writable: bool, current_generation: &str) -> Option<VerifiedExecutionTargetAssets> {
        if current_generation != self.generation_id {
            return None;
        }
        let selection = self.resolve_document_open(descriptor, requested_surface_id, writable)?;
        let package = self.packages.iter().find(|retained| {
            retained.plugin_id == selection.package.plugin_id
                && retained.package.package.0 == selection.package.package_id
                && retained.version == selection.package.version
                && hex_lower(&retained.component_sha256) == selection.package.component_sha256
                && hex_lower(&retained.package.hash.0) == selection.package.component_blake3
                && hex_lower(&retained.descriptor_sha256) == selection.package.descriptor_byte_sha256
        })?;
        if package.component.byte_length() == 0 || package.descriptor_bytes.is_empty() || package.component.byte_length() > TRUSTED_COMPONENT_MAX_BYTES || package.descriptor_bytes.len() as u64 > TRUSTED_DESCRIPTOR_MAX_BYTES {
            return None;
        }
        let browser_actor = match (&selection.browser_actor, &package.browser_actor, &package.browser_actor_asset) {
            (DocumentOpenBrowserActorV1::None, _, _) if !matches!(selection.surface.renderer_target, DocumentOpenRendererTargetV1::Wasm) => None,
            (selected, retained, Some(asset)) if selected == retained && matches!(selected, DocumentOpenBrowserActorV1::ClosedBrowserActor { .. }) && asset.byte_length() != 0 && asset.byte_length() <= DOCUMENT_BROWSER_ACTOR_MAX_BYTES => {
                Some(asset.clone())
            }
            _ => return None,
        };
        Some(VerifiedExecutionTargetAssets { selection, component: package.component.clone(), descriptor: Arc::clone(&package.descriptor_bytes), browser_actor })
    }

    /// 🎯 Resolves one exact descriptor, subject role, and optional surface preference without fallback. Without a preference
    /// the role's most general surface opens the document ([`most_general_dialect`]): a json document opens in the
    /// `rfc8259/*` editor, never refused because an `i-json` editor opens it too (the semio MCP workspace names no surface).
    pub fn resolve_document_open(&self, descriptor: &DocumentDescriptor, requested_surface_id: Option<&str>, writable: bool) -> Option<VerifiedDocumentOpenSelectionV1> {
        let role = if writable { DocumentOpenSurfaceRoleV1::Editor } else { DocumentOpenSurfaceRoleV1::Viewer };
        let matches = self.open_targets.iter().filter(|selection| {
            selection.package.plugin_id == descriptor.owner.plugin_id
                && selection.package.package_id == descriptor.owner.package_id
                && selection.package.version == descriptor.owner.version
                && selection.package.component_sha256 == descriptor.owner.package_hash
                && selection.artifact.kind == descriptor.artifact_kind
                && selection.artifact.schema == descriptor.artifact_schema
                && selection.artifact.pack_schema_hash == descriptor.pack_schema_hash
                && selection.surface.role == role
                && requested_surface_id.is_none_or(|requested| selection.surface.surface_id == requested)
        });
        most_general_dialect(matches.collect(), |selection| &selection.parent_dialect).one().cloned()
    }
}

impl TrustedArtifactCatalog for VerifiedTrustedCatalog {
    type Codec = VerifiedNativeArtifactCodec;

    async fn resolve<'a>(&'a self, required: &TrustedArtifactIdentity) -> Result<&'a Self::Codec, AuthorityError> {
        self.codecs.iter().find(|entry| &entry.identity == required).ok_or_else(|| AuthorityError::Catalog("descriptor identity is absent from the verified trusted catalog".to_string()))
    }
}
