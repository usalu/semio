
use super::*;
use directory::os_directory::schema::space_artifact_creation::{SpaceArtifactCreateV1, SpaceArtifactCreationCatalogV1, SpaceArtifactCreationStatusV1};

struct GenesisFixtureCodec {
    identity: TrustedArtifactIdentity,
    pair: ArtifactPair,
    calls: std::sync::atomic::AtomicUsize,
}

impl TrustedArtifactCodec for GenesisFixtureCodec {
    fn identity(&self) -> &TrustedArtifactIdentity {
        &self.identity
    }
    async fn validate_pair(&self, pair: &ArtifactPair, _stage: ArtifactValidationStage, context: &OperationContext<'_>) -> Result<(), AuthorityError> {
        assert_eq!(pair, &self.pair);
        context.checkpoint()
    }
    async fn apply_operation(&self, _pair: ArtifactPair, _operation: &super::super::AcceptedArtifactOperation, _context: &OperationContext<'_>) -> Result<ArtifactPair, AuthorityError> {
        panic!("genesis must not apply an edit")
    }
}

impl TrustedArtifactGenesisCodec for GenesisFixtureCodec {
    async fn initial_pair(&self, document_id: &str, dialect: &ArtifactDialect, context: &OperationContext<'_>) -> Result<ArtifactPair, AuthorityError> {
        assert_eq!(document_id, "artifact-11223344556677889900aabbccddeeff");
        assert_eq!(dialect.artifact_kind, self.identity.artifact_kind);
        self.calls.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        context.checkpoint()?;
        Ok(self.pair.clone())
    }
}

impl TrustedArtifactCatalog for GenesisFixtureCodec {
    type Codec = Self;
    async fn resolve<'a>(&'a self, _required: &TrustedArtifactIdentity) -> Result<&'a Self::Codec, AuthorityError> {
        Ok(self)
    }
}

struct GenesisControl {
    cancelled: bool,
    now: u64,
    stages: std::sync::Mutex<Vec<AuthorityProgressStage>>,
}

impl super::super::AuthorityOperationControl for GenesisControl {
    fn now_ms(&self) -> u64 {
        self.now
    }
    fn is_cancelled(&self) -> bool {
        self.cancelled
    }
    fn report(&self, progress: AuthorityProgress) {
        self.stages.lock().unwrap().push(progress.stage);
    }
}

#[tokio::test]
async fn genesis_materialization_binds_exact_zero_history_and_independent_sha256() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/🌱️artifact-genesis-v1/🔣️.json")).unwrap();
    let descriptor: DocumentDescriptor = directory::os_pack::json::from_json_str(&fixture["expected"]["descriptor"].to_string()).unwrap();
    let expected: ArtifactCheckpoint = directory::os_pack::json::from_json_str(&fixture["expected"]["checkpoint"].to_string()).unwrap();
    let current: serde_json::Value = serde_json::from_str(include_str!("../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/🌱️artifact-genesis-v1/📤️current.json")).unwrap();
    for row in current["cases"].as_array().unwrap() {
        let encoded = row["command"].to_string();
        let parsed = directory::os_pack::json::from_json_str::<directory::os_directory::CheckpointPublicationCommandV1>(&encoded).ok();
        assert_eq!(parsed.as_ref().is_some_and(|command| command.validate()), row["accepted"].as_bool().unwrap(), "{}", row["id"]);
        if let Some(command) = parsed.filter(|command| command.validate()) {
            let canonical = directory::os_pack::json::to_json_string(&command);
            assert_eq!(directory::os_directory::CheckpointPublicationCommandV1::parse_canonical_json(&canonical), Some(command.clone()));
            let independent: directory::os_directory::CheckpointPublicationCurrentV1 = serde_json::from_value(row["command"]["expectedCurrent"].clone()).unwrap();
            assert_eq!(command.expected_current, independent);
        }
    }
    let open: serde_json::Value = serde_json::from_str(include_str!("../../../../../🧰️framework/🛍️products/💻️os/🧫️fixtures/📇️directory/🧭️document-open-plan-v1.json")).unwrap();
    let valid_plan: directory::os_directory::DocumentOpenPlanV1 = directory::os_pack::json::from_json_str(&open["validPlan"].to_string()).unwrap();
    let valid_lease = directory::os_directory::lease_fields_from_plan_v1(&valid_plan, 1, 1, None).unwrap();
    for row in open["checkpointPresenceCases"].as_array().unwrap() {
        let mut plan = open["validPlan"].clone();
        let mut lease: serde_json::Value = serde_json::from_str(&directory::os_pack::json::to_json_string(&valid_lease)).unwrap();
        for value in [&mut plan, &mut lease] {
            match row["kind"].as_str().unwrap() {
                "missing" => {
                    value.as_object_mut().unwrap().remove("checkpoint");
                }
                "null" => value["checkpoint"] = serde_json::Value::Null,
                "present" => {}
                _ => panic!("unknown checkpoint presence case"),
            }
        }
        assert_eq!(directory::os_pack::json::from_json_str::<directory::os_directory::DocumentOpenPlanV1>(&plan.to_string()).is_ok(), row["accepted"].as_bool().unwrap());
        assert_eq!(directory::os_pack::json::from_json_str::<directory::os_directory::DocumentExecutionTargetLeaseFieldsV1>(&lease.to_string()).is_ok(), row["accepted"].as_bool().unwrap());
    }
    println!("[DEBUG] Required committed checkpoint: neutral plan/lease presence cases=6; descriptor-only HTTP routes not executed");
    for row in fixture["frontiers"].as_array().unwrap() {
        let scope: DocumentScope = directory::os_pack::json::from_json_str(&row["scope"].to_string()).unwrap();
        let frontier: ArtifactFrontier = directory::os_pack::json::from_json_str(&row["frontier"].to_string()).unwrap();
        assert_eq!(frontier.is_genesis_for(&scope), row["genesis"].as_bool().unwrap(), "{}", row["id"]);
        assert_eq!(frontier.is_edited_for(&scope), row["edited"].as_bool().unwrap(), "{}", row["id"]);
        let mut plan: directory::os_directory::DocumentOpenPlanV1 = directory::os_pack::json::from_json_str(&open["validPlan"].to_string()).unwrap();
        plan.scope = scope;
        plan.checkpoint.baseline_frontier = frontier;
        let accepted = row["genesis"].as_bool().unwrap() || row["edited"].as_bool().unwrap();
        assert_eq!(plan.validate(open["nowMs"].as_u64().unwrap()).is_ok(), accepted, "{}: open", row["id"]);
        assert_eq!(directory::os_directory::lease_fields_from_plan_v1(&plan, 1, 1, None).is_ok(), accepted, "{}: lease", row["id"]);
    }
    for case in ["exact", "cancelled", "expired", "empty-pack", "empty-spr", "over-budget", "foreign-identity", "foreign-kind", "caller-document"] {
        let identity = TrustedArtifactIdentity::from_descriptor(&descriptor);
        let mut codec = GenesisFixtureCodec {
            identity: identity.clone(),
            pair: ArtifactPair { pack: serde_json::from_value(fixture["initialPair"]["pack"].clone()).unwrap(), spr: serde_json::from_value(fixture["initialPair"]["spr"].clone()).unwrap() },
            calls: std::sync::atomic::AtomicUsize::new(0),
        };
        let mut request = ArtifactGenesisRequest { scope: expected.scope.clone(), kind_id: identity.artifact_kind.clone() };
        let mut limits = super::super::AuthorityLimits::maximum();
        match case {
            "empty-pack" => codec.pair.pack.clear(),
            "empty-spr" => codec.pair.spr.clear(),
            "over-budget" => limits.max_pair_bytes = 1,
            "foreign-identity" => codec.identity.package_hash = "44".repeat(32),
            "foreign-kind" => request.kind_id = "foreign-kind".into(),
            "caller-document" => request.scope.document_id = "caller-document".into(),
            _ => {}
        }
        let control = GenesisControl { cancelled: case == "cancelled", now: if case == "expired" { 2000 } else { 1000 }, stages: std::sync::Mutex::new(Vec::new()) };
        let context = OperationContext::new(2000, limits, &control);
        let dialect = directory::os_pack::json::from_json_str(&fixture["dialect"].to_string()).unwrap();
        let result = materialize_selected_genesis(&codec, request, identity, dialect, &context).await;
        assert_eq!(result.is_ok(), case == "exact", "{case}");
        if let Ok(actual) = result {
            assert_eq!(actual.descriptor, descriptor);
            assert_eq!(actual.candidate.checkpoint, expected);
            assert_eq!(actual.candidate.pair, codec.pair);
            let encoded = super::super::checkpoint_id_encoding_v1(&actual.candidate.checkpoint).unwrap();
            assert_eq!(directory::os_directory::hex_lower(&encoded), fixture["expected"]["checkpointEncodingHex"]);
            assert!(encoded.starts_with(fixture["genesisDomain"].as_str().unwrap().as_bytes()));
            let mut parented = actual.candidate.checkpoint;
            parented.parent_checkpoint_id = Some(ArtifactHash([1; 32]));
            assert!(super::super::checkpoint_id_encoding_v1(&parented).is_err());
            assert_eq!(control.stages.lock().unwrap().last(), Some(&AuthorityProgressStage::Derived));
        } else {
            assert!(!control.stages.lock().unwrap().contains(&AuthorityProgressStage::Derived));
        }
        assert!(codec.calls.load(std::sync::atomic::Ordering::SeqCst) <= 1);
    }
    println!("[DEBUG] genesis: neutral frontiers=8 private materializer=9 independent Node SHA256=5 factory once=1 no domain edit=1; no durable publication");
}

#[test]
fn directory_document_index_is_ordered_idempotent_and_backend_descriptor_bound() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/📇️document-index-v1/🔣️.json")).unwrap();
    for row in fixture["cases"].as_array().unwrap() {
        let events: Vec<directory::os_directory::DirectoryEvent> = directory::os_pack::json::from_json_str(&row["events"].to_string()).unwrap();
        let client = events.iter().fold(directory::os_directory::DirectoryReadModel::default(), directory::os_directory::fold);
        assert_eq!(client.spaces["space-fixture"].indexed_documents.len(), row["clientRows"].as_u64().unwrap() as usize, "{}", row["id"]);
        let mut backend = crate::directory::MemoryArtifactProjection::default();
        let result = backend.fold_atomically(&events);
        assert_eq!(result.is_ok(), row["backendAccepted"].as_bool().unwrap(), "{}: {result:?}", row["id"]);
        if result.is_err() {
            assert_eq!(backend, crate::directory::MemoryArtifactProjection::default());
        }
    }
}

#[test]
fn creation_contract_matches_neutral_intents_and_ready_only_coordinates() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/🌱️space-artifact-creation-v1/🔣️.json")).unwrap();
    for row in fixture["requests"].as_array().unwrap() {
        let independent = serde_json::from_value::<SpaceArtifactCreateV1>(row["value"].clone()).ok();
        let accepted = row["accepted"].as_bool().unwrap();
        assert_eq!(independent.as_ref().is_some_and(SpaceArtifactCreateV1::validate), accepted, "{}", row["id"]);
        let source = independent.as_ref().map(serde_json::to_string).transpose().unwrap().unwrap_or_else(|| row["value"].to_string());
        let actual = SpaceArtifactCreateV1::parse_canonical_json(&source);
        assert_eq!(actual.is_some(), accepted, "{}", row["id"]);
        if let Some(actual) = actual {
            assert_eq!(Some(&actual), independent.as_ref());
            assert_eq!(directory::os_pack::json::to_json_string(&actual), source);
            assert!(SpaceArtifactCreateV1::parse_canonical_json(&format!("{source}\n")).is_none());
            assert!(SpaceArtifactCreateV1::parse_canonical_json(&source.replacen("{", "{\"schema\":\"duplicate\",", 1)).is_none());
        }
    }
    for row in fixture["statuses"].as_array().unwrap() {
        let independent = serde_json::from_value::<SpaceArtifactCreationStatusV1>(row["value"].clone()).ok();
        let accepted = row["accepted"].as_bool().unwrap();
        assert_eq!(independent.as_ref().is_some_and(SpaceArtifactCreationStatusV1::validate), accepted, "{}", row["id"]);
        let source = independent.as_ref().map(serde_json::to_string).transpose().unwrap().unwrap_or_else(|| row["value"].to_string());
        let actual = SpaceArtifactCreationStatusV1::parse_canonical_json(&source);
        assert_eq!(actual.is_some(), accepted, "{}", row["id"]);
        if let Some(actual) = actual {
            assert_eq!(Some(&actual), independent.as_ref());
        }
    }
    for row in fixture["catalogs"].as_array().unwrap() {
        let independent = serde_json::from_value::<SpaceArtifactCreationCatalogV1>(row["value"].clone()).ok();
        let accepted = row["accepted"].as_bool().unwrap();
        assert_eq!(independent.as_ref().is_some_and(SpaceArtifactCreationCatalogV1::validate), accepted, "{}", row["id"]);
        let source = independent.as_ref().map(serde_json::to_string).transpose().unwrap().unwrap_or_else(|| row["value"].to_string());
        let actual = SpaceArtifactCreationCatalogV1::parse_canonical_json(&source);
        assert_eq!(actual.is_some(), accepted, "{}", row["id"]);
        if let Some(actual) = actual {
            assert_eq!(Some(&actual), independent.as_ref());
        }
    }
    for row in fixture["rawJson"].as_array().unwrap() {
        let source = row["source"].as_str().unwrap();
        let accepted = row["accepted"].as_bool().unwrap();
        let (own, independent) = if row["type"] == "request" {
            (SpaceArtifactCreateV1::parse_canonical_json(source).is_some(), serde_json::from_str::<SpaceArtifactCreateV1>(source).ok().is_some_and(|value| value.validate() && serde_json::to_string(&value).unwrap() == source))
        } else if row["type"] == "catalog" {
            (SpaceArtifactCreationCatalogV1::parse_canonical_json(source).is_some(), serde_json::from_str::<SpaceArtifactCreationCatalogV1>(source).ok().is_some_and(|value| value.validate() && serde_json::to_string(&value).unwrap() == source))
        } else {
            (SpaceArtifactCreationStatusV1::parse_canonical_json(source).is_some(), serde_json::from_str::<SpaceArtifactCreationStatusV1>(source).ok().is_some_and(|value| value.validate() && serde_json::to_string(&value).unwrap() == source))
        };
        assert_eq!((own, independent), (accepted, accepted), "{}", row["id"]);
    }
}
