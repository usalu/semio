//! 📇️ Registration plans, the plugin runtime registry, contributions and the owner mutation roster.
use semio_framework::{ActionArgDef, PluginManifest};
use semio_framework_value_derive::{FromValue, ToValue};
use std::collections::{BTreeMap, BTreeSet};
/// 🚪️ `io`'s `ArtifactRef`/`ArtifactKindId` vocabulary is not glob-re-exported at the
/// `semio-framework-os-kernel` crate root (deliberate — see that crate's own glue.rs comment on
/// the `io` mount), so it is named through the `store::io::` path everywhere in this file,
/// exactly like the sibling `🎞️gif` migration leaf (`semio_framework_artifact_reference::ArtifactDialect`) already does.
use semio_framework_artifact_reference::ArtifactKindId;
use store::os_store::serialization::ArtifactPack;
use semio_framework_ui_locale::LocalizedLabel;
/// 🏅️🪆️🎯️ Standards/subsets dialect vocabulary (ticket 26/08/10/STDIO-ARTIFACTS-AND-IO phase
/// 2). Defined in `semio_framework` so plugins and the OS product
/// share one definition without an inverted dependency; re-exported here verbatim.
use semio_framework_os_kernel::io::{ComposerEntry, SubsetValidatorEntry};
#[cfg(test)]
use crate::app_contract::ArtifactApp;
#[cfg(test)]
use crate::declarations;
use crate::artifact_declaration::ArtifactDeclaration;
use crate::artifact_declaration::DocumentCodecSpec;
use crate::artifact_declaration::FlowExtensionDeclaration;
use crate::artifact_declaration::FlowExtensionRegistry;
use crate::artifact_declaration::HostMediaHandlerDeclaration;
use crate::artifact_declaration::HostMediaHandlerRegistry;
use crate::artifact_identity::ArtifactDefinitionRegistry;
use crate::sdk_fault::PluginAssemblyError;
use crate::artifact_inference::ArtifactInferenceService;
use crate::artifact_inference::ArtifactInferenceServiceKey;
use crate::artifact_inference::ArtifactInferenceServiceMetadata;
use crate::artifact_inference::ArtifactInferenceServiceRegistry;
use crate::store_lifecycle::OwnerMutationRoster;
#[cfg(test)]
use crate::artifact_identity::ArtifactDefinition;
#[cfg(test)]
use crate::artifact_identity::ArtifactIdentity;
#[cfg(test)]
use crate::surface_contract::EditorApp;



/// 🧭️ One immutable plugin runtime catalogue assembled before any shared registry commit.
pub(crate) struct ArtifactRegistrationPlan {
    schemas: Vec<::semio_framework_schema_registry::ArtifactSchemaDescriptor>,
    inferences: Vec<::semio_framework_schema_registry::ArtifactInferenceDescriptor>,
    inference_services: Vec<ArtifactInferenceService>,
    routed_inferences: Vec<ArtifactInferenceServiceMetadata>,
    formats: Vec<semio_framework_os_kernel::io::FormatDescriptor>,
    subset_validators: Vec<&'static SubsetValidatorEntry>,
    composers: Vec<&'static ComposerEntry>,
    languages: Vec<semio_framework_dsl::LanguageSpec>,
    document_codecs: Vec<store::os_store::serialization::ArtifactCodec>,
    document_bindings: Vec<semio_framework_os_kernel::io::ArtifactCodecBinding>,
    native_snapshots: Vec<semio_framework_os_kernel::io::io_mechanism::NativeSnapshotRegistration>,
    migrations: Vec<store::os_store::codec_registry::DialectMigration>,
    schema_documents: Vec<::semio_framework_schema_registry::ScopeSchemaExports>,
    app_schemas: Vec<::semio_framework_schema_registry::AppSchemaDescriptor>,
    owner: String,
    host_media_handlers: Vec<HostMediaHandlerDeclaration>,
    flow_extensions: Vec<FlowExtensionDeclaration>,
}

impl ArtifactRegistrationPlan {
    /// 🧾️ Collects every channel of the owned `declarations` and of the `hosted` ones, except the hosted inference
    /// services: those stay listed and executed by their owner's assembly alone.
    pub(crate) fn from_declarations(
        declarations: &[ArtifactDeclaration],
        hosted: &[ArtifactDeclaration],
        app_schemas: Vec<::semio_framework_schema_registry::AppSchemaDescriptor>,
        foreign_document_codecs: &[DocumentCodecSpec],
        owner: impl Into<String>,
        host_media_handlers: Vec<HostMediaHandlerDeclaration>,
        flow_extensions: Vec<FlowExtensionDeclaration>,
        routed_inferences: Vec<ArtifactInferenceServiceMetadata>,
    ) -> Self {
        let mut plan = Self {
            schemas: Vec::new(),
            inferences: Vec::new(),
            inference_services: Vec::new(),
            routed_inferences,
            formats: Vec::new(),
            subset_validators: Vec::new(),
            composers: Vec::new(),
            languages: Vec::new(),
            document_codecs: Vec::new(),
            document_bindings: Vec::new(),
            native_snapshots: Vec::new(),
            migrations: Vec::new(),
            schema_documents: Vec::new(),
            app_schemas,
            owner: owner.into(),
            host_media_handlers,
            flow_extensions,
        };
        for declaration in declarations {
            plan.inference_services.extend(declaration.inference_services.iter().copied());
        }
        for (channel, rows) in [(semio_framework_os_kernel::io::ArtifactCodecBindingChannel::OwnedDeclaration, declarations), (semio_framework_os_kernel::io::ArtifactCodecBindingChannel::HostedDeclaration, hosted)] {
            for declaration in rows {
                plan.schemas.extend(declaration.schemas.iter().copied());
                plan.inferences.extend(declaration.inferences.iter().copied());
                plan.formats.extend(declaration.formats.iter().cloned());
                plan.subset_validators.extend(declaration.subset_validators.iter().copied());
                plan.composers.extend(declaration.composers.iter().copied());
                plan.languages.extend(declaration.languages.iter().map(|spec| **spec));
                for spec in declaration.document_codecs.iter() {
                    let codec = spec.codec();
                    plan.document_bindings.push(spec.binding(channel, &plan.owner, Some(declaration.definition.identity().as_str().to_string()), &codec));
                    plan.document_codecs.push(codec.clone());
                    plan.native_snapshots.extend(semio_framework_os_kernel::io::io_mechanism::NativeSnapshotRegistration::from_capability(spec.dialect.into(), codec));
                }
                plan.migrations.extend(declaration.migrations.iter().cloned());
                plan.schema_documents.extend(declaration.schema_documents.iter().copied());
            }
        }
        for spec in foreign_document_codecs.iter() {
            let codec = spec.codec();
            plan.document_bindings.push(spec.binding(semio_framework_os_kernel::io::ArtifactCodecBindingChannel::ForeignApp, &plan.owner, None, &codec));
            plan.document_codecs.push(codec.clone());
            plan.native_snapshots.extend(semio_framework_os_kernel::io::io_mechanism::NativeSnapshotRegistration::from_capability(spec.dialect.into(), codec));
        }
        plan
    }

    /// 🧬️ Freezes every definition, schema, app, language, and inference before external state changes.
    pub(crate) fn into_runtime(self, definitions: ArtifactDefinitionRegistry) -> Result<(PluginRuntimeRegistry, semio_framework_os_kernel::io::ArtifactAssemblyRegistryPlan), PluginAssemblyError> {
        let Self {
            schemas,
            inferences,
            inference_services: declared_inference_services,
            routed_inferences,
            formats,
            subset_validators,
            composers,
            languages,
            document_codecs,
            document_bindings,
            native_snapshots,
            migrations,
            schema_documents,
            app_schemas,
            owner,
            host_media_handlers,
            flow_extensions,
        } = self;
        let mut inference_services = ArtifactInferenceServiceRegistry::new();
        for service in &declared_inference_services {
            inference_services.register(*service).map_err(|error| PluginAssemblyError::new("plugin-assembly.inference", error.to_string()))?;
        }
        let mut inference_catalog: BTreeMap<ArtifactInferenceServiceKey, ArtifactInferenceServiceMetadata> = inference_services.metadata().into_iter().map(|metadata| (metadata.into(), metadata)).collect();
        for metadata in &routed_inferences {
            if metadata.owner != owner || metadata.artifact_kind.trim().is_empty() || metadata.artifact_schema.trim().is_empty() || metadata.artifact_schema.trim().is_empty() || metadata.inference_schema.trim().is_empty() {
                return Err(PluginAssemblyError::new("plugin-assembly.routed-inference", format!("invalid routed inference metadata for {}/{}", metadata.artifact_kind, metadata.inference_schema)));
            }
            let key = (*metadata).into();
            if let Some(existing) = inference_catalog.get(&key) {
                if existing != metadata {
                    return Err(PluginAssemblyError::new("plugin-assembly.routed-inference", format!("conflicting routed inference metadata for {}/{}", key.artifact_kind, key.inference_schema)));
                }
            } else {
                inference_catalog.insert(key, *metadata);
            }
        }
        let host_media_handlers = HostMediaHandlerRegistry::from_declarations(&owner, &host_media_handlers)?;
        let flow_extensions = FlowExtensionRegistry::from_declarations(&owner, &flow_extensions)?;
        semio_framework_os_kernel::io::io_mechanism::preflight_native_snapshots(&native_snapshots).map_err(|error| PluginAssemblyError::new("plugin-assembly.snapshot", error.to_string()))?;
        semio_framework_os_kernel::io::preflight_artifact_codec_bindings(&document_bindings, &document_codecs).map_err(|error| PluginAssemblyError::new("plugin-assembly.codec-binding", error.to_string()))?;
        Ok((
            PluginRuntimeRegistry {
                definitions,
                document_bindings: document_bindings.clone(),
                schemas,
                inferences,
                schema_documents,
                inference_services,
                routed_inferences,
                languages,
                app_schemas,
                host_media_handlers,
                flow_extensions,
                owner_mutations: BTreeMap::new(),
                contributed_mutations: BTreeMap::new(),
            },
            semio_framework_os_kernel::io::ArtifactAssemblyRegistryPlan { composer_entries: composers, subset_validators, format_descriptors: formats, document_codecs, document_bindings, dialect_migrations: migrations, native_snapshots, catalog_bindings:Vec::new() },
        ))
    }
}

#[cfg(test)]
mod original_codec_binding_laws {
    use super::*;
    include!("../../🧪️tests/🪪️codec-binding/🦀️.rs");
}

/// 🧬️ Authoritative per-plugin runtime declarations. It is complete before global IO/store rows mutate.
pub(crate) struct PluginRuntimeRegistry {
    definitions: ArtifactDefinitionRegistry,
    document_bindings: Vec<semio_framework_os_kernel::io::ArtifactCodecBinding>,
    schemas: Vec<::semio_framework_schema_registry::ArtifactSchemaDescriptor>,
    inferences: Vec<::semio_framework_schema_registry::ArtifactInferenceDescriptor>,
    schema_documents: Vec<::semio_framework_schema_registry::ScopeSchemaExports>,
    inference_services: ArtifactInferenceServiceRegistry,
    routed_inferences: Vec<ArtifactInferenceServiceMetadata>,
    /// 🕳️ Collected at `into_runtime` time and never consumed. Languages: plugin inits also register grammars one by one
    /// (`dsl::register_language`, overwrite semantics, 51 sites) and the batch registry compares hook fn addresses, so
    /// publishing the declared set needs its own census first. App schemas: no OS-wide app-schema catalog exists yet.
    #[allow(dead_code, reason = "captured but unwired — see comment above")]
    languages: Vec<semio_framework_dsl::LanguageSpec>,
    #[allow(dead_code, reason = "captured but unwired — see comment above")]
    app_schemas: Vec<::semio_framework_schema_registry::AppSchemaDescriptor>,
    pub(crate) host_media_handlers: HostMediaHandlerRegistry,
    pub(crate) flow_extensions: FlowExtensionRegistry,
    owner_mutations: BTreeMap<String, WireMutationRosterEntry>,
    contributed_mutations: BTreeMap<String, ContributedMutationRuntimeEntry>,
}

impl PluginRuntimeRegistry {
    pub(crate) fn empty() -> Self {
        Self {
            definitions: ArtifactDefinitionRegistry::new(),
            document_bindings: Vec::new(),
            schemas: Vec::new(),
            inferences: Vec::new(),
            schema_documents: Vec::new(),
            inference_services: ArtifactInferenceServiceRegistry::new(),
            routed_inferences: Vec::new(),
            languages: Vec::new(),
            app_schemas: Vec::new(),
            host_media_handlers: HostMediaHandlerRegistry::new(),
            flow_extensions: FlowExtensionRegistry::new(),
            owner_mutations: BTreeMap::new(),
            contributed_mutations: BTreeMap::new(),
        }
    }

    pub(crate) fn inference_services(&self) -> &ArtifactInferenceServiceRegistry {
        &self.inference_services
    }

    pub(crate) fn routed_inferences(&self) -> &[ArtifactInferenceServiceMetadata] {
        &self.routed_inferences
    }

    /// 🧾️ Reads the immutable definitions published by this plugin assembly.
    pub fn definitions(&self) -> &ArtifactDefinitionRegistry {
        &self.definitions
    }

    /// 🪪️ Reads the immutable original codec bindings retained by this plugin assembly.
    pub fn document_bindings(&self) -> &[semio_framework_os_kernel::io::ArtifactCodecBinding] {
        &self.document_bindings
    }

    pub(crate) fn retain_document_bindings(&mut self, bindings: Vec<semio_framework_os_kernel::io::ArtifactCodecBinding>) {
        self.document_bindings.extend(bindings);
    }

    /// 📚️ Adds the plugin-level shared schema documents (`PluginBuilder::schema_documents`) to the ones
    /// [`Self::publish_declared_catalogs`] commits.
    pub(crate) fn share_schema_documents(&mut self, documents: Vec<::semio_framework_schema_registry::ScopeSchemaExports>) {
        self.schema_documents.extend(documents);
    }

    /// 🪣️ Publishes the shared schema documents the framework and the OS own — the framework value schema, the store's
    /// document model (a composed child, its owner stamp, a link, a blob) and the io vocabulary — into the OS-wide catalogs:
    /// what a contract or input schema of any plugin may `$ref` without publishing it. Every app construction runs it (the
    /// catalog tolerates exact duplicates), so an instance resolves those references whether or not a plugin assembly ran in
    /// its process (design §23).
    pub(crate) fn publish_framework_schema_documents() -> Result<(), ::semio_framework_schema_registry::SchemaExportRegistryError> {
        semio_framework_value::register_value_schema_exports()?;
        store::register_store_schema_exports()?;
        store::io_schema::register_io_schema_exports()
    }

    /// 📌️ Publishes the store's and io vocabulary's own schema documents, the declared shared schema documents, and the
    /// schema and inference descriptors the `.artifact(…)` declarations contributed into the OS-wide
    /// catalogs — the same two descriptor channels `commit_artifact_declarations` publishes for the declaration tree, each
    /// preflighted then committed, exact duplicates tolerated, a conflicting descriptor fatal. The snapshot-edit route
    /// resolves an editor's document schema there; unpublished, every stdio editor edit was refused
    /// `snapshot-edit.schema-unregistered`.
    pub(crate) fn publish_declared_catalogs(&self) -> Result<(), PluginAssemblyError> {
        let documents_error = |error: ::semio_framework_schema_registry::SchemaExportRegistryError| PluginAssemblyError::new("plugin-assembly.declaration-schema-documents", error.to_string());
        Self::publish_framework_schema_documents().map_err(documents_error)?;
        for documents in &self.schema_documents {
            ::semio_framework_schema_registry::register_scope_schema_exports(*documents).map_err(documents_error)?;
        }
        ::semio_framework_schema_registry::register_artifact_schema_descriptors(self.schemas.clone()).map_err(|error| PluginAssemblyError::new("plugin-assembly.declaration-schema", error.to_string()))?;
        ::semio_framework_schema_registry::register_artifact_inference_descriptors(self.inferences.clone()).map_err(|error| PluginAssemblyError::new("plugin-assembly.declaration-inference", error.to_string()))
    }

    pub(crate) fn extend_contributions(
        &mut self,
        inference_services: Vec<ArtifactInferenceService>,
        owner_rosters: &[OwnerMutationRoster],
        contributed_mutations: Vec<(String, ContributedMutationRuntimeEntry)>,
    ) -> Result<(), PluginAssemblyError> {
        for service in inference_services {
            let metadata = service.metadata();
            if let Some(route) = self.routed_inferences.iter().find(|route| route.artifact_kind == metadata.artifact_kind && route.inference_schema == metadata.inference_schema) {
                if route != &metadata {
                    return Err(PluginAssemblyError::new("plugin-assembly.contribution-inference", format!("contributed inference conflicts with routed inference {}/{}", metadata.artifact_kind, metadata.inference_schema)));
                }
            }
            self.inference_services.register(service).map_err(|error| PluginAssemblyError::new("plugin-assembly.contribution-inference", error.to_string()))?;
        }
        for provider in owner_rosters {
            let (artifact_schema, kinds, input_schemas) = provider();
            for (index, descriptor) in kinds.iter().enumerate() {
                let entry = WireMutationRosterEntry::owner(artifact_schema, descriptor, input_schemas.get(index).copied());
                if let Some(existing) = self.owner_mutations.get(&entry.mutation_id) {
                    if existing != &entry {
                        return Err(PluginAssemblyError::new("plugin-assembly.owner-mutation-roster", format!("conflicting owner mutation roster entry for {:?}", entry.mutation_id)));
                    }
                } else {
                    self.owner_mutations.insert(entry.mutation_id.clone(), entry);
                }
            }
        }
        for (mutation_id, entry) in contributed_mutations {
            if let Some(existing) = self.contributed_mutations.get(&mutation_id) {
                if existing.roster != entry.roster || !std::ptr::fn_addr_eq(existing.plan_cold, entry.plan_cold) {
                    return Err(PluginAssemblyError::new("plugin-assembly.contribution-mutation", format!("conflicting contributed mutation registration for {mutation_id:?}")));
                }
            } else {
                self.contributed_mutations.insert(mutation_id, entry);
            }
        }
        Ok(())
    }

    pub(crate) fn mutation_roster_entries(&self) -> Vec<WireMutationRosterEntry> {
        let mut entries: Vec<_> = self.owner_mutations.values().cloned().collect();
        entries.extend(self.contributed_mutations.values().map(|entry| entry.roster.clone()));
        entries.sort_by(|left, right| left.mutation_id.cmp(&right.mutation_id));
        entries
    }

    pub(crate) fn contributed_mutation_plan(&self, mutation_id: &str, artifact_kind: &str, snapshot_pack: &[u8], payload: &[u8]) -> Result<ContributedMutationPlanOutput, ContributedMutationExecutionError> {
        let entry = self.contributed_mutations.get(mutation_id).ok_or_else(|| ContributedMutationExecutionError::new("artifact-mutation.not-registered", format!("no contributed mutation service for {mutation_id:?}")))?;
        if entry.roster.artifact_kind.as_deref() != Some(artifact_kind) {
            return Err(ContributedMutationExecutionError::new("artifact-mutation.artifact-kind-mismatch", format!("mutation {mutation_id:?} targets {:?}, request claims {artifact_kind:?}", entry.roster.artifact_kind)));
        }
        (entry.plan_cold)(snapshot_pack, payload)
    }
}

/// 📌️ Atomically commits all external IO/store registry rows after the local runtime is frozen.
pub(crate) fn commit_artifact_registration_plan(assembly: &semio_framework_schema_registry::assembly::Transaction, plan: semio_framework_os_kernel::io::ArtifactAssemblyRegistryPlan) -> Result<(), PluginAssemblyError> {
    semio_framework_os_kernel::io::commit_artifact_assembly_registry_plan(assembly, plan).map_err(|error| PluginAssemblyError::new("plugin-assembly.registry", error.to_string()))
}

/// 🗂️ One plugin's contribution of mutations and/or inferences onto an artifact kind OWNED by a
/// plugin it directly depends on — contract freeze `26/08/16/PLUGIN-DEPENDENCIES-ARTIFACT-
/// CONTRIBUTIONS-AND-COMPOSITE-MUTATIONS` §3/§4. Built through `ArtifactContribution::builder`
/// (self-accumulating, no typestate — only `artifact_kind` is mandatory and it is supplied up
/// front), consumed by `PluginBuilder::contributes`/`ExtensionBundle::contributes`, which resolve
/// it against their own plugin id (`.resolve`) and hand the result to `register_contributions`
/// for the frozen gates before any runtime registration.
pub struct ArtifactContribution {
    artifact_kind: String,
    mutations: Vec<ContributedMutationSpec>,
    inference_services: Vec<ArtifactInferenceService>,
    inference_depends_on: BTreeMap<String, Vec<String>>,
}

/// 🗂️ One `.mutation::<Snapshot, Op, K>()` call, still missing the contributor's own plugin id
/// (only known once a builder that already carries it calls `.contributes()`) — see
/// `ArtifactContribution::resolve`.
struct ContributedMutationSpec {
    target_artifact_schema: String,
    semantics: semio_framework::ContributedMutationSemantics,
    schema_version: u32,
    algorithm_version: u32,
    plan_cold: ColdContributedMutationPlan,
}

impl ArtifactContribution {
    /// 🪪️ Starts a contribution targeting one artifact kind owned by a plugin dependency.
    pub async fn builder(artifact_kind: impl Into<String>) -> Self {
        Self { artifact_kind: artifact_kind.into(), mutations: Vec::new(), inference_services: Vec::new(), inference_depends_on: BTreeMap::new() }
    }

    /// 🕸️ Declares one contributed composite-mutation kind. `K`'s payload is this plugin's own
    /// (the contributor's); `Snapshot`/`Op` are the TARGET artifact's own types (from the
    /// dependency crate) — the bound `K: protocol::CompositeMutationKind<Snapshot, Op>` makes a
    /// contribution that does not typecheck against the target artifact's own mutation vocabulary
    /// impossible to express: a `K` whose `plan` cannot drive a `Planner<Snapshot, Op>` simply does
    /// not satisfy it. `target_artifact_schema` is `Snapshot`'s owning `ArtifactApp::DOCUMENT_SCHEMA`
    /// (mirrors `.document_codec_bare`'s own `impl Into<String>` — a bare `Snapshot` type carries no
    /// compile-time document-schema constant of its own). The frozen id
    /// `"<target-document-schema>#<contributor-plugin-id>:<kebab-kind>"` is only fully assembled at
    /// `.resolve()`, once the contributor's own plugin id is known.
    ///
    /// 🚫️async: E4 fn-pointer slot — `ContributedMutationSpec.plan_cold: ColdContributedMutationPlan`
    /// is a bare fn pointer; an `async fn` item's pointer type is unnameable. The body's own
    /// awaits (`decode_pack`/`decode_contributed_wire`/`plan_of`/etc.) are all pure, no-real-
    /// suspension calls (io-async-signatures), so the whole body is bridged via `resolve_ready`.
    pub async fn mutation<Snapshot, Op, K>(mut self, target_artifact_schema: impl Into<String>, schema_version: u32, algorithm_version: u32) -> Self
    where
        Snapshot: Clone + ArtifactPack + 'static,
        Op: ::protocol::Mutation<Snapshot> + ::protocol::OpBinary + 'static,
        K: ::protocol::CompositeMutationKind<Snapshot, Op> + 'static,
    {
        fn plan_cold<Snapshot, Op, K>(snapshot_pack: &[u8], payload: &[u8]) -> Result<ContributedMutationPlanOutput, ContributedMutationExecutionError>
        where
            Snapshot: Clone + ArtifactPack,
            Op: ::protocol::Mutation<Snapshot> + ::protocol::OpBinary,
            K: ::protocol::CompositeMutationKind<Snapshot, Op>,
        {
            ::semio_framework_async::poll::resolve_ready(async {
                let base = Snapshot::decode_pack(snapshot_pack).map_err(|error| ContributedMutationExecutionError::new("artifact-mutation.snapshot-decode", error.to_string()))?;
                let kind: K = decode_contributed_wire(payload).await.map_err(|error| ContributedMutationExecutionError::new("artifact-mutation.payload-decode", error))?;
                let steps = ::protocol::plan_of(&kind, &base).map_err(|error| ContributedMutationExecutionError::new("artifact-mutation.plan", error.to_string()))?;
                let label = kind.label();
                let mut owner_ops = Vec::new();
                let mut foreign = Vec::new();
                for step in steps {
                    match step {
                        ::protocol::PlanStep::Local(op) => owner_ops.push(op.encode_op().map_err(|error| ContributedMutationExecutionError::new("artifact-mutation.op-encode", error.to_string()))?),
                        ::protocol::PlanStep::Foreign(hop) => foreign.push(hop),
                    }
                }
                Ok(ContributedMutationPlanOutput { owner_ops, label, foreign })
            })
        }
        self.mutations.push(ContributedMutationSpec {
            target_artifact_schema: target_artifact_schema.into(),
            semantics: semio_framework::ContributedMutationSemantics { verb: K::SEMANTICS.verb.to_string(), entity: K::SEMANTICS.entity.to_string(), kind: K::SEMANTICS.kind.to_string(), record: K::SEMANTICS.record.to_string() },
            schema_version,
            algorithm_version,
            plan_cold: plan_cold::<Snapshot, Op, K>,
        });
        self
    }

    /// 💡️ Declares one contributed executable inference service — same native
    /// `ArtifactInferenceService` shape a declaration's own `.inference_services(...)` takes; only
    /// the manifest-level gate (`register_contributions`) distinguishes "owned" from "contributed"
    /// (owner==self&&artifact_kind==self vs owner==contributor&&artifact_kind==target).
    pub async fn inference_service(mut self, service: ArtifactInferenceService) -> Self {
        self.inference_services.push(service);
        self
    }

    /// 🔗️ Declares `inference_schema`'s contributed-inference dependency ordering — folded into
    /// `ContributedInferenceMetadata.depends_on` at `.resolve()`.
    pub fn inference_depends_on(mut self, inference_schema: impl Into<String>, depends_on: impl IntoIterator<Item = impl Into<String>>) -> Self {
        self.inference_depends_on.entry(inference_schema.into()).or_default().extend(depends_on.into_iter().map(Into::into));
        self
    }

    /// ✅️ Finalizes the contribution — infallible (every frozen gate needs the contributor's own
    /// plugin id, only known once a builder's `.contributes()` calls `.resolve()`).
    pub fn build(self) -> Self {
        self
    }

    /// 🏗️ Assembles the manifest-facing `ArtifactContributionDescriptor` plus the native runtime
    /// pieces (inference services to register verbatim, and `(mutation_id, entry)` pairs for
    /// `commit_contributed_mutation_services`) now that `plugin_id` (the contributor) is known.
    /// Pure — no registry side effects; the caller runs `register_contributions` on the result
    /// before committing anything.
    pub(crate) fn resolve(self, plugin_id: &str) -> (semio_framework::ArtifactContributionDescriptor, Vec<ArtifactInferenceService>, Vec<(String, ContributedMutationRuntimeEntry)>) {
        let mut mutation_metadata = Vec::with_capacity(self.mutations.len());
        let mut mutation_runtime = Vec::with_capacity(self.mutations.len());
        for spec in self.mutations {
            let mutation_id = format!("{}#{}:{}", spec.target_artifact_schema, plugin_id, spec.semantics.kind);
            let roster = WireMutationRosterEntry::contributed(plugin_id, &self.artifact_kind, mutation_id.clone(), &spec.semantics);
            mutation_metadata.push(semio_framework::ContributedMutationMetadata { mutation_id: mutation_id.clone(), semantics: spec.semantics, schema_version: spec.schema_version, algorithm_version: spec.algorithm_version });
            mutation_runtime.push((mutation_id, ContributedMutationRuntimeEntry { roster, plan_cold: spec.plan_cold }));
        }
        let inferences = self
            .inference_services
            .iter()
            .map(|service| {
                let metadata = service.metadata();
                semio_framework::ContributedInferenceMetadata {
                    owner: metadata.owner.to_string(),
                    artifact_kind: metadata.artifact_kind.to_string(),
                    artifact_schema: metadata.artifact_schema.to_string(),
                    artifact_schema_version: metadata.artifact_schema_version,
                    inference_schema: metadata.inference_schema.to_string(),
                    inference_schema_version: metadata.inference_schema_version,
                    algorithm_version: metadata.algorithm_version,
                    policy_version: metadata.policy_version,
                    contributor: plugin_id.to_string(),
                    depends_on: self.inference_depends_on.get(metadata.inference_schema).cloned().unwrap_or_default(),
                    payload: metadata.payload.map(Into::into),
                }
            })
            .collect();
        let descriptor = semio_framework::ArtifactContributionDescriptor { artifact_kind: self.artifact_kind, mutations: mutation_metadata, inferences };
        (descriptor, self.inference_services, mutation_runtime)
    }
}

/// 🚧️ Typed failure of the contract-freeze §4 registration gates (dependency directness, mutation
/// id namespacing, contributed-inference identity) — pure over already-assembled manifest data,
/// never touches a runtime registry.
#[derive(Clone, Debug, PartialEq, Eq)]
enum ContributionRegistrationError {
    DependencyNotDeclared { plugin_id: String, artifact_kind: String, owner: String },
    InvalidArtifactKind(String),
    MalformedMutationId { plugin_id: String, mutation_id: String },
    CollidesWithOwnerKind(String),
    InferenceOwnerMismatch { plugin_id: String, inference_schema: String, owner: String, contributor: String },
    InferenceTargetMismatch { inference_schema: String, expected: String, actual: String },
}

impl std::fmt::Display for ContributionRegistrationError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::DependencyNotDeclared { plugin_id, artifact_kind, owner } => write!(formatter, "contribution targets artifact kind {artifact_kind:?} (owned by plugin {owner:?}), which is not a direct dependency of {plugin_id:?}"),
            Self::InvalidArtifactKind(kind) => write!(formatter, "contribution targets artifact kind {kind:?}, which is not canonical `<domain>.<plugin>.<artifact>` grammar"),
            Self::MalformedMutationId { plugin_id, mutation_id } => write!(formatter, "contributed mutation id {mutation_id:?} does not match the frozen grammar `<target-document-schema>#{plugin_id}:<kebab-kind>`"),
            Self::CollidesWithOwnerKind(id) => write!(formatter, "contributed mutation id {id:?} collides with the bare owner-mutation-id grammar (`<document-schema>#<kebab-kind>`, no contributor `:` segment)"),
            Self::InferenceOwnerMismatch { plugin_id, inference_schema, owner, contributor } => {
                write!(formatter, "contributed inference {inference_schema:?} has owner/contributor {owner:?}/{contributor:?}, expected both to equal contributor {plugin_id:?}")
            }
            Self::InferenceTargetMismatch { inference_schema, expected, actual } => write!(formatter, "contributed inference {inference_schema:?} targets artifact_kind {actual:?}, expected the contribution's own target {expected:?}"),
        }
    }
}

impl std::error::Error for ContributionRegistrationError {}

#[cfg(test)]
include!("../../🧪️tests/🔬️app-owned-contribution-error/🦀️.rs");

/// ✅️ Contract freeze §4 registration gates, run over already-assembled manifest data (never a
/// live registry): (a) a contribution's target artifact kind must be OWNED by a plugin the
/// contributor directly depends on; (b) every contributed mutation id must match the frozen
/// `"<target-document-schema>#<contributor-plugin-id>:<kebab-kind>"` grammar, which by
/// construction can never collide with the colon-free owner-mutation-id grammar; (c) every
/// contributed inference's `owner`/`contributor` must both equal `plugin_id`, and its
/// `artifact_kind` must equal the contribution's own target. Called by both
/// `PluginBuilder::try_build` and `ExtensionBundle::contributes` — see those call sites for how the
/// pure `&[ArtifactContributionDescriptor]` this validates is assembled first.
pub(crate) fn register_contributions(plugin_id: &str, dependencies: &[semio_framework::PluginDependency], contributions: &[semio_framework::ArtifactContributionDescriptor]) -> Result<(), ContributionRegistrationError> {
    for descriptor in contributions {
        let target = ArtifactKindId::parse(&descriptor.artifact_kind).map_err(|_| ContributionRegistrationError::InvalidArtifactKind(descriptor.artifact_kind.clone()))?;
        let owner = target.plugin();
        if !dependencies.iter().any(|dependency| dependency.plugin_id == owner) {
            return Err(ContributionRegistrationError::DependencyNotDeclared { plugin_id: plugin_id.to_string(), artifact_kind: descriptor.artifact_kind.clone(), owner: owner.to_string() });
        }
        for mutation in &descriptor.mutations {
            let Some(hash_at) = mutation.mutation_id.rfind('#') else {
                return Err(ContributionRegistrationError::MalformedMutationId { plugin_id: plugin_id.to_string(), mutation_id: mutation.mutation_id.clone() });
            };
            let suffix = &mutation.mutation_id[hash_at + 1..];
            if !suffix.contains(':') {
                return Err(ContributionRegistrationError::CollidesWithOwnerKind(mutation.mutation_id.clone()));
            }
            let expected_suffix = format!("{plugin_id}:{}", mutation.semantics.kind);
            if suffix != expected_suffix {
                return Err(ContributionRegistrationError::MalformedMutationId { plugin_id: plugin_id.to_string(), mutation_id: mutation.mutation_id.clone() });
            }
        }
        for inference in &descriptor.inferences {
            if inference.owner != plugin_id || inference.contributor != plugin_id {
                return Err(ContributionRegistrationError::InferenceOwnerMismatch {
                    plugin_id: plugin_id.to_string(),
                    inference_schema: inference.inference_schema.clone(),
                    owner: inference.owner.clone(),
                    contributor: inference.contributor.clone(),
                });
            }
            if inference.artifact_kind != descriptor.artifact_kind {
                return Err(ContributionRegistrationError::InferenceTargetMismatch { inference_schema: inference.inference_schema.clone(), expected: descriptor.artifact_kind.clone(), actual: inference.artifact_kind.clone() });
            }
        }
    }
    Ok(())
}

/// ✅️ Contract freeze §3's surface twin of `register_contributions`: an app surface bound to an
/// artifact kind this manifest does NOT own must name that kind's owner among its dependencies —
/// the same rule the host's `AppRouter::register_manifest` enforces at load, moved to assembly
/// time so a plugin can never ship a manifest that the router will exclude (ticket
/// 26/09/05/S-END-TO-END lane H: `demonstrator` shipped eight such surfaces and was excluded from
/// every session's routing). Ownership is derived exactly once, from the canonical
/// `<domain>.<plugin>.<artifact>` grammar `preflight_artifact_identity` already forces on every declared
/// kind — never a second dependency list to keep in sync. Returns one message per breaching
/// surface, in manifest order; empty means the manifest is admissible.
pub(crate) fn surface_dependency_breaches(manifest: &PluginManifest) -> Vec<String> {
use semio_framework_artifact_reference::io::text::artifact_reference::{DialectCoordinateText as _};

    let owned: BTreeSet<&str> = manifest.artifact_kinds.iter().map(|spec| spec.id.as_str()).collect();
    let mut breaches = Vec::new();
    for app in &manifest.apps {
        if owned.contains(app.dialect.artifact_kind.as_str()) {
            continue;
        }
        let Ok(kind) = ArtifactKindId::parse(&app.dialect.artifact_kind) else { continue };
        let owner = kind.plugin();
        if owner == manifest.plugin_id || manifest.dependencies.iter().any(|dependency| dependency.plugin_id == owner) {
            continue;
        }
        breaches.push(format!("app `{}` contributes a surface for `{}` owned by `{owner}`, which `{}` does not declare as a dependency", app.id, app.dialect.to_coordinate(), manifest.plugin_id));
    }
    breaches
}

/// 🏠️ A surface of an artifact another plugin owns (its dialect's `s.<owner>.<artifact>`, the grammar
/// [`surface_dependency_breaches`] reads) opens that owner's documents — HOSTING, never owning: the kind specs it carries
/// leave the app and become `hosted_artifact_kinds` rows naming that owner, so no codec row, creation claim or kind
/// ownership of the owner moves into the host (demonstrator embedding the cad/gis/procedural/process/sourcing editors
/// re-owned their kinds and made every hub creation of them ambiguous, 2026-09-29). Runs after the surface dependency
/// gate, so every owner named here is a declared dependency.
pub(crate) fn host_foreign_surface_kinds(manifest: &mut PluginManifest) {
    let owned: BTreeSet<String> = manifest.artifact_kinds.iter().map(|spec| spec.id.clone()).collect();
    for app in &mut manifest.apps {
        if owned.contains(&app.dialect.artifact_kind) {
            continue;
        }
        let Ok(kind) = ArtifactKindId::parse(&app.dialect.artifact_kind) else { continue };
        if kind.plugin() == manifest.plugin_id {
            continue;
        }
        for spec in std::mem::take(&mut app.artifact_kinds) {
            if !manifest.hosted_artifact_kinds.iter().any(|row| row.id == spec.id && row.schema == spec.schema) {
                manifest.hosted_artifact_kinds.push(semio_framework::HostedArtifactKind { id: spec.id, schema: spec.schema, owner: kind.plugin().to_string() });
            }
        }
    }
}

/// 🗂️ One roster row of `contributor.list-artifact-mutations` — an owner mutation
/// (`"<document-schema>#<kebab-kind>"`, `contributor`/`artifact_kind` both `None`) or a contributed
/// one (`contributor`/`artifact_kind` both `Some`) — with the input descriptors a history editor offers.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue)]
#[value(rename_all = "camelCase")]
pub struct WireMutationRosterEntry {
    pub mutation_id: String,
    pub verb: String,
    pub entity: String,
    pub kind: String,
    pub record: String,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub contributor: Option<String>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub artifact_kind: Option<String>,
    pub inputs: WireMutationInputs,
}

/// 🎛️ The editable inputs of one roster row: the descriptors its leaf payload schema declares
/// (`semio_framework::mutation_input_defs`), the reason that schema declares none a user could edit,
/// or `opaque` when the row publishes no payload schema (a contributed plan edits its target's own ops).
#[derive(Clone, Debug, PartialEq, ToValue, FromValue)]
#[value(tag = "status", rename_all = "camelCase", rename_all_fields = "camelCase")]
pub enum WireMutationInputs {
    Declared { inputs: Vec<ActionArgDef> },
    Refused { error: semio_framework::InputSchemaError },
    Opaque,
}

impl WireMutationInputs {
    /// 🧬️ Reads one leaf payload schema, resolving cross-document `$ref`s through the registered schema catalog.
    fn of(payload_schema: Option<&'static str>) -> Self {
        match payload_schema.map(|schema| semio_framework::mutation_input_defs(schema, &semio_framework::registered_input_schema_document)) {
            None => Self::Opaque,
            Some(Ok(inputs)) => Self::Declared { inputs },
            Some(Err(error)) => Self::Refused { error },
        }
    }
}

impl WireMutationRosterEntry {
    fn owner(artifact_schema: &str, descriptor: &::protocol::SemanticDescriptor, payload_schema: Option<&'static str>) -> Self {
        Self {
            mutation_id: format!("{artifact_schema}#{}", descriptor.kind),
            verb: descriptor.verb.to_string(),
            entity: descriptor.entity.to_string(),
            kind: descriptor.kind.to_string(),
            record: descriptor.record.to_string(),
            contributor: None,
            artifact_kind: None,
            inputs: WireMutationInputs::of(payload_schema),
        }
    }

    fn contributed(plugin_id: &str, artifact_kind: &str, mutation_id: String, semantics: &semio_framework::ContributedMutationSemantics) -> Self {
        Self {
            mutation_id,
            verb: semantics.verb.clone(),
            entity: semantics.entity.clone(),
            kind: semantics.kind.clone(),
            record: semantics.record.clone(),
            contributor: Some(plugin_id.to_string()),
            artifact_kind: Some(artifact_kind.to_string()),
            inputs: WireMutationInputs::Opaque,
        }
    }
}

/// 🗂️ Type-erased plan result of one `contributor.artifact-mutation-plan` execution: the target
/// artifact's own encoded ops, a human label, and any further foreign hops — mirrors
/// `protocol::PlanStep`'s split without exposing the target's concrete `Op` type.
#[derive(Clone, Debug, PartialEq)]
pub struct ContributedMutationPlanOutput {
    pub owner_ops: Vec<Vec<u8>>,
    pub label: LocalizedLabel,
    pub foreign: Vec<::protocol::ForeignStep>,
}

/// ⚠️ Structured failure from a type-erased contributed-mutation-plan execution boundary — mirrors
/// `ArtifactInferenceExecutionError`'s shape for the mutation-contribution seam.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ContributedMutationExecutionError {
    pub code: &'static str,
    pub message: String,
}

// 🚫️async: E1 pure error constructor consumed pervasively by sync closures
// (`.map_err`/`.ok_or_else`) — see R9.
impl ContributedMutationExecutionError {
    pub fn new(code: &'static str, message: impl Into<String>) -> Self {
        Self { code, message: message.into() }
    }
}

impl std::fmt::Display for ContributedMutationExecutionError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{}: {}", self.code, self.message)
    }
}

impl std::error::Error for ContributedMutationExecutionError {}

/// 🧰️ Native type-erased contributed-mutation-plan entry: target snapshot pack + contributor
/// payload bytes in, target ops/label/foreign out. Non-capturing fn pointer monomorphized per
/// `(Snapshot, Op, K)` by `ArtifactContribution::mutation`, exactly like `DocumentCodecSpec`/
/// `ArtifactInferenceService`'s own thunks.
pub type ColdContributedMutationPlan = fn(&[u8], &[u8]) -> Result<ContributedMutationPlanOutput, ContributedMutationExecutionError>;

/// 🧰️ One committed contributed-mutation runtime entry — the wire roster row plus the fn pointer
/// `wire_artifact_mutation_plan` calls.
struct ContributedMutationRuntimeEntry {
    roster: WireMutationRosterEntry,
    plan_cold: ColdContributedMutationPlan,
}

/// 🎞️ `contributor.artifact-mutation-plan`'s request envelope: target snapshot pack + contributor
/// payload in, plus the identity/echo fields (`artifact_kind`/`mutation_id`/`revision`/
/// `generation`) `wire_artifact_mutation_plan` validates and echoes back.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct WireArtifactMutationPlanRequest {
    pub artifact_kind: String,
    pub mutation_id: String,
    pub revision: u64,
    pub generation: u64,
    pub snapshot_pack: Vec<u8>,
    pub payload: Vec<u8>,
}

/// 🎞️ `contributor.artifact-mutation-plan`'s result envelope — echoes the request's identity
/// fields alongside the plan's `owner_ops`/`label`/`foreign`.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct WireArtifactMutationPlanResult {
    pub artifact_kind: String,
    pub mutation_id: String,
    pub revision: u64,
    pub generation: u64,
    pub owner_ops: Vec<Vec<u8>>,
    pub label: LocalizedLabel,
    pub foreign: Vec<::protocol::ForeignStep>,
}

// 🌱️ `K: CompositeMutationKind<..>` now bounds on `semio_framework_value::ToValue`/`semio_framework_value::FromValue`
// (see that trait's own doc), so this wire bridge speaks the first-party codec directly —
// no more `to_dsl_value`/`from_dsl_value` serde bridge, no `serde_json` in the loop.
#[cfg(test)]
async fn encode_contributed_wire<T: semio_framework_value::ToValue>(value: &T) -> Vec<u8> {
    store::pack_rt::encode_wire_value(&semio_framework_value::ToValue::to_value(value))
}

async fn decode_contributed_wire<T: semio_framework_value::FromValue>(bytes: &[u8]) -> Result<T, String> {
    let value = store::pack_rt::decode_wire_value(bytes).map_err(|error| error.to_string())?;
    semio_framework_value::FromValue::from_value(value).map_err(|error| error.to_string())
}

static OWNER_MUTATION_ROSTER_REGISTRY: std::sync::OnceLock<std::sync::RwLock<BTreeMap<String, WireMutationRosterEntry>>> = std::sync::OnceLock::new();

static CONTRIBUTED_MUTATION_REGISTRY: std::sync::OnceLock<std::sync::RwLock<BTreeMap<String, ContributedMutationRuntimeEntry>>> = std::sync::OnceLock::new();

async fn owner_mutation_roster_registry() -> &'static std::sync::RwLock<BTreeMap<String, WireMutationRosterEntry>> {
    OWNER_MUTATION_ROSTER_REGISTRY.get_or_init(|| std::sync::RwLock::new(BTreeMap::new()))
}

async fn contributed_mutation_registry() -> &'static std::sync::RwLock<BTreeMap<String, ContributedMutationRuntimeEntry>> {
    CONTRIBUTED_MUTATION_REGISTRY.get_or_init(|| std::sync::RwLock::new(BTreeMap::new()))
}

/// 📌️ Commits every document app's owner mutation roster (`PluginBuilder::document_app`'s
/// captured `(artifact_schema, kinds)` providers) — idempotent for byte-identical re-registration
/// (mirrors `ArtifactInferenceServiceRegistry::register`), a typed conflict otherwise.
#[cfg(test)]
async fn commit_owner_mutation_roster(providers: &[OwnerMutationRoster]) -> Result<(), PluginAssemblyError> {
    let mut registry = owner_mutation_roster_registry().await.write().map_err(|_| PluginAssemblyError::new("plugin-assembly.owner-mutation-roster", "owner mutation roster registry is poisoned"))?;
    for provider in providers {
        let (artifact_schema, kinds, input_schemas) = provider();
        for (index, descriptor) in kinds.iter().enumerate() {
            let entry = WireMutationRosterEntry::owner(artifact_schema, descriptor, input_schemas.get(index).copied());
            if let Some(existing) = registry.get(&entry.mutation_id) {
                if *existing != entry {
                    return Err(PluginAssemblyError::new("plugin-assembly.owner-mutation-roster", format!("conflicting owner mutation roster entry for {:?}", entry.mutation_id)));
                }
                continue;
            }
            registry.insert(entry.mutation_id.clone(), entry);
        }
    }
    Ok(())
}

/// 📌️ Commits every resolved contribution's `(mutation_id, runtime entry)` pair — same
/// idempotent-or-typed-conflict discipline as `commit_owner_mutation_roster`.
pub(crate) async fn commit_contributed_mutation_services(entries: Vec<(String, ContributedMutationRuntimeEntry)>) -> Result<(), PluginAssemblyError> {
    let mut registry = contributed_mutation_registry().await.write().map_err(|_| PluginAssemblyError::new("plugin-assembly.contribution-mutation-registry", "contributed mutation registry is poisoned"))?;
    for (mutation_id, entry) in entries {
        if let Some(existing) = registry.get(&mutation_id) {
            if existing.roster != entry.roster || !std::ptr::fn_addr_eq(existing.plan_cold, entry.plan_cold) {
                return Err(PluginAssemblyError::new("plugin-assembly.contribution-mutation-registry", format!("conflicting contributed mutation registration for {mutation_id:?}")));
            }
            continue;
        }
        registry.insert(mutation_id, entry);
    }
    Ok(())
}

/// 📖️ This process's deterministic mutation roster (owner + contributed rows, sorted by
/// `mutation_id`) — `plugin_runtime::wire_list_artifact_mutations`'s data source.
pub(crate) async fn mutation_roster_entries() -> Vec<WireMutationRosterEntry> {
    let mut entries: Vec<WireMutationRosterEntry> = owner_mutation_roster_registry().await.read().map(|registry| registry.values().cloned().collect()).unwrap_or_default();
    entries.extend(contributed_mutation_registry().await.read().map(|registry| registry.values().map(|entry| entry.roster.clone()).collect::<Vec<_>>()).unwrap_or_default());
    entries.sort_by(|a, b| a.mutation_id.cmp(&b.mutation_id));
    entries
}

/// 🎯️ Runs a registered contributed mutation's plan against `snapshot_pack`/`payload`, first
/// verifying `artifact_kind` echoes the mutation's own registered target — mirrors
/// `wire_artifact_infer`'s `validate_wire_request_metadata` discipline for the mutation-plan seam.
pub(crate) async fn contributed_mutation_plan(mutation_id: &str, artifact_kind: &str, snapshot_pack: &[u8], payload: &[u8]) -> Result<ContributedMutationPlanOutput, ContributedMutationExecutionError> {
    let registry = contributed_mutation_registry().await.read().map_err(|_| ContributedMutationExecutionError::new("artifact-mutation.unavailable", "contributed mutation registry is poisoned"))?;
    let entry = registry.get(mutation_id).ok_or_else(|| ContributedMutationExecutionError::new("artifact-mutation.not-registered", format!("no contributed mutation service for {mutation_id:?}")))?;
    if entry.roster.artifact_kind.as_deref() != Some(artifact_kind) {
        return Err(ContributedMutationExecutionError::new("artifact-mutation.artifact-kind-mismatch", format!("mutation {mutation_id:?} targets {:?}, request claims {artifact_kind:?}", entry.roster.artifact_kind)));
    }
    (entry.plan_cold)(snapshot_pack, payload)
}

#[cfg(test)]
include!("../../🧪️tests/🔬️app-artifact-contribution/🦀️.rs");
