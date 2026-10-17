//! 📋️ Bundle selection and validation: the selected closure, profile generation ids and the whole trust-bundle checks.

use super::*;

#[derive(Debug)]
pub struct SelectedTrustedBundleV1 {
    pub package_indices: Vec<usize>,
    pub profile: TrustedBundleProfileV1,
}

pub fn selected_closure_digest(identities: &[TrustedBundleIdentityV1]) -> Result<[u8; 32], AuthorityError> {
    let mut encoded = b"semio/hub/trusted-profile-selected-closure/v1\0".to_vec();
    encoded.extend_from_slice(&u32::try_from(identities.len()).map_err(catalog_error)?.to_be_bytes());
    for identity in identities {
        for value in [identity.plugin_id.as_bytes(), identity.package_id.as_bytes(), identity.version.as_bytes()] {
            append_document_open_catalog_field(&mut encoded, value)?;
        }
    }
    Ok(Sha256::digest(&encoded))
}

pub fn package_actor_renderer(package: &TrustedBundlePackageV1) -> &'static str {
    if package.open_targets.iter().any(|target| matches!(target.renderer_target, TrustedBundleRendererTarget::Wasm)) {
        "wasm"
    } else {
        "react"
    }
}

/// 🧮️ Frames resolved dependency identities in native tuple order for every profile generation.
pub fn append_trusted_profile_dependencies(encoded: &mut Vec<u8>, dependencies: &[TrustedBundleIdentityV1]) -> Result<(), AuthorityError> {
    let mut dependencies = dependencies.iter().collect::<Vec<_>>();
    dependencies.sort();
    encoded.extend_from_slice(&u32::try_from(dependencies.len()).map_err(catalog_error)?.to_be_bytes());
    for dependency in dependencies {
        for value in [dependency.plugin_id.as_bytes(), dependency.package_id.as_bytes(), dependency.version.as_bytes()] {
            append_document_open_catalog_field(encoded, value)?;
        }
    }
    Ok(())
}

/// 🎯️ The whole SET, in one canonical order, so a generation id names every creatable kind it
/// admits — not merely the first (ticket 26/09/18 slice TC3b). The count is framed ahead of the
/// rows exactly as the selected closure's is, so adding a target can never collide with a
/// different bundle whose rows happen to concatenate identically.
pub fn trusted_profile_generation(bundle: &TrustedBundleV1, profile: &TrustedBundleProfileV1) -> Result<String, AuthorityError> {
    let mut encoded = Vec::new();
    encoded.extend_from_slice(b"semio/hub/trusted-profile-generation/v2\0");
    append_document_open_catalog_field(&mut encoded, profile.id.as_bytes())?;
    encoded.extend_from_slice(&u32::try_from(profile.selected_closure.len()).map_err(catalog_error)?.to_be_bytes());
    for identity in &profile.selected_closure {
        let package = bundle
            .packages
            .iter()
            .find(|package| package.plugin_id == identity.plugin_id && package.package_id == identity.package_id && package.version == identity.version)
            .ok_or_else(|| catalog("profile generation package is outside its selected closure"))?;
        let role = match package.role {
            TrustedBundlePackageRole::Plugin => b"plugin".as_slice(),
            TrustedBundlePackageRole::Extension => b"extension".as_slice(),
        };
        for value in [
            package.plugin_id.as_bytes(),
            package.package_id.as_bytes(),
            package.version.as_bytes(),
            role,
            decode_digest(&package.component.sha256, "profile component sha256")?.as_slice(),
            decode_digest(&package.component.blake3, "profile component blake3")?.as_slice(),
            decode_digest(&package.descriptor.sha256, "profile descriptor sha256")?.as_slice(),
            package.receipt.encode_canonical().as_slice(),
            package.execution_protocol.app_channel_version.to_be_bytes().as_slice(),
        ] {
            append_document_open_catalog_field(&mut encoded, value)?;
        }
        package.browser_actor.validate(DocumentBrowserActorSourceV1 { component_sha256: &package.component.sha256, descriptor_byte_sha256: &package.descriptor.sha256 }, package_actor_renderer(package))?;
        package.browser_actor.append_generation(&mut encoded)?;
        append_document_open_catalog_field(&mut encoded, package.plugin_module.path.as_bytes())?;
        encoded.extend_from_slice(&package.plugin_module.byte_length.to_be_bytes());
        append_document_open_catalog_field(&mut encoded, decode_digest(&package.plugin_module.sha256, "profile plugin module sha256")?.as_slice())?;
        append_document_open_catalog_field(&mut encoded, decode_digest(&package.plugin_module.blake3, "profile plugin module blake3")?.as_slice())?;
        append_trusted_profile_dependencies(&mut encoded, &package.dependencies)?;
        let mut codecs = package.native_codecs.iter().collect::<Vec<_>>();
        codecs.sort_by(|left, right| (&left.artifact_kind, &left.artifact_schema, &left.pack_schema_hash).cmp(&(&right.artifact_kind, &right.artifact_schema, &right.pack_schema_hash)));
        encoded.extend_from_slice(&u32::try_from(codecs.len()).map_err(catalog_error)?.to_be_bytes());
        for codec in codecs {
            append_document_open_catalog_field(&mut encoded, codec.artifact_kind.as_bytes())?;
            append_document_open_catalog_field(&mut encoded, codec.artifact_schema.as_bytes())?;
            append_document_open_catalog_field(&mut encoded, decode_digest(&codec.pack_schema_hash, "profile codec pack schema hash")?.as_slice())?;
        }
    }
    let mut selected = profile.open_targets.iter().collect::<Vec<_>>();
    selected.sort_by(|left, right| {
        (&left.package.plugin_id, &left.package.package_id, &left.package.version, &left.target.artifact_kind, &left.target.artifact_schema, &left.target.surface_id, left.target.role as u8)
            .cmp(&(&right.package.plugin_id, &right.package.package_id, &right.package.version, &right.target.artifact_kind, &right.target.artifact_schema, &right.target.surface_id, right.target.role as u8))
    });
    encoded.extend_from_slice(&u32::try_from(selected.len()).map_err(catalog_error)?.to_be_bytes());
    for selection in selected {
        let package = bundle
            .packages
            .iter()
            .find(|package| package.plugin_id == selection.package.plugin_id && package.package_id == selection.package.package_id && package.version == selection.package.version)
            .ok_or_else(|| catalog("profile generation open-target package is absent"))?;
        let target = &selection.target;
        let role = match target.role {
            TrustedBundleOpenRole::Viewer => b"viewer".as_slice(),
            TrustedBundleOpenRole::Editor => b"editor".as_slice(),
        };
        let renderer = match target.renderer_target {
            TrustedBundleRendererTarget::React => b"react".as_slice(),
            TrustedBundleRendererTarget::Wgpu => b"wgpu".as_slice(),
            TrustedBundleRendererTarget::Wasm => b"wasm".as_slice(),
        };
        for value in [
            package.plugin_id.as_bytes(),
            package.package_id.as_bytes(),
            package.version.as_bytes(),
            decode_digest(&package.component.sha256, "open target component sha256")?.as_slice(),
            decode_digest(&package.component.blake3, "open target component blake3")?.as_slice(),
            decode_digest(&package.descriptor.sha256, "open target descriptor sha256")?.as_slice(),
            package.execution_protocol.app_channel_version.to_be_bytes().as_slice(),
            target.artifact_kind.as_bytes(),
            target.artifact_schema.as_bytes(),
            decode_digest(&target.pack_schema_hash, "open target pack schema hash")?.as_slice(),
            target.parent_dialect.artifact_kind.as_bytes(),
            target.parent_dialect.standard.as_bytes(),
            target.parent_dialect.subset.as_bytes(),
            target.surface_id.as_bytes(),
            target.app_id.as_bytes(),
            target.window_kind_id.as_bytes(),
            role,
            renderer,
            [u8::from(target.grant.read), u8::from(target.grant.write), u8::from(target.grant.observe)].as_slice(),
        ] {
            append_document_open_catalog_field(&mut encoded, value)?;
        }
    }
    Ok(hex_lower(&Sha256::digest(&encoded)))
}

/// 🧬️ Recomputes the two canonical profile digests without trusting carried digest fields.
pub fn trusted_profile_digests_json(bundle_bytes: &[u8], profile_id: &str) -> Result<(String, String), AuthorityError> {
    let bundle: TrustedBundleV1 = serde_json::from_slice(bundle_bytes).map_err(catalog_error)?;
    let profile = bundle.profiles.iter().find(|profile| profile.id == profile_id).ok_or_else(|| catalog("trusted profile digest source is missing"))?;
    Ok((hex_lower(&selected_closure_digest(&profile.selected_closure)?), trusted_profile_generation(&bundle, profile)?))
}

/// 🎯️ An open target carries TWO artifact-kind ids from two deliberately distinct spaces, and they
/// are never equal in a real bundle: `artifact_kind` is the manifest `ArtifactKindSpec::id`
/// (`stdio.json` — `validate_descriptor_open_target` requires a manifest kind with exactly that id
/// and schema), while `parent_dialect` is the owning app's `Dialect`, whose kind is the descriptor
/// id (`s.stdio.json` — the same function requires `app.dialect == target.parent_dialect`). Every
/// stdio artifact ships both spellings (`📜️native-codec-factories.json`: `artifact_kind` is
/// `stdio.<x>` for all of them; every `Viewer::builder(…)` dialect is `s.stdio.<x>`). This loop
/// used to refuse a target whose two spellings differed, which made every real stdio bundle
/// unloadable and left `artifactAuthority` permanently not-ready. Binding is enforced where it is
/// meaningful: to a native codec of the same package below, and to the descriptor's own app and
/// artifact kind in `validate_descriptor_open_target`.
pub fn validate_bundle(bundle: &TrustedBundleV1, profile_id: &str) -> Result<SelectedTrustedBundleV1, AuthorityError> {
    if bundle.schema_version != TRUSTED_BUNDLE_SCHEMA_VERSION || bundle.packages.is_empty() || bundle.packages.len() > TRUSTED_CATALOG_MAX_PACKAGES || bundle.profiles.is_empty() || bundle.profiles.len() > TRUSTED_BUNDLE_MAX_PROFILES || !valid_identity(profile_id) {
        return Err(catalog("trusted bundle shape or version is invalid"));
    }
    let mut plugins = BTreeMap::new();
    let mut package_ids = BTreeSet::new();
    let mut paths = BTreeSet::new();
    for (index, package) in bundle.packages.iter().enumerate() {
        let identity = TrustedBundleIdentityV1 { plugin_id: package.plugin_id.clone(), package_id: package.package_id.clone(), version: package.version.clone() };
        validate_identity(&identity)?;
        if plugins.insert(package.plugin_id.as_str(), index).is_some() || !package_ids.insert(package.package_id.as_str()) {
            return Err(catalog("trusted plugin or package identity is duplicated"));
        }
        Version::parse(&package.version).map_err(catalog_error)?;
        ensure_count(package.dependencies.len(), TRUSTED_PACKAGE_MAX_DEPENDENCIES, "trusted dependency count")?;
        ensure_count(package.native_codecs.len(), TRUSTED_CATALOG_MAX_CODECS, "trusted codec count")?;
        ensure_count(package.open_targets.len(), TRUSTED_CATALOG_MAX_OPEN_TARGETS, "trusted document-open target count")?;
        validate_file(&TrustedBundleFileV1 { path: package.component.path.clone(), byte_length: package.component.byte_length, sha256: package.component.sha256.clone() }, TRUSTED_COMPONENT_MAX_BYTES)?;
        decode_digest(&package.component.blake3, "component blake3")?;
        validate_file(&package.descriptor, TRUSTED_DESCRIPTOR_MAX_BYTES)?;
        if !paths.insert(package.component.path.clone()) || !paths.insert(package.descriptor.path.clone()) {
            return Err(catalog("trusted file path is reused across package records"));
        }
        package.browser_actor.validate(DocumentBrowserActorSourceV1 { component_sha256: &package.component.sha256, descriptor_byte_sha256: &package.descriptor.sha256 }, package_actor_renderer(package))?;
        if let Some(file) = package.browser_actor.file() {
            if !paths.insert(file.path) {
                return Err(catalog("trusted browser actor path is reused across package records"));
            }
        }
        let module = &package.plugin_module;
        if module.path.is_empty() || module.path.len() > TRUSTED_RELATIVE_PATH_MAX_BYTES || !(1..=TRUSTED_PLUGIN_MODULE_BUNDLE_MAX_BYTES).contains(&module.byte_length) {
            return Err(catalog("trusted plugin module record is empty or exceeds its fixed boundary"));
        }
        decode_digest(&module.sha256, "plugin module sha256")?;
        decode_digest(&module.blake3, "plugin module blake3")?;
        if !paths.insert(module.path.clone()) {
            return Err(catalog("trusted plugin module path is reused across package records"));
        }
        let mut dependencies = BTreeSet::new();
        for dependency in &package.dependencies {
            validate_identity(dependency)?;
            Version::parse(&dependency.version).map_err(catalog_error)?;
            if !dependencies.insert(dependency) || dependency.plugin_id == package.plugin_id {
                return Err(catalog("trusted dependency identity is duplicated or self-referential"));
            }
        }
        let mut codec_identities = BTreeSet::new();
        for codec in &package.native_codecs {
            if !valid_identity(&codec.artifact_kind) || !valid_identity(&codec.artifact_schema) || decode_digest(&codec.pack_schema_hash, "pack schema hash")? == [0; 32] || !codec_identities.insert((codec.artifact_kind.as_str(), codec.artifact_schema.as_str())) {
                return Err(catalog("trusted native codec identity is empty, zero, or duplicated"));
            }
        }
        let mut open_target_keys = BTreeSet::new();
        for target in &package.open_targets {
            let expected_grant = TrustedBundleGrantV1 { read: true, write: matches!(target.role, TrustedBundleOpenRole::Editor), observe: true };
            if !valid_open_identity(&target.artifact_kind)
                || !valid_open_identity(&target.artifact_schema)
                || !valid_open_identity(&target.surface_id)
                || !valid_open_identity(&target.app_id)
                || !valid_open_identity(&target.window_kind_id)
                || [&target.parent_dialect.artifact_kind, &target.parent_dialect.standard, &target.parent_dialect.subset].into_iter().any(|value| !valid_open_identity(value))
            {
                return Err(catalog("trusted document-open target names an empty, padded or control-bearing identity"));
            }
            if target.grant != expected_grant {
                return Err(catalog("trusted document-open target grant differs from the one its role fixes"));
            }
            if decode_digest(&target.pack_schema_hash, "open target pack schema hash")? == [0; 32] {
                return Err(catalog("trusted document-open target pack schema hash is zero"));
            }
            open_target_codec_package(&bundle.packages, package, target)?;
            if !open_target_keys.insert((target.artifact_kind.as_str(), target.artifact_schema.as_str(), target.surface_id.as_str(), target.role as u8)) {
                return Err(catalog("trusted document-open target key is duplicated within one package"));
            }
        }
    }
    for package in &bundle.packages {
        for dependency in &package.dependencies {
            let index = *plugins.get(dependency.plugin_id.as_str()).ok_or_else(|| catalog("trusted bundle dependency is incomplete"))?;
            let expected = &bundle.packages[index];
            if expected.package_id != dependency.package_id || expected.version != dependency.version {
                return Err(catalog("trusted bundle dependency identity conflicts with its package record"));
            }
        }
    }
    let mut profiles = BTreeSet::new();
    let mut selected = None;
    for profile in &bundle.profiles {
        if !valid_identity(&profile.id)
            || profile.selected_closure.is_empty()
            || profile.selected_closure.len() > TRUSTED_CATALOG_MAX_PACKAGES
            || !profiles.insert(profile.id.as_str())
            || decode_digest(&profile.selected_closure_sha256, "selected closure sha256")? == [0; 32]
            || decode_digest(&profile.generation_id, "profile generation id")? == [0; 32]
        {
            return Err(catalog("trusted bundle profile is empty, oversized, or duplicated"));
        }
        let mut prior_plugin = None;
        let mut closure = BTreeSet::new();
        for identity in &profile.selected_closure {
            validate_identity(identity)?;
            if prior_plugin.is_some_and(|prior: &str| prior >= identity.plugin_id.as_str()) {
                return Err(catalog("trusted bundle selected closure is not in canonical plugin order"));
            }
            prior_plugin = Some(identity.plugin_id.as_str());
            let index = *plugins.get(identity.plugin_id.as_str()).ok_or_else(|| catalog("trusted bundle selected closure is incomplete"))?;
            let expected = &bundle.packages[index];
            if expected.package_id != identity.package_id || expected.version != identity.version || !closure.insert(index) {
                return Err(catalog("trusted bundle selected closure conflicts with its package record"));
            }
        }
        if hex_lower(&selected_closure_digest(&profile.selected_closure)?) != profile.selected_closure_sha256 {
            return Err(catalog("trusted bundle selected closure digest differs"));
        }
        if profile.open_targets.is_empty() || profile.open_targets.len() > TRUSTED_CATALOG_MAX_OPEN_TARGETS {
            return Err(catalog("trusted profile declares no document-open target or exceeds the generation ceiling"));
        }
        let mut profile_target_keys = BTreeSet::new();
        for selection in &profile.open_targets {
            validate_identity(&selection.package)?;
            let target_index = *plugins.get(selection.package.plugin_id.as_str()).ok_or_else(|| catalog("trusted profile open target package is absent"))?;
            let target_package = &bundle.packages[target_index];
            if !closure.contains(&target_index) || target_package.package_id != selection.package.package_id || target_package.version != selection.package.version || !target_package.open_targets.contains(&selection.target) {
                return Err(catalog("trusted profile open target is outside its selected closure"));
            }
            if !profile_target_keys.insert((
                selection.package.plugin_id.as_str(),
                selection.package.package_id.as_str(),
                selection.target.artifact_kind.as_str(),
                selection.target.artifact_schema.as_str(),
                selection.target.surface_id.as_str(),
                selection.target.role as u8,
            )) {
                return Err(catalog("trusted profile declares the same document-open target twice"));
            }
        }
        if profile.id == "local-stdio-gis-open-v1" {
            let identities = profile.selected_closure.iter().map(|identity| (identity.plugin_id.as_str(), identity.package_id.as_str())).collect::<Vec<_>>();
            let gis = bundle.packages.iter().find(|package| package.plugin_id == "gis");
            let stdio = bundle.packages.iter().find(|package| package.plugin_id == "stdio");
            if identities != [("gis", "semio:gis"), ("stdio", "semio:stdio")]
                || bundle.packages.len() != 2
                || gis.is_none_or(|package| {
                    package.native_codecs.len() != 2
                        || package.dependencies.as_slice() != std::slice::from_ref(&profile.selected_closure[1])
                        || !package.native_codecs.iter().any(|codec| codec.artifact_kind == "s.gis.gismap" && codec.artifact_schema == "gis.map")
                        || !package.native_codecs.iter().any(|codec| codec.artifact_kind == "s.gis.gisterrain" && codec.artifact_schema == "gis.terrain")
                })
                || stdio.is_none_or(|package| package.native_codecs.len() != 29 || !package.dependencies.is_empty())
                || profile.open_targets.len() != bundle.packages.iter().map(|package| package.open_targets.len()).sum::<usize>()
            {
                return Err(catalog("local stdio plus GIS profile is not its exact closed two-package native-codec closure opening every package target"));
            }
        }
        if profile.id == profile_id {
            selected = Some((profile, closure));
        }
    }
    let (profile, closure) = selected.ok_or_else(|| catalog("selected trusted bundle profile is missing"))?;
    let mut indegree = BTreeMap::new();
    let mut dependents: BTreeMap<usize, Vec<usize>> = BTreeMap::new();
    for &index in &closure {
        let package = &bundle.packages[index];
        indegree.insert(index, package.dependencies.len());
        for dependency in &package.dependencies {
            let dependency_index = *plugins.get(dependency.plugin_id.as_str()).ok_or_else(|| catalog("selected dependency closure is incomplete"))?;
            let dependency_package = &bundle.packages[dependency_index];
            if !closure.contains(&dependency_index) || dependency_package.package_id != dependency.package_id || dependency_package.version != dependency.version {
                return Err(catalog("selected dependency identity conflicts with its package record"));
            }
            dependents.entry(dependency_index).or_default().push(index);
        }
    }
    let mut ready: BTreeSet<(String, usize)> = indegree.iter().filter(|(_, degree)| **degree == 0).map(|(index, _)| (bundle.packages[*index].plugin_id.clone(), *index)).collect();
    let mut order = Vec::with_capacity(closure.len());
    while let Some((_, index)) = ready.pop_first() {
        order.push(index);
        for dependent in dependents.get(&index).into_iter().flatten() {
            let degree = indegree.get_mut(dependent).expect("selected dependent has indegree");
            *degree -= 1;
            if *degree == 0 {
                ready.insert((bundle.packages[*dependent].plugin_id.clone(), *dependent));
            }
        }
    }
    if order.len() != closure.len() {
        return Err(catalog("selected dependency closure contains a cycle"));
    }
    Ok(SelectedTrustedBundleV1 { package_indices: order, profile: profile.clone() })
}
