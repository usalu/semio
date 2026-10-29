//! 🧮️ Codec spec and capability sources: the document codec spec, the capability sources and the declaration operations over them. See [`super`].

use super::*;

/// 🗂️ A monomorphized, non-capturing thunk pairing a `schema` string with the fn pointer that
/// registers a document codec under it — lets `.document_codec::<A>()`/`.document_codec_bare()`
/// store the registration as inert data instead of performing it immediately, matching every
/// other declaration field's "described now, committed once by `PluginBuilder::try_build()`" contract. The
/// `schema` sits alongside the thunk (not baked into it) because `bare`'s schema is a runtime
/// `impl Into<String>`, not a type-level const like `A::DOCUMENT_SCHEMA` — a `fn()` thunk cannot
/// close over it without capturing, which would break the "plain fn pointer" contract this type
/// exists to keep.
pub struct DocumentCodecSpec {
    pub(super) schema: String,
    pub dialect: Dialect,
    pub(super) extension: &'static str,
    pub(super) codec: fn(String) -> store::os_store::serialization::ArtifactCodec,
    pub app_id: String,
    pub(super) foreign: bool,
}

impl DocumentCodecSpec {
    pub async fn of<A: ArtifactApp>() -> Self
    where
        A::Mutation: Sync,
    {
        fn codec<A: ArtifactApp>(schema: String) -> store::os_store::serialization::ArtifactCodec
        where
            A::Mutation: Sync,
        {
            store::native_artifact_codec::<A::Snapshot>(schema)
        }
        let app = A::default();
        DocumentCodecSpec { schema: A::DOCUMENT_SCHEMA.to_string(), dialect: A::DIALECT, extension: <A::Snapshot as pack::snapshot::ArtifactPackMetadata>::EXTENSION, codec: codec::<A>, app_id: app.instance_id().await.to_string(), foreign: false }
    }

    pub fn foreign<A: ArtifactApp>(schema: impl Into<String>) -> Self
    where
        A::Mutation: Sync,
    {
        fn codec<A: ArtifactApp>(schema: String) -> store::os_store::serialization::ArtifactCodec
        where
            A::Mutation: Sync,
        {
            store::native_artifact_codec::<A::Snapshot>(schema)
        }
        let app = A::default();
        let app_id = ::semio_framework_async::poll::resolve_ready(app.instance_id()).to_string();
        DocumentCodecSpec { schema: schema.into(), dialect: A::DIALECT, extension: <A::Snapshot as pack::snapshot::ArtifactPackMetadata>::EXTENSION, codec: codec::<A>, app_id, foreign: true }
    }

    /// 🗂️ `of::<A>()`'s app-less sibling — see `.document_codec_bare()`'s own doc for why this
    /// exists. Registers straight against `store::os_store::codec_registry::register_document_codec`, bypassing
    /// `register_document_codec_for_app`'s `A::` indirection since there is no `A` to name.
    pub fn bare<Snapshot, Mutation>(schema: impl Into<String>, dialect: Dialect) -> Self
    where
        Snapshot: PartialEq + semio_framework_value::retirement::RetireOwned + pack::snapshot::ArtifactDsl + ArtifactPack + ::semio_framework_os_kernel::os_store::serialization::ArtifactStoreSnapshot + ::semio_framework_os_kernel::StoreState,
        Mutation: PartialEq + semio_framework_value::retirement::RetireOwned + Sync + store::ArtifactCanonicalJsonTree + ::semio_framework_os_kernel::StoreMutationFor<Snapshot>,
    {
        fn codec<Snapshot, Mutation>(schema: String) -> store::os_store::serialization::ArtifactCodec
        where
            Snapshot: PartialEq + semio_framework_value::retirement::RetireOwned + pack::snapshot::ArtifactDsl + ArtifactPack + ::semio_framework_os_kernel::os_store::serialization::ArtifactStoreSnapshot + ::semio_framework_os_kernel::StoreState,
            Mutation: PartialEq + semio_framework_value::retirement::RetireOwned + Sync + store::ArtifactCanonicalJsonTree + ::semio_framework_os_kernel::StoreMutationFor<Snapshot>,
        {
            store::native_artifact_codec::<Snapshot>(schema)
        }
        DocumentCodecSpec { schema: schema.into(), dialect, extension: <Snapshot as pack::snapshot::ArtifactDsl>::EXTENSION, codec: codec::<Snapshot, Mutation>, app_id: String::new(), foreign: false }
    }

    /// 🧬️ Selects the document-owned original semantic issuer for native stores.
    pub fn bare_with_owners<Snapshot, Mutation, Source>(schema: impl Into<String>, dialect: Dialect) -> Self
    where
        Snapshot: Clone
            + PartialEq
            + semio_framework_value::ToValue
            + semio_framework_value::FromValue
            + semio_framework_value::retirement::RetireOwned
            + Send
            + Sync
            + pack::snapshot::ArtifactDsl
            + ArtifactPack
            + ::semio_framework_os_kernel::os_store::serialization::ArtifactStoreSnapshot
            + 'static,
        Mutation: ::semio_framework_os_kernel::StoreMutationFor<Snapshot>
            + PartialEq
            + semio_framework_value::ToValue
            + semio_framework_value::FromValue
            + semio_framework_value::retirement::RetireOwned
            + Send
            + Sync
            + ::protocol::OpText
            + ::protocol::OpBinary
            + 'static
            + store::ArtifactCanonicalJsonTree,
        Source: store::ArtifactCodecOwnerSource<Snapshot,Mutation>+Send+Sync+'static,
    {
        fn codec<Snapshot, Mutation, Source>(schema: String) -> store::os_store::serialization::ArtifactCodec
        where
            Snapshot: Clone
                + PartialEq
                + semio_framework_value::ToValue
                + semio_framework_value::FromValue
                + semio_framework_value::retirement::RetireOwned
                + Send
                + Sync
                + pack::snapshot::ArtifactDsl
                + ArtifactPack
                + ::semio_framework_os_kernel::os_store::serialization::ArtifactStoreSnapshot
                + 'static,
            Mutation: ::semio_framework_os_kernel::StoreMutationFor<Snapshot>
                + PartialEq
                + semio_framework_value::ToValue
                + semio_framework_value::FromValue
                + semio_framework_value::retirement::RetireOwned
                + Send
                + Sync
                + ::protocol::OpText
                + ::protocol::OpBinary
                + 'static
                + store::ArtifactCanonicalJsonTree,
        Source: store::ArtifactCodecOwnerSource<Snapshot,Mutation>+Send+Sync+'static,
        {
            store::bare_artifact_codec_with_owners::<Snapshot, Mutation,Source>(schema)
        }
        DocumentCodecSpec { schema: schema.into(), dialect, extension: <Snapshot as pack::snapshot::ArtifactDsl>::EXTENSION, codec: codec::<Snapshot, Mutation,Source>, app_id: String::new(), foreign: false }
    }

    pub fn codec(&self) -> store::os_store::serialization::ArtifactCodec {
        (self.codec)(self.schema.clone())
    }

    pub fn binding(
        &self,
        channel: semio_framework_os_kernel::io::ArtifactCodecBindingChannel,
        contributor: &str,
        artifact_kind: Option<String>,
        codec: &store::os_store::serialization::ArtifactCodec,
    ) -> semio_framework_os_kernel::io::ArtifactCodecBinding {
        let apps = if self.app_id.is_empty() { Vec::new() } else { vec![self.app_id.to_string()] };
        semio_framework_os_kernel::io::ArtifactCodecBinding::from_codec(channel, contributor, artifact_kind, apps, Some(self.dialect.into()), None, codec)
    }

    pub fn preflight_foreign(&self, document_app_ids: &BTreeSet<String>) -> Result<(), PluginAssemblyError> {
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
pub(super) struct ArtifactCapabilitySources<'a> {
    pub(super) schemas: &'a [::semio_framework_schema_registry::ArtifactSchemaDescriptor],
    pub(super) inferences: &'a [::semio_framework_schema_registry::ArtifactInferenceDescriptor],
    pub(super) inference_services: &'a [ArtifactInferenceService],
    pub(super) composers: &'a [&'static ComposerEntry],
    pub(super) formats: &'a [semio_framework_os_kernel::io::FormatDescriptor],
    pub(super) subset_validators: &'a [&'static SubsetValidatorEntry],
    pub(super) languages: &'a [&'static semio_framework_dsl::LanguageSpec],
    pub(super) document_codecs: &'a [DocumentCodecSpec],
    pub(super) schema_documents: &'a [::semio_framework_schema_registry::ScopeSchemaExports],
}

impl ArtifactCapabilitySources<'_> {
    pub(super) fn requirements(self, owner: &str) -> Result<Vec<ArtifactRuntimeCapabilityRequirement>, ArtifactDefinitionError> {
        use semio_framework_artifact_reference::io::text::artifact_reference::DialectCoordinateText as _;

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
            rows.push(ArtifactRuntimeCapabilityRequirement::new(ArtifactCapabilityKind::composer(), semio_framework_plugin_identity::composer_identity_claims(owner, entry.writes, entry.reads)?));
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
        .requirements(&self.kind)
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
    pub fn preflight(&self, plugin_id: &str, definitions: &mut ArtifactDefinitionRegistry) -> Result<(), PluginAssemblyError> {
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
    pub fn preflight_hosted(&self, plugin_id: &str, dependencies: &[semio_framework::PluginDependency]) -> Result<(), PluginAssemblyError> {
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
