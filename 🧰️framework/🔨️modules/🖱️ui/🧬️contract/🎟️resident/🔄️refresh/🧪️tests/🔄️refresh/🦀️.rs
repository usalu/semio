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
    crate::ui_document_resident_limits(nodes)
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

/// 🎟️ Why a retained caller right-sizes: the aggregate admits only a HANDFUL of ceiling-sized surfaces
/// out of the sixty-four slots the ledger offers, because the per-surface ceiling is a maximum one
/// pathological surface may reach and never the price a real body pays.
///
/// 🧾️ ticket 26/09/09/PROCEDURAL-3D-END-TO-END: the aggregate is no longer `4 * UI_RESIDENT_SURFACE_BYTES`
/// but `UI_RESIDENT_SLOTS * UI_RESIDENT_DOCUMENT_BYTES`, so the handful is six rather than three — and the
/// gap between six and sixty-four is exactly the reason right-sizing is not optional.
#[test]
fn retained_refresh_aggregate_admits_only_a_handful_of_ceiling_sized_surfaces() {
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
    assert!(count < UI_RESIDENT_SLOTS, "the ceiling is a maximum, never a price: {count} ceiling-sized roots against {UI_RESIDENT_SLOTS} slots");
    assert_eq!(UI_RESIDENT_SLOTS * UI_RESIDENT_DOCUMENT_BYTES, UI_RESIDENT_AGGREGATE_BYTES, "the aggregate uses the declared per-slot record-byte baseline");
    assert_eq!(refusal, Some(UiResidentFault::Capacity));
    assert_eq!(UiResidentPermit::snapshot().unwrap(), before);
}

/// 🎟️ A COLD document — one that names no census before it opens — prices itself as an EMPTY document and
/// climbs one record at a time, so every one of the [`UI_RESIDENT_SLOTS`] slots the slot ledger offers is
/// reachable. Reserving the per-surface ceiling instead capped the item ledger at
/// `UI_RESIDENT_AGGREGATE_ITEMS / UI_RESIDENT_SURFACE_ITEMS` = 31 concurrent roots, and the hostile
/// fixtures that walk all sixty-four slots refused at the thirty-second with `ArenaFull`, leaked their
/// reservations on the panic and starved every later test sharing the process ledger
/// (ticket 26/09/17/WGPU-RENDERER-REACT-PARITY wave 2–6 integration).
#[test]
fn cold_document_roots_are_priced_from_their_census_and_reach_every_slot() {
    let before = UiResidentPermit::snapshot().unwrap();
    let empty = crate::ui_document_resident_limits(0);
    assert!(empty.items < UI_RESIDENT_SURFACE_ITEMS && empty.bytes < UI_RESIDENT_SURFACE_BYTES, "an empty document never costs the per-surface ceiling: {empty:?}");
    assert!(UI_RESIDENT_SLOTS * empty.items <= UI_RESIDENT_AGGREGATE_ITEMS, "every slot is reachable on the item ledger");
    let mut builders: Vec<crate::UiDocumentBuilder> = Vec::new();
    for generation in 1..=UI_DOCUMENT_LEASE_SLOTS as u64 {
        let surface = SurfaceId::try_from(format!("cold.census.{generation}").as_str()).unwrap();
        let mut builder = crate::UiDocumentBuilder::try_new(generation, surface, UiRevision(generation), Some(UiNodeId(1)), generation).expect("every slot admits a cold root");
        builder.try_push(tests::leaf_record(1, "node")).expect("the cold root climbs to its first record");
        builders.push(builder);
    }
    let held = UiResidentPermit::snapshot().unwrap();
    assert_eq!(held.used_slots - before.used_slots, UI_DOCUMENT_LEASE_SLOTS);
    let refused = crate::UiDocumentBuilder::try_new(99, SurfaceId::try_from("cold.census.max-plus-one").unwrap(), UiRevision(99), Some(UiNodeId(1)), 99).expect_err("max plus one is refused by the SLOT ledger, never by a ceiling price");
    assert_eq!(refused.0, crate::UiDocumentBuildError::ArenaFull);
    let one_record = crate::ui_document_resident_limits(1);
    assert_eq!(held.bytes - before.bytes, UI_DOCUMENT_LEASE_SLOTS * one_record.bytes);
    assert_eq!(held.items - before.items, UI_DOCUMENT_LEASE_SLOTS * one_record.items);
    eprintln!("[DEBUG] cold-document-census slots={UI_DOCUMENT_LEASE_SLOTS} empty={empty:?} one-record={one_record:?} ceiling-items={UI_RESIDENT_SURFACE_ITEMS}");
    for mut builder in builders {
        for _ in 0..1 << 16 {
            if builder.close_step() {
                break;
            }
        }
        assert!(builder.terminal_is_empty(), "a cold root retires within its bounded budget");
    }
    drain_pages();
    assert_eq!(UiResidentPermit::snapshot().unwrap(), before);
}

/// 🎟️ A populated 128-node document reserves its complete record and assembly backing, so the byte
/// ledger reaches its independent ceiling before the sixty-four-position slot ledger.
#[test]
fn populated_full_documents_admit_sixty_and_refuse_the_sixty_first_on_bytes() {
    let data = fixture();
    let nodes = data["documentNodes"].as_u64().unwrap() as usize;
    let full = surface_limits(nodes);
    let admitted_roots = data["fullDocumentAdmittedRoots"].as_u64().unwrap() as usize;
    let refused_root = data["fullDocumentRefusedRoot"].as_u64().unwrap() as usize;
    assert_eq!(full.bytes, data["fullDocumentBytes"].as_u64().unwrap() as usize);
    assert_eq!(UiResidentPermit::contract_backing_bytes(), data["measured"]["fixedBackingBytes"].as_u64().unwrap() as usize);
    let before = UiResidentPermit::snapshot().unwrap();
    let mut admitted = Vec::new();
    for root in 1..=admitted_roots {
        admitted.push(open_surface(&format!("full.census.{root}"), root as u64, nodes).expect("the populated full-document byte ledger admits this root"));
    }
    let held = UiResidentPermit::snapshot().unwrap();
    assert_eq!(held.used_slots - before.used_slots, admitted_roots);
    assert_eq!(held.bytes - before.bytes, admitted_roots * full.bytes);
    assert_eq!(refused_root, admitted_roots + 1);
    assert_eq!(
        open_surface(&format!("full.census.{refused_root}"), refused_root as u64, nodes).expect_err("the next populated document exceeds the independent byte ceiling"),
        UiResidentFault::Capacity
    );
    assert!(held.used_slots - before.used_slots < UI_RESIDENT_SLOTS, "the byte ledger refuses with slot positions still free");
    for lease in &mut admitted {
        retire(lease, 1024, 32768);
    }
    drain_pages();
    assert_eq!(UiResidentPermit::snapshot().unwrap(), before);
}
//#endregion 🔄️RefreshCycle
