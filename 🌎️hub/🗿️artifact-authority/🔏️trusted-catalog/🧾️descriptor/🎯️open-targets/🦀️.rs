//! 🎯️ Document-open target derivation: the one pairing rule, hosted codec targets, creation preference, the most general dialect and the sort key.

use super::*;

/// 🗂️ The one pairing rule between a verified descriptor's artifact kinds and its surfaces, shared by
/// publication ([`descriptor_open_targets`]) and verification ([`validate_descriptor_open_target`]). A
/// plugin-level kind (`PluginBuilder::artifact_kind`, the channel GIS still uses) is opened only by the
/// surfaces whose own dialect names it, even when a sibling app lists it as an input (GIS's terrain editor
/// lists the map). Any other kind is opened by the editor that declares it itself (where a plugin migrated
/// onto the declaration tree, ticket 26/08/17/CLEAN-ARTIFACT-STANDARD-SUBSET-MECHANISM, stitches its spec)
/// and by the viewers of that editor's dialect, the read-only surface of the same documents. A HOSTED kind
/// (`PluginManifest::hosted_artifact_kinds`) opens in the host surfaces whose dialect names it (a family hosting its owner's
/// kind, `PluginBuilder::host_artifact`) or whose io presents it — an embedded editor of another plugin's artifact
/// (demonstrator's cad editor presents `3d.cad`) — and in the viewers of a presenting editor's dialect.
fn app_opens_kind(descriptor: &PackageDescriptor, app: &semio_framework::AppDefinition, artifact_kind: &str, artifact_schema: &str) -> bool {
    let declares = |kinds: &[semio_framework::ArtifactKindSpec]| kinds.iter().any(|kind| kind.id == artifact_kind && kind.schema == artifact_schema);
    if declares(&descriptor.manifest.artifact_kinds) {
        return app.dialect.artifact_kind == artifact_kind;
    }
    if descriptor.manifest.hosted_artifact_kinds.iter().any(|kind| kind.id == artifact_kind && kind.schema == artifact_schema) {
        let presents = |candidate: &semio_framework::AppDefinition| candidate.io.artifact.id == artifact_kind && candidate.io.artifact_schema == artifact_schema;
        return app.dialect.artifact_kind == artifact_kind
            || presents(app)
            || (app.role == semio_framework::AppRole::Viewer && descriptor.manifest.apps.iter().any(|editor| editor.role == semio_framework::AppRole::Editor && editor.dialect == app.dialect && presents(editor)));
    }
    match app.role {
        semio_framework::AppRole::Editor => declares(&app.artifact_kinds),
        semio_framework::AppRole::Viewer => declares(&app.artifact_kinds) || descriptor.manifest.apps.iter().any(|editor| editor.role == semio_framework::AppRole::Editor && editor.dialect == app.dialect && declares(&editor.artifact_kinds)),
    }
}

/// 🎯️ Every document-open surface one verified descriptor declares, in app then kind declaration order:
/// each (editor or viewer, kind) pair [`app_opens_kind`] admits, in the app's first window kind, rendered by
/// the catalog's closed wasm actor, with the grant its role fixes (a viewer reads and observes, never writes),
/// so a read-only member opens the same document through its viewer. `os-hub trusted-catalog open-targets`
/// answers exactly this list, so a publisher never re-derives the rule.
pub fn descriptor_open_targets(descriptor: &PackageDescriptor) -> Vec<semio_hub_trusted_catalog_contract::schema::TrustedDescriptorOpenTargetV1> {
    if descriptor.execution != semio_framework::ExecutionMode::Isolated {
        return Vec::new();
    }
    let mut targets = Vec::new();
    for app in &descriptor.manifest.apps {
        if app.id != semio_framework::surface_app_id(&app.dialect, app.role) {
            continue;
        }
        let role = match app.role {
            semio_framework::AppRole::Viewer => TrustedBundleOpenRole::Viewer,
            semio_framework::AppRole::Editor => TrustedBundleOpenRole::Editor,
        };
        let mut seen = BTreeSet::new();
        let editors = descriptor.manifest.apps.iter().filter(|editor| app.role == semio_framework::AppRole::Viewer && editor.role == semio_framework::AppRole::Editor && editor.dialect == app.dialect);
        let owned = descriptor.manifest.artifact_kinds.iter().chain(app.artifact_kinds.iter()).chain(editors.flat_map(|editor| editor.artifact_kinds.iter())).map(|kind| (kind.id.as_str(), kind.schema.as_str()));
        let hosted = descriptor.manifest.hosted_artifact_kinds.iter().map(|kind| (kind.id.as_str(), kind.schema.as_str()));
        for (kind_id, kind_schema) in owned.chain(hosted) {
            if !seen.insert((kind_id, kind_schema)) || !app_opens_kind(descriptor, app, kind_id, kind_schema) {
                continue;
            }
            targets.push(semio_hub_trusted_catalog_contract::schema::TrustedDescriptorOpenTargetV1 {
                artifact_kind: kind_id.to_string(),
                artifact_schema: kind_schema.to_string(),
                surface_id: app.id.clone(),
                app_id: app.id.clone(),
                window_kind_id: app.window_kinds.first().id.clone(),
                role,
                renderer_target: TrustedBundleRendererTarget::Wasm,
                parent_dialect: app.dialect.clone(),
                grant: TrustedBundleGrantV1 { read: true, write: matches!(role, TrustedBundleOpenRole::Editor), observe: true },
            });
        }
    }
    targets
}

/// 🏠️ The package whose native codec one document-open target of `host` binds: `host` itself when it declares that exact
/// codec, else — for a kind `host` hosts — the ONE declared dependency of `host` in this catalog that declares the exact
/// codec (the load then holds it to the hosted row's explicit owner). Codec rows never move: a host carries none for the
/// kinds it hosts, and no owner is ever read off a kind id.
pub fn open_target_codec_package<'a>(packages: &'a [TrustedBundlePackageV1], host: &'a TrustedBundlePackageV1, target: &TrustedBundleOpenTargetV1) -> Result<&'a TrustedBundlePackageV1, AuthorityError> {
    let binds = |package: &TrustedBundlePackageV1| package.native_codecs.iter().any(|codec| codec.artifact_kind == target.artifact_kind && codec.artifact_schema == target.artifact_schema && codec.pack_schema_hash == target.pack_schema_hash);
    if binds(host) {
        return Ok(host);
    }
    let mut owners = packages.iter().filter(|package| package.plugin_id != host.plugin_id && host.dependencies.iter().any(|dependency| dependency.plugin_id == package.plugin_id) && binds(package));
    let owner = owners.next().ok_or_else(|| catalog("trusted document-open target binds no exact native codec of its own package or of a declared dependency in the catalog"))?;
    if owners.next().is_some() {
        return Err(catalog("trusted document-open target binds an exact native codec more than one declared dependency declares"));
    }
    Ok(owner)
}

/// 🏠️ The distinct HOSTED codec identities of `host`'s `selected` open targets, each with the declared dependency that owns
/// it ([`open_target_codec_package`]). A document a host creates or opens names the host as its descriptor owner (the host's
/// component ships the surface), so the catalog executes it under the host's own identity: the owner's native codec when the
/// hub links one, and the host's component for genesis and every guest codec call — never the owner's component, which need
/// not ship the surface (the `stdio` component ships none of its families' apps).
pub fn hosted_codec_targets<'a>(packages: &'a [TrustedBundlePackageV1], host: &'a TrustedBundlePackageV1, selected: impl Fn(&TrustedBundleOpenTargetV1) -> bool) -> Result<Vec<(&'a TrustedBundlePackageV1, &'a TrustedBundleOpenTargetV1)>, AuthorityError> {
    let mut hosted: Vec<(&TrustedBundlePackageV1, &TrustedBundleOpenTargetV1)> = Vec::new();
    for target in host.open_targets.iter().filter(|target| selected(target)) {
        let owner = open_target_codec_package(packages, host, target)?;
        if owner.plugin_id != host.plugin_id && !hosted.iter().any(|(_, known)| known.artifact_kind == target.artifact_kind && known.artifact_schema == target.artifact_schema && known.pack_schema_hash == target.pack_schema_hash) {
            hosted.push((owner, target));
        }
    }
    Ok(hosted)
}

/// 🌱️ The owner-preferred creation rule over two candidate tiers — the owners' editors, then the hosts' editors: the first
/// non-empty tier decides by [`most_general_dialect`], so a host never makes an owner's kind ambiguous, a whole-standard
/// editor creates for its subset editors, and two most general editors (two owners, two standards) never pick one.
pub fn owner_preferred_creation<T>(owned: Vec<T>, hosted: Vec<T>, dialect: impl Fn(&T) -> &semio_framework::ArtifactDialect) -> MostGeneralDialect<T> {
    most_general_dialect(if owned.is_empty() { hosted } else { owned }, dialect)
}

/// 🌳️ What [`most_general_dialect`] answers over the surfaces of one role that share a document.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MostGeneralDialect<T> {
    One(T),
    Ambiguous,
    Empty,
}

impl<T> MostGeneralDialect<T> {
    /// 🎯️ The one answer, when there is one.
    pub fn one(self) -> Option<T> {
        match self {
            Self::One(candidate) => Some(candidate),
            Self::Ambiguous | Self::Empty => None,
        }
    }
}

/// 🌳️ The ONE most-general-dialect rule ([fixture](../../../🧰️framework/🛍️products/💻️os/🔨️modules/🚪️io/🧫️fixtures/🌳️most-general-dialect/🔣️.json),
/// read by the guest's `artifact_codec_owner` too): the candidate no other candidate covers ([`dialect_covers`]); several
/// uncovered candidates are ambiguous and refused rather than resolved by order.
pub fn most_general_dialect<T>(candidates: Vec<T>, dialect: impl Fn(&T) -> &semio_framework::ArtifactDialect) -> MostGeneralDialect<T> {
    let covered = candidates.iter().map(|candidate| candidates.iter().any(|other| dialect_covers(dialect(other), dialect(candidate)))).collect::<Vec<_>>();
    let mut uncovered = candidates.into_iter().zip(covered).filter_map(|(candidate, covered)| (!covered).then_some(candidate));
    match (uncovered.next(), uncovered.next()) {
        (Some(one), None) => MostGeneralDialect::One(one),
        (Some(_), Some(_)) => MostGeneralDialect::Ambiguous,
        (None, _) => MostGeneralDialect::Empty,
    }
}

/// 🌳️ Whether `general` is the whole-standard dialect (`<kind>@<standard>/*`) of `subset`'s own kind and standard: every
/// document of a subset of a standard is a document of the whole standard, so the whole-standard surface covers it.
fn dialect_covers(general: &semio_framework::ArtifactDialect, subset: &semio_framework::ArtifactDialect) -> bool {
    let any = semio_framework::SubsetId::ANY.0;
    general.subset == any && subset.subset != any && general.artifact_kind == subset.artifact_kind && general.standard == subset.standard
}

/// 🏷️ The declaration of `selection`'s exact kind among `kinds`.
pub fn declared_kind<'a>(mut kinds: impl Iterator<Item = &'a semio_framework::ArtifactKindSpec>, selection: &VerifiedDocumentOpenSelectionV1) -> Option<&'a semio_framework::ArtifactKindSpec> {
    kinds.find(|kind| kind.id == selection.artifact.kind && kind.schema == selection.artifact.schema)
}

/// 📤️ Answers `os-hub trusted-catalog open-targets`: decodes one bounded descriptor exactly as the loader
/// does and serializes [`descriptor_open_targets`] as `TrustedCatalogDescriptorOpenTargetsV1`.
pub fn descriptor_open_targets_answer(descriptor_bytes: &[u8]) -> Result<Vec<u8>, AuthorityError> {
    if descriptor_bytes.is_empty() || descriptor_bytes.len() as u64 > TRUSTED_DESCRIPTOR_MAX_BYTES {
        return Err(catalog("descriptor bytes are empty or exceed the trusted descriptor bound"));
    }
    let descriptor = decode_package_descriptor(descriptor_bytes)?;
    let targets = descriptor_open_targets(&descriptor);
    if targets.len() > TRUSTED_CATALOG_MAX_OPEN_TARGETS {
        return Err(AuthorityError::ResourceLimit("trusted document-open target count"));
    }
    let mut bytes = serde_json::to_vec(&semio_hub_trusted_catalog_contract::schema::TrustedCatalogDescriptorOpenTargetsV1 { schema: semio_hub_trusted_catalog_contract::schema::TRUSTED_CATALOG_DESCRIPTOR_OPEN_TARGETS_SCHEMA, targets }).map_err(catalog_error)?;
    bytes.push(b'\n');
    Ok(bytes)
}

pub fn validate_descriptor_open_target(descriptor: &PackageDescriptor, target: &TrustedBundleOpenTargetV1) -> Result<semio_framework::ArtifactDialect, AuthorityError> {
    let expected_role = match target.role {
        TrustedBundleOpenRole::Viewer => semio_framework::AppRole::Viewer,
        TrustedBundleOpenRole::Editor => semio_framework::AppRole::Editor,
    };
    let app = descriptor.manifest.apps.iter().find(|app| app.id == target.app_id).ok_or_else(|| catalog("document-open target app is absent from the verified descriptor"))?;
    if !app_opens_kind(descriptor, app, &target.artifact_kind, &target.artifact_schema)
        || app.id != target.surface_id
        || app.id != semio_framework::surface_app_id(&app.dialect, app.role)
        || app.role != expected_role
        || app.dialect != target.parent_dialect
        || [&app.dialect.artifact_kind, &app.dialect.standard, &app.dialect.subset].into_iter().any(|value| !valid_open_identity(value))
        || !app.window_kinds.iter().any(|window| window.id == target.window_kind_id)
        || descriptor.execution != semio_framework::ExecutionMode::Isolated
        || target.renderer_target != TrustedBundleRendererTarget::Wasm
    {
        return Err(catalog("document-open target surface, app, window, role, renderer, or artifact differs from the verified descriptor"));
    }
    Ok(app.dialect.clone())
}

pub fn document_open_target_sort_key(target: &VerifiedDocumentOpenSelectionV1) -> [&str; 18] {
    let role = match target.surface.role {
        DocumentOpenSurfaceRoleV1::Viewer => "viewer",
        DocumentOpenSurfaceRoleV1::Editor => "editor",
    };
    let renderer = match target.surface.renderer_target {
        DocumentOpenRendererTargetV1::React => "react",
        DocumentOpenRendererTargetV1::Wgpu => "wgpu",
        DocumentOpenRendererTargetV1::Wasm => "wasm",
    };
    [
        &target.package.plugin_id,
        &target.package.package_id,
        &target.package.version,
        &target.package.component_sha256,
        &target.package.component_blake3,
        &target.package.descriptor_byte_sha256,
        &target.artifact.kind,
        &target.artifact.schema,
        &target.artifact.pack_schema_hash,
        &target.parent_dialect.artifact_kind,
        &target.parent_dialect.standard,
        &target.parent_dialect.subset,
        &target.surface.surface_id,
        &target.surface.app_id,
        &target.surface.window_kind_id,
        role,
        renderer,
        if target.grant.write { "111" } else { "101" },
    ]
}

pub fn sort_open_targets(targets: &mut [VerifiedDocumentOpenSelectionV1]) {
    targets.sort_by(|left, right| document_open_target_sort_key(left).cmp(&document_open_target_sort_key(right)));
}
