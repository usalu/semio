//! 📜️ Artifact declarations: the declaration builder, document codecs, capability sources, host media handlers and flow extensions.
use semio_framework::kernel::CapabilityRequirement;
use serde_json::Value;
use std::collections::{BTreeMap, BTreeSet};
/// 🚪️ `io`'s `ArtifactRef`/`ArtifactKindId` vocabulary is not glob-re-exported at the
/// `semio-framework-os-kernel` crate root (deliberate — see that crate's own glue.rs comment on
/// the `io` mount), so it is named through the `store::io::` path everywhere in this file,
/// exactly like the sibling `🎞️gif` migration leaf (`semio_framework_artifact_reference::ArtifactDialect`) already does.
use semio_framework_artifact_reference::ArtifactKindId;
use pack::snapshot::ArtifactPack;
/// 🗂️ `OsMediaCapability`/`ArtifactKindSpec` now live in `semio-framework-core` (both this crate and
/// `semio-framework-os` already depend on it) the same way the legacy format enum used to — re-exported
/// here verbatim instead of duplicated, so `AppBuilder::artifact_kind(...)` and
/// `semio_framework_os`'s artifact catalog registry share one definition.
use semio_framework::ArtifactKindSpec;
/// 🏅️🪆️🎯️ Standards/subsets dialect vocabulary (ticket 26/08/10/STDIO-ARTIFACTS-AND-IO phase
/// 2). Defined in `semio_framework` so plugins and the OS product
/// share one definition without an inverted dependency; re-exported here verbatim.
use {semio_framework_artifact_reference::{ArtifactDialect, Dialect}, semio_framework_os_kernel::io::{ComposerEntry, SubsetValidatorEntry}};
use crate::app_contract::ArtifactApp;
use crate::artifact_identity::ArtifactCapabilityKind;
use crate::artifact_identity::ArtifactDefinition;
use crate::sdk_fault::ArtifactDefinitionError;
use crate::artifact_identity::ArtifactDefinitionRegistry;
use crate::artifact_identity::ArtifactIdentity;
use crate::artifact_identity::ArtifactIdentityClaim;
use crate::artifact_identity::ArtifactIdentityNamespace;
use crate::sdk_fault::PluginAssemblyError;
use crate::artifact_identity::preflight_artifact_identity;
use crate::artifact_identity::require_declared_capability_or_record;
use crate::artifact_inference::ArtifactInferenceService;



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
    kind: String,
    pub(crate) schemas: Vec<::semio_framework_schema_registry::ArtifactSchemaDescriptor>,
    pub(crate) inferences: Vec<::semio_framework_schema_registry::ArtifactInferenceDescriptor>,
    pub(crate) inference_services: Vec<ArtifactInferenceService>,
    pub(crate) composers: Vec<&'static ComposerEntry>,
    pub(crate) formats: Vec<semio_framework_os_kernel::io::FormatDescriptor>,
    pub(crate) subset_validators: Vec<&'static SubsetValidatorEntry>,
    pub(crate) languages: Vec<&'static semio_framework_dsl::LanguageSpec>,
    pub(crate) document_codecs: Vec<DocumentCodecSpec>,
    pub(crate) migrations: Vec<store::os_store::codec_registry::DialectMigration>,
    pub(crate) schema_documents: Vec<::semio_framework_schema_registry::ScopeSchemaExports>,
    /// 🧒️🔗️ Pulled from `<Snapshot as ArtifactCompositionFields>::{child_slots,link_slots}` via
    /// `.composition::<Snapshot>()` — never settable directly (UCAS review, 2026-08-12): the
    /// derive-emitted trait impl IS the truth, so a hand-written list would be unwritable rather
    /// than merely discouraged. No registration function consumes these yet (UCAS's composition
    /// runtime reads `ArtifactCompositionFields` straight off the snapshot type on demand); they
    /// are captured here so the declaration is a complete, single-source manifest of the
    /// artifact's shape, not because `register_all` calls anything with them today.
    #[allow(dead_code, reason = "manifest-completeness capture, no reader yet — see comment above")]
    child_slots: &'static [::semio_framework_schema_composition::ChildSlotSpec],
    #[allow(dead_code, reason = "manifest-completeness capture, no reader yet — see comment above")]
    link_slots: &'static [::semio_framework_schema_composition::LinkSlotSpec],
    pub(crate) capabilities: Vec<CapabilityRequirement>,
    pub(crate) definition: ArtifactDefinition,
    definition_error: Option<ArtifactDefinitionError>,
}

/// 🧾️ One exact immutable capability claim-set required by a declared runtime facet.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ArtifactRuntimeCapabilityRequirement {
    kind: ArtifactCapabilityKind,
    claims: Vec<ArtifactIdentityClaim>,
}

impl ArtifactRuntimeCapabilityRequirement {
    fn new(kind: ArtifactCapabilityKind, mut claims: Vec<ArtifactIdentityClaim>) -> Self {
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
    kind: String,
    pub(crate) schemas: Vec<::semio_framework_schema_registry::ArtifactSchemaDescriptor>,
    pub(crate) inferences: Vec<::semio_framework_schema_registry::ArtifactInferenceDescriptor>,
    pub(crate) inference_services: Vec<ArtifactInferenceService>,
    pub(crate) composers: Vec<&'static ComposerEntry>,
    pub(crate) formats: Vec<semio_framework_os_kernel::io::FormatDescriptor>,
    pub(crate) subset_validators: Vec<&'static SubsetValidatorEntry>,
    pub(crate) languages: Vec<&'static semio_framework_dsl::LanguageSpec>,
    pub(crate) document_codecs: Vec<DocumentCodecSpec>,
    pub(crate) migrations: Vec<store::os_store::codec_registry::DialectMigration>,
    pub(crate) schema_documents: Vec<::semio_framework_schema_registry::ScopeSchemaExports>,
    child_slots: &'static [::semio_framework_schema_composition::ChildSlotSpec],
    link_slots: &'static [::semio_framework_schema_composition::LinkSlotSpec],
    pub(crate) capabilities: Vec<CapabilityRequirement>,
    pub(crate) definition: ArtifactDefinition,
    definition_error: Option<ArtifactDefinitionError>,
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

impl ArtifactDeclarationBuilder<DeclarationReady> {
    /// 🧾️ Lists exact runtime capability rows without attempting plugin assembly.
    pub async fn runtime_capability_requirements(&self) -> Result<Vec<ArtifactRuntimeCapabilityRequirement>, ArtifactDefinitionError> {
        ArtifactCapabilitySources {
            schemas: &self.schemas,
            inferences: &self.inferences,
            inference_services: &self.inference_services,
            composers: &self.composers,
            formats: &self.formats,
            subset_validators: &self.subset_validators,
            languages: &self.languages,
            document_codecs: &self.document_codecs,
            schema_documents: &self.schema_documents,
        }
        .requirements()
    }

    /// 🧬️ Appends further schema descriptors for independently versioned standards or profiles.
    pub fn schemas(self, items: impl IntoIterator<Item = ::semio_framework_schema_registry::ArtifactSchemaDescriptor>) -> Self {
        ::semio_framework_async::poll::resolve_ready(self.schemas_async(items))
    }

    async fn schemas_async(mut self, items: impl IntoIterator<Item = ::semio_framework_schema_registry::ArtifactSchemaDescriptor>) -> Self {
        for item in items {
            require_declared_capability_or_record(&self.definition, &mut self.definition_error, &ArtifactCapabilityKind::schema(), ArtifactIdentityClaim::new(ArtifactIdentityNamespace::schema(), item.id).map(|claim| vec![claim]));
            self.schemas.push(item);
        }
        self
    }

    /// 💡️ Appends inference descriptors for transactional registration. Repeatable.
    pub fn inferences(self, items: impl IntoIterator<Item = ::semio_framework_schema_registry::ArtifactInferenceDescriptor>) -> Self {
        ::semio_framework_async::poll::resolve_ready(self.inferences_async(items))
    }

    async fn inferences_async(mut self, items: impl IntoIterator<Item = ::semio_framework_schema_registry::ArtifactInferenceDescriptor>) -> Self {
        for item in items {
            require_declared_capability_or_record(&self.definition, &mut self.definition_error, &ArtifactCapabilityKind::inference(), ArtifactIdentityClaim::new(ArtifactIdentityNamespace::schema(), item.id).map(|claim| vec![claim]));
            self.inferences.push(item);
        }
        self
    }

    /// 🧠️ Appends executable native inference services. Each service is ownership- and
    /// schema-checked against this declaration before deterministic registry insertion.
    pub fn inference_services(self, items: impl IntoIterator<Item = ArtifactInferenceService>) -> Self {
        ::semio_framework_async::poll::resolve_ready(self.inference_services_async(items))
    }

    async fn inference_services_async(mut self, items: impl IntoIterator<Item = ArtifactInferenceService>) -> Self {
        for service in items {
            let metadata = service.metadata();
            require_declared_capability_or_record(
                &self.definition,
                &mut self.definition_error,
                &ArtifactCapabilityKind::inference(),
                ArtifactIdentityClaim::new(ArtifactIdentityNamespace::schema(), metadata.inference_schema).map(|claim| vec![claim]),
            );
            self.inference_services.push(service);
        }
        self
    }

    /// 🎹️ Appends this artifact's composer table for transactional registration.
    pub fn composers(self, entries: &'static [ComposerEntry]) -> Self {
        ::semio_framework_async::poll::resolve_ready(self.composers_async(entries))
    }

    async fn composers_async(mut self, entries: &'static [ComposerEntry]) -> Self {
use semio_framework_artifact_reference::io::text::artifact_reference::{DialectCoordinateText as _};

        for entry in entries {
            require_declared_capability_or_record(
                &self.definition,
                &mut self.definition_error,
                &ArtifactCapabilityKind::composer(),
                ArtifactIdentityClaim::new(ArtifactIdentityNamespace::dialect(), ArtifactDialect::from(entry.writes).to_coordinate()).map(|claim| vec![claim]),
            );
            self.composers.push(entry);
        }
        self
    }

    /// 🗂️ Appends format rows (`register_format_descriptors`).
    pub fn formats(self, rows: impl IntoIterator<Item = semio_framework_os_kernel::io::FormatDescriptor>) -> Self {
        ::semio_framework_async::poll::resolve_ready(self.formats_async(rows))
    }

    async fn formats_async(mut self, rows: impl IntoIterator<Item = semio_framework_os_kernel::io::FormatDescriptor>) -> Self {
        for row in rows {
            let mut claims = Vec::with_capacity(row.mimes.len() + row.extensions.len());
            let mut claim_error = None;
            for mime in row.mimes.iter() {
                match ArtifactIdentityClaim::new(ArtifactIdentityNamespace::mime(), mime.clone()) {
                    Ok(claim) => claims.push(claim),
                    Err(error) => {
                        claim_error = Some(error);
                        break;
                    }
                }
            }
            if claim_error.is_none() {
                for extension in row.extensions.iter() {
                    match ArtifactIdentityClaim::new(ArtifactIdentityNamespace::extension(), extension.clone()) {
                        Ok(claim) => claims.push(claim),
                        Err(error) => {
                            claim_error = Some(error);
                            break;
                        }
                    }
                }
            }
            let claims_result = match claim_error {
                Some(error) => Err(error),
                None => Ok(claims),
            };
            require_declared_capability_or_record(&self.definition, &mut self.definition_error, &ArtifactCapabilityKind::representation(), claims_result);
            self.formats.push(row);
        }
        self
    }

    /// 🧾️ Appends this artifact's subset-validator table (`register_subset_validator`, one call per entry).
    pub fn subset_validators(self, entries: &'static [SubsetValidatorEntry]) -> Self {
        ::semio_framework_async::poll::resolve_ready(self.subset_validators_async(entries))
    }

    async fn subset_validators_async(mut self, entries: &'static [SubsetValidatorEntry]) -> Self {
use semio_framework_artifact_reference::io::text::artifact_reference::{DialectCoordinateText as _};

        for entry in entries {
            require_declared_capability_or_record(
                &self.definition,
                &mut self.definition_error,
                &ArtifactCapabilityKind::subset_validator(),
                ArtifactIdentityClaim::new(ArtifactIdentityNamespace::validated_dialect(), ArtifactDialect::from(entry.dialect).to_coordinate()).map(|claim| vec![claim]),
            );
            self.subset_validators.push(entry);
        }
        self
    }

    /// 📖️ Appends this artifact's grammar table (`register_language`, one call per entry).
    pub fn languages(self, specs: &'static [semio_framework_dsl::LanguageSpec]) -> Self {
        ::semio_framework_async::poll::resolve_ready(self.languages_async(specs))
    }

    async fn languages_async(mut self, specs: &'static [semio_framework_dsl::LanguageSpec]) -> Self {
        for spec in specs {
            require_declared_capability_or_record(&self.definition, &mut self.definition_error, &ArtifactCapabilityKind::grammar(), ArtifactIdentityClaim::new(ArtifactIdentityNamespace::grammar(), spec.id).map(|claim| vec![claim]));
        }
        self.languages.extend(specs);
        self
    }

    /// 🗂️ Declares the document codec for one document-owning `ArtifactApp`
    /// (`register_document_codec_for_app::<A>`, keyed by `A::DOCUMENT_SCHEMA`). Repeatable:
    /// every schema-owned codec is retained and conflict-checked before runtime registration.
    pub fn document_codec<A: ArtifactApp>(self) -> Self
    where
        A::Mutation: Sync,
    {
        ::semio_framework_async::poll::resolve_ready(self.document_codec_async::<A>())
    }

    async fn document_codec_async<A: ArtifactApp>(mut self) -> Self
    where
        A::Mutation: Sync,
    {
        let codec = DocumentCodecSpec::of::<A>().await;
        let namespace_codec = ArtifactIdentityNamespace::codec();
        let claims = match ArtifactIdentityClaim::new(namespace_codec, codec.schema.clone()) {
            Ok(schema) => match ArtifactIdentityClaim::codec_extension(&codec.schema, codec.extension) {
                Ok(extension) => Ok(vec![schema, extension]),
                Err(error) => Err(error),
            },
            Err(error) => Err(error),
        };
        require_declared_capability_or_record(&self.definition, &mut self.definition_error, &ArtifactCapabilityKind::codec(), claims);
        self.document_codecs.push(codec);
        self
    }

    /// 🗂️ Sibling of `.document_codec::<A>()` for a library artifact with ZERO `ArtifactApp`s to
    /// bind to (ticket 26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE W1d, gap A — see
    /// `📓️w1d-declaration-gaps-report.md`). `.document_codec::<A>()` is keyed off `A::DOCUMENT_SCHEMA`
    /// and calls `register_document_codec_for_app::<A>`, which requires a real `ArtifactApp`; a
    /// headless library plugin (energy's `EnergyModelSnapshot`/`EnergyModelMutation` is the
    /// motivating case) has no such type. Same bounds as `store::artifact_codec`; it appends
    /// to the same plural codec capability set as `.document_codec::<A>()`.
    pub fn document_codec_bare<Snapshot, Mutation>(self, schema: impl Into<String>, dialect: Dialect) -> Self
    where
        Snapshot: Clone + PartialEq + semio_framework_value::ToValue + semio_framework_value::FromValue + semio_framework_value::retirement::RetireOwned + Send + Sync + pack::snapshot::ArtifactDsl + ArtifactPack + ::semio_framework_os_kernel::os_store::serialization::ArtifactStoreSnapshot + 'static,
        Mutation: ::protocol::Mutation<Snapshot> + PartialEq + semio_framework_value::ToValue + semio_framework_value::FromValue + semio_framework_value::retirement::RetireOwned + Send + Sync + ::protocol::OpText + ::protocol::OpBinary + 'static + store::ArtifactCanonicalJsonTree,
    {
        ::semio_framework_async::poll::resolve_ready(self.document_codec_bare_async::<Snapshot, Mutation>(schema, dialect))
    }

    async fn document_codec_bare_async<Snapshot, Mutation>(mut self, schema: impl Into<String>, dialect: Dialect) -> Self
    where
        Snapshot: Clone + PartialEq + semio_framework_value::ToValue + semio_framework_value::FromValue + semio_framework_value::retirement::RetireOwned + Send + Sync + pack::snapshot::ArtifactDsl + ArtifactPack + ::semio_framework_os_kernel::os_store::serialization::ArtifactStoreSnapshot + 'static,
        Mutation: ::protocol::Mutation<Snapshot> + PartialEq + semio_framework_value::ToValue + semio_framework_value::FromValue + semio_framework_value::retirement::RetireOwned + Send + Sync + ::protocol::OpText + ::protocol::OpBinary + 'static + store::ArtifactCanonicalJsonTree,
    {
        let codec = DocumentCodecSpec::bare::<Snapshot, Mutation>(schema, dialect);
        let namespace_codec = ArtifactIdentityNamespace::codec();
        let claims = match ArtifactIdentityClaim::new(namespace_codec, codec.schema.clone()) {
            Ok(schema) => match ArtifactIdentityClaim::codec_extension(&codec.schema, codec.extension) {
                Ok(extension) => Ok(vec![schema, extension]),
                Err(error) => Err(error),
            },
            Err(error) => Err(error),
        };
        require_declared_capability_or_record(&self.definition, &mut self.definition_error, &ArtifactCapabilityKind::codec(), claims);
        self.document_codecs.push(codec);
        self
    }

    /// 📚️ Appends shared schema documents of this artifact — named exports of its own scope that its facets `$ref`
    /// and the four fixed facets cannot carry — committed into the OS-wide export registry with the other catalogs, so
    /// every contract referencing them resolves. One `schema` capability row per export, claiming it as
    /// [`ArtifactIdentityClaim::schema_export`].
    pub fn schema_documents(self, documents: ::semio_framework_schema_registry::ScopeSchemaExports) -> Self {
        ::semio_framework_async::poll::resolve_ready(self.schema_documents_async(documents))
    }

    async fn schema_documents_async(mut self, documents: ::semio_framework_schema_registry::ScopeSchemaExports) -> Self {
        for export in documents.exports {
            require_declared_capability_or_record(&self.definition, &mut self.definition_error, &ArtifactCapabilityKind::schema(), ArtifactIdentityClaim::schema_export(documents.scope, export.id).map(|claim| vec![claim]));
        }
        self.schema_documents.push(documents);
        self
    }

    /// 🧭️ Appends dialect migrations for transactional registration.
    pub fn migrations(mut self, items: impl IntoIterator<Item = store::os_store::codec_registry::DialectMigration>) -> Self {
        self.migrations.extend(items);
        self
    }

    /// 🧒️🔗️ Pulls `child_slots`/`link_slots` from `<Snapshot as ArtifactCompositionFields>` — the
    /// ONLY way to set them (UCAS review, 2026-08-12: a hand-written list could silently disagree
    /// with the `#[derive(ArtifactSchema)]`-emitted impl the composition runtime actually reads,
    /// so there is deliberately no other setter).
    pub fn composition<Snapshot: ::semio_framework_schema_composition::ArtifactCompositionFields>(mut self) -> Self {
        self.child_slots = Snapshot::child_slots();
        self.link_slots = Snapshot::link_slots();
        self
    }

    /// 🔒️ Declares a capability requirement owned by this artifact (unioned into
    /// `PluginManifest.capabilities` at assembly time) — IO is artifact-owned, so an artifact
    /// that reads assets declares this on itself, not on the plugin.
    pub fn capability(mut self, capability: CapabilityRequirement) -> Self {
        if !self.capabilities.contains(&capability) {
            self.capabilities.push(capability);
        }
        self
    }

    /// ✅️ Validates the declaration-local contract before assembly.
    pub fn try_build(self) -> Result<ArtifactDeclaration, ArtifactDefinitionError> {
        if let Some(error) = self.definition_error {
            return Err(error);
        }
        Ok(ArtifactDeclaration {
            kind: self.kind,
            schemas: self.schemas,
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
            definition_error: None,
        })
    }
}

/// 🗂️ A monomorphized, non-capturing thunk pairing a `schema` string with the fn pointer that
/// registers a document codec under it — lets `.document_codec::<A>()`/`.document_codec_bare()`
/// store the registration as inert data instead of performing it immediately, matching every
/// other declaration field's "described now, committed once by `PluginBuilder::try_build()`" contract. The
/// `schema` sits alongside the thunk (not baked into it) because `bare`'s schema is a runtime
/// `impl Into<String>`, not a type-level const like `A::DOCUMENT_SCHEMA` — a `fn()` thunk cannot
/// close over it without capturing, which would break the "plain fn pointer" contract this type
/// exists to keep.
pub struct DocumentCodecSpec {
    schema: String,
    pub(crate) dialect: Dialect,
    extension: &'static str,
    codec: fn(String) -> store::os_store::serialization::ArtifactCodec,
    pub(crate) app_id: String,
    foreign: bool,
}

impl DocumentCodecSpec {
    pub(crate) async fn of<A: ArtifactApp>() -> Self
    where
        A::Mutation: Sync,
    {
        fn codec<A: ArtifactApp>(schema: String) -> store::os_store::serialization::ArtifactCodec
        where
            A::Mutation: Sync,
        {
            store::bare_artifact_codec::<A::Snapshot, A::Mutation>(schema)
        }
        let app = A::default();
        DocumentCodecSpec { schema: A::DOCUMENT_SCHEMA.to_string(), dialect: A::DIALECT, extension: <A::Snapshot as pack::snapshot::ArtifactDsl>::EXTENSION, codec: codec::<A>, app_id: app.instance_id().await.to_string(), foreign: false }
    }

    pub(crate) fn foreign<A: ArtifactApp>(schema: impl Into<String>) -> Self
    where
        A::Mutation: Sync,
    {
        fn codec<A: ArtifactApp>(schema: String) -> store::os_store::serialization::ArtifactCodec
        where
            A::Mutation: Sync,
        {
            store::bare_artifact_codec::<A::Snapshot, A::Mutation>(schema)
        }
        let app = A::default();
        let app_id = ::semio_framework_async::poll::resolve_ready(app.instance_id()).to_string();
        DocumentCodecSpec { schema: schema.into(), dialect: A::DIALECT, extension: <A::Snapshot as pack::snapshot::ArtifactDsl>::EXTENSION, codec: codec::<A>, app_id, foreign: true }
    }

    /// 🗂️ `of::<A>()`'s app-less sibling — see `.document_codec_bare()`'s own doc for why this
    /// exists. Registers straight against `store::os_store::codec_registry::register_document_codec`, bypassing
    /// `register_document_codec_for_app`'s `A::` indirection since there is no `A` to name.
    pub(crate) fn bare<Snapshot, Mutation>(schema: impl Into<String>, dialect: Dialect) -> Self
    where
        Snapshot: Clone + PartialEq + semio_framework_value::ToValue + semio_framework_value::FromValue + semio_framework_value::retirement::RetireOwned + Send + Sync + pack::snapshot::ArtifactDsl + ArtifactPack + ::semio_framework_os_kernel::os_store::serialization::ArtifactStoreSnapshot + 'static,
        Mutation: ::protocol::Mutation<Snapshot> + PartialEq + semio_framework_value::ToValue + semio_framework_value::FromValue + semio_framework_value::retirement::RetireOwned + Send + Sync + ::protocol::OpText + ::protocol::OpBinary + 'static + store::ArtifactCanonicalJsonTree,
    {
        fn codec<Snapshot, Mutation>(schema: String) -> store::os_store::serialization::ArtifactCodec
        where
            Snapshot: Clone + PartialEq + semio_framework_value::ToValue + semio_framework_value::FromValue + semio_framework_value::retirement::RetireOwned + Send + Sync + pack::snapshot::ArtifactDsl + ArtifactPack + ::semio_framework_os_kernel::os_store::serialization::ArtifactStoreSnapshot + 'static,
            Mutation: ::protocol::Mutation<Snapshot> + PartialEq + semio_framework_value::ToValue + semio_framework_value::FromValue + semio_framework_value::retirement::RetireOwned + Send + Sync + ::protocol::OpText + ::protocol::OpBinary + 'static + store::ArtifactCanonicalJsonTree,
        {
            store::bare_artifact_codec::<Snapshot, Mutation>(schema)
        }
        DocumentCodecSpec { schema: schema.into(), dialect, extension: <Snapshot as pack::snapshot::ArtifactDsl>::EXTENSION, codec: codec::<Snapshot, Mutation>, app_id: String::new(), foreign: false }
    }

    pub(crate) fn codec(&self) -> store::os_store::serialization::ArtifactCodec {
        (self.codec)(self.schema.clone())
    }

    pub(crate) fn binding(&self, channel: semio_framework_os_kernel::io::ArtifactCodecBindingChannel, contributor: &str, artifact_kind: Option<String>, codec: &store::os_store::serialization::ArtifactCodec) -> semio_framework_os_kernel::io::ArtifactCodecBinding {
        let apps = if self.app_id.is_empty() { Vec::new() } else { vec![self.app_id.to_string()] };
        semio_framework_os_kernel::io::ArtifactCodecBinding::from_codec(channel, contributor, artifact_kind, apps, Some(self.dialect.into()), None, codec)
    }

    pub(crate) fn preflight_foreign(&self, document_app_ids: &BTreeSet<String>) -> Result<(), PluginAssemblyError> {
        if !self.foreign {
            return Ok(());
        }
        if self.schema.trim().is_empty() || self.extension.trim().is_empty() {
            return Err(PluginAssemblyError::new("plugin-assembly.foreign-document-codec-schema", "foreign document codecs require non-empty schema and extension"));
        }
        if !document_app_ids.contains(self.app_id.as_str()) {
            return Err(PluginAssemblyError::new("plugin-assembly.foreign-document-codec-owner", format!("foreign document codec for app {:?} requires that app to be declared by this plugin", self.app_id)));
        }
        semio_framework_os_kernel::io::io_mechanism::preflight_native_snapshots(&semio_framework_os_kernel::io::io_mechanism::NativeSnapshotRegistration::from_capability(self.dialect.into(), self.codec()).into_iter().collect::<Vec<_>>())
            .map_err(|error| PluginAssemblyError::new("plugin-assembly.foreign-snapshot", error.to_string()))?;
        Ok(())
    }
}

/// 🧾️ Borrowed runtime declarations inspected for exact capability requirements.
#[derive(Clone, Copy)]
struct ArtifactCapabilitySources<'a> {
    schemas: &'a [::semio_framework_schema_registry::ArtifactSchemaDescriptor],
    inferences: &'a [::semio_framework_schema_registry::ArtifactInferenceDescriptor],
    inference_services: &'a [ArtifactInferenceService],
    composers: &'a [&'static ComposerEntry],
    formats: &'a [semio_framework_os_kernel::io::FormatDescriptor],
    subset_validators: &'a [&'static SubsetValidatorEntry],
    languages: &'a [&'static semio_framework_dsl::LanguageSpec],
    document_codecs: &'a [DocumentCodecSpec],
    schema_documents: &'a [::semio_framework_schema_registry::ScopeSchemaExports],
}

impl ArtifactCapabilitySources<'_> {
    fn requirements(self) -> Result<Vec<ArtifactRuntimeCapabilityRequirement>, ArtifactDefinitionError> {
use semio_framework_artifact_reference::io::text::artifact_reference::{DialectCoordinateText as _};

        let Self { schemas, inferences, inference_services, composers, formats, subset_validators, languages, document_codecs, schema_documents } = self;
        let mut rows = Vec::new();
        for schema in schemas {
            rows.push(ArtifactRuntimeCapabilityRequirement::new(ArtifactCapabilityKind::schema(), vec![ArtifactIdentityClaim::new(ArtifactIdentityNamespace::schema(), schema.id)?]));
        }
        for inference in inferences {
            rows.push(ArtifactRuntimeCapabilityRequirement::new(ArtifactCapabilityKind::inference(), vec![ArtifactIdentityClaim::new(ArtifactIdentityNamespace::schema(), inference.id)?]));
        }
        for service in inference_services {
            rows.push(ArtifactRuntimeCapabilityRequirement::new(ArtifactCapabilityKind::inference(), vec![ArtifactIdentityClaim::new(ArtifactIdentityNamespace::schema(), service.metadata().inference_schema)?]));
        }
        for entry in composers {
            rows.push(ArtifactRuntimeCapabilityRequirement::new(ArtifactCapabilityKind::composer(), vec![ArtifactIdentityClaim::new(ArtifactIdentityNamespace::dialect(), ArtifactDialect::from(entry.writes).to_coordinate())?]));
        }
        for row in formats {
            let mut claims = Vec::with_capacity(row.mimes.len() + row.extensions.len());
            for mime in row.mimes.iter() {
                claims.push(ArtifactIdentityClaim::new(ArtifactIdentityNamespace::mime(), mime.clone())?);
            }
            for extension in row.extensions.iter() {
                claims.push(ArtifactIdentityClaim::new(ArtifactIdentityNamespace::extension(), extension.clone())?);
            }
            rows.push(ArtifactRuntimeCapabilityRequirement::new(ArtifactCapabilityKind::representation(), claims));
        }
        for entry in subset_validators {
            rows.push(ArtifactRuntimeCapabilityRequirement::new(ArtifactCapabilityKind::subset_validator(), vec![ArtifactIdentityClaim::new(ArtifactIdentityNamespace::validated_dialect(), ArtifactDialect::from(entry.dialect).to_coordinate())?]));
        }
        for spec in languages {
            rows.push(ArtifactRuntimeCapabilityRequirement::new(ArtifactCapabilityKind::grammar(), vec![ArtifactIdentityClaim::new(ArtifactIdentityNamespace::grammar(), spec.id)?]));
        }
        for codec in document_codecs {
            rows.push(ArtifactRuntimeCapabilityRequirement::new(
                ArtifactCapabilityKind::codec(),
                vec![ArtifactIdentityClaim::new(ArtifactIdentityNamespace::codec(), codec.schema.clone())?, ArtifactIdentityClaim::codec_extension(&codec.schema, codec.extension)?],
            ));
        }
        for documents in schema_documents {
            for export in documents.exports {
                rows.push(ArtifactRuntimeCapabilityRequirement::new(ArtifactCapabilityKind::schema(), vec![ArtifactIdentityClaim::schema_export(documents.scope, export.id)?]));
            }
        }
        Ok(rows)
    }
}

impl ArtifactDeclaration {
    /// 📚️ Returns this declaration's one authoritative artifact definition.
    pub fn definition(&self) -> &ArtifactDefinition {
        &self.definition
    }

    /// 🏠️ The descriptor rows a package hosting this declaration opens: its kind with each document schema its codecs
    /// decode, owned by the plugin of its canonical kind (`s.<owner>.<artifact>`, the grammar every declaration carries) —
    /// see [`semio_framework::HostedArtifactKind`].
    pub fn hosted_kinds(&self) -> Result<Vec<semio_framework::HostedArtifactKind>, PluginAssemblyError> {
        let kind = ArtifactKindId::parse(self.kind.as_str()).map_err(|_| PluginAssemblyError::new("plugin-assembly.artifact-kind", "artifact identity must use canonical <domain>.<plugin>.<artifact> grammar"))?;
        Ok(self.document_codecs.iter().map(|codec| semio_framework::HostedArtifactKind { id: self.definition.identity().as_str().to_string(), schema: codec.schema.clone(), owner: kind.plugin().to_string() }).collect())
    }

    /// 🧾️ Reads each exact declared dialect and its first-party document codec without publishing it.
    pub fn document_codec_bindings(&self) -> Vec<(Dialect, store::os_store::serialization::ArtifactCodec)> {
        self.document_codecs.iter().map(|spec| (spec.dialect, spec.codec())).collect()
    }

    /// 🪪️ Reads each original codec together with its declaration and optional app context.
    pub fn document_codec_contexts(&self, channel: semio_framework_os_kernel::io::ArtifactCodecBindingChannel, contributor: &str) -> Vec<semio_framework_os_kernel::io::ArtifactCodecBinding> {
        self.document_codecs.iter().map(|spec| spec.binding(channel, contributor, Some(self.definition.identity().as_str().to_string()), &spec.codec())).collect()
    }

    /// 🧾️ Lists every exact runtime capability required by this immutable declaration.
    pub fn runtime_capability_requirements(&self) -> Result<Vec<ArtifactRuntimeCapabilityRequirement>, ArtifactDefinitionError> {
        ArtifactCapabilitySources {
            schemas: &self.schemas,
            inferences: &self.inferences,
            inference_services: &self.inference_services,
            composers: &self.composers,
            formats: &self.formats,
            subset_validators: &self.subset_validators,
            languages: &self.languages,
            document_codecs: &self.document_codecs,
            schema_documents: &self.schema_documents,
        }
        .requirements()
    }

    /// 🛡️ Inserts every attached definition into a caller-owned plugin registry. The caller
    /// owns the transaction boundary so independently declared artifacts are checked together
    /// before any runtime registration can mutate its legacy registry.
    fn register_definitions(&self, registry: &mut ArtifactDefinitionRegistry) -> Result<(), ArtifactDefinitionError> {
        if let Some(error) = &self.definition_error {
            return Err(error.clone());
        }
        registry.register(self.definition.clone())?;
        Ok(())
    }

    /// 🛡️ Validates this declaration into one plugin-owned registry without side effects.
    pub(crate) fn preflight(&self, plugin_id: &str, definitions: &mut ArtifactDefinitionRegistry) -> Result<(), PluginAssemblyError> {
        preflight_artifact_identity(plugin_id, &self.kind)?;
        self.register_definitions(definitions).map_err(PluginAssemblyError::definition)?;
        self.preflight_channels()?;
        for service in &self.inference_services {
            let metadata = service.metadata();
            if metadata.owner != plugin_id || metadata.artifact_kind != self.kind {
                return Err(PluginAssemblyError::new("plugin-assembly.inference-owner", format!("inference service {:?}/{:?} does not belong to plugin {plugin_id:?} artifact {:?}", metadata.owner, metadata.artifact_kind, self.kind)));
            }
            if !self.schemas.iter().any(|schema| metadata.artifact_schema == schema.id) {
                return Err(PluginAssemblyError::new("plugin-assembly.inference-schema", format!("inference service {:?} has no declared artifact schema", metadata.inference_schema)));
            }
            if !self.inferences.iter().any(|descriptor| descriptor.id == metadata.inference_schema) {
                return Err(PluginAssemblyError::new("plugin-assembly.inference-descriptor", format!("inference service {:?} has no descriptor", metadata.inference_schema)));
            }
        }
        Ok(())
    }

    /// 🏠️ Validates a declaration another package owns for hosting in `plugin_id`'s guest, without side effects: the
    /// kind is owned by a plugin `plugin_id` directly depends on — never by `plugin_id` itself, which declares its own
    /// kinds — and every channel stays inside it. The definition is not registered: it stays its owner's.
    pub(crate) fn preflight_hosted(&self, plugin_id: &str, dependencies: &[semio_framework::PluginDependency]) -> Result<(), PluginAssemblyError> {
        let kind = ArtifactKindId::parse(self.kind.as_str()).map_err(|_| PluginAssemblyError::new("plugin-assembly.artifact-kind", "artifact identity must use canonical <domain>.<plugin>.<artifact> grammar"))?;
        if kind.plugin() == plugin_id {
            return Err(PluginAssemblyError::new("plugin-assembly.hosted-artifact-owner", format!("plugin {plugin_id:?} hosts {:?}, a kind it owns and declares", self.kind)));
        }
        if !dependencies.iter().any(|dependency| dependency.plugin_id == kind.plugin()) {
            return Err(PluginAssemblyError::new("plugin-assembly.hosted-artifact-dependency", format!("plugin {plugin_id:?} hosts {:?} without depending on its owner {:?}", self.kind, kind.plugin())));
        }
        self.preflight_channels()
    }

    /// 🧭️ Every composer, subset validator, dialect migration and shared schema document of this declaration stays inside its own kind.
    fn preflight_channels(&self) -> Result<(), PluginAssemblyError> {
        for spec in &self.document_codecs {
            if spec.dialect.artifact_kind != self.kind || spec.dialect.standard.0.is_empty() || spec.dialect.subset.0.is_empty() {
                return Err(PluginAssemblyError::new("plugin-assembly.snapshot-owner", format!("snapshot codec {:?} requires an exact dialect of artifact {:?}", spec.schema, self.kind)));
            }
        }
        let snapshots: Vec<_> = self.document_codecs.iter().filter_map(|spec| semio_framework_os_kernel::io::io_mechanism::NativeSnapshotRegistration::from_capability(spec.dialect.into(), spec.codec())).collect();
        semio_framework_os_kernel::io::io_mechanism::preflight_native_snapshots(&snapshots).map_err(|error| PluginAssemblyError::new("plugin-assembly.snapshot", error.to_string()))?;
        for entry in &self.composers {
            if entry.writes.artifact_kind != self.kind && !entry.reads.iter().any(|dialect| dialect.artifact_kind == self.kind) {
                return Err(PluginAssemblyError::new("plugin-assembly.composer-owner", format!("composer for {:?} neither reads nor writes that artifact", self.kind)));
            }
        }
        for entry in &self.subset_validators {
            if entry.dialect.artifact_kind != self.kind {
                return Err(PluginAssemblyError::new("plugin-assembly.subset-owner", format!("subset validator for {:?} validates {}", self.kind, entry.dialect.artifact_kind)));
            }
        }
        for migration in &self.migrations {
            if migration.from.artifact_kind != self.kind || migration.to.artifact_kind != self.kind {
                return Err(PluginAssemblyError::new("plugin-assembly.migration-owner", format!("migration for {:?} names {}→{}", self.kind, migration.from.artifact_kind, migration.to.artifact_kind)));
            }
        }
        for documents in &self.schema_documents {
            if documents.scope != self.kind {
                return Err(PluginAssemblyError::new("plugin-assembly.schema-documents-owner", format!("schema documents of scope {:?} declared by artifact {:?}", documents.scope, self.kind)));
            }
        }
        Ok(())
    }
}

/// 🧷️ Typed mesh-to-document executable retained by the plugin runtime for mesh import.
pub type MeshDocumentImporter = fn(&semio_framework::MeshData) -> Result<semio_framework_pack_json::Value, String>;

/// 🧷️ Typed document-to-SVG executable retained by the plugin runtime for 2D export.
pub type TwoDSvgDocumentRenderer = fn(&Value) -> Result<(String, u32, u32), String>;

/// 🧭️ The closed set of host-media contribution roles.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum HostMediaHandlerKind {
    MeshImport,
    TwoDSvgExport,
}

impl HostMediaHandlerKind {
    fn as_str(self) -> &'static str {
        match self {
            Self::MeshImport => "mesh-import",
            Self::TwoDSvgExport => "2d-svg-export",
        }
    }
}

/// ⚙️ Complete process-local identity of a typed host-media executable.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum HostMediaExecutableIdentity {
    MeshImport(usize),
    TwoDSvgExport(usize),
}

/// 📋️ Immutable host-media descriptor visible to a runtime host.
#[derive(Clone, Debug, PartialEq)]
pub struct HostMediaHandlerDescriptor {
    pub id: String,
    pub owner: String,
    pub kind: HostMediaHandlerKind,
    pub artifact_kind: ArtifactKindSpec,
    pub artifact_schema: String,
    pub file_stem: Option<String>,
    pub executable_identity: HostMediaExecutableIdentity,
}

/// 🧾️ Descriptor-owned typed host-media contribution, frozen before any aggregate commit.
#[derive(Clone, Debug)]
pub struct HostMediaHandlerDeclaration {
    id: ArtifactIdentity,
    kind: HostMediaHandlerKind,
    artifact_kind: ArtifactKindSpec,
    artifact_schema: String,
    file_stem: Option<String>,
    executable: HostMediaExecutable,
}

#[derive(Clone, Copy, Debug)]
enum HostMediaExecutable {
    MeshImport(MeshDocumentImporter),
    TwoDSvgExport(TwoDSvgDocumentRenderer),
}

impl HostMediaHandlerDeclaration {
    /// 🏗️ Describes one mesh bridge without invoking its converter during assembly.
    pub fn mesh_import(id: impl Into<String>, artifact_kind: ArtifactKindSpec, artifact_schema: impl Into<String>, importer: MeshDocumentImporter) -> Result<Self, PluginAssemblyError> {
        let declaration = Self {
            id: ArtifactIdentity::parse(id).map_err(PluginAssemblyError::definition)?,
            kind: HostMediaHandlerKind::MeshImport,
            artifact_kind,
            artifact_schema: artifact_schema.into(),
            file_stem: None,
            executable: HostMediaExecutable::MeshImport(importer),
        };
        declaration.validate_shape()?;
        Ok(declaration)
    }

    /// 🏗️ Describes one 2D SVG renderer without invoking it during assembly.
    pub fn two_d_svg_export(id: impl Into<String>, artifact_kind: ArtifactKindSpec, artifact_schema: impl Into<String>, file_stem: impl Into<String>, renderer: TwoDSvgDocumentRenderer) -> Result<Self, PluginAssemblyError> {
        let declaration = Self {
            id: ArtifactIdentity::parse(id).map_err(PluginAssemblyError::definition)?,
            kind: HostMediaHandlerKind::TwoDSvgExport,
            artifact_kind,
            artifact_schema: artifact_schema.into(),
            file_stem: Some(file_stem.into()),
            executable: HostMediaExecutable::TwoDSvgExport(renderer),
        };
        declaration.validate_shape()?;
        Ok(declaration)
    }

    /// 🪪️ Returns the stable contribution id.
    pub fn id(&self) -> &str {
        self.id.as_str()
    }

    /// ⚙️ Returns the complete executable identity used for idempotence and conflict rejection.
    pub fn executable_identity(&self) -> HostMediaExecutableIdentity {
        match self.executable {
            HostMediaExecutable::MeshImport(importer) => HostMediaExecutableIdentity::MeshImport(importer as usize),
            HostMediaExecutable::TwoDSvgExport(renderer) => HostMediaExecutableIdentity::TwoDSvgExport(renderer as usize),
        }
    }

    fn validate_shape(&self) -> Result<(), PluginAssemblyError> {
        if self.artifact_kind.id.trim().is_empty() || self.artifact_kind.schema.trim().is_empty() || self.artifact_schema.trim().is_empty() {
            return Err(PluginAssemblyError::new("plugin-assembly.host-media-shape", "host-media contributions require non-empty artifact kind and document schema"));
        }
        if self.artifact_kind.schema != self.artifact_schema {
            return Err(PluginAssemblyError::new(
                "plugin-assembly.host-media-schema",
                format!("host-media contribution {:?} schema {:?} does not equal artifact-kind schema {:?}", self.id.as_str(), self.artifact_schema, self.artifact_kind.schema),
            ));
        }
        if matches!(self.kind, HostMediaHandlerKind::TwoDSvgExport) && self.file_stem.as_deref().is_none_or(|file_stem| file_stem.trim().is_empty()) {
            return Err(PluginAssemblyError::new("plugin-assembly.host-media-file-stem", "2D SVG export contributions require a non-empty file stem"));
        }
        Ok(())
    }

    pub(crate) fn preflight(&self, owner: &str, declared_media_kinds: &BTreeMap<String, ArtifactKindSpec>) -> Result<(), PluginAssemblyError> {
        ArtifactIdentity::parse(owner.to_string()).map_err(PluginAssemblyError::definition)?;
        self.validate_shape()?;
        let self_id = self.id.as_str();
        let declared = declared_media_kinds
            .get(&self.artifact_kind.id)
            .ok_or_else(|| PluginAssemblyError::new("plugin-assembly.host-media-owner", format!("host-media contribution {:?} targets undeclared artifact kind {:?}", self_id, self.artifact_kind.id)))?;
        if declared != &self.artifact_kind {
            return Err(PluginAssemblyError::new("plugin-assembly.host-media-kind", format!("host-media contribution {:?} does not use the plugin-owned descriptor for {:?}", self.id.as_str(), self.artifact_kind.id)));
        }
        Ok(())
    }

    fn descriptor(&self, owner: &str) -> HostMediaHandlerDescriptor {
        HostMediaHandlerDescriptor {
            id: self.id.as_str().to_string(),
            owner: owner.to_string(),
            kind: self.kind,
            artifact_kind: self.artifact_kind.clone(),
            artifact_schema: self.artifact_schema.clone(),
            file_stem: self.file_stem.clone(),
            executable_identity: self.executable_identity(),
        }
    }
}

/// 📥️ Typed mesh bridge runtime request.
#[derive(Clone, Debug, PartialEq)]
pub struct MeshImportRequest {
    pub artifact_kind: String,
    pub artifact_schema: String,
    pub mesh: semio_framework::MeshData,
}

/// 📤️ Typed mesh bridge runtime result.
#[derive(Clone, Debug, PartialEq)]
pub struct MeshImportResult {
    pub document: semio_framework_pack_json::Value,
}

/// 📥️ Typed 2D SVG export runtime request.
#[derive(Clone, Debug, PartialEq)]
pub struct TwoDSvgExportRequest {
    pub artifact_kind: String,
    pub artifact_schema: String,
    pub document: Value,
}

/// 📤️ Typed 2D SVG export runtime result.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TwoDSvgExportResult {
    pub file_stem: String,
    pub svg: String,
    pub width: u32,
    pub height: u32,
}

/// 🚧️ Typed host-media runtime rejection.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct HostMediaRuntimeError {
    pub code: String,
    pub message: String,
}

impl std::fmt::Display for HostMediaRuntimeError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{}: {}", self.code, self.message)
    }
}

impl std::error::Error for HostMediaRuntimeError {}

/// ⚙️ Immutable executable identity for a flow-extension contribution.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct FlowExtensionExecutableIdentity {
    pub package_id: String,
    pub module_id: String,
    pub entrypoint: String,
    pub abi: String,
}

impl FlowExtensionExecutableIdentity {
    /// 🏗️ Names a native executable entry without accepting an installer callback.
    pub fn native(package_id: impl Into<String>, module_id: impl Into<String>, entrypoint: impl Into<String>) -> Result<Self, PluginAssemblyError> {
        let identity = Self { package_id: package_id.into(), module_id: module_id.into(), entrypoint: entrypoint.into(), abi: "semio.flow.extension.native.v1".into() };
        for value in [&identity.package_id, &identity.module_id, &identity.entrypoint, &identity.abi] {
            ArtifactIdentity::parse(value.clone()).map_err(PluginAssemblyError::definition)?;
        }
        Ok(identity)
    }
}

/// 🧾️ One closed contribution category inside a `flow.extension` manifest.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum FlowExtensionContributionKind {
    Schema,
    Operator,
    Widget,
    Command,
    Setting,
}

/// 🧾️ One externally addressable contribution declared by a flow extension.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct FlowExtensionContribution {
    pub kind: FlowExtensionContributionKind,
    pub id: String,
}

impl FlowExtensionContribution {
    /// 🏗️ Creates one non-empty typed flow-extension contribution identity.
    pub fn new(kind: FlowExtensionContributionKind, id: impl Into<String>) -> Result<Self, PluginAssemblyError> {
        let id = id.into();
        ArtifactIdentity::parse(id.clone()).map_err(PluginAssemblyError::definition)?;
        Ok(Self { kind, id })
    }
}

/// 📋️ Schema-owned, immutable `flow.extension` manifest descriptor.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FlowExtensionManifest {
    pub id: String,
    pub name: String,
    pub version: String,
    pub activation_events: Vec<String>,
    pub contributions: Vec<FlowExtensionContribution>,
}

impl FlowExtensionManifest {
    /// 🏗️ Creates a typed flow-extension manifest with an explicit startup activation.
    pub fn new(id: impl Into<String>, name: impl Into<String>, version: impl Into<String>) -> Result<Self, PluginAssemblyError> {
        let manifest = Self { id: id.into(), name: name.into(), version: version.into(), activation_events: vec!["onStartup".into()], contributions: Vec::new() };
        manifest.validate()?;
        Ok(manifest)
    }

    /// 🧩️ Adds one typed contribution to this immutable descriptor.
    pub fn contribution(mut self, contribution: FlowExtensionContribution) -> Result<Self, PluginAssemblyError> {
        if self.contributions.iter().any(|candidate| candidate.kind == contribution.kind && candidate.id == contribution.id) {
            return Ok(self);
        }
        self.contributions.push(contribution);
        Ok(self)
    }

    fn validate(&self) -> Result<(), PluginAssemblyError> {
        ArtifactIdentity::parse(self.id.clone()).map_err(PluginAssemblyError::definition)?;
        if self.name.trim().is_empty() || self.version.trim().is_empty() || self.activation_events.iter().any(|event| event.trim().is_empty()) {
            return Err(PluginAssemblyError::new("plugin-assembly.flow-extension-manifest", "flow extension manifests require non-empty name, version, and activation events"));
        }
        let mut contributions = BTreeSet::new();
        for contribution in &self.contributions {
            ArtifactIdentity::parse(contribution.id.clone()).map_err(PluginAssemblyError::definition)?;
            if !contributions.insert((contribution.kind, contribution.id.clone())) {
                return Err(PluginAssemblyError::new("plugin-assembly.flow-extension-contribution", format!("duplicate flow extension contribution {:?}", contribution.id)));
            }
        }
        Ok(())
    }
}

/// 📋️ Immutable runtime descriptor for one linked flow extension.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FlowExtensionDescriptor {
    pub id: String,
    pub owner: String,
    pub domain: String,
    pub schema: String,
    pub extension_id: String,
    pub manifest: FlowExtensionManifest,
    pub executable_identity: FlowExtensionExecutableIdentity,
}

/// 🌊️ Immutable `flow.extension` contribution; it carries no mutating installer function.
#[derive(Clone, Debug)]
pub struct FlowExtensionDeclaration {
    id: ArtifactIdentity,
    extension_id: ArtifactIdentity,
    manifest: FlowExtensionManifest,
    executable_identity: FlowExtensionExecutableIdentity,
}

impl FlowExtensionDeclaration {
    /// 🏗️ Declares a `flow.extension` manifest and its immutable native executable identity.
    pub fn new(id: impl Into<String>, manifest: FlowExtensionManifest, executable_identity: FlowExtensionExecutableIdentity) -> Result<Self, PluginAssemblyError> {
        let declaration = Self { id: ArtifactIdentity::parse(id).map_err(PluginAssemblyError::definition)?, extension_id: ArtifactIdentity::parse(manifest.id.clone()).map_err(PluginAssemblyError::definition)?, manifest, executable_identity };
        declaration.validate_shape()?;
        Ok(declaration)
    }

    /// 🪪️ Returns the stable contribution id.
    pub fn id(&self) -> &str {
        self.id.as_str()
    }

    fn validate_shape(&self) -> Result<(), PluginAssemblyError> {
        self.manifest.validate()
    }

    pub(crate) fn preflight(&self, owner: &str) -> Result<(), PluginAssemblyError> {
        ArtifactIdentity::parse(owner.to_string()).map_err(PluginAssemblyError::definition)?;
        self.validate_shape()
    }

    fn descriptor(&self, owner: &str) -> FlowExtensionDescriptor {
        FlowExtensionDescriptor {
            id: self.id.as_str().to_string(),
            owner: owner.to_string(),
            domain: "flow.extension".into(),
            schema: "flow.extension".into(),
            extension_id: self.extension_id.as_str().to_string(),
            manifest: self.manifest.clone(),
            executable_identity: self.executable_identity.clone(),
        }
    }
}

struct RegisteredHostMediaHandler {
    descriptor: HostMediaHandlerDescriptor,
    executable: HostMediaExecutable,
}

impl RegisteredHostMediaHandler {
    fn identical_to(&self, other: &Self) -> bool {
        self.descriptor == other.descriptor
            && match (self.executable, other.executable) {
                (HostMediaExecutable::MeshImport(left), HostMediaExecutable::MeshImport(right)) => std::ptr::fn_addr_eq(left, right),
                (HostMediaExecutable::TwoDSvgExport(left), HostMediaExecutable::TwoDSvgExport(right)) => std::ptr::fn_addr_eq(left, right),
                _ => false,
            }
    }
}

pub(crate) struct HostMediaHandlerRegistry {
    by_id: BTreeMap<String, RegisteredHostMediaHandler>,
    by_target: BTreeMap<(HostMediaHandlerKind, String), String>,
}

impl HostMediaHandlerRegistry {
    pub(crate) fn new() -> Self {
        Self { by_id: BTreeMap::new(), by_target: BTreeMap::new() }
    }

    pub(crate) fn from_declarations(owner: &str, declarations: &[HostMediaHandlerDeclaration]) -> Result<Self, PluginAssemblyError> {
        let mut registry = Self::new();
        for declaration in declarations {
            registry.register(owner, declaration)?;
        }
        Ok(registry)
    }

    fn register(&mut self, owner: &str, declaration: &HostMediaHandlerDeclaration) -> Result<(), PluginAssemblyError> {
        let descriptor = declaration.descriptor(owner);
        let entry = RegisteredHostMediaHandler { descriptor: descriptor.clone(), executable: declaration.executable };
        if let Some(existing) = self.by_id.get(&descriptor.id) {
            if existing.identical_to(&entry) {
                return Ok(());
            }
            return Err(PluginAssemblyError::new("plugin-assembly.host-media-id", format!("host-media contribution {:?} has a conflicting descriptor or executable identity", descriptor.id)));
        }
        let target = (descriptor.kind, descriptor.artifact_kind.id.clone());
        if let Some(existing) = self.by_target.get(&target) {
            return Err(PluginAssemblyError::new("plugin-assembly.host-media-target", format!("host-media target {}:{} is already owned by {:?}", descriptor.kind.as_str(), descriptor.artifact_kind.id, existing)));
        }
        self.by_target.insert(target, descriptor.id.clone());
        self.by_id.insert(descriptor.id, entry);
        Ok(())
    }

    pub(crate) fn descriptors(&self) -> Vec<HostMediaHandlerDescriptor> {
        self.by_id.values().map(|entry| entry.descriptor.clone()).collect()
    }

    pub(crate) fn mesh_import(&self, request: &MeshImportRequest) -> Result<MeshImportResult, HostMediaRuntimeError> {
        let entry = self.lookup(HostMediaHandlerKind::MeshImport, &request.artifact_kind, &request.artifact_schema)?;
        let HostMediaExecutable::MeshImport(importer) = entry.executable else {
            return Err(HostMediaRuntimeError { code: "host-media.executable-kind".into(), message: format!("host-media target {:?} has no mesh executable", request.artifact_kind) });
        };
        importer(&request.mesh).map(|document| MeshImportResult { document }).map_err(|message| HostMediaRuntimeError { code: "host-media.mesh-import".into(), message })
    }

    pub(crate) fn two_d_svg_export(&self, request: &TwoDSvgExportRequest) -> Result<TwoDSvgExportResult, HostMediaRuntimeError> {
        let entry = self.lookup(HostMediaHandlerKind::TwoDSvgExport, &request.artifact_kind, &request.artifact_schema)?;
        let HostMediaExecutable::TwoDSvgExport(renderer) = entry.executable else {
            return Err(HostMediaRuntimeError { code: "host-media.executable-kind".into(), message: format!("host-media target {:?} has no 2D SVG executable", request.artifact_kind) });
        };
        let (svg, width, height) = renderer(&request.document).map_err(|message| HostMediaRuntimeError { code: "host-media.2d-svg".into(), message })?;
        Ok(TwoDSvgExportResult { file_stem: entry.descriptor.file_stem.clone().expect("2D SVG declaration requires a file stem"), svg, width, height })
    }

    fn lookup(&self, kind: HostMediaHandlerKind, artifact_kind: &str, artifact_schema: &str) -> Result<&RegisteredHostMediaHandler, HostMediaRuntimeError> {
        let kind_str = kind.as_str();
        let id = self.by_target.get(&(kind, artifact_kind.to_string())).ok_or_else(|| HostMediaRuntimeError { code: "host-media.not-declared".into(), message: format!("no {} declaration for artifact kind {:?}", kind_str, artifact_kind) })?;
        let entry = self.by_id.get(id).expect("host-media target index always points to an entry");
        if entry.descriptor.artifact_schema != artifact_schema {
            return Err(HostMediaRuntimeError {
                code: "host-media.schema-mismatch".into(),
                message: format!("host-media declaration {:?} expects schema {:?}, request supplied {:?}", entry.descriptor.id, entry.descriptor.artifact_schema, artifact_schema),
            });
        }
        Ok(entry)
    }
}

pub(crate) struct FlowExtensionRegistry {
    by_id: BTreeMap<String, FlowExtensionDescriptor>,
    by_extension_id: BTreeMap<String, String>,
}

impl FlowExtensionRegistry {
    pub(crate) fn new() -> Self {
        Self { by_id: BTreeMap::new(), by_extension_id: BTreeMap::new() }
    }

    pub(crate) fn from_declarations(owner: &str, declarations: &[FlowExtensionDeclaration]) -> Result<Self, PluginAssemblyError> {
        let mut registry = Self::new();
        for declaration in declarations {
            registry.register(owner, declaration)?;
        }
        Ok(registry)
    }

    fn register(&mut self, owner: &str, declaration: &FlowExtensionDeclaration) -> Result<(), PluginAssemblyError> {
        let descriptor = declaration.descriptor(owner);
        if let Some(existing) = self.by_id.get(&descriptor.id) {
            if existing == &descriptor {
                return Ok(());
            }
            return Err(PluginAssemblyError::new("plugin-assembly.flow-extension-id", format!("flow extension contribution {:?} has a conflicting descriptor or executable identity", descriptor.id)));
        }
        if let Some(existing) = self.by_extension_id.get(&descriptor.extension_id) {
            return Err(PluginAssemblyError::new("plugin-assembly.flow-extension-target", format!("flow extension {:?} is already owned by {:?}", descriptor.extension_id, existing)));
        }
        self.by_extension_id.insert(descriptor.extension_id.clone(), descriptor.id.clone());
        self.by_id.insert(descriptor.id.clone(), descriptor);
        Ok(())
    }

    pub(crate) fn descriptors(&self) -> Vec<FlowExtensionDescriptor> {
        self.by_id.values().cloned().collect()
    }
}
