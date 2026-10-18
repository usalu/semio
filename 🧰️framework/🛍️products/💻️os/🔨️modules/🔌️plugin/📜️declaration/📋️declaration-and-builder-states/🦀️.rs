//! 📋️ Declaration and builder states: the artifact declaration, its runtime capability requirements and the declaration builder with its schema and ready states. See [`super`].

use super::*;

/// 🔖️ Everything an artifact declares as data for `PluginBuilder::try_build()` — see
/// `.🧬semio/🦑️repo/🎫️tickets/26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE/📓️w1-mechanism-design.md`. Every
/// field mirrors exactly one global registration function reachable from plugin code (census:
/// `📓️w0-d-sdk-surface.md` §6); the app-scoped `register_app_schema_descriptor` has no field
/// here because it belongs to `.document_app`, while flow extensions use plugin-level
/// `FlowExtensionDeclaration` rather than a setup registrar. Built
/// only through `ArtifactDeclaration::builder(definition)`, a consuming typestate builder mirroring
/// `PluginBuilder`'s own shape, so a declaration missing its mandatory `schema` is a compile
/// error rather than a runtime panic. Every field is module-private — a plugin crate can
/// describe a declaration through the builder's methods but never read or hand-assemble one;
/// only the transactional assembly plan ever walks it.
pub struct ArtifactDeclaration {
    #[doc(hidden)]
    pub kind: String,
    pub schemas: Vec<::semio_framework_schema_registry::ArtifactSchemaDescriptor>,
    pub inferences: Vec<::semio_framework_schema_registry::ArtifactInferenceDescriptor>,
    pub inference_services: Vec<ArtifactInferenceService>,
    pub composers: Vec<&'static ComposerEntry>,
    pub formats: Vec<semio_framework_os_kernel::io::FormatDescriptor>,
    pub subset_validators: Vec<&'static SubsetValidatorEntry>,
    pub languages: Vec<&'static semio_framework_dsl::LanguageSpec>,
    pub document_codecs: Vec<DocumentCodecSpec>,
    pub migrations: Vec<store::os_store::codec_registry::DialectMigration>,
    pub schema_documents: Vec<::semio_framework_schema_registry::ScopeSchemaExports>,
    /// 🧒️🔗️ Pulled from `<Snapshot as ArtifactCompositionFields>::{child_slots,link_slots}` via
    /// `.composition::<Snapshot>()` — never settable directly (UCAS review, 2026-08-12): the
    /// derive-emitted trait impl IS the truth, so a hand-written list would be unwritable rather
    /// than merely discouraged. No registration function consumes these yet (UCAS's composition
    /// runtime reads `ArtifactCompositionFields` straight off the snapshot type on demand); they
    /// are captured here so the declaration is a complete, single-source manifest of the
    /// artifact's shape, not because `register_all` calls anything with them today.
    #[allow(dead_code, reason = "manifest-completeness capture, no reader yet — see comment above")]
    #[doc(hidden)]
    pub child_slots: &'static [::semio_framework_schema_composition::ChildSlotSpec],
    #[allow(dead_code, reason = "manifest-completeness capture, no reader yet — see comment above")]
    #[doc(hidden)]
    pub link_slots: &'static [::semio_framework_schema_composition::LinkSlotSpec],
    pub capabilities: Vec<CapabilityRequirement>,
    pub definition: ArtifactDefinition,
    #[doc(hidden)]
    pub definition_error: Option<ArtifactDefinitionError>,
}

/// 🧾️ One exact immutable capability claim-set required by a declared runtime facet.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ArtifactRuntimeCapabilityRequirement {
    pub(super) kind: ArtifactCapabilityKind,
    pub(super) claims: Vec<ArtifactIdentityClaim>,
}

impl ArtifactRuntimeCapabilityRequirement {
    pub(super) fn new(kind: ArtifactCapabilityKind, mut claims: Vec<ArtifactIdentityClaim>) -> Self {
        claims.sort();
        Self { kind, claims }
    }

    /// 🏷️ Returns the strict runtime capability category.
    pub async fn kind(&self) -> &ArtifactCapabilityKind {
        &self.kind
    }

    /// 🧷️ Returns the complete canonical claim set; partial matches are never valid.
    pub async fn claims(&self) -> &[ArtifactIdentityClaim] {
        &self.claims
    }
}

/// 🏷️ Declaration builder has a definition only — next call must be `.schema(...)`.
pub struct NeedsSchema;

/// ✅️ Declaration builder has a definition + `schema` — ready for every other facet plus `.try_build()`.
pub struct DeclarationReady;

/// 🏗️ Consuming typestate builder for [`ArtifactDeclaration`] — mirrors [`PluginBuilder`]'s own
/// missing-mandatory-field-is-a-compile-error shape.
pub struct ArtifactDeclarationBuilder<State> {
    pub(super) kind: String,
    pub schemas: Vec<::semio_framework_schema_registry::ArtifactSchemaDescriptor>,
    pub inferences: Vec<::semio_framework_schema_registry::ArtifactInferenceDescriptor>,
    pub inference_services: Vec<ArtifactInferenceService>,
    pub composers: Vec<&'static ComposerEntry>,
    pub formats: Vec<semio_framework_os_kernel::io::FormatDescriptor>,
    pub subset_validators: Vec<&'static SubsetValidatorEntry>,
    pub languages: Vec<&'static semio_framework_dsl::LanguageSpec>,
    pub document_codecs: Vec<DocumentCodecSpec>,
    pub migrations: Vec<store::os_store::codec_registry::DialectMigration>,
    pub schema_documents: Vec<::semio_framework_schema_registry::ScopeSchemaExports>,
    pub(super) child_slots: &'static [::semio_framework_schema_composition::ChildSlotSpec],
    pub(super) link_slots: &'static [::semio_framework_schema_composition::LinkSlotSpec],
    pub capabilities: Vec<CapabilityRequirement>,
    pub definition: ArtifactDefinition,
    pub(super) definition_error: Option<ArtifactDefinitionError>,
    _state: std::marker::PhantomData<State>,
}

impl ArtifactDeclaration {
    /// 🪪️ Starts a declaration from its one authoritative artifact definition.
    pub fn builder(definition: ArtifactDefinition) -> ArtifactDeclarationBuilder<NeedsSchema> {
        let kind = definition.identity.value.clone();
        ArtifactDeclarationBuilder {
            kind,
            schemas: Vec::new(),
            inferences: Vec::new(),
            inference_services: Vec::new(),
            composers: Vec::new(),
            formats: Vec::new(),
            subset_validators: Vec::new(),
            languages: Vec::new(),
            document_codecs: Vec::new(),
            migrations: Vec::new(),
            schema_documents: Vec::new(),
            child_slots: &[],
            link_slots: &[],
            capabilities: Vec::new(),
            definition,
            definition_error: None,
            _state: std::marker::PhantomData,
        }
    }
}

impl ArtifactDeclarationBuilder<NeedsSchema> {
    /// 🧬️ Sets the artifact's four-facet schema descriptor — mandatory, so this is the one call
    /// that unlocks every other declaration method.
    pub fn schema(self, descriptor: ::semio_framework_schema_registry::ArtifactSchemaDescriptor) -> ArtifactDeclarationBuilder<DeclarationReady> {
        ::semio_framework_async::poll::resolve_ready(self.schema_async(descriptor))
    }

    async fn schema_async(mut self, descriptor: ::semio_framework_schema_registry::ArtifactSchemaDescriptor) -> ArtifactDeclarationBuilder<DeclarationReady> {
        require_declared_capability_or_record(&self.definition, &mut self.definition_error, &ArtifactCapabilityKind::schema(), ArtifactIdentityClaim::new(ArtifactIdentityNamespace::schema(), descriptor.id).map(|claim| vec![claim]));
        ArtifactDeclarationBuilder {
            kind: self.kind,
            schemas: vec![descriptor],
            inferences: self.inferences,
            inference_services: self.inference_services,
            composers: self.composers,
            formats: self.formats,
            subset_validators: self.subset_validators,
            languages: self.languages,
            document_codecs: self.document_codecs,
            migrations: self.migrations,
            schema_documents: self.schema_documents,
            child_slots: self.child_slots,
            link_slots: self.link_slots,
            capabilities: self.capabilities,
            definition: self.definition,
            definition_error: self.definition_error,
            _state: std::marker::PhantomData,
        }
    }
}
