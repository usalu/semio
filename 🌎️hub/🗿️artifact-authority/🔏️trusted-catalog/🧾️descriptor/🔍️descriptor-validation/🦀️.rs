//! 🔍️ Package descriptor validation: the semantic descriptor against its trust record, strict decoding and the load progress report.

use super::*;

/// 🪪️ A decoded semantic descriptor must be exactly the package its trust record names, speaking this hub's app channel,
/// and the record's shipped receipt must bind exactly that descriptor to exactly the record's component bytes. A refusal
/// names the package and every field that differs; a catalog published for another app channel is named as such, because
/// the operator's remedy (publish the catalog again with this tree) differs from a tampered descriptor's.
pub fn validate_descriptor(record: &TrustedBundlePackageV1, descriptor: &PackageDescriptor, packages: &[TrustedBundlePackageV1]) -> Result<(), AuthorityError> {
    let differing: Vec<&str> = [
        (descriptor.descriptor_version != 1, "descriptorVersion"),
        (!valid_package_id(&descriptor.package_id) || descriptor.package_id != record.package_id, "packageId"),
        (!record.role.matches(descriptor.role), "role"),
        (descriptor.manifest.plugin_id != record.plugin_id, "pluginId"),
        (descriptor.manifest.version != record.version, "version"),
        (descriptor.execution_protocol != record.execution_protocol, "executionProtocol"),
        (record.receipt.wasm_bytes != record.component.byte_length, "receipt wasmBytes"),
    ]
    .into_iter()
    .filter_map(|(differs, field)| differs.then_some(field))
    .collect();
    if !differing.is_empty() {
        return Err(catalog(&format!("package {} descriptor identity does not exactly match its trust record: {}", record.package_id, differing.join(", "))));
    }
    if descriptor.execution_protocol.app_channel_version != directory::os_spr::CHANNEL_VERSION {
        return Err(catalog(&format!(
            "package {} was published for app channel {} but this hub speaks app channel {}: publish the trusted catalog again with this hub's tree",
            record.package_id,
            descriptor.execution_protocol.app_channel_version,
            directory::os_spr::CHANNEL_VERSION
        )));
    }
    record.receipt.binds(directory::os_component_receipt::ComponentProfileV1::Release, &record.plugin_id, &record.package_id, &record.component.sha256, &record.descriptor.sha256).map_err(|refusal| catalog(&format!("package {} {refusal}", record.package_id)))?;
    if descriptor.manifest.dependencies.len() != record.dependencies.len() {
        return Err(catalog("decoded manifest dependency count does not match the trust record"));
    }
    let records: BTreeMap<&str, &TrustedBundlePackageV1> = packages.iter().map(|package| (package.plugin_id.as_str(), package)).collect();
    let mut manifest_dependencies = BTreeSet::new();
    for dependency in &descriptor.manifest.dependencies {
        if !manifest_dependencies.insert(dependency.plugin_id.as_str()) {
            return Err(catalog("decoded manifest dependency is duplicated"));
        }
        let expected = record.dependencies.iter().find(|entry| entry.plugin_id == dependency.plugin_id).ok_or_else(|| catalog("decoded manifest dependency is absent from the trust record"))?;
        let package = records.get(expected.plugin_id.as_str()).ok_or_else(|| catalog("decoded manifest dependency package is missing"))?;
        let version = Version::parse(&package.version).map_err(catalog_error)?;
        if expected.package_id != package.package_id || expected.version != package.version || !dependency.version.matches(&version) {
            return Err(catalog("decoded manifest dependency version or package identity conflicts"));
        }
    }
    let mut manifest_kinds = BTreeSet::new();
    for kind in &descriptor.manifest.artifact_kinds {
        if !manifest_kinds.insert(kind.id.as_str()) {
            return Err(catalog("decoded manifest artifact kind is duplicated"));
        }
        if !record.native_codecs.iter().any(|codec| codec.artifact_kind == kind.id && codec.artifact_schema == kind.schema) {
            return Err(catalog("decoded manifest artifact kind is absent from the trust record"));
        }
    }
    let mut hosted_kinds = BTreeSet::new();
    for kind in &descriptor.manifest.hosted_artifact_kinds {
        if !hosted_kinds.insert((kind.id.as_str(), kind.schema.as_str())) || manifest_kinds.contains(kind.id.as_str()) {
            return Err(catalog("decoded manifest hosted kind is duplicated or also owned"));
        }
        if kind.owner.is_empty() || kind.owner == record.plugin_id {
            return Err(catalog("decoded manifest hosted kind names no other owner package"));
        }
        if !manifest_dependencies.contains(kind.owner.as_str()) || !records.contains_key(kind.owner.as_str()) {
            return Err(catalog("decoded manifest hosted kind's owner is not a declared dependency present in the catalog"));
        }
    }
    Ok(())
}

pub fn decode_package_descriptor(bytes: &[u8]) -> Result<PackageDescriptor, AuthorityError> {
    let mut options = os_store::PackDecodeOptions::default();
    options.limits.max_file_len = TRUSTED_DESCRIPTOR_MAX_BYTES;
    options.limits.max_segment_len = TRUSTED_DESCRIPTOR_MAX_BYTES;
    options.limits.max_symbols = 131_072;
    options.limits.max_items = 262_144;
    options.limits.max_depth = 64;
    options.limits.max_total_alloc = TRUSTED_DESCRIPTOR_MAX_MATERIALIZATION;
    let value = directory::os_pack::pack_rt::decode_wire_value_with_options(bytes, &options).map_err(catalog_error)?;
    reject_duplicate_descriptor_fields(&value)?;
    let canonical = directory::os_pack::pack_rt::encode_wire_value(&value);
    if canonical != bytes {
        return Err(catalog("package descriptor is not its exact canonical schema projection"));
    }
    let descriptor: PackageDescriptor = semio_framework_value::FromValue::from_value(value).map_err(catalog_error)?;
    let projection = semio_framework_value::ToValue::to_value(&descriptor);
    if directory::os_pack::pack_rt::encode_wire_value(&projection) != bytes { return Err(catalog("package descriptor is not its exact canonical schema projection")); }
    Ok(descriptor)
}

fn reject_duplicate_descriptor_fields(value: &DslValue) -> Result<(), AuthorityError> {
    match value {
        DslValue::Array(items) => {
            for item in items {
                reject_duplicate_descriptor_fields(item)?;
            }
        }
        DslValue::Object(entries) => {
            let mut keys = BTreeSet::new();
            for (key, item) in entries {
                if !keys.insert(key.as_str()) {
                    return Err(catalog("package descriptor contains a duplicate object field"));
                }
                reject_duplicate_descriptor_fields(item)?;
            }
        }
        _ => {}
    }
    Ok(())
}

pub fn report_package_progress(context: &OperationContext<'_>, package_position: usize, package_phase: u64, total_units: u64) -> Result<(), AuthorityError> {
    let position = u64::try_from(package_position).map_err(catalog_error)?;
    let completed_units = position.checked_mul(4).and_then(|units| units.checked_add(package_phase)).ok_or_else(|| catalog("catalog progress overflow"))?;
    context.report(AuthorityProgress { stage: AuthorityProgressStage::CatalogLoading, completed_units, total_units })
}
