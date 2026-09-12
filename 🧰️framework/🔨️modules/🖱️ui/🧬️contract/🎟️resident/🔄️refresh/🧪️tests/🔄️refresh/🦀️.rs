use super::*;

//#region 🔄️RefreshCycle
fn fixture() -> serde_json::Value {
    serde_json::from_str(include_str!("../../🧫️fixtures/🔣️.json")).unwrap()
}

fn surfaces(data: &serde_json::Value) -> Vec<(String, usize)> {
    data["surfaces"].as_array().unwrap().iter().map(|entry| (entry["id"].as_str().unwrap().to_string(), entry["nodes"].as_u64().unwrap() as usize)).collect()
}

/// 📐️ The exact reservation one published document asks the aggregate for — the arithmetic
/// `🌉️ProgramBridge/🎯️targets/🧊️wgpu`'s `assemble_browser_document` runs before it opens a root.
fn surface_limits(nodes: usize) -> UiResidentLimits {
    let items = nodes.saturating_add(2).min(UI_RESIDENT_SURFACE_ITEMS);
    let bytes = nodes.saturating_add(2).saturating_mul(size_of::<UiNodeRecord>()).saturating_add(UiDocumentAssembly::required_open_bytes()).min(UI_RESIDENT_SURFACE_BYTES);
    UiResidentLimits { items, bytes }
}

fn open_surface(surface: &str, generation: u64, nodes: usize) -> Result<UiDocumentLease, UiResidentFault> {
    let mut permit = None;
    UiResidentPermit::try_reserve(surface_limits(nodes), &mut permit, 32768)?;
    assert!(permit.is_some(), "a granted reservation yields a permit");
    let mut assembly = UiDocumentAssembly::default();
    let mut surface_id = Some(SurfaceId::try_from(surface).unwrap());
    let identity = UiDocumentAssemblyIdentity { generation, revision: UiRevision(1), root: Some(UiNodeId(1)), layout_epoch: 0 };
    for _ in 0..64 {
        assembly.open_with_permit(&mut permit, &mut surface_id, identity, 1, 32768).unwrap();
        if permit.is_none() && surface_id.is_none() {
            break;
        }
    }
    assert!(permit.is_none() && surface_id.is_none(), "the root opened within its opportunity budget");
    for index in 0..nodes {
        let mut record = Some(tests::leaf_record(index as u64 + 1, "node"));
        for _ in 0..4096 {
            assembly.place_one(&mut record, 1, 32768).unwrap();
            if record.is_none() {
                break;
            }
        }
        assert!(record.is_none(), "one node was placed within its opportunity budget");
    }
    let mut lease = None;
    for _ in 0..64 {
        if assembly.finish_into(&mut lease, UiRevision(1), 1, 32768).unwrap().complete {
            break;
        }
    }
    Ok(lease.expect("a finished assembly publishes its lease"))
}

/// ♻️ Drives ONE lease to terminal the way the wgpu shell's retirement registry now does — a PAGE
/// grant, never one item per step.
fn retire(lease: &mut UiDocumentLease, items: usize, bytes: usize) -> usize {
    for step in 0..1 << 20 {
        if lease.close_step_with_grant(items, bytes).unwrap().complete && lease.terminal_is_empty() {
            return step + 1;
        }
    }
    panic!("a published document did not retire within its bounded budget");
}

fn drain_pages() {
    for _ in 0..UI_DOCUMENT_LEASE_SLOTS * 4 {
        if close_ui_document_page_with_grant(1024, 32768).unwrap().complete {
            break;
        }
    }
}

/// ♻️ One refresh replaces one resident root at a time, so the aggregate never carries two full sets.
#[test]
fn retained_refresh_never_needs_a_second_full_resident_set() {
    let data = fixture();
    assert_eq!(data["aggregateBytes"].as_u64().unwrap() as usize, UI_RESIDENT_AGGREGATE_BYTES);
    assert_eq!(data["surfaceBytes"].as_u64().unwrap() as usize, UI_RESIDENT_SURFACE_BYTES);
    assert_eq!(data["surfaceItems"].as_u64().unwrap() as usize, UI_RESIDENT_SURFACE_ITEMS);
    assert_eq!(data["nodeRecordBytes"].as_u64().unwrap() as usize, size_of::<UiNodeRecord>());
    assert_eq!(data["openBytes"].as_u64().unwrap() as usize, UiDocumentAssembly::required_open_bytes());
    let items = data["retirementGrantItems"].as_u64().unwrap() as usize;
    let bytes = data["retirementGrantBytes"].as_u64().unwrap() as usize;
    let surfaces = surfaces(&data);
    let before = UiResidentPermit::snapshot().unwrap();
    let mut live: Vec<Option<UiDocumentLease>> = surfaces.iter().map(|_| None).collect();
    let mut peak_roots = 0usize;
    let mut peak_bytes = 0usize;
    let mut faults = 0usize;
    let mut steps = 0usize;
    for refresh in 0..data["refreshes"].as_u64().unwrap() {
        for (index, (surface, nodes)) in surfaces.iter().enumerate() {
            // ♻️ This surface's PREVIOUS document is driven to terminal before its replacement asks
            // the aggregate for credit — the whole law.
            if let Some(mut previous) = live[index].take() {
                steps += retire(&mut previous, items, bytes);
                drain_pages();
            }
            match open_surface(surface, refresh + 1, *nodes) {
                Ok(lease) => live[index] = Some(lease),
                Err(fault) => {
                    assert_eq!(fault, UiResidentFault::Capacity);
                    faults += 1;
                }
            }
            let snapshot = UiResidentPermit::snapshot().unwrap();
            peak_roots = peak_roots.max(snapshot.used_slots - before.used_slots);
            peak_bytes = peak_bytes.max(snapshot.bytes - before.bytes);
        }
    }
    for lease in live.iter_mut().flatten() {
        retire(lease, items, bytes);
    }
    drain_pages();
    eprintln!(
        "[DEBUG] resident-refresh surfaces={} refreshes={} peak-roots={peak_roots} peak-bytes={peak_bytes} faults={faults} retire-steps={steps} node-record-bytes={} open-bytes={} fixed-backing={}",
        surfaces.len(),
        data["refreshes"],
        size_of::<UiNodeRecord>(),
        UiDocumentAssembly::required_open_bytes(),
        UiResidentPermit::contract_backing_bytes()
    );
    assert_eq!(faults, data["permitFaults"].as_u64().unwrap() as usize);
    assert_eq!(peak_roots, data["peakResidentRoots"].as_u64().unwrap() as usize);
    assert_eq!(peak_roots, surfaces.len());
    assert_eq!(peak_bytes, data["peakResidentBytes"].as_u64().unwrap() as usize);
    assert_eq!(UiResidentPermit::snapshot().unwrap(), before);
}

/// 🩸️ The order `refresh_ui` used to run — every replacement opens while EVERY previous document is
/// still resident — needs a second FULL set: twice the roots and twice the bytes.
#[test]
fn retained_refresh_double_buffered_order_needs_a_second_full_set() {
    let data = fixture();
    let surfaces = surfaces(&data);
    let before = UiResidentPermit::snapshot().unwrap();
    let mut first: Vec<UiDocumentLease> = surfaces.iter().map(|(surface, nodes)| open_surface(surface, 1, *nodes).expect("the first full set is admitted")).collect();
    let single = UiResidentPermit::snapshot().unwrap();
    let mut second: Vec<UiDocumentLease> = surfaces.iter().map(|(surface, nodes)| open_surface(surface, 2, *nodes).expect("the second full set is admitted against an otherwise empty ledger")).collect();
    let doubled = UiResidentPermit::snapshot().unwrap();
    for lease in first.iter_mut().chain(second.iter_mut()) {
        retire(lease, 1024, 32768);
    }
    drain_pages();
    eprintln!(
        "[DEBUG] resident-refresh-double-buffered single-roots={} single-bytes={} doubled-roots={} doubled-bytes={}",
        single.used_slots - before.used_slots,
        single.bytes - before.bytes,
        doubled.used_slots - before.used_slots,
        doubled.bytes - before.bytes
    );
    assert_eq!(doubled.used_slots - before.used_slots, data["doubleBufferedPeakRoots"].as_u64().unwrap() as usize);
    assert_eq!(doubled.bytes - before.bytes, data["doubleBufferedPeakBytes"].as_u64().unwrap() as usize);
    assert_eq!(doubled.used_slots - before.used_slots, 2 * (single.used_slots - before.used_slots));
    assert_eq!(doubled.bytes - before.bytes, 2 * (single.bytes - before.bytes));
    assert_eq!(UiResidentPermit::snapshot().unwrap(), before);
}

/// 🎟️ Why a retained caller right-sizes: the aggregate admits only THREE ceiling-sized surfaces.
#[test]
fn retained_refresh_aggregate_admits_only_three_ceiling_sized_surfaces() {
    let data = fixture();
    let before = UiResidentPermit::snapshot().unwrap();
    let mut admitted = Vec::new();
    let mut refusal = None;
    for _ in 0..data["ceilingSizedRoots"].as_u64().unwrap() + 1 {
        let mut permit = None;
        match UiResidentPermit::try_reserve(UiResidentLimits { items: UI_RESIDENT_SURFACE_ITEMS, bytes: UI_RESIDENT_SURFACE_BYTES }, &mut permit, 32768) {
            Ok(true) => admitted.push(permit.unwrap()),
            Ok(false) => panic!("a reservation with an admitted grant is never silently skipped"),
            Err(fault) => refusal = Some(fault),
        }
    }
    let count = admitted.len();
    for permit in &mut admitted {
        assert!(permit.close_step(1).unwrap().complete);
    }
    eprintln!("[DEBUG] resident-refresh-ceiling admitted={count} refusal={refusal:?} surface-bytes={UI_RESIDENT_SURFACE_BYTES} aggregate={UI_RESIDENT_AGGREGATE_BYTES} fixed-backing={}", UiResidentPermit::contract_backing_bytes());
    assert_eq!(count, data["ceilingSizedRoots"].as_u64().unwrap() as usize);
    assert_eq!(refusal, Some(UiResidentFault::Capacity));
    assert_eq!(UiResidentPermit::snapshot().unwrap(), before);
}
//#endregion 🔄️RefreshCycle
