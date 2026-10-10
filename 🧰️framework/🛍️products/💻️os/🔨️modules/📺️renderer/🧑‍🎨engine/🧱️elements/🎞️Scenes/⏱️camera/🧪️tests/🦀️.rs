use super::*;
use semio_framework_trace::HeapAllocationObservation;
fn fixture() -> serde_json::Value { serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap() }
fn grant(fixture: &serde_json::Value) -> RetainedCloneGrant {
    let root = &fixture["rootGrant"];
    RetainedCloneGrant { maximum_items: root["maximumItems"].as_u64().unwrap() as usize, maximum_copy_bytes: root["maximumCopyBytes"].as_u64().unwrap() as usize, maximum_capacity_bytes: root["maximumCapacityBytes"].as_u64().unwrap() as usize, maximum_release_bytes: root["maximumReleaseBytes"].as_u64().unwrap() as usize, maximum_depth: root["maximumDepth"].as_u64().unwrap() as usize }
}
fn text(row: &serde_json::Value) -> [&str; 6] { [row["host"].as_str().unwrap(), row["surface"].as_str().unwrap(), row["owner"]["host"].as_str().unwrap_or(""), row["owner"]["window"].as_str().unwrap_or(""), row["owner"]["surface"].as_str().unwrap_or(""), row["owner"]["key"]["explicit"].as_str().unwrap_or("")] }
fn close(mut cursor: DirectoryCursor<u64>, grant: RetainedCloneGrant) -> usize {
    let mut released = 0;
    for _ in 0..2048 {
        let (step, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| if cursor.has_retiring() { cursor.close_retiring(grant) } else if cursor.peek().is_some() { cursor.close_current(grant) } else { cursor.close_backing(grant) });
        let step = step.unwrap();
        assert!(step.progress().fits(grant));
        assert_eq!(heap, HeapAllocationObservation { requested_bytes: step.progress().retained_capacity_bytes, released_bytes: step.progress().released_bytes, largest_release_bytes: step.progress().released_bytes, overflowed: false });
        released += step.progress().released_bytes;
        if cursor.terminal_is_empty() { break; }
    }
    assert!(cursor.terminal_is_empty(), "original camera storage closes within the unchanged finite fixture bound");
    released
}

#[test]
fn camera_packed_originals_match_the_independent_corpus_and_exact_heap_receipts() {
    let fixture = fixture();
    let grant = grant(&fixture);
    for row in fixture["cases"].as_array().unwrap() {
        let mut owner = DirectoryOwner::default();
        owner.install(grant).unwrap();
        let fields = text(row);
        let (receipt, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| owner.schedule(fields, 7u64, row["deadlineMs"].as_f64().unwrap(), grant));
        let receipt = receipt.unwrap();
        let body_bytes = row["packed"]["bytes"].as_u64().unwrap() as usize;
        assert!(receipt.fits(grant));
        assert_eq!(heap, HeapAllocationObservation { requested_bytes: body_bytes, ..Default::default() });
        assert_eq!(receipt.retained_capacity_bytes, body_bytes);
        let record = owner.get(fields[0]).unwrap();
        let expected = fields.concat();
        assert_eq!(record.original_bytes(), expected.as_bytes());
        for (index, value) in fields.iter().enumerate() { assert_eq!(record.text(index), *value); }
        let original = record.original_body_ptr();
        let ((cursor, receipt), heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| owner.begin(grant).unwrap());
        assert_eq!(heap, HeapAllocationObservation { requested_bytes: receipt.retained_capacity_bytes, ..Default::default() });
        assert_eq!(cursor.peek().unwrap().1.original_body_ptr(), original);
        assert!(owner.terminal_is_empty());
        let backing = receipt.retained_capacity_bytes;
        assert_eq!(close(cursor, grant), body_bytes + backing);
        eprintln!("[DEBUG] camera {} exact-body={} original-directory={} terminal=true", row["id"], body_bytes, backing);
    }
}

#[test]
fn camera_undergrants_are_inert_on_original_body_and_directory_pointers() {
    let fixture = fixture();
    let grant = grant(&fixture);
    let fields = text(&fixture["cases"][1]);
    let mut owner = DirectoryOwner::<u64>::default();
    owner.install(grant).unwrap();
    for short in [RetainedCloneGrant { maximum_items: 0, ..grant }, RetainedCloneGrant { maximum_copy_bytes: 0, ..grant }, RetainedCloneGrant { maximum_capacity_bytes: 0, ..grant }, RetainedCloneGrant { maximum_depth: 0, ..grant }] {
        let (result, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| owner.schedule(fields, 1, 120.0, short));
        assert!(result.is_err());
        assert_eq!(heap, HeapAllocationObservation::default());
        assert_eq!(owner.len(), 0);
    }
    owner.schedule(fields, 1, 120.0, grant).unwrap();
    let original_body = owner.get(fields[0]).unwrap().original_body_ptr();
    for short in [RetainedCloneGrant { maximum_items: 0, ..grant }, RetainedCloneGrant { maximum_copy_bytes: 0, ..grant }, RetainedCloneGrant { maximum_capacity_bytes: 0, ..grant }, RetainedCloneGrant { maximum_depth: 0, ..grant }] {
        let (result, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| owner.begin(short));
        assert!(result.is_err());
        assert_eq!(heap, HeapAllocationObservation::default());
        assert_eq!(owner.get(fields[0]).unwrap().original_body_ptr(), original_body);
    }
    let (mut cursor, _) = owner.begin(grant).unwrap();
    let directory = cursor.original_directory_ptr();
    for short in [RetainedCloneGrant { maximum_items: 0, ..grant }, RetainedCloneGrant { maximum_copy_bytes: 0, ..grant }, RetainedCloneGrant { maximum_release_bytes: 0, ..grant }, RetainedCloneGrant { maximum_depth: 0, ..grant }] {
        let (step, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| cursor.close_current(short));
        assert_eq!(step.unwrap().progress(), Default::default());
        assert_eq!(heap, HeapAllocationObservation::default());
        assert_eq!(cursor.original_directory_ptr(), directory);
        assert_eq!(cursor.peek().unwrap().1.original_body_ptr(), original_body);
    }
    close(cursor, grant);
}

#[test]
fn camera_same_identity_updates_without_birth_and_future_restore_returns_the_original_body() {
    let fixture = fixture();
    let grant = grant(&fixture);
    let fields = text(&fixture["cases"][1]);
    let mut owner = DirectoryOwner::<u64>::default();
    owner.install(grant).unwrap();
    owner.schedule(fields, 1, 120.0, grant).unwrap();
    let pointer = owner.get(fields[0]).unwrap().original_body_ptr();
    let (receipt, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| owner.schedule(fields, 1, 240.0, grant));
    assert_eq!(heap, HeapAllocationObservation::default());
    assert_eq!(receipt.unwrap().copied_items, 1);
    assert_eq!(owner.get(fields[0]).unwrap().original_body_ptr(), pointer);
    let (mut cursor, _) = owner.begin(grant).unwrap();
    let (restored, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| owner.restore(&mut cursor, grant));
    assert_eq!(heap, HeapAllocationObservation::default());
    assert!(matches!(restored.unwrap(), RestoreStep::Returned { progress } if progress.fits(grant) && progress.retained_capacity_bytes == 0 && progress.released_bytes == 0));
    assert_eq!(owner.get(fields[0]).unwrap().original_body_ptr(), pointer);
    assert_eq!(owner.get(fields[0]).unwrap().at_ms, 240.0);
    assert!(cursor.peek().is_none());
    close(cursor, grant);
    let (cursor, _) = owner.begin(grant).unwrap();
    close(cursor, grant);
}

#[test]
fn camera_replacement_keeps_both_originals_until_their_separate_paid_release() {
    let fixture = fixture();
    let grant = grant(&fixture);
    let fields = text(&fixture["cases"][1]);
    let mut owner = DirectoryOwner::<u64>::default();
    owner.install(grant).unwrap();
    owner.schedule(fields, 1, 120.0, grant).unwrap();
    let previous = owner.get(fields[0]).unwrap().original_body_ptr();
    let (receipt, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| owner.schedule(fields, 2, 240.0, grant));
    assert_eq!(heap, HeapAllocationObservation { requested_bytes: receipt.unwrap().retained_capacity_bytes, ..Default::default() });
    assert_ne!(owner.get(fields[0]).unwrap().original_body_ptr(), previous);
    assert_eq!(owner.len(), 1);
    let (mut cursor, transfer) = owner.begin(grant).unwrap();
    assert!(cursor.has_retiring());
    let (step, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| cursor.close_retiring(grant));
    let original = step.unwrap().progress();
    assert_eq!(heap, HeapAllocationObservation { released_bytes: original.released_bytes, largest_release_bytes: original.released_bytes, ..Default::default() });
    assert_eq!(original.released_bytes, fields.concat().len());
    assert!(!cursor.has_retiring());
    assert_eq!(close(cursor, grant), fields.concat().len() + transfer.retained_capacity_bytes);
}

#[test]
fn camera_cursor_refuses_authority_escalation_and_closed_headers_are_not_text_owners() {
    let fixture = fixture();
    let grant = grant(&fixture);
    let fields = text(&fixture["cases"][0]);
    let mut owner = DirectoryOwner::<u64>::default();
    owner.install(grant).unwrap();
    owner.schedule(fields, 1, 120.0, grant).unwrap();
    let (mut cursor, _) = owner.begin(grant).unwrap();
    let original = cursor.peek().unwrap().1.original_body_ptr();
    let enlarged = RetainedCloneGrant { maximum_release_bytes: grant.maximum_release_bytes + 1, ..grant };
    let (result, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| cursor.close_current(enlarged));
    assert_eq!(result.unwrap_err().kind, ValueRefusalKind::InvariantViolated);
    assert_eq!(heap, HeapAllocationObservation::default());
    assert_eq!(cursor.peek().unwrap().1.original_body_ptr(), original);
    close(cursor, grant);
    owner.schedule(fields, 1, 120.0, grant).unwrap();
    owner.close_step(grant).unwrap();
    assert!(owner.get(fields[0]).is_none());
    assert_eq!(owner.len(), 1);
    owner.close_step(grant).unwrap();
    assert!(owner.terminal_is_empty());
}

#[test]
fn camera_exact_capacity_restores_originals_and_retains_a_full_cursor_on_refusal() {
    let fixture = fixture();
    let grant = grant(&fixture);
    let law = &fixture["boundaries"];
    let identities = &law["identities"];
    let count = identities["count"].as_u64().unwrap() as usize;
    assert_eq!(count, CAPACITY);
    let hosts: Vec<String> = (0..count).map(|index| format!("{}{index}", identities["prefix"].as_str().unwrap())).collect();
    let surface = identities["surface"].as_str().unwrap();
    let metadata = identities["metadata"].as_u64().unwrap();
    let deadline = identities["deadlineMs"].as_f64().unwrap();
    let mut owner = DirectoryOwner::default();
    owner.install(grant).unwrap();
    for host in &hosts {
        let (receipt, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| owner.schedule([host, surface, "", "", "", ""], metadata, deadline, grant));
        let receipt = receipt.unwrap();
        assert!(receipt.fits(grant));
        assert_eq!(heap, HeapAllocationObservation { requested_bytes: host.len() + surface.len(), ..Default::default() });
        assert_eq!(receipt.retained_capacity_bytes, heap.requested_bytes);
    }
    let original: Vec<_> = hosts.iter().map(|host| owner.get(host).unwrap().original_body_ptr()).collect();
    let overflow = &law["overflow"];
    let (denied, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| owner.schedule([overflow["host"].as_str().unwrap(), surface, "", "", "", ""], metadata, deadline, grant));
    assert_eq!(denied.unwrap_err().kind, ValueRefusalKind::WorkLimit);
    assert_eq!(heap, HeapAllocationObservation::default());
    assert_eq!(owner.len(), overflow["retainedEntries"].as_u64().unwrap() as usize);
    for (host, pointer) in hosts.iter().zip(&original) { assert_eq!(owner.get(host).unwrap().original_body_ptr(), *pointer); }
    let ((mut cursor, transfer), heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| owner.begin(grant).unwrap());
    assert_eq!(heap, HeapAllocationObservation { requested_bytes: transfer.retained_capacity_bytes, ..Default::default() });
    assert!(transfer.fits(grant));
    assert_eq!(owner.len(), law["checkout"]["globalEntries"].as_u64().unwrap() as usize);
    assert_eq!(cursor.len(), law["checkout"]["cursorEntries"].as_u64().unwrap() as usize);
    let directory = cursor.original_directory_ptr();
    for (host, pointer) in hosts.iter().zip(&original) {
        let (restored, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| owner.restore(&mut cursor, grant));
        assert!(matches!(restored.unwrap(), RestoreStep::Returned { progress } if progress.fits(grant) && progress.retained_capacity_bytes == 0 && progress.released_bytes == 0));
        assert_eq!(heap, HeapAllocationObservation::default());
        assert_eq!(cursor.original_directory_ptr(), directory);
        let record = owner.get(host).unwrap();
        assert_eq!(record.original_body_ptr(), *pointer);
        assert_eq!((record.metadata, record.at_ms), (metadata, deadline));
    }
    assert_eq!(owner.len(), law["restore"]["globalEntries"].as_u64().unwrap() as usize);
    assert_eq!(cursor.len(), law["restore"]["cursorEntries"].as_u64().unwrap() as usize);
    assert_eq!(close(cursor, grant), transfer.retained_capacity_bytes);
    let other_hosts: Vec<String> = hosts.iter().map(|host| format!("original-{host}")).collect();
    let mut other = DirectoryOwner::default();
    other.install(grant).unwrap();
    for host in &other_hosts { other.schedule([host, surface, "", "", "", ""], metadata, deadline, grant).unwrap(); }
    let (mut cursor, _) = other.begin(grant).unwrap();
    let original_body = cursor.peek().unwrap().1.original_body_ptr();
    let directory = cursor.original_directory_ptr();
    let (restored, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| owner.restore(&mut cursor, grant));
    assert!(matches!(restored.unwrap(), RestoreStep::Retained { progress } if progress == Default::default()));
    assert_eq!(heap, HeapAllocationObservation::default());
    assert_eq!(cursor.original_directory_ptr(), directory);
    assert_eq!(cursor.peek().unwrap().1.original_body_ptr(), original_body);
    assert_eq!(owner.len(), law["occupiedRestore"]["globalEntries"].as_u64().unwrap() as usize);
    assert_eq!(cursor.len(), law["occupiedRestore"]["cursorEntries"].as_u64().unwrap() as usize);
    for (host, pointer) in hosts.iter().zip(&original) { assert_eq!(owner.get(host).unwrap().original_body_ptr(), *pointer); }
    close(cursor, grant);
    let (cursor, _) = owner.begin(grant).unwrap();
    close(cursor, grant);
    assert!(owner.terminal_is_empty() && other.terminal_is_empty());
    eprintln!("[DEBUG] camera exact256 restored original bodies,257th/full-cursor refusal inert, original backing closed");
}

#[test]
fn camera_identifier_maximum_plus_one_is_inert_on_a_vacant_original_owner() {
    let fixture = fixture();
    let grant = grant(&fixture);
    let law = &fixture["boundaries"]["identifier"];
    let denied = "x".repeat(law["attemptedBytes"].as_u64().unwrap() as usize);
    let maximum = "x".repeat(law["maximumBytes"].as_u64().unwrap() as usize);
    let mut owner = DirectoryOwner::<u64>::default();
    owner.install(grant).unwrap();
    for fields in [[denied.as_str(), "surface", "", "", "", ""], ["host", denied.as_str(), "", "", "", ""]] {
        let (result, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| owner.schedule(fields, 7, 120.0, grant));
        assert_eq!(result.unwrap_err().kind, ValueRefusalKind::WorkLimit);
        assert_eq!(heap, HeapAllocationObservation::default());
        assert_eq!(owner.len(), law["retainedEntries"].as_u64().unwrap() as usize);
        assert!(owner.terminal_is_empty());
    }
    let (receipt, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| owner.schedule([&maximum, &maximum, "", "", "", ""], 7, 120.0, grant));
    assert!(receipt.unwrap().fits(grant));
    assert_eq!(heap, HeapAllocationObservation { requested_bytes: maximum.len() * 2, ..Default::default() });
    assert_eq!(owner.get(&maximum).unwrap().original_bytes().len(), maximum.len() * 2);
    let (cursor, _) = owner.begin(grant).unwrap();
    close(cursor, grant);
    assert!(owner.terminal_is_empty());
    eprintln!("[DEBUG] camera both identifier256 boundaries admitted,257-byte denial on vacant owner had zero physical effect");
}

#[test]
fn borrowed_node_key_keeps_original_text_and_matches_the_independent_corpus_without_heap_effects() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../../../../../../../🔨️modules/🖱️ui/🔑️node-key/🧫️fixtures/🔣️.json" )).unwrap();
    for row in fixture["cases"].as_array().unwrap() {
        let value = &row["owned"];
        let owned = match value["explicit"].as_str() {
            Some(text) => ui_wgpu::wgpu::NodeKey::Explicit(text.to_owned()),
            None => ui_wgpu::wgpu::NodeKey::Positional(value["positional"][0].as_u64().unwrap() as u32, value["positional"][1].as_u64().unwrap() as u32),
        };
        let other = &row["borrowed"];
        let key = match other["explicit"].as_str() {
            Some(text) => ui_wgpu::wgpu::NodeKeyRef::Explicit(text),
            None => ui_wgpu::wgpu::NodeKeyRef::Positional(other["positional"][0].as_u64().unwrap() as u32, other["positional"][1].as_u64().unwrap() as u32),
        };
        let ((matches, original), heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| {
            let witness = owned.borrowed();
            (witness == key, match (&owned, witness) {
                (ui_wgpu::wgpu::NodeKey::Explicit(text), ui_wgpu::wgpu::NodeKeyRef::Explicit(borrowed)) => text.as_ptr() == borrowed.as_ptr() && text.len() == borrowed.len(),
                (ui_wgpu::wgpu::NodeKey::Positional(a, b), ui_wgpu::wgpu::NodeKeyRef::Positional(x, y)) => (*a, *b) == (x, y),
                _ => false,
            })
        });
        assert_eq!(heap, HeapAllocationObservation::default());
        assert!(original);
        assert_eq!(matches, row["matches"].as_bool().unwrap());
    }
}
