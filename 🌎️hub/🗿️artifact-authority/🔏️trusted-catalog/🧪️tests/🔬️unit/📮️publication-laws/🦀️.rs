/// 📮️ Laws of descriptor wire materialization and of trusted publication revisions, races and leaf substitution.

#[test]
fn trusted_descriptor_wire_materialization_matches_neutral_boundaries() {
    let fixture: serde_json::Value = serde_json::from_str(semio_hub_testing::hub_source("../🧰️framework/🔨️modules/🎒️pack/🌱️value/🧪️tests/🧫️fixtures/🧮️wire-materialization/🔣️.json")).unwrap();
    for row in fixture["cases"].as_array().unwrap() {
        let encoded = row["rawHex"].as_str().unwrap();
        let bytes: Vec<_> = (0..encoded.len()).step_by(2).map(|index| u8::from_str_radix(&encoded[index..index + 2], 16).unwrap()).collect();
        let limits = &row["limits"];
        let mut options = os_store::PackDecodeOptions::default();
        options.limits.max_file_len = limits["maxFileLen"].as_u64().unwrap();
        options.limits.max_segment_len = limits["maxSegmentLen"].as_u64().unwrap();
        options.limits.max_symbols = u32::try_from(limits["maxSymbols"].as_u64().unwrap()).unwrap();
        options.limits.max_depth = u16::try_from(limits["maxDepth"].as_u64().unwrap()).unwrap();
        options.limits.max_items = limits["maxItems"].as_u64().unwrap();
        options.limits.max_total_alloc = u64::MAX;
        let decoded = os_pack::pack_rt::decode_wire_value_with_options(&bytes, &options);
        match row["grammar"]["outcome"].as_str().unwrap() {
            "accepted" => assert_eq!(serde_json::to_value(decoded.expect("admitted bounded value")).unwrap(), row["grammar"]["value"], "{}", row["id"]),
            "limit" => assert!(matches!(decoded, Err(os_store::PackError::LimitExceeded(_))), "{}: {decoded:?}", row["id"]),
            "malformed" => {
                assert!(matches!(decoded, Err(os_store::PackError::Malformed { .. })), "{}: {decoded:?}", row["id"]);
                assert!(matches!(os_pack::pack_rt::decode_wire_value(&bytes), Err(os_store::PackError::Malformed { .. })), "default wire facade: {}", row["id"]);
            }
            "truncated" => {
                let expected = row["grammar"]["offset"].as_u64().unwrap();
                assert_eq!(expected, bytes.len() as u64);
                assert!(matches!(decoded, Err(os_store::PackError::Truncated(offset)) if offset == expected), "{}: {decoded:?}", row["id"]);
                assert!(matches!(os_pack::pack_rt::decode_wire_value(&bytes), Err(os_store::PackError::Truncated(offset)) if offset == expected), "default wire facade: {}", row["id"]);
            }
            outcome => panic!("unknown bounded wire outcome {outcome}"),
        }
    }
    let mut amplified = Vec::new();
    os_pack::pack_rt::write_varint_u64(&mut amplified, 1);
    os_pack::pack_rt::write_varint_u64(&mut amplified, 2 * 1024 * 1024);
    amplified.resize(amplified.len() + 2 * 1024 * 1024, b'x');
    amplified.extend_from_slice(&[1, 1, 0x11, 0x0c, 32]);
    for _ in 0..32 {
        amplified.extend_from_slice(&[0x06, 0]);
    }
    assert!((amplified.len() as u64) < TRUSTED_DESCRIPTOR_MAX_BYTES);
    let result = decode_package_descriptor(&amplified);
    assert!(matches!(&result, Err(AuthorityError::Catalog(message)) if message.contains("max_total_alloc")), "actual Hub materialization fence: {result:?}");
}

use directory::os_directory::io::binary::artifact_hash::hex_lower;
use std::sync::Arc;
use directory::io::ArtifactCatalogCapability;
use directory::os_store::serialization::ArtifactCodec;
use semio_hub_artifact_authority::{AuthorityError, AuthorityProgress, AuthorityProgressStage, OperationContext, TrustedArtifactIdentity};
use std::collections::BTreeSet;
use directory::os_component_receipt::ComponentReceiptV1;
use directory::os_directory::{DocumentDescriptor, DocumentOpenGrantV1, DocumentOpenSurfaceRoleV1};
use directory::os_directory::schema::DocumentOpenBrowserActorV1;
use semio_framework::{DslValue, PackageDescriptor};
use semio_hub_trusted_catalog::{GUEST_CALL_CANCELLATION_POLL, GUEST_CODEC_BUDGET, GuestCodecVerificationCacheV1, GuestResidencyLedgerV1, GuestResidencyV1, GuestResidentFootprintV1, TrustedCatalogLoadProgressCellV1, TrustedCatalogLoader, TrustedCatalogPublicationOutcome, TrustedCatalogPublisher, VerifiedTrustedCatalog, guest_codec_engine_identity, guest_verification_concurrency, interpret_off_worker};
use semio_framework_hash::{Hasher, Sha256};
use semio_hub_trusted_catalog_descriptor::{MostGeneralDialect, NativeCodecBinding, NativeCodecProviderPackageV1, NativeCodecProviderRequirementV1, NativeCodecProviderSourceV1, TrustedCatalogAsset, TrustedRetainedFile, VerifiedTrustedPackage, append_trusted_profile_dependencies, decode_package_descriptor, descriptor_open_targets, descriptor_open_targets_answer, most_general_dialect, open_target_codec_package, owner_preferred_creation, selected_closure_digest, trusted_profile_generation, validate_bundle, validate_descriptor, validate_descriptor_open_target, validate_native_bindings};
use std::path::Path;
use semio_hub_trusted_catalog_contract::schema::{TRUSTED_BUNDLE_SCHEMA_VERSION, TRUSTED_CATALOG_GUEST_RESIDENCY, TrustedBundleFileV1, TrustedBundleGrantV1, TrustedBundleIdentityV1, TrustedBundleOpenRole, TrustedBundleOpenTargetV1, TrustedBundlePackageRole, TrustedBundlePackageV1, TrustedBundleProfileV1, TrustedBundleRendererTarget, TrustedBundleV1, TrustedCatalogCurrentPointerV1, TrustedCatalogGuestResidencyStateV1, TrustedCatalogGuestResidencyV1, TrustedCatalogPackagePhaseV1, publication_revision};
use semio_hub_artifact_authority::adapters::TRUSTED_CATALOG_MAX_CODECS;
use semio_hub_trusted_catalog_contract::{TRUSTED_COMPONENT_MAX_BYTES, TRUSTED_DESCRIPTOR_MAX_BYTES, TRUSTED_IDENTITY_MAX_BYTES, TRUSTED_READ_CHUNK_BYTES, TrustedCatalogDataRoot, TrustedCatalogGenerationRoot, TrustedCatalogRelativePathV1, catalog, catalog_error, dual_hash, ensure_count, valid_identity, valid_package_id, validate_file};
use directory::os_store;
#[cfg(all(any(test, feature = "testkit"),unix))]
use semio_hub_trusted_catalog_contract::create_fifo_fixture;
#[cfg(any(test,feature="integration-fixtures"))]
use semio_hub_trusted_catalog::{fixture_component_receipt, headless_stdio_fixture_package, write_fixture_plugin_module};
/// 🧳️ Stages exact existing fixture bytes under a real private publication data root.
fn stage_publication_fixture(fixture: &FixtureDirectory, data: &Path) -> serde_json::Value {
    let generation = fixture.bundle["profiles"][0]["generationId"].as_str().unwrap();
    let root = data.join("trusted-catalog/generations").join(generation);
    std::fs::create_dir_all(&root).unwrap();
    for record in fixture.bundle["packages"].as_array().unwrap() {
        let mut paths = vec![record["component"]["path"].as_str().unwrap().to_owned(), record["descriptor"]["path"].as_str().unwrap().to_owned(), record["pluginModule"]["path"].as_str().unwrap().to_owned()];
        if let Some(path) = record["browserActor"]["path"].as_str() {
            paths.push(path.to_owned());
        }
        let manifest: serde_json::Value = serde_json::from_slice(&std::fs::read(fixture.root.join(record["pluginModule"]["path"].as_str().unwrap())).unwrap()).unwrap();
        paths.extend(manifest["files"].as_array().unwrap().iter().map(|file| semio_hub_trusted_catalog_contract::plugin_module::plugin_module_blob_path(file["sha256"].as_str().unwrap())));
        for path in &paths {
            let destination = root.join(path);
            std::fs::create_dir_all(destination.parent().unwrap()).unwrap();
            std::fs::copy(fixture.root.join(path), destination).unwrap();
        }
    }
    let bundle = serde_json::to_vec(&fixture.bundle).unwrap();
    std::fs::write(root.join("trusted-catalog.json"), &bundle).unwrap();
    serde_json::json!({"schema":"semio.hub.trusted-catalog-publication/v1","requestId":"11".repeat(16),"profileId":"fixture","generationId":generation,"bundleSha256":hex_lower(&Sha256::digest(&bundle)),"expectedCurrentSha256":null})
}

/// 🧾️ Keeps indeterminate Windows visibility explicit in platform-neutral native tests.
fn publication_fixture_receipt(outcome: TrustedCatalogPublicationOutcome) -> serde_json::Value {
    let (bytes, expected) = match outcome {
        TrustedCatalogPublicationOutcome::Durable(bytes) => (bytes, "durable"),
        TrustedCatalogPublicationOutcome::Unconfirmed(bytes) => (bytes, "replaced-unconfirmed"),
    };
    let receipt: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
    assert_eq!(receipt["outcome"], expected);
    receipt
}

#[test]
fn trusted_publication_revision_matches_neutral_closed_u64_cases() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/📤️publication/🔄️cas.json")).unwrap();
    for row in fixture["cases"].as_array().unwrap() {
        let current = row["currentRevision"].as_str().map(publication_revision).transpose();
        let result = current.and_then(|current| {
            if let Some(expected) = row["expectedRevision"].as_str() {
                if Some(publication_revision(expected)?) != current {
                    return Err(catalog("stale fixture token"));
                }
            }
            current.unwrap_or(0).checked_add(1).ok_or_else(|| catalog("revision exhausted"))
        });
        assert_eq!(result.is_ok(), row["accepted"].as_bool().unwrap(), "{}", row["id"]);
        if let Some(next) = row["nextRevision"].as_str() {
            assert_eq!(result.unwrap().to_string(), next);
        }
    }
}

#[tokio::test]
async fn trusted_publication_cas_races_and_aba_use_real_catalog_loader() {
    let a = prepared_fixture();
    let b = prepared_fixture();
    let data = a.root.join("publication-data");
    let mut command_a = stage_publication_fixture(&a, &data);
    let mut command_b = stage_publication_fixture(&b, &data);
    let provider_a = FixtureProviderSource::new(vec![a.binding()]);
    let provider_b = FixtureProviderSource::new(vec![b.binding()]);
    let control = TestControl::new();
    let context = control.context();
    let initial = publication_fixture_receipt(TrustedCatalogPublisher::publish_current(&data, &serde_json::to_vec(&command_a).unwrap(), &provider_a, &context).await.unwrap());
    assert_eq!(initial["publicationRevision"], "1");
    assert!(document_codec(&a.schema).await.unwrap().is_none(), "publication installed a codec");
    command_b["expectedCurrentSha256"] = initial["currentSha256"].clone();
    command_b["requestId"] = "22".repeat(16).into();
    let middle = publication_fixture_receipt(TrustedCatalogPublisher::publish_current(&data, &serde_json::to_vec(&command_b).unwrap(), &provider_b, &context).await.unwrap());
    command_a["expectedCurrentSha256"] = middle["currentSha256"].clone();
    command_a["requestId"] = "33".repeat(16).into();
    let restored = publication_fixture_receipt(TrustedCatalogPublisher::publish_current(&data, &serde_json::to_vec(&command_a).unwrap(), &provider_a, &context).await.unwrap());
    assert_eq!(restored["publicationRevision"], "3");
    assert_eq!(restored["generationId"], initial["generationId"]);
    assert_ne!(restored["currentSha256"], initial["currentSha256"]);
    let pointer_path = data.join("trusted-catalog/current.json");
    let before = std::fs::read(&pointer_path).unwrap();
    command_a["expectedCurrentSha256"] = initial["currentSha256"].clone();
    assert!(TrustedCatalogPublisher::publish_current(&data, &serde_json::to_vec(&command_a).unwrap(), &provider_a, &context).await.is_err());
    assert_eq!(std::fs::read(&pointer_path).unwrap(), before);
    let loaded = TrustedCatalogLoader::load_current(&data, &provider_a, &context).await.unwrap().unwrap();
    assert_eq!(loaded.generation_id(), initial["generationId"].as_str().unwrap());
    command_a["expectedCurrentSha256"] = restored["currentSha256"].clone();
    command_b["expectedCurrentSha256"] = restored["currentSha256"].clone();
    command_a["requestId"] = "44".repeat(16).into();
    command_b["requestId"] = "55".repeat(16).into();
    let bytes_a = serde_json::to_vec(&command_a).unwrap();
    let bytes_b = serde_json::to_vec(&command_b).unwrap();
    let (left, right) = tokio::join!(TrustedCatalogPublisher::publish_current(&data, &bytes_a, &provider_a, &context), TrustedCatalogPublisher::publish_current(&data, &bytes_b, &provider_b, &context));
    assert_ne!(left.is_ok(), right.is_ok(), "competing current token admitted two or zero publishers");
    let winner = publication_fixture_receipt(left.or(right).unwrap());
    assert_eq!(winner["publicationRevision"], "4");
    let provider = if winner["generationId"] == command_a["generationId"] { &provider_a } else { &provider_b };
    assert_eq!(TrustedCatalogLoader::load_current(&data, provider, &context).await.unwrap().unwrap().generation_id(), winner["generationId"].as_str().unwrap());
    let mut maximum = TrustedCatalogCurrentPointerV1::decode(&std::fs::read(&pointer_path).unwrap()).unwrap();
    maximum.publication_revision = u64::MAX.to_string();
    let maximum_bytes = maximum.encode().unwrap();
    std::fs::write(&pointer_path, &maximum_bytes).unwrap();
    let mut exhausted = if winner["generationId"] == command_a["generationId"] { command_a } else { command_b };
    exhausted["expectedCurrentSha256"] = hex_lower(&Sha256::digest(&maximum_bytes)).into();
    assert!(TrustedCatalogPublisher::publish_current(&data, &serde_json::to_vec(&exhausted).unwrap(), provider, &context).await.is_err());
    assert_eq!(std::fs::read(&pointer_path).unwrap(), maximum_bytes);
}

struct PublicationLeafSwap {
    path: PathBuf,
    swapped: AtomicBool,
}

impl AuthorityOperationControl for PublicationLeafSwap {
    fn now_ms(&self) -> u64 {
        0
    }
    fn is_cancelled(&self) -> bool {
        false
    }
    fn report(&self, progress: AuthorityProgress) {
        if progress.stage == AuthorityProgressStage::CatalogResolved && !self.swapped.swap(true, Ordering::SeqCst) {
            std::fs::write(&self.path, b"substituted-after-verified-candidate").unwrap();
        }
    }
}

#[tokio::test]
async fn trusted_publication_post_verification_leaf_substitution_preserves_current() {
    for field in ["component", "descriptor", "browserActor", "pluginModule", "bundle"] {
        let a = prepared_fixture();
        let b = prepared_fixture();
        let data = a.root.join("publication-data");
        let command_a = stage_publication_fixture(&a, &data);
        let mut command_b = stage_publication_fixture(&b, &data);
        let provider_a = FixtureProviderSource::new(vec![a.binding()]);
        let provider_b = FixtureProviderSource::new(vec![b.binding()]);
        let control = TestControl::new();
        let context = control.context();
        let current = publication_fixture_receipt(TrustedCatalogPublisher::publish_current(&data, &serde_json::to_vec(&command_a).unwrap(), &provider_a, &context).await.unwrap());
        command_b["expectedCurrentSha256"] = current["currentSha256"].clone();
        let relative = if field == "bundle" { "trusted-catalog.json" } else { b.bundle["packages"][0][field]["path"].as_str().unwrap() };
        let path = data.join("trusted-catalog/generations").join(command_b["generationId"].as_str().unwrap()).join(relative);
        let swap = PublicationLeafSwap { path, swapped: AtomicBool::new(false) };
        let swap_context = OperationContext::new(u64::MAX, AuthorityLimits::maximum(), &swap);
        let pointer = data.join("trusted-catalog/current.json");
        let before = std::fs::read(&pointer).unwrap();
        assert!(TrustedCatalogPublisher::publish_current(&data, &serde_json::to_vec(&command_b).unwrap(), &provider_b, &swap_context).await.is_err(), "{field}");
        assert!(swap.swapped.load(Ordering::SeqCst));
        assert_eq!(std::fs::read(&pointer).unwrap(), before);
        assert!(document_codec(&b.schema).await.unwrap().is_none(), "failed candidate installed a codec");
        assert_eq!(TrustedCatalogLoader::load_current(&data, &provider_a, &context).await.unwrap().unwrap().generation_id(), current["generationId"].as_str().unwrap());
    }
}

#[tokio::test]
async fn trusted_publication_command_refuses_invalid_authority_before_initial_pointer() {
    let fixture = prepared_fixture();
    let data = fixture.root.join("publication-data");
    let command = stage_publication_fixture(&fixture, &data);
    let provider = FixtureProviderSource::new(vec![fixture.binding()]);
    let control = TestControl::new();
    for field in ["bundlePath", "publicationRevision", "requestId", "expectedCurrentSha256"] {
        let mut rejected = command.clone();
        match field {
            "bundlePath" => rejected[field] = "/untrusted".into(),
            "publicationRevision" => rejected[field] = "1".into(),
            "requestId" => rejected[field] = "".into(),
            _ => {
                rejected.as_object_mut().unwrap().remove(field);
            }
        }
        assert!(TrustedCatalogPublisher::publish_current(&data, &serde_json::to_vec(&rejected).unwrap(), &provider, &control.context()).await.is_err(), "{field}");
        assert!(!data.join("trusted-catalog/current.json").exists());
        assert!(provider.calls.lock().unwrap().is_empty());
    }
    control.cancelled.store(true, Ordering::SeqCst);
    assert!(TrustedCatalogPublisher::publish_current(&data, &serde_json::to_vec(&command).unwrap(), &provider, &control.context()).await.is_err());
    assert!(!data.join("trusted-catalog/current.json").exists());
}
