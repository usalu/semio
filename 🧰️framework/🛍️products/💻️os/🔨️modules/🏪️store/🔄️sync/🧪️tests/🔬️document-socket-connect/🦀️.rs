//! 🔬️ Language-agnostic document-socket connect fixture — Rust runner. The same
//! `🏪️store/🧫️fixtures/document-socket-connect-v1/🔣️.json` pins the authority admission table, the
//! subprotocol offer and the kind-identity resolution both document actors (native and browser) dial by.

use super::{commands_frames_within, document_pack_schema_hash, document_socket_authority_admits, document_socket_hello, document_socket_protocols, replica_hlc_seed, DocumentSocketBinding};
use crate::os_directory::client::DocumentSocketAuthorityV1;

const FIXTURE: &str = include_str!("../../../🧫️fixtures/document-socket-connect-v1/🔣️.json");

fn fixture() -> serde_json::Value {
    serde_json::from_str(FIXTURE).expect("document-socket-connect fixture json")
}

fn text<'a>(value: &'a serde_json::Value, overrides: &'a serde_json::Value, field: &str) -> Option<&'a str> {
    match overrides.get(field) {
        Some(serde_json::Value::Null) => None,
        Some(value) => value.as_str(),
        None => value.get(field).and_then(serde_json::Value::as_str),
    }
}

fn digest(value: &str) -> [u8; 32] {
    crate::os_directory::client::decode_lower_hex_32(value).expect("fixture digest")
}

fn fixture_lease(schema: &str, pack_schema_hash: &str) -> crate::os_directory::DocumentExecutionTargetLeaseFieldsV1 {
    crate::os_directory::DocumentExecutionTargetLeaseFieldsV1 {
        schema: "semio.os.document-execution-target-lease/v1".into(),
        version: 1,
        scope: crate::os_directory::DocumentScope::new("space-a", "shared-document"),
        descriptor_digest_v1: "11".repeat(32),
        catalog: crate::os_directory::DocumentOpenCatalogV1 { generation_id: "22".repeat(32) },
        package: crate::os_directory::DocumentOpenPackageV1 {
            plugin_id: "fixture.plugin".into(),
            package_id: "fixture.package".into(),
            version: "1.0.0".into(),
            component_sha256: "33".repeat(32),
            component_blake3: "44".repeat(32),
            descriptor_byte_sha256: "55".repeat(32),
            execution_protocol: crate::os_directory::DocumentExecutionProtocolV1 { app_channel_version: crate::os_spr::CHANNEL_VERSION },
        },
        component: crate::os_directory::DocumentExecutionTargetComponentV1 { sha256: "33".repeat(32), blake3: "44".repeat(32), byte_length: 1024 },
        descriptor: crate::os_directory::DocumentExecutionTargetDescriptorV1 { sha256: "55".repeat(32), byte_length: 512 },
        browser_actor: crate::os_directory::schema::DocumentExecutionTargetBrowserActorV1::None,
        artifact: crate::os_directory::DocumentOpenArtifactV1 { kind: "fixture".into(), schema: schema.into(), pack_schema_hash: pack_schema_hash.into() },
        parent_dialect: crate::os_directory::DocumentOpenParentDialectV1 { artifact_kind: "fixture".into(), standard: "1".into(), subset: "*".into() },
        surface: crate::os_directory::DocumentOpenSurfaceV1 {
            surface_id: "fixture.editor".into(),
            app_id: "fixture.app".into(),
            window_kind_id: "fixture.window".into(),
            role: crate::os_directory::DocumentOpenSurfaceRoleV1::Editor,
            renderer_target: crate::os_directory::DocumentOpenRendererTargetV1::Wgpu,
        },
        grant: crate::os_directory::DocumentOpenGrantV1 { read: true, write: true, observe: true },
        checkpoint: crate::os_directory::DocumentOpenCheckpointV1 {
            checkpoint_id: "77".repeat(32),
            descriptor_digest_v1: "11".repeat(32),
            baseline_frontier: crate::os_directory::ArtifactFrontier { document_id: "shared-document".into(), head_edit_ordinal: 0, head_edit_id: String::new(), last_commit_seq: 0, chain_hash: crate::os_directory::ArtifactHash::new([0; 32]) },
            aggregate_sha256: "88".repeat(32),
        },
        revalidation: crate::os_directory::DocumentOpenRevalidationV1 { directory_revision: 7, membership_generation: 7, session_generation: Some(3), share_generation: None },
    }
}

fn authority(base: &serde_json::Value, overrides: &serde_json::Value) -> DocumentSocketAuthorityV1 {
    let field = |name: &str| text(base, overrides, name).expect("authority fixture field").to_string();
    let schema = field("schema");
    let lease = fixture_lease(&schema, &field("packSchemaHash"));
    DocumentSocketAuthorityV1 {
        admitted_lease: None,
        hub_origin: field("hubOrigin"),
        expires_at_unix_ms: overrides.get("expiresAtUnixMs").or_else(|| base.get("expiresAtUnixMs")).and_then(serde_json::Value::as_u64).expect("authority deadline"),
        scope: crate::os_directory::DocumentScope::new(field("spaceId"), field("documentId")),
        descriptor_digest_v1: lease.descriptor_digest_v1,
        catalog: lease.catalog,
        package: lease.package,
        artifact: lease.artifact,
        parent_dialect: lease.parent_dialect,
        pack_schema_hash: digest(&field("packSchemaHash")),
        surface: crate::os_directory::DocumentOpenSurfaceV1 { surface_id: field("surfaceId"), ..lease.surface },
        browser_actor: crate::os_directory::DocumentOpenBrowserActorV1::None,
        grant: lease.grant,
        checkpoint: lease.checkpoint,
        revalidation: lease.revalidation,
    }
}

/// 🧪️ Every row of the admission table: an authority is admitted only for the exact binding, until
/// its deadline, from the bound origin (a trailing slash is the same origin).
#[test]
fn the_admission_table_admits_only_the_exact_binding() {
    let fixture = fixture();
    let empty = serde_json::Value::Object(serde_json::Map::new());
    for case in fixture["cases"].as_array().expect("cases") {
        let name = case["name"].as_str().expect("case name");
        let binding_overrides = case.get("binding").unwrap_or(&empty);
        let base_binding = &fixture["binding"];
        let pack_schema_hash = digest(text(base_binding, binding_overrides, "packSchemaHash").expect("binding pack hash"));
        let binding = DocumentSocketBinding {
            hub_base_url: text(base_binding, binding_overrides, "hubBaseUrl").expect("binding origin"),
            space_id: text(base_binding, binding_overrides, "spaceId").expect("binding space"),
            document_id: text(base_binding, binding_overrides, "documentId").expect("binding document"),
            schema: text(base_binding, binding_overrides, "schema").expect("binding schema"),
            pack_schema_hash,
            surface: text(base_binding, binding_overrides, "surface"),
            lease: None,
        };
        let authority = authority(&fixture["authority"], case.get("authority").unwrap_or(&empty));
        let now_ms = case.get("nowMs").or_else(|| fixture.get("nowMs")).and_then(serde_json::Value::as_u64).expect("now");
        assert_eq!(document_socket_authority_admits(&authority, &binding, now_ms), case["admits"].as_bool().expect("admits"), "{name}");
    }
}

/// 🧪️ A lease-bound binding admits only an authority that was admitted for that exact lease.
#[test]
fn a_lease_bound_binding_requires_the_admitted_lease() {
    let fixture = fixture();
    let empty = serde_json::Value::Object(serde_json::Map::new());
    let base = &fixture["binding"];
    let lease = fixture_lease(base["schema"].as_str().expect("schema"), base["packSchemaHash"].as_str().expect("hash"));
    let binding = DocumentSocketBinding {
        hub_base_url: base["hubBaseUrl"].as_str().expect("origin"),
        space_id: base["spaceId"].as_str().expect("space"),
        document_id: base["documentId"].as_str().expect("document"),
        schema: base["schema"].as_str().expect("schema"),
        pack_schema_hash: digest(base["packSchemaHash"].as_str().expect("hash")),
        surface: None,
        lease: Some(&lease),
    };
    let unleased = authority(&fixture["authority"], &empty);
    assert!(!document_socket_authority_admits(&unleased, &binding, 1000), "an authority admitted without the lease cannot carry a lease-bound binding");
}

/// 🧪️ The subprotocol offer is the receipt protocol then the capability, in that order — the single
/// header value a browser sends is exactly their `", "` join.
#[test]
fn the_subprotocol_offer_is_the_receipt_protocol_then_the_capability() {
    let fixture = fixture();
    let protocols = &fixture["protocols"];
    let offer = document_socket_protocols(protocols["receiptProtocol"].as_str().expect("protocol"), protocols["capability"].as_str().expect("capability"));
    let expected: Vec<String> = protocols["offer"].as_array().expect("offer").iter().map(|value| value.as_str().expect("offer entry").to_string()).collect();
    assert_eq!(offer.to_vec(), expected);
    assert_eq!(offer.join(", "), protocols["headerValue"].as_str().expect("header"));
}

/// 🧪️ Kind identity with no linked codec comes only from a lease for the same schema.
#[semio_framework_async_macros::async_test]
async fn a_kind_without_a_codec_takes_its_pack_identity_only_from_a_matching_lease() {
    let fixture = fixture();
    for case in fixture["packIdentity"].as_array().expect("pack identity cases") {
        let name = case["name"].as_str().expect("name");
        let lease = case["lease"].as_object().map(|lease| fixture_lease(lease["schema"].as_str().expect("lease schema"), lease["packSchemaHash"].as_str().expect("lease hash")));
        let expected = case["packSchemaHash"].as_str().map(digest);
        assert_eq!(document_pack_schema_hash(case["schema"].as_str().expect("schema"), lease.as_ref()).await, expected, "{name}");
    }
}

/// 🧬️ A component codec whose `codec.pack-schema-hash` answers with the fixture's digest, or faults.
struct FixtureComponentCodec {
    schema: String,
    pack_schema_hash: Option<[u8; 32]>,
}

impl crate::os_store::ComponentDocumentCodec for FixtureComponentCodec {
    fn schema(&self) -> &str {
        &self.schema
    }

    fn pack_schema_hash(&self) -> crate::os_store::ComponentDocumentCodecFuture<'_, [u8; 32]> {
        Box::pin(async move { self.pack_schema_hash.ok_or_else(|| crate::os_store::VcsError::ValidationFailed("fixture component cannot answer".into())) })
    }

    fn print_mirror<'a>(&'a self, _pack: &'a [u8], _spr: &'a [u8]) -> crate::os_store::ComponentDocumentCodecFuture<'a, crate::os_store::ArtifactTextFiles> {
        Box::pin(async move { Err(crate::os_store::VcsError::Deserialize("fixture component prints no mirror".into())) })
    }

    fn genesis<'a>(&'a self, _document_id: &'a str) -> crate::os_store::ComponentDocumentCodecFuture<'a, crate::os_store::ComponentDocumentGenesis> {
        Box::pin(async move { Err(crate::os_store::VcsError::ValidationFailed("fixture component mints no genesis".into())) })
    }
}

/// 🧪️ A kind whose owning component a host mounted takes its identity from that component — asked the
/// way the hub's trusted catalog asks it — before any lease; a component that cannot answer is no
/// identity at all, and a component owning another kind leaves this one to its lease (ticket 26/09/23
/// slice WG8, B2). Every case owns a distinct schema: the component registry is process-wide.
#[semio_framework_async_macros::async_test]
async fn a_mounted_component_codec_is_the_kind_identity_before_any_lease() {
    let fixture = fixture();
    for case in fixture["componentIdentity"].as_array().expect("component identity cases") {
        let name = case["name"].as_str().expect("name");
        let component = &case["component"];
        crate::os_store::register_component_document_codec(std::sync::Arc::new(FixtureComponentCodec {
            schema: component["schema"].as_str().expect("component schema").to_string(),
            pack_schema_hash: component["packSchemaHash"].as_str().map(digest),
        }))
        .expect("component codec registry admits the fixture codec");
        let lease = case["lease"].as_object().map(|lease| fixture_lease(lease["schema"].as_str().expect("lease schema"), lease["packSchemaHash"].as_str().expect("lease hash")));
        let expected = case["packSchemaHash"].as_str().map(digest);
        assert_eq!(document_pack_schema_hash(case["schema"].as_str().expect("schema"), lease.as_ref()).await, expected, "{name}");
    }
}

/// 🧪️ The hello carries the binding's schema, pack identity and the resume point, wire and protocol v1.
#[test]
fn the_hello_names_the_binding_and_its_resume_point() {
    let frame = document_socket_hello("block.2d", [7; 32], Some("resume".into()), None);
    assert_eq!(frame, crate::os_spr::ClientFrame::SocketHelloV1 { wire_version: 1, protocol_version: 1, schema: "block.2d".into(), pack_schema_hash: [7; 32], resume_token: Some("resume".into()), frontier: None });
}

/// 🧪️ Two replicas never share a clock seed (H4): two draws of platform entropy differ.
#[test]
fn two_replicas_draw_distinct_clock_seeds() {
    assert_ne!(replica_hlc_seed().expect("entropy"), replica_hlc_seed().expect("entropy"));
}

fn sized_envelope(index: usize, payload_bytes: usize) -> crate::os_spr::MutationEnvelope {
    crate::os_spr::MutationEnvelope {
        mutation_id: crate::os_spr::MutationId(format!("mutation-{index}")),
        document_id: crate::os_spr::ArtifactId("doc-a".into()),
        actor: crate::os_spr::ActorId("hub.v1.actor".into()),
        dependencies: Vec::new(),
        diff: crate::os_spr::ArtifactDiff { schema: crate::os_spr::SchemaId("block.2d".into()), payload: vec![7; payload_bytes] },
        inverse: crate::os_spr::InverseMutation { schema: crate::os_spr::SchemaId("block.2d".into()), payload: Vec::new() },
        timestamp: crate::os_spr::HybridLogicalTimestamp::new(1, 1),
    }
}

/// 🧪️ A relay larger than the socket's frame ceiling leaves as consecutive batches that each fit, in
/// envelope order; the one envelope that cannot fit even alone is the plan's `oversized`, never a frame.
#[semio_framework_async_macros::async_test]
async fn a_relay_is_split_into_frames_within_the_socket_ceiling() {
    let envelopes: Vec<_> = (0..4).map(|index| sized_envelope(index, 1_000)).chain(std::iter::once(sized_envelope(4, 6_000))).collect();
    let single = commands_frames_within(usize::MAX, 0, vec![envelopes[0].clone()], vec![envelopes[0].clone()]).await.frames[0].bytes.len();
    let ceiling = single * 2 + 64;
    let plan = commands_frames_within(ceiling, 40, envelopes.clone(), envelopes.clone()).await;
    assert!(plan.frames.iter().all(|frame| frame.bytes.len() <= ceiling), "every frame fits the ceiling");
    assert_eq!(plan.frames.iter().map(|frame| frame.batch_id).collect::<Vec<_>>(), (40..40 + plan.frames.len() as u64).collect::<Vec<_>>(), "batch ids run consecutively");
    let relayed: Vec<_> = plan.frames.iter().flat_map(|frame| frame.local.iter().map(|envelope| envelope.mutation_id.0.clone())).collect();
    assert_eq!(relayed, ["mutation-0", "mutation-1", "mutation-2", "mutation-3"], "order is kept and nothing fitting is dropped");
    assert_eq!(plan.oversized.iter().map(|envelope| envelope.mutation_id.0.as_str()).collect::<Vec<_>>(), ["mutation-4"]);
    let unbounded = commands_frames_within(usize::MAX, 7, envelopes.clone(), envelopes).await;
    assert_eq!((unbounded.frames.len(), unbounded.frames[0].batch_id, unbounded.oversized.len()), (1, 7, 0), "an unbounded socket sends one batch");
}
