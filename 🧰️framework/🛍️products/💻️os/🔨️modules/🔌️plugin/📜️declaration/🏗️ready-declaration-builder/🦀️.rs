//! 🏗️ Ready declaration builder: the operations of a declaration builder that already has its schema. See [`super`].

use super::*;

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
        .requirements(&self.kind)
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
        use semio_framework_artifact_reference::io::text::artifact_reference::DialectCoordinateText as _;

        for entry in entries {
            require_declared_capability_or_record(&self.definition, &mut self.definition_error, &ArtifactCapabilityKind::composer(), semio_framework_plugin_identity::composer_identity_claims(&self.kind, entry.writes, entry.reads));
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
        use semio_framework_artifact_reference::io::text::artifact_reference::DialectCoordinateText as _;

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
        Snapshot: PartialEq + semio_framework_value::ToValue + semio_framework_value::FromValue + semio_framework_value::retirement::RetireOwned + Send + Sync + pack::snapshot::ArtifactDsl + ArtifactPack + ::semio_framework_os_kernel::os_store::serialization::ArtifactStoreSnapshot + 'static,
        Mutation: ::protocol::Mutation<Snapshot> + PartialEq + semio_framework_value::ToValue + semio_framework_value::FromValue + semio_framework_value::retirement::RetireOwned + Send + Sync + ::protocol::OpText + ::protocol::OpBinary + 'static + store::ArtifactCanonicalJsonTree,
    {
        ::semio_framework_async::poll::resolve_ready(self.document_codec_bare_async::<Snapshot, Mutation>(schema, dialect))
    }

    async fn document_codec_bare_async<Snapshot, Mutation>(self, schema: impl Into<String>, dialect: Dialect) -> Self
    where
        Snapshot: PartialEq + semio_framework_value::ToValue + semio_framework_value::FromValue + semio_framework_value::retirement::RetireOwned + Send + Sync + pack::snapshot::ArtifactDsl + ArtifactPack + ::semio_framework_os_kernel::os_store::serialization::ArtifactStoreSnapshot + 'static,
        Mutation: ::protocol::Mutation<Snapshot> + PartialEq + semio_framework_value::ToValue + semio_framework_value::FromValue + semio_framework_value::retirement::RetireOwned + Send + Sync + ::protocol::OpText + ::protocol::OpBinary + 'static + store::ArtifactCanonicalJsonTree,
    {
        self.append_document_codec(DocumentCodecSpec::bare::<Snapshot,Mutation>(schema,dialect))
    }

    /// 🧬️ Registers an explicit original owner source under the ordinary capability claims.
    pub fn document_codec_bare_with_owners<Snapshot, Mutation, Source>(self, schema: impl Into<String>, dialect: Dialect) -> Self
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
        Mutation: ::protocol::Mutation<Snapshot>
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
        self.append_document_codec(DocumentCodecSpec::bare_with_owners::<Snapshot,Mutation,Source>(schema,dialect))
    }

    fn append_document_codec(mut self,codec:DocumentCodecSpec)->Self {
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
