//! 🏗️ The trusted-catalog loader: current-generation load, fixture load and the verification of one selected bundle into a verified catalog.

use super::*;

/// 🏗️ Stateless verifier for one explicitly selected immutable trust bundle.
pub struct TrustedCatalogLoader;

impl TrustedCatalogLoader {
    /// 🛡️ Opens the current immutable generation beneath one server-owned Hub data root.
    pub async fn load_current(data_path: &Path, providers: &dyn NativeCodecProviderSourceV1, context: &OperationContext<'_>) -> Result<Option<VerifiedTrustedCatalog>, AuthorityError> {
        Self::load_current_observed(data_path, providers, &Arc::default(), context).await
    }

    /// 📈️ Opens the current generation and reports every package's load progress into the host's `progress` cell.
    pub async fn load_current_observed(data_path: &Path, providers: &dyn NativeCodecProviderSourceV1, progress: &Arc<TrustedCatalogLoadProgressCellV1>, context: &OperationContext<'_>) -> Result<Option<VerifiedTrustedCatalog>, AuthorityError> {
        if !data_path.try_exists().map_err(catalog_error)? {
            return Ok(None);
        }
        let data_root = TrustedCatalogDataRoot::open_server_owned(data_path)?;
        let Some(current_file) = data_root.open_current()? else {
            return Ok(None);
        };
        let current_bytes = current_file.read_bounded(64 * 1024, context).await?;
        let current = TrustedCatalogCurrentPointerV1::decode(&current_bytes)?;
        let expected_bundle_sha256 = decode_digest(&current.bundle_sha256, "trusted bundle sha256")?;
        let generation_root = Arc::new(data_root.open_generation(&current.generation_id)?);
        let bundle_path = TrustedCatalogRelativePathV1::parse("trusted-catalog.json")?;
        let bundle_bytes = generation_root.read_regular(&bundle_path, TRUSTED_BUNDLE_MAX_BYTES, context).await?;
        if sha256(&bundle_bytes, context).await? != expected_bundle_sha256 {
            return Err(catalog("trusted bundle differs from the current pointer digest"));
        }
        let verifications = GuestCodecVerificationCacheV1::beside(data_path);
        let verified = Self::load_selected(&generation_root, bundle_path, bundle_bytes, &current.profile_id, providers, &verifications, progress, context).await?;
        if verified.generation_id() != current.generation_id {
            return Err(catalog("trusted current pointer generation differs from the selected profile"));
        }
        Ok(Some(verified))
    }

    #[cfg(any(test, feature = "integration-fixtures"))]
    pub async fn load_fixture(bundle_path: &Path, profile_id: &str, providers: &dyn NativeCodecProviderSourceV1, context: &OperationContext<'_>) -> Result<VerifiedTrustedCatalog, AuthorityError> {
        Self::load_fixture_observed(bundle_path, profile_id, providers, &Arc::default(), context).await
    }

    #[cfg(any(test, feature = "integration-fixtures"))]
    pub async fn load_fixture_observed(bundle_path: &Path, profile_id: &str, providers: &dyn NativeCodecProviderSourceV1, progress: &Arc<TrustedCatalogLoadProgressCellV1>, context: &OperationContext<'_>) -> Result<VerifiedTrustedCatalog, AuthorityError> {
        let path = std::fs::canonicalize(bundle_path).map_err(catalog_error)?;
        let fixture_root = path.parent().ok_or_else(|| catalog("bundle has no containing directory"))?;
        let generation_root = Arc::new(TrustedCatalogGenerationRoot::open_fixture_owned(fixture_root)?);
        let bundle_path = TrustedCatalogRelativePathV1::parse(path.file_name().and_then(|name| name.to_str()).ok_or_else(|| catalog("fixture bundle name is not UTF-8"))?)?;
        let bundle_bytes = generation_root.read_regular(&bundle_path, TRUSTED_BUNDLE_MAX_BYTES, context).await?;
        Self::load_selected(&generation_root, bundle_path, bundle_bytes, profile_id, providers, &GuestCodecVerificationCacheV1::disabled(), progress, context).await
    }

    async fn load_selected(
        root: &Arc<TrustedCatalogGenerationRoot>,
        bundle_path: TrustedCatalogRelativePathV1,
        bundle_bytes: Vec<u8>,
        profile_id: &str,
        providers: &dyn NativeCodecProviderSourceV1,
        verifications: &GuestCodecVerificationCacheV1,
        progress: &Arc<TrustedCatalogLoadProgressCellV1>,
        context: &OperationContext<'_>,
    ) -> Result<VerifiedTrustedCatalog, AuthorityError> {
        let (catalog, registration) = Self::verify_selected(root, bundle_path, bundle_bytes, profile_id, providers, verifications, progress, context).await?;
        let assembly = semio_framework_schema_registry::assembly::begin().map_err(catalog_error)?;
        os_store::codec_registry::preflight_document_codecs_in_assembly(&assembly, &registration.document_codecs).map_err(catalog_error)?;
        directory::io::preflight_artifact_codec_bindings_in_assembly(&assembly,&registration.document_bindings,&registration.document_codecs).map_err(catalog_error)?;
        directory::io::preflight_artifact_catalog_bindings_in_assembly(&assembly,&registration.catalog_bindings,&registration.document_codecs).map_err(catalog_error)?;
        context.checkpoint()?;
        directory::io::commit_artifact_assembly_registry_plan(&assembly, registration).map_err(catalog_error)?;
        Ok(catalog)
    }

    /// 📦️ One selected package whose files, digests, descriptor and browser actor are already
    /// verified, held until the whole closure is proven so no provider sees a package belonging to a
    /// closure that is still able to be refused. Component and actor bytes are not held: each was
    /// read, hashed and dropped, and is retained by identity.
    pub(super) async fn verify_selected(
        root: &Arc<TrustedCatalogGenerationRoot>,
        bundle_path: TrustedCatalogRelativePathV1,
        bundle_bytes: Vec<u8>,
        profile_id: &str,
        providers: &dyn NativeCodecProviderSourceV1,
        verifications: &GuestCodecVerificationCacheV1,
        progress: &Arc<TrustedCatalogLoadProgressCellV1>,
        context: &OperationContext<'_>,
    ) -> Result<(VerifiedTrustedCatalog, ArtifactAssemblyRegistryPlan), AuthorityError> {
        context.report(AuthorityProgress { stage: AuthorityProgressStage::Preflight, completed_units: 0, total_units: 1 })?;
        let guest_runtime = Arc::new(semio_framework_plugin_interpreter_runtime::OwnedRuntime::new());
        let residency = GuestResidencyLedgerV1::new(TRUSTED_CATALOG_GUEST_RESIDENCY);
        let progress = Arc::clone(progress);
        let bundle: TrustedBundleV1 = serde_json::from_slice(&bundle_bytes).map_err(catalog_error)?;
        let SelectedTrustedBundleV1 { package_indices: order, profile } = validate_bundle(&bundle, profile_id)?;
        progress.select(
            order
                .iter()
                .map(|index| {
                    let record = &bundle.packages[*index];
                    TrustedCatalogPackageProgressV1 { plugin_id: record.plugin_id.clone(), component_bytes: record.component.byte_length, phase: TrustedCatalogPackagePhaseV1::Pending, rows: u64::try_from(record.native_codecs.len()).unwrap_or(u64::MAX), rows_pinned: 0, rows_verified: 0 }
                })
                .collect(),
        );
        let order_len = u64::try_from(order.len()).map_err(|error| catalog_error(error))?;
        let total_units = order_len.checked_mul(4).and_then(|units| units.checked_add(1)).ok_or_else(|| catalog("catalog progress total overflow"))?;
        let requirements = order
            .iter()
            .flat_map(|index| {
                let record = &bundle.packages[*index];
                record.native_codecs.iter().map(move |codec| NativeCodecProviderRequirementV1 {
                    package: NativeCodecProviderPackageV1 { plugin_id: &record.plugin_id, package_id: &record.package_id, version: &record.version },
                    artifact_kind: &codec.artifact_kind,
                    artifact_schema: &codec.artifact_schema,
                    pack_schema_hash: &codec.pack_schema_hash,
                })
            })
            .collect::<Vec<_>>();
        providers.preflight_selection(&requirements)?;
        drop(requirements);
        let mut packages = Vec::with_capacity(order.len());
        let mut codecs = Vec::new();
        let mut open_targets = Vec::new();
        let mut registration = ArtifactAssemblyRegistryPlan::default();
        let mut resolved_paths = BTreeSet::from([bundle_path]);

        struct StagedTrustedPackage<'a> {
            position: usize,
            record: &'a TrustedBundlePackageV1,
            component: TrustedCatalogAsset,
            component_sha256: [u8; 32],
            component_blake3: [u8; 32],
            descriptor_bytes: Vec<u8>,
            descriptor_sha256: [u8; 32],
            descriptor: PackageDescriptor,
            browser_actor: DocumentOpenBrowserActorV1,
            browser_actor_asset: Option<TrustedCatalogAsset>,
            plugin_module: VerifiedPluginModule,
        }

        let mut staged = Vec::with_capacity(order.len());
        let mut plugin_module_files = BTreeMap::new();
        for (position, index) in order.into_iter().enumerate() {
            context.checkpoint()?;
            progress.phase(position, TrustedCatalogPackagePhaseV1::Reading);
            let record = &bundle.packages[index];
            let component_path = TrustedCatalogRelativePathV1::parse(&record.component.path)?;
            if !resolved_paths.insert(component_path.clone()) {
                return Err(catalog("trusted file path is already used by the selected closure"));
            }
            let component_bytes = root.read_regular(&component_path, TRUSTED_COMPONENT_MAX_BYTES, context).await?;
            verify_length(record.component.byte_length, component_bytes.len())?;
            let (component_sha256, component_blake3) = dual_hash(&component_bytes, context).await?;
            drop(component_bytes);
            verify_digest(&record.component.sha256, component_sha256, "component sha256")?;
            verify_digest(&record.component.blake3, component_blake3, "component blake3")?;
            let component = TrustedCatalogAsset::retained(TrustedRetainedFile::new(Arc::clone(root), component_path, record.component.byte_length, component_sha256, Some(component_blake3)));
            progress.component_read(position);
            report_package_progress(context, position, 1, total_units)?;

            let descriptor_path = TrustedCatalogRelativePathV1::parse(&record.descriptor.path)?;
            if !resolved_paths.insert(descriptor_path.clone()) {
                return Err(catalog("trusted file path is already used by the selected closure"));
            }
            let descriptor_bytes = root.read_regular(&descriptor_path, TRUSTED_DESCRIPTOR_MAX_BYTES, context).await?;
            verify_length(record.descriptor.byte_length, descriptor_bytes.len())?;
            let descriptor_sha256 = sha256(&descriptor_bytes, context).await?;
            verify_digest(&record.descriptor.sha256, descriptor_sha256, "descriptor sha256")?;
            let descriptor = decode_package_descriptor(&descriptor_bytes)?;
            validate_descriptor(record, &descriptor, &bundle.packages)?;
            report_package_progress(context, position, 2, total_units)?;

            let browser_actor = record.browser_actor.identity();
            record.browser_actor.validate(DocumentBrowserActorSourceV1 { component_sha256: &hex_lower(&component_sha256), descriptor_byte_sha256: &hex_lower(&descriptor_sha256) }, package_actor_renderer(record))?;
            let browser_actor_asset = if let Some(file) = record.browser_actor.file() {
                let actor_path = TrustedCatalogRelativePathV1::parse(&file.path)?;
                if !resolved_paths.insert(actor_path.clone()) {
                    return Err(catalog("trusted browser actor path is already used by the selected closure"));
                }
                let bytes = root.read_regular(&actor_path, file.byte_length, context).await?;
                verify_length(file.byte_length, bytes.len())?;
                let actor_sha256 = sha256(&bytes, context).await?;
                drop(bytes);
                verify_digest(&file.sha256, actor_sha256, "browser actor sha256")?;
                Some(TrustedCatalogAsset::retained(TrustedRetainedFile::new(Arc::clone(root), actor_path, file.byte_length, actor_sha256, None)))
            } else {
                None
            };
            let plugin_module = verify_plugin_module(root, record, &hex_lower(&component_sha256), &hex_lower(&descriptor_sha256), &mut resolved_paths, &mut plugin_module_files, context).await?;
            report_package_progress(context, position, 3, total_units)?;
            progress.phase(position, TrustedCatalogPackagePhaseV1::Staged);
            staged.push(StagedTrustedPackage { position, record, component, component_sha256, component_blake3, descriptor_bytes, descriptor_sha256, descriptor, browser_actor, browser_actor_asset, plugin_module });
        }

        let selected = |record: &TrustedBundlePackageV1, target: &TrustedBundleOpenTargetV1| {
            profile.open_targets.iter().any(|selected| selected.package.plugin_id == record.plugin_id && selected.package.package_id == record.package_id && selected.package.version == record.version && selected.target == *target)
        };
        let mut previews = Vec::with_capacity(staged.len());
        let mut pending_rows = BTreeMap::new();
        let mut natively_bound = BTreeSet::new();
        for stage in &staged {
            context.checkpoint()?;
            let native_bindings = providers.preview(NativeCodecProviderPackageV1 { plugin_id: &stage.record.plugin_id, package_id: &stage.record.package_id, version: &stage.record.version }, &stage.descriptor, context)?;
            context.checkpoint()?;
            let bound = validate_native_bindings(&native_bindings)?.into_keys().collect::<BTreeSet<_>>();
            let hosted = hosted_codec_targets(&bundle.packages, stage.record, |target| selected(stage.record, target))?;
            let own_rows = stage.record.native_codecs.iter().filter(|expected| !bound.contains(&CodecKey::from_parts(&stage.record.plugin_id, &stage.record.package_id, &expected.artifact_kind, &expected.artifact_schema))).map(|expected| (&expected.artifact_schema, &expected.pack_schema_hash));
            let hosted_rows = hosted.iter().filter(|(owner, target)| !natively_bound.contains(&CodecKey::from_parts(&owner.plugin_id, &owner.package_id, &target.artifact_kind, &target.artifact_schema))).map(|(_, target)| (&target.artifact_schema, &target.pack_schema_hash));
            let mut rows: Vec<(String, [u8; 32])> = Vec::new();
            for (schema, hash) in own_rows.chain(hosted_rows) {
                let expected_hash = decode_digest(hash, "pack schema hash")?;
                if expected_hash == [0; 32] {
                    return Err(catalog("artifact codec schema hash is zero"));
                }
                if rows.iter().any(|(row, row_hash)| row == schema && *row_hash == expected_hash) || verifications.recall(&stage.component_sha256, schema).await == Some(expected_hash) {
                    continue;
                }
                rows.push((schema.clone(), expected_hash));
            }
            let total = stage.record.native_codecs.len() + hosted.len();
            let (total, pinned) = (u64::try_from(total).unwrap_or(u64::MAX), u64::try_from(total - rows.len()).unwrap_or(u64::MAX));
            natively_bound.extend(bound);
            progress.package(stage.position, |package| {
                package.rows = total;
                package.rows_pinned = pinned;
                if rows.is_empty() {
                    package.phase = TrustedCatalogPackagePhaseV1::Ready;
                }
            });
            pending_rows.insert(stage.position, rows);
            previews.push(native_bindings);
        }
        let verifications = Arc::new(verifications.clone());
        let mut guests = Vec::with_capacity(staged.len());

        for (stage, native_bindings) in staged.into_iter().zip(previews) {
            let StagedTrustedPackage { position, record, component, component_sha256, component_blake3, descriptor_bytes, descriptor_sha256, descriptor, browser_actor, browser_actor_asset, plugin_module } = stage;
            let catalog_package=DocumentOpenPackageV1{plugin_id:record.plugin_id.clone(),package_id:record.package_id.clone(),version:record.version.clone(),component_sha256:hex_lower(&component_sha256),component_blake3:hex_lower(&component_blake3),descriptor_byte_sha256:hex_lower(&descriptor_sha256),execution_protocol:DocumentExecutionProtocolV1{app_channel_version:descriptor.execution_protocol.app_channel_version},component_receipt:record.receipt.clone()};
            context.checkpoint()?;
            let binding_map = validate_native_bindings(&native_bindings)?;
            let mut consumed_bindings = BTreeSet::new();
            let package_ref = PackageRef { package: PackageId(record.package_id.clone()), hash: PackageHash(component_blake3) };
            let guest = Arc::new(GuestArtifactComponent {
                runtime: Arc::clone(&guest_runtime),
                package: package_ref.clone(),
                component: component.clone(),
                compiled: residency.register(record.component.byte_length),
                position,
                component_sha256,
                rows: pending_rows.remove(&position).unwrap_or_default(),
                verified: tokio::sync::OnceCell::new(),
                verifications: Arc::clone(&verifications),
                progress: Arc::clone(&progress),
            });
            guests.push(Arc::clone(&guest));

            for expected in &record.native_codecs {
                if codecs.len() >= TRUSTED_CATALOG_MAX_CODECS {
                    return Err(AuthorityError::ResourceLimit("trusted codec count"));
                }
                let key = CodecKey::from_parts(&record.plugin_id, &record.package_id, &expected.artifact_kind, &expected.artifact_schema);
                let expected_hash = decode_digest(&expected.pack_schema_hash, "pack schema hash")?;
                if expected_hash == [0; 32] {
                    return Err(catalog("artifact codec schema hash is zero"));
                }
                let binding = binding_map.get(&key);
                if let Some(binding) = binding {
                    if binding.codec.pack_schema_hash == [0; 32] || binding.codec.pack_schema_hash != expected_hash || binding.codec.schema != expected.artifact_schema {
                        return Err(catalog("native codec schema hash is zero or mismatched"));
                    }
                    consumed_bindings.insert(key);
                }
                let identity = TrustedArtifactIdentity {
                    plugin_id: record.plugin_id.clone(),
                    package_id: record.package_id.clone(),
                    version: record.version.clone(),
                    package_hash: hex_lower(&component_sha256),
                    artifact_kind: expected.artifact_kind.clone(),
                    artifact_schema: expected.artifact_schema.clone(),
                    pack_schema_hash: expected.pack_schema_hash.clone(),
                };
                if codecs.iter().any(|entry: &VerifiedNativeArtifactCodec| entry.identity == identity) {
                    return Err(catalog("duplicate exact trusted artifact identity"));
                }
                if let Some(binding) = binding {
                    registration.document_codecs.push(binding.codec.clone());
                }
                let owner_declaration=ArtifactCatalogBinding{contributor:catalog_package.clone(),owner:catalog_package.clone(),artifact:DocumentOpenArtifactV1{kind:expected.artifact_kind.clone(),schema:expected.artifact_schema.clone(),pack_schema_hash:expected.pack_schema_hash.clone()},target:None,capability:binding.map_or(ArtifactCatalogCapability::UnlinkedGuest,|binding|ArtifactCatalogCapability::linked(&binding.codec,binding.factory_id.clone()))};
                registration.catalog_bindings.push(owner_declaration.clone());
                codecs.push(VerifiedNativeArtifactCodec {
                    identity,
                    codec: binding.map(|binding| binding.codec.clone()),
                    guest: GuestArtifactCodecBinding { component: Arc::clone(&guest), artifact_schema: expected.artifact_schema.clone() },
                    owner_declaration,
                });
            }
            if consumed_bindings.len() != binding_map.len() {
                return Err(catalog("selected provider returned a binding outside its exact declared package closure"));
            }
            for target in &record.open_targets {
                if open_targets.len() >= TRUSTED_CATALOG_MAX_OPEN_TARGETS {
                    return Err(AuthorityError::ResourceLimit("trusted document-open target count"));
                }
                let parent_dialect = validate_descriptor_open_target(&descriptor, target)?;
                if !selected(record, target) {
                    continue;
                }
                let bound = open_target_codec_package(&bundle.packages, record, target)?;
                if bound.plugin_id != record.plugin_id && !descriptor.manifest.hosted_artifact_kinds.iter().any(|kind| kind.id == target.artifact_kind && kind.schema == target.artifact_schema && kind.owner == bound.plugin_id) {
                    return Err(catalog("document-open target binds a dependency's codec its descriptor does not host from that owner"));
                }
                let role = match target.role {
                    TrustedBundleOpenRole::Viewer => DocumentOpenSurfaceRoleV1::Viewer,
                    TrustedBundleOpenRole::Editor => DocumentOpenSurfaceRoleV1::Editor,
                };
                let renderer_target = match target.renderer_target {
                    TrustedBundleRendererTarget::React => DocumentOpenRendererTargetV1::React,
                    TrustedBundleRendererTarget::Wgpu => DocumentOpenRendererTargetV1::Wgpu,
                    TrustedBundleRendererTarget::Wasm => DocumentOpenRendererTargetV1::Wasm,
                };
                let selection = VerifiedDocumentOpenSelectionV1 {
                    parent_dialect,
                    package: DocumentOpenPackageV1 {
                        plugin_id: record.plugin_id.clone(),
                        package_id: record.package_id.clone(),
                        version: record.version.clone(),
                        component_sha256: hex_lower(&component_sha256),
                        component_blake3: hex_lower(&component_blake3),
                        descriptor_byte_sha256: hex_lower(&descriptor_sha256),
                        execution_protocol: DocumentExecutionProtocolV1 { app_channel_version: descriptor.execution_protocol.app_channel_version },
                        component_receipt: record.receipt.clone(),
                    },
                    artifact: DocumentOpenArtifactV1 { kind: target.artifact_kind.clone(), schema: target.artifact_schema.clone(), pack_schema_hash: target.pack_schema_hash.clone() },
                    surface: DocumentOpenSurfaceV1 { surface_id: target.surface_id.clone(), app_id: target.app_id.clone(), window_kind_id: target.window_kind_id.clone(), role, renderer_target },
                    grant: DocumentOpenGrantV1 { read: target.grant.read, write: target.grant.write, observe: target.grant.observe },
                    browser_actor: if matches!(renderer_target, DocumentOpenRendererTargetV1::Wasm) { browser_actor.clone() } else { DocumentOpenBrowserActorV1::None },
                };
                if open_targets.iter().any(|existing| document_open_target_sort_key(existing) == document_open_target_sort_key(&selection)) {
                    return Err(catalog("document-open target identity is duplicated"));
                }
                let owned=codecs.iter().find(|entry|entry.identity.plugin_id==bound.plugin_id&&entry.identity.package_id==bound.package_id&&entry.identity.version==bound.version&&entry.identity.artifact_kind==target.artifact_kind&&entry.identity.artifact_schema==target.artifact_schema&&entry.identity.pack_schema_hash==target.pack_schema_hash).ok_or_else(||catalog("document-open target's exact codec owner is not verified before selection"))?;
                registration.catalog_bindings.push(ArtifactCatalogBinding{contributor:selection.package.clone(),owner:owned.owner_declaration.owner.clone(),artifact:selection.artifact.clone(),target:Some(ArtifactCatalogTarget{parent_dialect:selection.parent_dialect.clone(),surface:selection.surface.clone(),grant:selection.grant,browser_actor:selection.browser_actor.clone()}),capability:owned.owner_declaration.capability.clone()});
                open_targets.push(selection);
            }
            for (owner, target) in hosted_codec_targets(&bundle.packages, record, |target| selected(record, target))? {
                if codecs.len() >= TRUSTED_CATALOG_MAX_CODECS {
                    return Err(AuthorityError::ResourceLimit("trusted codec count"));
                }
                let identity = TrustedArtifactIdentity {
                    plugin_id: record.plugin_id.clone(),
                    package_id: record.package_id.clone(),
                    version: record.version.clone(),
                    package_hash: hex_lower(&component_sha256),
                    artifact_kind: target.artifact_kind.clone(),
                    artifact_schema: target.artifact_schema.clone(),
                    pack_schema_hash: target.pack_schema_hash.clone(),
                };
                let owned = codecs
                    .iter()
                    .find(|entry| entry.identity.plugin_id == owner.plugin_id && entry.identity.package_id == owner.package_id && entry.identity.version == owner.version && entry.identity.artifact_kind == identity.artifact_kind && entry.identity.artifact_schema == identity.artifact_schema && entry.identity.pack_schema_hash == identity.pack_schema_hash)
                    .ok_or_else(|| catalog("hosted document-open target's owner codec is not verified before its host"))?;
                let codec = owned.codec.clone();
                let owner_declaration=owned.owner_declaration.clone();
                if codecs.iter().any(|entry| entry.identity == identity) {
                    return Err(catalog("duplicate exact trusted artifact identity"));
                }
                codecs.push(VerifiedNativeArtifactCodec { identity, codec, guest: GuestArtifactCodecBinding { component: Arc::clone(&guest), artifact_schema: target.artifact_schema.clone() },owner_declaration });
            }
            report_package_progress(context, position, 4, total_units)?;
            packages.push(VerifiedTrustedPackage {
                plugin_id: record.plugin_id.clone(),
                package: package_ref,
                version: record.version.clone(),
                dependencies: record.dependencies.iter().map(|dependency| dependency.plugin_id.clone()).collect(),
                plugin_module,
                component_sha256,
                descriptor_sha256,
                component,
                descriptor_bytes: descriptor_bytes.into(),
                receipt: record.receipt.clone(),
                browser_actor,
                browser_actor_asset,
                descriptor: Arc::new(descriptor),
            });
        }
        if codecs.is_empty() {
            return Err(catalog("selected profile exposes no executable artifact codec"));
        }
        sort_open_targets(&mut open_targets);
        if open_targets.len() != profile.open_targets.len() {
            return Err(catalog("selected profile resolved a different number of document-open targets than it declares"));
        }
        if open_targets.is_empty() {
            return Err(catalog("selected profile must resolve at least one document-open target"));
        }
        let generation_id = trusted_profile_generation(&bundle, &profile)?;
        if generation_id != profile.generation_id {
            return Err(catalog("trusted profile generation differs from the completely verified package, codec, and target closure"));
        }
        let artifact_kind_count = {
            let mut kinds = BTreeSet::new();
            for codec in &codecs {
                context.checkpoint()?;
                kinds.insert(codec.identity.artifact_kind.as_str());
            }
            kinds.len()
        };
        let catalog = VerifiedTrustedCatalog { residency, guests: guests.into_boxed_slice(), progress, packages: packages.into_boxed_slice(), codecs: codecs.into_boxed_slice(), artifact_kind_count, open_targets: open_targets.into_boxed_slice(), generation_id };
        context.report(AuthorityProgress { stage: AuthorityProgressStage::CatalogResolved, completed_units: total_units, total_units })?;
        for codec in &registration.document_codecs{let binding=ArtifactCodecBinding::schema_only(codec);if !registration.document_bindings.contains(&binding){registration.document_bindings.push(binding);}}
        Ok((catalog, registration))
    }
}
