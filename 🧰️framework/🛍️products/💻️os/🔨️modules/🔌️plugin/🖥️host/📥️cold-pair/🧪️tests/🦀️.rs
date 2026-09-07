use super::*;
use semio_framework::kernel::{ActorInstanceLifetime, ColdDocumentPairHeader};

fn wit_cursor(value: ColdDocumentPairCursor) -> wit_reactor::ColdDocumentPairCursor {
    wit_reactor::ColdDocumentPairCursor { lifetime: kernel_lifetime_to_wit(value.lifetime), transfer_generation: value.transfer_generation, page_index: value.page_index, page_count: value.page_count }
}
fn wit_frontier(value: ColdDocumentPairFrontier) -> wit_reactor::ColdDocumentPairFrontier {
    wit_reactor::ColdDocumentPairFrontier { document_id: value.document_id, head_edit_ordinal: value.head_edit_ordinal, head_edit_id: value.head_edit_id, last_commit_seq: value.last_commit_seq, chain_sha256: value.chain_sha256.to_vec() }
}
fn fixture_lifetime(value: &serde_json::Value) -> ActorInstanceLifetime {
    ActorInstanceLifetime { activation_generation: value["activationGeneration"].as_u64().unwrap(), instance_id: value["instanceId"].as_u64().unwrap().try_into().unwrap(), guest_lifetime: value["guestLifetime"].as_u64().unwrap() }
}
fn fixture_cursor(value: &serde_json::Value) -> ColdDocumentPairCursor {
    ColdDocumentPairCursor {
        lifetime: fixture_lifetime(&value["lifetime"]),
        transfer_generation: value["transferGeneration"].as_u64().unwrap(),
        page_index: value["pageIndex"].as_u64().unwrap().try_into().unwrap(),
        page_count: value["pageCount"].as_u64().unwrap().try_into().unwrap(),
    }
}
fn fixture_status(row: &serde_json::Value) -> ColdPairIngressStatus {
    match row["kind"].as_str().unwrap() {
        "idle" => ColdPairIngressStatus::Idle,
        "pageAccepted" => ColdPairIngressStatus::PageAccepted(fixture_cursor(&row["cursor"])),
        "backpressure" => ColdPairIngressStatus::Backpressure(fixture_cursor(&row["cursor"])),
        "loading" => ColdPairIngressStatus::Loading(fixture_cursor(&row["cursor"])),
        "fault" => ColdPairIngressStatus::Fault { cursor: fixture_cursor(&row["cursor"]), fault: serde_json::from_value(row["fault"].clone()).unwrap() },
        "applied" => {
            let value = &row["receipt"];
            ColdPairIngressStatus::Applied(ColdDocumentPairApplied {
                lifetime: fixture_lifetime(&value["lifetime"]),
                transfer_generation: value["transferGeneration"].as_u64().unwrap(),
                baseline_frontier: serde_json::from_value(value["baselineFrontier"].clone()).unwrap(),
                aggregate_sha256: serde_json::from_value(value["aggregateSha256"].clone()).unwrap(),
            })
        }
        _ => unreachable!(),
    }
}
fn wit_status(value: ColdPairIngressStatus) -> wit_reactor::ColdPairIngressStatus {
    use wit_reactor::ColdPairIngressStatus as W;
    match value {
        ColdPairIngressStatus::Idle => W::Idle,
        ColdPairIngressStatus::PageAccepted(value) => W::PageAccepted(wit_cursor(value)),
        ColdPairIngressStatus::Backpressure(value) => W::Backpressure(wit_cursor(value)),
        ColdPairIngressStatus::Loading(value) => W::Loading(wit_cursor(value)),
        ColdPairIngressStatus::Fault { cursor, fault } => W::Fault(wit_reactor::ColdDocumentPairFault { cursor: wit_cursor(cursor), fault }),
        ColdPairIngressStatus::Applied(value) => W::Applied(wit_reactor::ColdDocumentPairApplied {
            lifetime: kernel_lifetime_to_wit(value.lifetime),
            transfer_generation: value.transfer_generation,
            baseline_frontier: wit_frontier(value.baseline_frontier),
            aggregate_sha256: value.aggregate_sha256.to_vec(),
        }),
    }
}

#[test]
fn neutral_cold_ingress_variants_preserve_authority_and_refuse_hostile_wit() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../../../../../🔨️modules/🎭️actor/📥️cold-pair/🧪️fixture/🔣️.json")).unwrap();
    for row in fixture["statusRows"].as_array().unwrap() {
        let expected = fixture_status(row);
        let decoded = wit_cold_ingress_to_kernel(wit_status(expected.clone())).unwrap();
        assert_eq!(decoded, expected);
    }
    for row in fixture["hostileRows"].as_array().unwrap() {
        assert!(wit_cold_ingress_to_kernel(wit_status(fixture_status(row))).is_err(), "hostile {row}");
    }
    let applied = fixture_status(&fixture["statusRows"][4]);
    for length in [0, 31, 33] {
        let wit_reactor::ColdPairIngressStatus::Applied(mut bad) = wit_status(applied.clone()) else { unreachable!() };
        bad.aggregate_sha256 = vec![1; length];
        assert!(wit_cold_ingress_to_kernel(wit_reactor::ColdPairIngressStatus::Applied(bad)).is_err());
    }
    eprintln!("[DEBUG] native cold status variants=6 hostile-authorities=5 invalid-hash-lengths=3");
}

#[semio_framework_async_macros::async_test]
async fn native_cold_pages_use_dedicated_bounded_input() {
    let page = ColdDocumentPairPage {
        header: ColdDocumentPairHeader {
            lifetime: ActorInstanceLifetime { activation_generation: 1, instance_id: 7, guest_lifetime: 3 },
            transfer_generation: 4,
            descriptor_sha256: [1; 32],
            baseline_frontier: ColdDocumentPairFrontier { document_id: "map".into(), head_edit_ordinal: 8, head_edit_id: "edit-8".into(), last_commit_seq: 6, chain_sha256: [2; 32] },
            pack_sha256: [3; 32],
            spr_sha256: [4; 32],
            aggregate_sha256: [5; 32],
            pack_length: 3,
            spr_length: 2,
            page_count: 1,
        },
        page_index: 0,
        bytes: vec![1, 2, 3, 4, 5],
    };
    let event = Event::ColdDocumentPairPage(page.clone());
    let (ordinary, command, cold) = kernel_turn_inputs_to_wit(&[event.clone()], 7).await.unwrap();
    assert!(ordinary.is_empty() && command.is_none());
    let lifted = cold.unwrap();
    assert_eq!(lifted.bytes, page.bytes);
    assert_eq!(lifted.header.transfer_generation, 4);
    assert_eq!(lifted.header.lifetime.instance_id, 7);
    assert!(kernel_turn_inputs_to_wit(&[event.clone(), event], 7).await.is_err());
    for size in [0, 4, 6, 65537] {
        let mut invalid = page.clone();
        invalid.bytes = vec![0; size];
        assert!(kernel_cold_page_to_wit(&invalid).is_err());
    }
    eprintln!("[DEBUG] native cold dedicated-pages=1 ordinary-events=0 duplicate-refusals=1 invalid-lengths=4");
}
