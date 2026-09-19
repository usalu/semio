use super::*;

fn hex32(source: &str) -> [u8; 32] {
    assert_eq!(source.len(), 64);
    let mut output = [0; 32];
    for (index, byte) in output.iter_mut().enumerate() {
        *byte = u8::from_str_radix(&source[index * 2..index * 2 + 2], 16).unwrap();
    }
    output
}

fn patterned(length: usize, multiplier: usize, addend: usize) -> Vec<u8> {
    (0..length).map(|index| ((index * multiplier + addend) & 255) as u8).collect()
}

fn lifetime(guest_lifetime: u64) -> ActorInstanceLifetime {
    ActorInstanceLifetime { activation_generation: 41, instance_id: 7, guest_lifetime }
}

fn header(pack: &[u8], spr: &[u8], lifetime: ActorInstanceLifetime, transfer_generation: u64) -> ColdDocumentPairHeader {
    let mut aggregate = semio_framework_hash::Sha256::new();
    aggregate.update(pack);
    aggregate.update(spr);
    let total = pack.len() + spr.len();
    ColdDocumentPairHeader {
        lifetime,
        transfer_generation,
        descriptor_sha256: [0x11; 32],
        baseline_frontier: semio_framework::kernel::ColdDocumentPairFrontier { document_id: "shared-map".into(), head_edit_ordinal: 7, head_edit_id: "edit-7".into(), last_commit_seq: 5, chain_sha256: [0x22; 32] },
        pack_sha256: semio_framework_hash::Sha256::digest(pack),
        spr_sha256: semio_framework_hash::Sha256::digest(spr),
        aggregate_sha256: aggregate.finalize(),
        pack_length: pack.len() as u64,
        spr_length: spr.len() as u64,
        page_count: total.div_ceil(COLD_PAIR_PAGE_MAXIMUM_BYTES) as u32,
    }
}

fn page(header: &ColdDocumentPairHeader, pack: &[u8], spr: &[u8], index: u32) -> ColdDocumentPairPage {
    let offset = index as usize * COLD_PAIR_PAGE_MAXIMUM_BYTES;
    let length = header.page_length(index).unwrap();
    let mut bytes = Vec::with_capacity(length);
    if offset < pack.len() {
        let count = length.min(pack.len() - offset);
        bytes.extend_from_slice(&pack[offset..offset + count]);
    }
    if bytes.len() < length {
        let combined = offset + bytes.len();
        let spr_offset = combined - pack.len();
        bytes.extend_from_slice(&spr[spr_offset..spr_offset + length - bytes.len()]);
    }
    ColdDocumentPairPage { header: header.clone(), page_index: index, bytes }
}

fn close_all<const N: usize>(ingress: &mut ColdDocumentPairIngressRegistry<N>, lifetime: ActorInstanceLifetime) -> (usize, usize) {
    assert!(ingress.request_close(lifetime));
    let mut work = 0;
    let mut steps = 0;
    loop {
        let step = ingress.close_step(lifetime);
        assert!(step.wiped_bytes <= COLD_PAIR_PAGE_MAXIMUM_BYTES);
        work += step.wiped_bytes;
        steps += 1;
        if step.closed {
            return (work, steps);
        }
        assert!(steps <= COLD_PAIR_MAXIMUM_PAGES as usize);
    }
}

#[test]
fn cold_pair_ingress_streams_the_exact_four_mibibyte_pair_and_loads_once() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🔣️.json")).unwrap();
    let pack = patterned(fixture["exact"]["packLength"].as_u64().unwrap() as usize, 31, 7);
    let spr = patterned(fixture["exact"]["sprLength"].as_u64().unwrap() as usize, 17, 11);
    assert_eq!(semio_framework_hash::Sha256::digest(&pack), hex32(fixture["exact"]["packSha256"].as_str().unwrap()));
    assert_eq!(semio_framework_hash::Sha256::digest(&spr), hex32(fixture["exact"]["sprSha256"].as_str().unwrap()));
    let exact_header = header(&pack, &spr, lifetime(13), 51);
    assert_eq!(exact_header.aggregate_sha256, hex32(fixture["exact"]["aggregateSha256"].as_str().unwrap()));
    assert_eq!(exact_header.page_count, COLD_PAIR_MAXIMUM_PAGES);
    let mut ingress = ColdDocumentPairIngressRegistry::<16>::new();
    for index in 0..exact_header.page_count {
        let status = ingress.accept_page(&page(&exact_header, &pack, &spr, index), Some(exact_header.lifetime));
        if index + 1 == exact_header.page_count {
            assert!(matches!(status, ColdPairIngressStatus::Loading(cursor) if cursor == exact_header.cursor(index)));
        } else {
            assert!(matches!(status, ColdPairIngressStatus::PageAccepted(cursor) if cursor == exact_header.cursor(index)));
        }
        assert_eq!(ingress.retained_bytes(exact_header.lifetime), ((index as usize + 1) * COLD_PAIR_PAGE_MAXIMUM_BYTES).min(pack.len() + spr.len()));
    }
    assert!(ingress.begin_load(exact_header.lifetime, exact_header.transfer_generation, None).is_none());
    let load = ingress.begin_load(exact_header.lifetime, exact_header.transfer_generation, Some(exact_header.lifetime)).expect("verified pair load");
    assert_eq!(load.files().pack, pack);
    assert_eq!(load.files().spr, spr);
    assert_eq!(ingress.retained_bytes(exact_header.lifetime), pack.len() + spr.len());
    let status = ingress.finish_load(load, Ok(()), Some(exact_header.lifetime));
    assert!(
        matches!(status, ColdPairIngressStatus::Applied(receipt) if receipt.lifetime == exact_header.lifetime && receipt.transfer_generation == 51 && receipt.baseline_frontier == exact_header.baseline_frontier && receipt.aggregate_sha256 == exact_header.aggregate_sha256)
    );
    assert!(ingress.is_applied(exact_header.lifetime));
    assert!(matches!(ingress.accept_page(&page(&exact_header, &pack, &spr, 63), Some(exact_header.lifetime)), ColdPairIngressStatus::Backpressure(_)));
    let (wiped, steps) = close_all(&mut ingress, exact_header.lifetime);
    assert_eq!(wiped, pack.len() + spr.len());
    assert_eq!(steps, COLD_PAIR_MAXIMUM_PAGES as usize);
    assert!(!ingress.is_mounted(exact_header.lifetime));
}

#[test]
fn cold_pair_ingress_rechecks_live_and_rejects_hostile_pages_without_displacement() {
    let pack = patterned(COLD_PAIR_PAGE_MAXIMUM_BYTES, 3, 1);
    let spr = patterned(COLD_PAIR_PAGE_MAXIMUM_BYTES + 1, 5, 2);
    let exact_header = header(&pack, &spr, lifetime(13), 52);
    let mut ingress = ColdDocumentPairIngressRegistry::<1>::new();
    let mut malformed_first = page(&exact_header, &pack, &spr, 0);
    malformed_first.bytes.pop();
    assert!(matches!(ingress.accept_page(&malformed_first, Some(exact_header.lifetime)), ColdPairIngressStatus::Fault { .. }));
    assert!(!ingress.is_mounted(exact_header.lifetime));
    assert!(matches!(ingress.accept_page(&page(&exact_header, &pack, &spr, 0), Some(exact_header.lifetime)), ColdPairIngressStatus::PageAccepted(_)));
    assert!(matches!(ingress.accept_page(&page(&exact_header, &pack, &spr, 1), None), ColdPairIngressStatus::Fault { ref fault, .. } if fault == b"cold-pair.not-live"));
    let mut changed = page(&exact_header, &pack, &spr, 1);
    changed.header.descriptor_sha256[0] ^= 1;
    assert!(matches!(ingress.accept_page(&changed, Some(exact_header.lifetime)), ColdPairIngressStatus::Fault { .. }));
    let foreign_header = header(&pack, &spr, lifetime(14), 53);
    assert!(matches!(ingress.accept_page(&page(&foreign_header, &pack, &spr, 0), Some(foreign_header.lifetime)), ColdPairIngressStatus::Fault { ref fault, .. } if fault == b"cold-pair.slot-collision"));
    let mut short = page(&exact_header, &pack, &spr, 1);
    short.bytes.pop();
    assert!(matches!(ingress.accept_page(&short, Some(exact_header.lifetime)), ColdPairIngressStatus::Fault { .. }));
    assert_eq!(ingress.retained_bytes(exact_header.lifetime), COLD_PAIR_PAGE_MAXIMUM_BYTES);
    assert!(matches!(ingress.accept_page(&page(&exact_header, &pack, &spr, 1), Some(exact_header.lifetime)), ColdPairIngressStatus::PageAccepted(_)));
    let mut terminal = page(&exact_header, &pack, &spr, 2);
    terminal.bytes[0] ^= 1;
    assert!(matches!(ingress.accept_page(&terminal, Some(exact_header.lifetime)), ColdPairIngressStatus::Fault { ref fault, .. } if fault == b"cold-pair.hash"));
    assert!(ingress.begin_load(exact_header.lifetime, exact_header.transfer_generation, Some(exact_header.lifetime)).is_none());
    assert_eq!(close_all(&mut ingress, exact_header.lifetime).0, pack.len() + spr.len());
}

#[test]
fn cold_pair_ingress_keeps_the_structural_owner_across_load_cancel_and_bounded_close() {
    let pack = patterned(COLD_PAIR_PAGE_MAXIMUM_BYTES, 7, 3);
    let spr = vec![9];
    let exact_header = header(&pack, &spr, lifetime(13), 54);
    let mut ingress = ColdDocumentPairIngressRegistry::<1>::new();
    for index in 0..exact_header.page_count {
        ingress.accept_page(&page(&exact_header, &pack, &spr, index), Some(exact_header.lifetime));
    }
    let load = ingress.begin_load(exact_header.lifetime, 54, Some(exact_header.lifetime)).unwrap();
    assert_eq!(load.lifetime(), exact_header.lifetime);
    assert_eq!(ingress.retained_bytes(exact_header.lifetime), pack.len() + spr.len());
    assert!(ingress.request_close(exact_header.lifetime));
    assert_eq!(ingress.close_step(exact_header.lifetime), ColdDocumentPairCloseStep { wiped_bytes: 0, closed: false });
    assert!(matches!(ingress.finish_load(load, Ok(()), Some(exact_header.lifetime)), ColdPairIngressStatus::Fault { ref fault, .. } if fault == b"cold-pair.stale-load"));
    let (wiped, _) = close_all(&mut ingress, exact_header.lifetime);
    assert_eq!(wiped, pack.len() + spr.len());

    let reopened_header = header(&pack, &spr, lifetime(14), 55);
    for index in 0..reopened_header.page_count {
        ingress.accept_page(&page(&reopened_header, &pack, &spr, index), Some(reopened_header.lifetime));
    }
    let abandoned = ingress.begin_load(reopened_header.lifetime, 55, Some(reopened_header.lifetime)).unwrap();
    drop(abandoned);
    assert!(ingress.begin_load(reopened_header.lifetime, 55, Some(reopened_header.lifetime)).is_none());
    assert!(matches!(ingress.accept_page(&page(&exact_header, &pack, &spr, 1), Some(exact_header.lifetime)), ColdPairIngressStatus::Fault { .. }));
    assert_eq!(close_all(&mut ingress, reopened_header.lifetime).0, pack.len() + spr.len());
}

#[test]
fn cold_pair_ingress_final_live_fence_rejects_post_await_revocation() {
    let pack = patterned(COLD_PAIR_PAGE_MAXIMUM_BYTES, 7, 3);
    let spr = vec![9];
    let exact_header = header(&pack, &spr, lifetime(13), 56);
    let mut ingress = ColdDocumentPairIngressRegistry::<1>::new();
    for index in 0..exact_header.page_count {
        ingress.accept_page(&page(&exact_header, &pack, &spr, index), Some(exact_header.lifetime));
    }
    let load = ingress.begin_load(exact_header.lifetime, 56, Some(exact_header.lifetime)).unwrap();
    assert!(matches!(ingress.finish_load(load, Ok(()), None), ColdPairIngressStatus::Fault { ref fault, .. } if fault == b"cold-pair.not-live"));
    assert!(!ingress.is_applied(exact_header.lifetime));
    assert_eq!(close_all(&mut ingress, exact_header.lifetime).0, pack.len() + spr.len());
}

#[test]
fn cold_pair_ingress_charges_aggregate_reserved_capacity_until_final_close() {
    let pack = patterned(2 * 1024 * 1024, 3, 7);
    let spr = vec![9];
    let first_lifetime = ActorInstanceLifetime { activation_generation: 41, instance_id: 1, guest_lifetime: 13 };
    let second_lifetime = ActorInstanceLifetime { activation_generation: 41, instance_id: 2, guest_lifetime: 14 };
    let first = header(&pack, &spr, first_lifetime, 57);
    let second = header(&pack, &spr, second_lifetime, 58);
    let mut ingress = ColdDocumentPairIngressRegistry::<4>::new();
    assert!(matches!(ingress.accept_page(&page(&first, &pack, &spr, 0), Some(first_lifetime)), ColdPairIngressStatus::PageAccepted(_)));
    assert!(matches!(ingress.accept_page(&page(&second, &pack, &spr, 0), Some(second_lifetime)), ColdPairIngressStatus::Fault { ref fault, .. } if fault == b"cold-pair.capacity"));
    assert!(!ingress.is_mounted(second_lifetime));
    assert_eq!(close_all(&mut ingress, first_lifetime).0, COLD_PAIR_PAGE_MAXIMUM_BYTES);
    assert!(matches!(ingress.accept_page(&page(&second, &pack, &spr, 0), Some(second_lifetime)), ColdPairIngressStatus::PageAccepted(_)));
    assert_eq!(close_all(&mut ingress, second_lifetime).0, COLD_PAIR_PAGE_MAXIMUM_BYTES);
}

#[test]
fn cold_pair_ingress_is_an_exact_retained_native_close_participant() {
    let pack = patterned(COLD_PAIR_PAGE_MAXIMUM_BYTES, 11, 3);
    let spr = patterned(COLD_PAIR_PAGE_MAXIMUM_BYTES + 1, 13, 5);
    let close_lifetime = ActorInstanceLifetime { activation_generation: 1, instance_id: 7, guest_lifetime: 13 };
    let exact_header = header(&pack, &spr, close_lifetime, 59);
    let key = super::super::instance_lifetime::NativeCloseKey::fixture(exact_header.lifetime.instance_id, exact_header.lifetime.guest_lifetime);
    let foreign = super::super::instance_lifetime::NativeCloseKey::fixture(exact_header.lifetime.instance_id, exact_header.lifetime.guest_lifetime + 1);
    let mut ingress = ColdDocumentPairIngressRegistry::<1>::new();
    assert!(matches!(ingress.accept_page(&page(&exact_header, &pack, &spr, 0), Some(exact_header.lifetime)), ColdPairIngressStatus::PageAccepted(_)));
    ingress.preflight_close_instance(key).unwrap();
    ingress.reserve_close_instance(key).unwrap();
    assert!(ingress.reserve_close_instance(foreign).is_err());
    assert!(matches!(ingress.accept_page(&page(&exact_header, &pack, &spr, 1), Some(exact_header.lifetime)), ColdPairIngressStatus::Fault { ref fault, .. } if fault == b"cold-pair.not-live"));
    ingress.activate_close_instance(key).unwrap();
    let mut opportunities = 0;
    while !ingress.close_instance_complete(key).unwrap() {
        assert!(ingress.advance_close_one());
        opportunities += 1;
        assert!(opportunities <= exact_header.page_count);
    }
    assert!(!ingress.is_mounted(exact_header.lifetime));
    ingress.release_close_instance(key).unwrap();
}

#[test]
fn cold_pair_header_requires_an_active_checkpoint_frontier_and_exact_hashes() {
    let pack = vec![1];
    let spr = vec![2];
    let exact_header = header(&pack, &spr, lifetime(13), 60);
    assert_eq!(exact_header.validate(), Ok(()));
    let mut invalid = exact_header.clone();
    invalid.baseline_frontier.head_edit_id.clear();
    assert_eq!(invalid.validate(), Err("cold-pair.frontier"));
    invalid = exact_header.clone();
    invalid.baseline_frontier.last_commit_seq = invalid.baseline_frontier.head_edit_ordinal + 1;
    assert_eq!(invalid.validate(), Err("cold-pair.frontier"));
    invalid = exact_header;
    invalid.pack_sha256 = [0; 32];
    assert_eq!(invalid.validate(), Err("cold-pair.hash"));
}
