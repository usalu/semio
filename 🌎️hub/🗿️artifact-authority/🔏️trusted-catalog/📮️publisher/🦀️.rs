//! 📮️ Trusted-catalog publication: the cooperating-writer fence that publishes only a fully verified selection and its durable or unconfirmed outcome.

use super::*;

/// 🔐️ Publishes only a fully verified selection while holding the cooperating-writer OS fence.
pub struct TrustedCatalogPublisher;

/// 🚦️ Requires callers to distinguish durable selection from reconciliation-only visibility.
pub enum TrustedCatalogPublicationOutcome {
    Durable(Vec<u8>),
    Unconfirmed(Vec<u8>),
}

impl TrustedCatalogPublisher {
    /// 📤️ Consumes the closed command schema under a private server-owned catalog namespace.
    pub async fn publish_current(data_path: &Path, command_bytes: &[u8], providers: &dyn NativeCodecProviderSourceV1, context: &OperationContext<'_>) -> Result<TrustedCatalogPublicationOutcome, AuthorityError> {
        context.checkpoint()?;
        if command_bytes.is_empty() || command_bytes.len() > TRUSTED_CATALOG_PUBLICATION_MAX_BYTES { return Err(catalog("trusted publication command exceeds its bound")); }
        let fields: serde_json::Value = serde_json::from_slice(command_bytes).map_err(catalog_error)?;
        if !fields.as_object().is_some_and(|object| object.contains_key("expectedCurrentSha256")) { return Err(catalog("trusted publication expected current token is required")); }
        let command: TrustedCatalogPublicationCommandV1 = serde_json::from_slice(command_bytes).map_err(catalog_error)?;
        if command.schema != TRUSTED_CATALOG_PUBLICATION_SCHEMA || command.request_id.len() != 32 || !command.request_id.bytes().all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte)) || command.profile_id.is_empty() || command.profile_id.len() > TRUSTED_IDENTITY_MAX_BYTES || command.profile_id.chars().any(char::is_control) {
            return Err(catalog("trusted publication command has invalid identity"));
        }
        decode_digest(&command.generation_id, "trusted generation id")?;
        let bundle_digest = decode_digest(&command.bundle_sha256, "trusted bundle sha256")?;
        if let Some(expected) = &command.expected_current_sha256 { decode_digest(expected, "trusted expected current sha256")?; }
        let data = TrustedCatalogDataRoot::open_server_owned(data_path)?;
        let owner = data.acquire_publication(context).await?;
        let (observed, revision) = match owner.open_current()? {
            Some(file) => {
                let bytes = file.read_bounded(65_536, context).await?;
                let current = TrustedCatalogCurrentPointerV1::decode(&bytes)?;
                (Some(hex_lower(&sha256(&bytes, context).await?)), publication_revision(&current.publication_revision)?)
            }
            None => (None, 0),
        };
        if observed != command.expected_current_sha256 { return Err(catalog("trusted publication current token is stale")); }
        let revision = revision.checked_add(1).ok_or_else(|| catalog("trusted publication revision exhausted"))?;
        let generation = Arc::new(owner.open_generation(&command.generation_id)?);
        let relative = TrustedCatalogRelativePathV1::parse("trusted-catalog.json")?;
        let bundle_bytes = generation.read_regular(&relative, TRUSTED_BUNDLE_MAX_BYTES, context).await?;
        if sha256(&bundle_bytes, context).await? != bundle_digest { return Err(catalog("trusted publication candidate bundle differs from its digest")); }
        let bundle: TrustedBundleV1 = serde_json::from_slice(&bundle_bytes).map_err(catalog_error)?;
        let (verified, _) = TrustedCatalogLoader::verify_selected(&generation, relative, bundle_bytes, &command.profile_id, providers, &GuestCodecVerificationCacheV1::beside(data_path), &Arc::default(), context).await?;
        if verified.generation_id() != command.generation_id { return Err(catalog("trusted publication candidate generation differs from the verified profile")); }
        let mut published_module_files = BTreeSet::new();
        for package in &verified.packages {
            let record = bundle.packages.iter().find(|record| record.plugin_id == package.plugin_id && record.package_id == package.package.package.0 && record.version == package.version).ok_or_else(|| catalog("verified publication package lost its bundle record"))?;
            let mut files = vec![TrustedBundleFileV1 { path: record.component.path.clone(), byte_length: record.component.byte_length, sha256: record.component.sha256.clone() }, record.descriptor.clone()];
            if let Some(actor) = record.browser_actor.file() { files.push(actor); }
            files.push(TrustedBundleFileV1 { path: record.plugin_module.path.clone(), byte_length: record.plugin_module.byte_length, sha256: record.plugin_module.sha256.clone() });
            for file in &package.plugin_module.bundle.files {
                if published_module_files.insert(file.sha256.clone()) { files.push(TrustedBundleFileV1 { path: plugin_module_blob_path(&file.sha256), byte_length: file.byte_length, sha256: file.sha256.clone() }); }
            }
            for file in files {
                let bytes = generation.read_regular(&TrustedCatalogRelativePathV1::parse(&file.path)?, file.byte_length, context).await?;
                if bytes.len() as u64 != file.byte_length || sha256(&bytes, context).await? != decode_digest(&file.sha256, "trusted publication leaf sha256")? { return Err(catalog("trusted publication leaf changed after candidate verification")); }
            }
        }
        let final_bundle = generation.read_regular(&TrustedCatalogRelativePathV1::parse("trusted-catalog.json")?, TRUSTED_BUNDLE_MAX_BYTES, context).await?;
        if sha256(&final_bundle, context).await? != bundle_digest { return Err(catalog("trusted publication bundle changed after candidate verification")); }
        let pointer = TrustedCatalogCurrentPointerV1 { profile_id: command.profile_id, generation_id: command.generation_id, bundle_sha256: command.bundle_sha256, publication_revision: revision.to_string() };
        let current_bytes = pointer.encode()?;
        let current_sha256 = hex_lower(&sha256(&current_bytes, context).await?);
        let sync = owner.replace_current(&command.request_id, &current_bytes, context)?;
        let receipt = TrustedCatalogPublicationReceiptV1 { schema: TRUSTED_CATALOG_PUBLICATION_RECEIPT_SCHEMA, request_id: command.request_id, profile_id: pointer.profile_id, generation_id: pointer.generation_id, bundle_sha256: pointer.bundle_sha256, publication_revision: pointer.publication_revision, current_sha256, outcome: match sync { semio_hub_trusted_catalog_contract::TrustedPublicationSync::Durable => TRUSTED_CATALOG_PUBLICATION_OUTCOME_DURABLE, semio_hub_trusted_catalog_contract::TrustedPublicationSync::Unconfirmed => TRUSTED_CATALOG_PUBLICATION_OUTCOME_UNCONFIRMED } };
        let bytes = serde_json::to_vec(&receipt).map_err(catalog_error)?;
        Ok(match sync { semio_hub_trusted_catalog_contract::TrustedPublicationSync::Durable => TrustedCatalogPublicationOutcome::Durable(bytes), semio_hub_trusted_catalog_contract::TrustedPublicationSync::Unconfirmed => TrustedCatalogPublicationOutcome::Unconfirmed(bytes) })
    }
}
