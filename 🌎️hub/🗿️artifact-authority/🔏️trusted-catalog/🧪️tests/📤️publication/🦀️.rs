/// 🧳️ Stages exact existing fixture bytes under a real private publication data root.
fn stage_publication_fixture(fixture: &FixtureDirectory, data: &Path) -> serde_json::Value {
    let generation = fixture.bundle["profiles"][0]["generationId"].as_str().unwrap();
    let root = data.join("trusted-catalog/generations").join(generation);
    std::fs::create_dir_all(&root).unwrap();
    for record in fixture.bundle["packages"].as_array().unwrap() {
        let mut paths = vec![record["component"]["path"].as_str().unwrap(), record["descriptor"]["path"].as_str().unwrap()];
        if let Some(path) = record["browserActor"]["path"].as_str() {
            paths.push(path);
        }
        for path in paths {
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
    println!("[DEBUG] publication revision: neutral cases=8 nonzero-u64=exact ABA=revision-sensitive");
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
    println!("[DEBUG] publication real loader: first=1 ABA=3 stale=refused concurrent-winners=1 final-revision=4 activation-only-on-load");
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
    for field in ["component", "descriptor", "browserActor", "bundle"] {
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
    println!("[DEBUG] publication final native fence: substituted-leaves=4 prior-current=exact reload=verified no-candidate-activation");
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
    println!("[DEBUG] publication command: rejected-authority=4 cancelled=1 selected-provider-calls=0 current=absent");
}
