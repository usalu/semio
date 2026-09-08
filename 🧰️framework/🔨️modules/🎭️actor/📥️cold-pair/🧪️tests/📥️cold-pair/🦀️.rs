use crate::cold_pair::{ColdDocumentPairApplied, ColdDocumentPairCursor, ColdDocumentPairFrontier, ColdPairIngressStatus};
use crate::instance_lifetime::ActorInstanceLifetime;

fn lifetime() -> ActorInstanceLifetime {
    ActorInstanceLifetime { activation_generation: 7, instance_id: 3, guest_lifetime: 11 }
}

fn cursor() -> ColdDocumentPairCursor {
    ColdDocumentPairCursor { lifetime: lifetime(), transfer_generation: 13, page_index: 1, page_count: 2 }
}

fn frontier() -> ColdDocumentPairFrontier {
    ColdDocumentPairFrontier { document_id: "shared-map".into(), head_edit_ordinal: 17, head_edit_id: "edit-17".into(), last_commit_seq: 15, chain_sha256: [34; 32] }
}

#[semio_framework_async_macros::async_test]
async fn cold_pair_ingress_status_pack_round_trips_every_exact_variant() {
    let rows = [
        ColdPairIngressStatus::Idle,
        ColdPairIngressStatus::PageAccepted(cursor()),
        ColdPairIngressStatus::Backpressure(cursor()),
        ColdPairIngressStatus::Loading(cursor()),
        ColdPairIngressStatus::Applied(ColdDocumentPairApplied { lifetime: lifetime(), transfer_generation: 13, baseline_frontier: frontier(), aggregate_sha256: [51; 32] }),
        ColdPairIngressStatus::Fault { cursor: cursor(), fault: b"cold-pair.hash".to_vec() },
    ];
    for (tag, row) in rows.into_iter().enumerate() {
        let mut bytes = Vec::new();
        row.pack_encode(&mut bytes).await.expect("valid status encodes");
        assert_eq!(bytes[0], tag as u8);
        let mut offset = 0;
        let decoded = ColdPairIngressStatus::pack_decode(&bytes, &mut offset).await.expect("exact status decodes");
        assert_eq!(offset, bytes.len());
        assert_eq!(decoded, row);
    }
}

#[test]
fn cold_pair_ingress_neutral_fixture_has_exact_semantic_receipts_and_hostiles() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧪️fixture/🔣️.json")).expect("fixture json");
    let rows = fixture["statusRows"].as_array().expect("status rows");
    assert_eq!(rows.len(), 6);
    assert!(rows.iter().all(|row| row["kind"].as_str().is_some()));
    let hostile = fixture["hostileRows"].as_array().expect("hostile rows");
    assert_eq!(hostile.len(), 5);
    assert!(hostile.iter().all(|row| row["kind"].as_str().is_some()));
    assert!(ColdPairIngressStatus::PageAccepted(ColdDocumentPairCursor { lifetime: ActorInstanceLifetime { activation_generation: 0, ..lifetime() }, ..cursor() }).validate().is_err());
    assert!(ColdPairIngressStatus::Loading(ColdDocumentPairCursor { transfer_generation: 0, ..cursor() }).validate().is_err());
    assert!(ColdPairIngressStatus::Backpressure(ColdDocumentPairCursor { page_index: 2, ..cursor() }).validate().is_err());
    assert!(ColdPairIngressStatus::Applied(ColdDocumentPairApplied {
        lifetime: lifetime(),
        transfer_generation: 13,
        baseline_frontier: ColdDocumentPairFrontier { head_edit_ordinal: 14, last_commit_seq: 15, ..frontier() },
        aggregate_sha256: [51; 32]
    })
    .validate()
    .is_err());
    assert!(ColdPairIngressStatus::Fault { cursor: cursor(), fault: Vec::new() }.validate().is_err());
}

#[semio_framework_async_macros::async_test]
async fn cold_pair_ingress_decode_refuses_noncanonical_authority_before_publication() {
    let mut invalid = ColdPairIngressStatus::PageAccepted(ColdDocumentPairCursor { page_index: 2, ..cursor() });
    let mut bytes = Vec::new();
    assert!(invalid.pack_encode(&mut bytes).await.is_err());
    assert!(bytes.is_empty());
    invalid = ColdPairIngressStatus::Fault { cursor: cursor(), fault: vec![0; 4097] };
    assert!(invalid.pack_encode(&mut bytes).await.is_err());
    assert!(bytes.is_empty());

    let valid = ColdPairIngressStatus::PageAccepted(cursor());
    valid.pack_encode(&mut bytes).await.expect("valid cursor");
    bytes[0] = 6;
    let mut offset = 0;
    assert!(ColdPairIngressStatus::pack_decode(&bytes, &mut offset).await.is_err());
}
