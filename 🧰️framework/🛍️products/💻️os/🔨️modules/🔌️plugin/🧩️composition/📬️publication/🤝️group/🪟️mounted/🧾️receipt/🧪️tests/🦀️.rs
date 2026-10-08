#[test]
fn mounted_receipt_bytes_preserve_original_backing_and_all_native_allocation_axes() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();
    assert_eq!(fixture["maximumCapacityBytes"].as_u64().unwrap() as usize, MOUNTED_RECEIPT_MAXIMUM_BYTES);
    for row in fixture["cases"].as_array().unwrap() {
        let text = row["text"].as_str().unwrap().as_bytes();
        let initial = row["initialCapacity"].as_u64().unwrap() as usize;
        let mut owner = MountedReceiptBytes::from_original(Vec::with_capacity(initial));
        let mut offset = 0;
        for _ in 0..1000 {
            if offset == text.len() { break; }
            let additional = (text.len() - offset).min(64);
            let capacity = owner.next_capacity_byte_demand(additional).unwrap();
            let release = owner.next_growth_release_byte_demand();
            let grant = RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 64, maximum_capacity_bytes: capacity, maximum_release_bytes: release, maximum_depth: 64 };
            let pointer = owner.bytes.as_ptr();
            let prefix = owner.bytes.len();
            if capacity > 0 || release > 0 {
                for denied in [RetainedCloneGrant { maximum_items: 0, ..grant }, RetainedCloneGrant { maximum_capacity_bytes: capacity.saturating_sub(1), maximum_release_bytes: release.saturating_sub(1), ..grant }] {
                    let (step, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| owner.reserve_step(additional, denied).unwrap());
                    assert_eq!(step.progress(), RetainedCloneProgress::default());
                    assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
                    assert_eq!((owner.bytes.as_ptr(), owner.bytes.len()), (pointer, prefix));
                }
            }
            let ready = owner.ready_for(additional);
            let (step, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| if ready { owner.append_step(&text[offset..], grant) } else { owner.reserve_step(additional, grant) }.unwrap());
            let progress = step.progress();
            assert!(progress.fits(grant));
            assert_eq!((heap.requested_bytes, heap.released_bytes), (progress.retained_capacity_bytes, progress.released_bytes));
            if ready { offset += progress.copied_bytes; }
        }
        assert_eq!(offset, text.len());
        assert_eq!(owner.as_slice(), text);
        let oracle: serde_json::Value = serde_json::from_str(&serde_json::to_string(row["text"].as_str().unwrap()).unwrap()).unwrap();
        assert_eq!(owner.as_slice(), oracle.as_str().unwrap().as_bytes());
        let bytes = owner.next_close_byte_demand();
        let grant = RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 64, maximum_capacity_bytes: 0, maximum_release_bytes: bytes, maximum_depth: 64 };
        if bytes > 0 {
            for denied in [RetainedCloneGrant { maximum_items: 0, ..grant }, RetainedCloneGrant { maximum_release_bytes: bytes - 1, ..grant }] {
                let pointer = owner.bytes.as_ptr();
                let (step, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| owner.close_step(denied));
                assert_eq!(step.progress(), RetainedCloneProgress::default());
                assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
                assert_eq!(owner.bytes.as_ptr(), pointer);
            }
        }
        let (step, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| owner.close_step(grant));
        assert_eq!((heap.requested_bytes, heap.released_bytes), (0, step.progress().released_bytes));
        assert!(owner.terminal_is_empty());
        let (_, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| drop(owner));
        assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
        println!("[DEBUG] original mounted receipt case={} retained initial={initial} outputBytes={} copy64; birth/copy/original-release all separate; every denied original pointer preserved", row["id"], text.len());
    }
}

#[test]
fn mounted_receipt_bytes_cancel_every_original_growth_frontier_without_hidden_release() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();
    for (index, frontier) in fixture["cancellationFrontiers"].as_array().unwrap().iter().enumerate() {
        let mut original = Vec::with_capacity(64); original.extend_from_slice(&[17u8; 64]);
        let mut owner = MountedReceiptBytes::from_original(original);
        let pointer = owner.bytes.as_ptr();
        let grant = RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 16, maximum_capacity_bytes: 128, maximum_release_bytes: 0, maximum_depth: 64 };
        let (step, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| owner.reserve_step(1, grant).unwrap());
        assert_eq!((heap.requested_bytes, heap.released_bytes), (128, 0));
        assert_eq!(step.progress().retained_capacity_bytes, 128);
        for _ in 0..[0, 1, 4][index] {
            let (step, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| owner.reserve_step(1, RetainedCloneGrant { maximum_capacity_bytes: 0, ..grant }).unwrap());
            assert!(step.progress().copied_bytes <= 16);
            assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
        }
        assert_eq!(owner.bytes.as_ptr(), pointer);
        for bytes in [128, 64] {
            assert_eq!(owner.next_close_byte_demand(), bytes);
            let grant = RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 64, maximum_capacity_bytes: 0, maximum_release_bytes: bytes, maximum_depth: 64 };
            for denied in [RetainedCloneGrant { maximum_items: 0, ..grant }, RetainedCloneGrant { maximum_release_bytes: bytes - 1, ..grant }] {
                let (step, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| owner.close_step(denied));
                assert_eq!(step.progress(), RetainedCloneProgress::default());
                assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
                assert_eq!(owner.bytes.as_ptr(), pointer);
            }
            let (step, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| owner.close_step(grant));
            assert_eq!(step.progress().released_bytes, bytes);
            assert_eq!((heap.requested_bytes, heap.released_bytes), (0, bytes));
            assert!(owner.next_capacity_byte_demand(1).is_err());
        }
        assert!(owner.terminal_is_empty());
        let (_, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| drop(owner));
        assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
        println!("[DEBUG] original receipt growth cancellation={frontier} candidate128 then original64 exact funded frees, zero/one-below preserve pointer, terminal Drop0heap");
    }
}

struct MountedReceiptJsonString<'a>(&'a str);

impl store::ArtifactCanonicalJson for MountedReceiptJsonString<'_> {
    fn canonical_json_node(&self, path: &[usize]) -> Result<store::ArtifactCanonicalJsonNode<'_>, String> { if path.is_empty() { Ok(store::ArtifactCanonicalJsonNode::String(self.0)) } else { Err("mounted receipt fixture has only its original string root".into()) } }
}

#[test]
fn mounted_receipt_prepared_operation_prestages_exact_original_payload_before_acknowledgment() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();
    assert_eq!(fixture["operationHeader"], serde_json::json!([1, 7]));
    for row in fixture["cases"].as_array().unwrap() {
        let body = MountedReceiptJsonString(row["text"].as_str().unwrap());
        for hex in fixture["operationHexModes"].as_array().unwrap() {
            let hex = hex.as_bool().unwrap();
            let source = if hex { store::ArtifactPreparedOperationSource::HexJson { header: &[1, 7], prefix: b"value=", body: &body } } else { store::ArtifactPreparedOperationSource::CanonicalJson { header: &[1, 7], body: &body } };
            let json = serde_json::to_vec(body.0).unwrap();
            let mut expected = vec![1, 7];
            if hex { expected.extend_from_slice(b"value="); for byte in json { expected.extend_from_slice(&[b"0123456789abcdef"[usize::from(byte >> 4)], b"0123456789abcdef"[usize::from(byte & 15)]]); } } else { expected.extend(json); }
            let mut owner = MountedPreparedOperationBytes::new();
            for _ in 0..4000 {
                if owner.is_complete() { break; }
                let capacity = owner.next_capacity_byte_demand().unwrap();
                let release = owner.next_growth_release_byte_demand().unwrap();
                let grant = RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 64, maximum_capacity_bytes: capacity, maximum_release_bytes: release, maximum_depth: 64 };
                let pointer = owner.output.bytes.as_ptr();
                let prefix = owner.output.bytes.len();
                let (step, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| owner.advance(source, RetainedCloneGrant { maximum_items: 0, ..grant }).unwrap());
                assert_eq!(step.progress(), Default::default());
                assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
                if capacity > 0 || release > 0 {
                    let denied = RetainedCloneGrant { maximum_capacity_bytes: capacity.saturating_sub(1), maximum_release_bytes: release.saturating_sub(1), ..grant };
                    let (step, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| owner.advance(source, denied).unwrap());
                    assert_eq!(step.progress(), Default::default());
                    assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
                    assert_eq!((owner.output.bytes.as_ptr(), owner.output.bytes.len()), (pointer, prefix));
                }
                let (step, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| owner.advance(source, grant).unwrap());
                let progress = step.progress();
                assert!(progress.fits(grant));
                assert_eq!((heap.requested_bytes, heap.released_bytes), (progress.retained_capacity_bytes, progress.released_bytes));
                assert!(progress.copied_bytes <= 64);
            }
            assert!(owner.is_complete());
            assert_eq!(owner.output.as_slice(), expected);
            let grant = RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 64, maximum_capacity_bytes: 0, maximum_release_bytes: 0, maximum_depth: 64 };
            let (bytes, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| owner.take(grant).unwrap());
            assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
            assert_eq!(bytes, expected);
            assert!(owner.terminal_is_empty());
            let (_, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| drop(owner));
            assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
            let mut bytes = MountedReceiptBytes::from_original(bytes);
            let demand = bytes.next_close_byte_demand();
            let (step, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| bytes.close_step(RetainedCloneGrant { maximum_release_bytes: demand, ..grant }));
            assert_eq!((heap.requested_bytes, heap.released_bytes), (0, step.progress().released_bytes));
            assert!(bytes.terminal_is_empty());
            println!("[DEBUG] mounted prepared operation original={} hex={hex} bytes={} exact serde oracle; codec and append copy64 in separate turns; every birth and original free independently funded", row["id"], expected.len());
        }
    }
}

#[test]
fn mounted_receipt_prepared_operation_cancellation_retires_only_funded_original_backing() {
    let body = MountedReceiptJsonString("group:📐雪 original-owned-parent-and-child original-owned-parent-and-child original-owned-parent-and-child");
    let source = store::ArtifactPreparedOperationSource::CanonicalJson { header: &[1, 7], body: &body };
    for frontier in [1, 2, 3, 9, 15] {
        let mut owner = MountedPreparedOperationBytes::new();
        for _ in 0..frontier {
            let grant = RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 64, maximum_capacity_bytes: owner.next_capacity_byte_demand().unwrap(), maximum_release_bytes: owner.next_growth_release_byte_demand().unwrap(), maximum_depth: 64 };
            let (step, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| owner.advance(source, grant).unwrap());
            assert_eq!((heap.requested_bytes, heap.released_bytes), (step.progress().retained_capacity_bytes, step.progress().released_bytes));
        }
        for _ in 0..8 {
            if owner.terminal_is_empty() { break; }
            let demand = owner.next_close_byte_demand().unwrap();
            let grant = RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 64, maximum_capacity_bytes: 0, maximum_release_bytes: demand, maximum_depth: 64 };
            let (step, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| owner.close_step(RetainedCloneGrant { maximum_items: 0, ..grant }).unwrap());
            assert_eq!(step.progress(), Default::default());
            assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
            if demand > 0 {
                let pointer = owner.output.bytes.as_ptr();
                let (step, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| owner.close_step(RetainedCloneGrant { maximum_release_bytes: demand - 1, ..grant }).unwrap());
                assert_eq!(step.progress(), Default::default());
                assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
                assert_eq!(owner.output.bytes.as_ptr(), pointer);
            }
            let (step, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| owner.close_step(grant).unwrap());
            assert!(step.progress().fits(grant));
            assert_eq!((heap.requested_bytes, heap.released_bytes), (0, step.progress().released_bytes));
        }
        assert!(owner.terminal_is_empty());
        let (_, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| drop(owner));
        assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
        println!("[DEBUG] mounted prepared operation cancel frontier={frontier}: inline codec first, whole original candidate/output separately funded, terminal Drop0heap");
    }
}

#[test]
fn mounted_receipt_operation_list_retains_every_original_inverse_length_and_payload() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();
    for hex in fixture["operationHexModes"].as_array().unwrap() {
        let hex = hex.as_bool().unwrap();
        for indices in fixture["operationLists"].as_array().unwrap() {
            let bodies: Vec<_> = indices.as_array().unwrap().iter().map(|index| MountedReceiptJsonString(fixture["cases"][index.as_u64().unwrap() as usize]["text"].as_str().unwrap())).collect();
            let operations: Vec<_> = bodies.iter().map(|body| {
                let json = serde_json::to_vec(body.0).unwrap(); let mut bytes = vec![1, 7];
                if hex { bytes.extend_from_slice(b"value="); for byte in json { bytes.extend_from_slice(&[b"0123456789abcdef"[usize::from(byte >> 4)], b"0123456789abcdef"[usize::from(byte & 15)]]); } } else { bytes.extend(json); }
                bytes
            }).collect();
            let expected = protocol::encode_ops_vec(&operations);
            let mut owner = MountedPreparedOperationsBytes::new(bodies.len());
            for _ in 0..8000 {
                if owner.is_complete() { break; }
                let source = owner.operation_index().map(|index| if hex { store::ArtifactPreparedOperationSource::HexJson { header: &[1, 7], prefix: b"value=", body: &bodies[index] } } else { store::ArtifactPreparedOperationSource::CanonicalJson { header: &[1, 7], body: &bodies[index] } });
                let capacity = owner.next_capacity_byte_demand().unwrap(); let release = owner.next_release_byte_demand().unwrap();
                let grant = RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 64, maximum_capacity_bytes: capacity, maximum_release_bytes: release, maximum_depth: 64 };
                let (step, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| owner.advance(source, RetainedCloneGrant { maximum_items: 0, ..grant }).unwrap());
                assert_eq!(step.progress(), Default::default()); assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
                if capacity > 0 || release > 0 {
                    let pointer = owner.output.bytes.as_ptr(); let prefix = owner.output.bytes.len();
                    let (step, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| owner.advance(source, RetainedCloneGrant { maximum_capacity_bytes: capacity.saturating_sub(1), maximum_release_bytes: release.saturating_sub(1), ..grant }).unwrap());
                    assert_eq!(step.progress(), Default::default()); assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0)); assert_eq!((owner.output.bytes.as_ptr(), owner.output.bytes.len()), (pointer, prefix));
                }
                let (step, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| owner.advance(source, grant).unwrap());
                assert!(step.progress().fits(grant)); assert_eq!((heap.requested_bytes, heap.released_bytes), (step.progress().retained_capacity_bytes, step.progress().released_bytes));
            }
            assert!(owner.is_complete()); assert_eq!(owner.output.as_slice(), expected);
            let grant = RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 64, maximum_capacity_bytes: 0, maximum_release_bytes: 0, maximum_depth: 64 };
            let (bytes, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| owner.take(grant).unwrap());
            assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0)); assert_eq!(bytes, expected); assert!(owner.terminal_is_empty());
            let (_, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| drop(owner)); assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
            let mut bytes = MountedReceiptBytes::from_original(bytes); let demand = bytes.next_close_byte_demand();
            let (step, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| bytes.close_step(RetainedCloneGrant { maximum_release_bytes: demand, ..grant })); assert_eq!((heap.requested_bytes, heap.released_bytes), (0, step.progress().released_bytes)); assert!(bytes.terminal_is_empty());
            println!("[DEBUG] original mounted inverse framing operations={} hex={hex} bytes={}: exact count/length varints and complete original serde payload; every operation backing freed separately from framed output", bodies.len(), expected.len());
        }
    }
}

#[test]
fn mounted_receipt_operation_list_cancels_codec_payload_and_framed_growth_frontiers() {
    let body = MountedReceiptJsonString("group:📐雪 original-owned-parent-and-child original-owned-parent-and-child original-owned-parent-and-child");
    let source = store::ArtifactPreparedOperationSource::HexJson { header: &[1, 7], prefix: b"value=", body: &body };
    for frontier in [1, 4, 12, 27, 60] {
        let mut owner = MountedPreparedOperationsBytes::new(4);
        for _ in 0..frontier {
            let grant = RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 64, maximum_capacity_bytes: owner.next_capacity_byte_demand().unwrap(), maximum_release_bytes: owner.next_release_byte_demand().unwrap(), maximum_depth: 64 };
            let (step, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| owner.advance(Some(source), grant).unwrap());
            assert!(step.progress().fits(grant)); assert_eq!((heap.requested_bytes, heap.released_bytes), (step.progress().retained_capacity_bytes, step.progress().released_bytes));
        }
        for _ in 0..12 {
            if owner.terminal_is_empty() { break; }
            let demand = owner.next_close_byte_demand().unwrap(); let grant = RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 64, maximum_capacity_bytes: 0, maximum_release_bytes: demand, maximum_depth: 64 };
            let (step, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| owner.close_step(RetainedCloneGrant { maximum_items: 0, ..grant }).unwrap()); assert_eq!(step.progress(), Default::default()); assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
            if demand > 0 { let (step, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| owner.close_step(RetainedCloneGrant { maximum_release_bytes: demand - 1, ..grant }).unwrap()); assert_eq!(step.progress(), Default::default()); assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0)); }
            let (step, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| owner.close_step(grant).unwrap()); assert!(step.progress().fits(grant)); assert_eq!((heap.requested_bytes, heap.released_bytes), (0, step.progress().released_bytes));
        }
        assert!(owner.terminal_is_empty()); let (_, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| drop(owner)); assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
        println!("[DEBUG] original inverse-list cancellation frontier={frontier}: inline codec, unframed payload, growth candidate, and framed output all separately funded; terminal Drop0heap");
    }
}

#[test]
fn mounted_receipt_kernel_mutation_preserves_every_original_causal_field_before_publication() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap(); let meta = &fixture["receiptMetadata"];
    for dependency_count in [0, 3] {
        let dependencies: Vec<_> = meta["dependencies"].as_array().unwrap().iter().take(dependency_count).map(|id| MutationId(id.as_str().unwrap().into())).collect();
        for row in fixture["cases"].as_array().unwrap() {
            let forward = serde_json::to_vec(row["text"].as_str().unwrap()).unwrap(); let inverse = protocol::encode_ops_vec(&[serde_json::to_vec(meta["mutationId"].as_str().unwrap()).unwrap(), forward.clone()]);
            let source = MountedKernelMutationReceiptSource { document: ArtifactHandle(19), mutation_id: meta["mutationId"].as_str().unwrap(), invocation_id: meta["invocationId"].as_str().unwrap(), schema: meta["schema"].as_str().unwrap(), schema_separator: meta["schemaSeparator"].as_str().unwrap(), schema_suffix: meta["schemaSuffix"].as_str().unwrap(), author: meta["author"].as_str().unwrap(), dependencies: &dependencies, inverse: &inverse, base_version: ArtifactVersion(meta["baseVersion"].as_u64().unwrap()), timestamp: HybridLogicalTimestamp { actor: meta["clockActor"].as_u64().unwrap(), physical_ms: meta["physicalMs"].as_u64().unwrap(), logical: meta["logical"].as_u64().unwrap() }, undo_policy: UndoPolicy::ExactBaseOnly };
            let mut original_forward = Vec::with_capacity(8194); original_forward.extend_from_slice(&forward); let original_pointer = original_forward.as_ptr();
            let (mut owner, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| MountedKernelMutationReceipt::new(&source, original_forward));
            assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0)); assert_eq!(owner.forward.bytes.as_ptr(), original_pointer);
            for _ in 0..8000 {
                if owner.is_complete() { break; }
                let capacity = owner.next_capacity_byte_demand(&source).unwrap(); let release = owner.next_release_byte_demand();
                let grant = RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 64, maximum_capacity_bytes: capacity, maximum_release_bytes: release, maximum_depth: 64 };
                let (step, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| owner.advance(&source, RetainedCloneGrant { maximum_items: 0, ..grant }).unwrap()); assert_eq!(step.progress(), Default::default()); assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
                if capacity > 0 || release > 0 { let (step, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| owner.advance(&source, RetainedCloneGrant { maximum_capacity_bytes: capacity.saturating_sub(1), maximum_release_bytes: release.saturating_sub(1), ..grant }).unwrap()); assert_eq!(step.progress(), Default::default()); assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0)); }
                let (step, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| owner.advance(&source, grant).unwrap()); assert!(step.progress().fits(grant)); assert_eq!((heap.requested_bytes, heap.released_bytes), (step.progress().retained_capacity_bytes, step.progress().released_bytes)); assert_eq!(owner.forward.bytes.as_ptr(), original_pointer);
            }
            assert!(owner.is_complete());
            let grant = RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 64, maximum_capacity_bytes: 0, maximum_release_bytes: 0, maximum_depth: 64 };
            let (result, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| owner.take(grant).unwrap()); assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0)); assert!(owner.terminal_is_empty());
            let (mutation, undo_id, undo_inverse) = result;
            assert_eq!(mutation.id.0, source.mutation_id); assert_eq!(mutation.document, source.document); assert_eq!(mutation.invocation_id.0, source.invocation_id); assert_eq!(mutation.base_version, source.base_version); assert_eq!(mutation.timestamp, source.timestamp); assert_eq!(mutation.author.0, source.author);
            assert_eq!(mutation.diff.schema.0, format!("{}{}{}", source.schema, source.schema_separator, source.schema_suffix)); assert_eq!(mutation.diff.payload, forward); assert_eq!(mutation.diff.payload.as_ptr(), original_pointer);
            assert_eq!(mutation.inverse.target_mutation, mutation.id); assert_eq!(mutation.inverse.inverse_diff.schema.0, format!("{}{}{}.inverse", source.schema, source.schema_separator, source.schema_suffix)); assert_eq!(mutation.inverse.inverse_diff.payload, inverse); assert_eq!(mutation.inverse.base_version, source.base_version); assert_eq!(mutation.inverse.undo_policy, source.undo_policy);
            assert_eq!(mutation.dependencies, dependencies); assert_eq!(mutation.inverse.dependencies, dependencies); assert_eq!(undo_inverse.dependencies, dependencies); assert_eq!(undo_id, mutation.id); assert_eq!(undo_inverse, mutation.inverse);
            if dependency_count > 0 { assert_ne!(mutation.dependencies.as_ptr(), dependencies.as_ptr()); assert_ne!(mutation.inverse.dependencies.as_ptr(), mutation.dependencies.as_ptr()); assert_ne!(undo_inverse.dependencies.as_ptr(), mutation.inverse.dependencies.as_ptr()); }
            let (_, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| drop(owner)); assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
            println!("[DEBUG] original kernel receipt={} dependencies={dependency_count}: original forward8194 pointer transferred, exact IDs/schema/HLC/base/author/inverse preserved; three independent causal vectors, every birth/free funded, final assembly0heap", row["id"]);
        }
    }
}

#[test]
fn mounted_receipt_kernel_mutation_cancellation_retains_original_owners_under_partial_grants() {
    let dependencies = vec![MutationId("dependency:one".into()), MutationId("dependency:雪".into()), MutationId("dependency:three".into())];
    let inverse = [19; 257];
    let source = MountedKernelMutationReceiptSource { document: ArtifactHandle(19), mutation_id: "mutation:📐雪", invocation_id: "group:original-雪", schema: "semio.fixture", schema_separator: ".", schema_suffix: "operation", author: "actor:📐", dependencies: &dependencies, inverse: &inverse, base_version: ArtifactVersion(17), timestamp: HybridLogicalTimestamp { actor: 5, physical_ms: 23, logical: 7 }, undo_policy: UndoPolicy::ExactBaseOnly };
    for frontier in [0, 1, 30, 90, 140, 190] {
        let mut forward = Vec::with_capacity(8194); forward.extend_from_slice(&[1, 7, 9]); let pointer = forward.as_ptr(); let mut owner = MountedKernelMutationReceipt::new(&source, forward);
        for _ in 0..frontier { let grant = RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 64, maximum_capacity_bytes: owner.next_capacity_byte_demand(&source).unwrap(), maximum_release_bytes: owner.next_release_byte_demand(), maximum_depth: 64 }; let (step, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| owner.advance(&source, grant).unwrap()); assert!(step.progress().fits(grant)); assert_eq!((heap.requested_bytes, heap.released_bytes), (step.progress().retained_capacity_bytes, step.progress().released_bytes)); }
        assert_eq!(owner.forward.bytes.as_ptr(), pointer);
        for _ in 0..100 {
            if owner.terminal_is_empty() { break; }
            let demand = owner.next_close_byte_demand(); let grant = RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 64, maximum_capacity_bytes: 0, maximum_release_bytes: demand, maximum_depth: 64 };
            let (step, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| owner.close_step(RetainedCloneGrant { maximum_items: 0, ..grant })); assert_eq!(step.progress(), Default::default()); assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
            if demand > 0 { let (step, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| owner.close_step(RetainedCloneGrant { maximum_release_bytes: demand - 1, ..grant })); assert_eq!(step.progress(), Default::default()); assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0)); }
            let (step, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| owner.close_step(grant)); assert!(step.progress().fits(grant)); assert_eq!((heap.requested_bytes, heap.released_bytes), (0, step.progress().released_bytes));
        }
        assert!(owner.terminal_is_empty()); let (_, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| drop(owner)); assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
        println!("[DEBUG] original host receipt cancellation={frontier}: original8194 forward first, all UTF8/inverse/dependency backing separately funded; denied whole frees0heap and terminal Drop0heap");
    }
}

#[test]
fn mounted_receipt_child_document_handle_hashes_original_utf8_with_bounded_zero_heap_turns() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();
    for (index, row) in fixture["cases"].as_array().unwrap().iter().enumerate() {
        let source = row["text"].as_str().unwrap();
        for copy in [1, 3, 7, 64] {
            let mut owner = MountedArtifactHandleIssuer::new(); let pointer = source.as_ptr();
            let grant = RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: copy, maximum_capacity_bytes: 0, maximum_release_bytes: 0, maximum_depth: 64 };
            for _ in 0..1000 {
                if owner.ready() { break; }
                let offset = owner.offset;
                let (step, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| owner.advance(source, RetainedCloneGrant { maximum_items: 0, ..grant }).unwrap()); assert_eq!(step.progress(), Default::default()); assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0)); assert_eq!(owner.offset, offset);
                if !source.is_empty() { let (step, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| owner.advance(source, RetainedCloneGrant { maximum_copy_bytes: 0, ..grant }).unwrap()); assert_eq!(step.progress(), Default::default()); assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0)); assert_eq!(owner.offset, offset); }
                let (step, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| owner.advance(source, grant).unwrap()); assert!(step.progress().fits(grant)); assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0)); assert_eq!(source.as_ptr(), pointer);
            }
            assert!(owner.ready());
            let (handle, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| owner.take(grant).unwrap()); assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0)); assert_eq!(format!("{:032x}", handle.0), fixture["artifactHandles"][index].as_str().unwrap());
            println!("[DEBUG] original child handle={} copy={copy}: exact Node/BigInt two-lane handle {:?}, original pointer retained, every turn0heap", row["id"], handle);
        }
    }
}
