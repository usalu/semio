use super::*;
use semio_framework_value::{
    retained_clone::RetainedCloneSource,
    retirement::{controlled::ControlledRetirement, RetireOwned},
    RetainedAllocationEffect, RetainedWorkOutcome,
};
use semio_framework_value_persistent::list::persistent::{PersistentList, PERSISTENT_PAGE_FANOUT};
use semio_framework_value_shared_issuance::{SealedShared, SharedIssuer};
use semio_framework_value_testing_admission::{bind_reply, drive_reply, driven, driven_quotes, BankWallet, EffectBank};
use semio_framework_trace::observe_heap_allocations_on_this_thread as observe;

#[derive(semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = crate)]
struct Row {
    label: String,
    rank: i64,
}

impl Clone for Row {
    fn clone(&self) -> Self {
        panic!("original canonical tree must borrow its rows")
    }
}

fn grant(demand: RetirementDemand) -> RetainedCloneGrant {
    RetainedCloneGrant {
        maximum_items: 1,
        maximum_copy_bytes: demand.copy_bytes.max(4096),
        maximum_capacity_bytes: demand.capacity_bytes,
        maximum_release_bytes: demand.release_bytes,
        maximum_depth: demand.depth,
    }
}

fn effect_turn<R>(bank: &mut EffectBank, grant: RetainedCloneGrant, operation: impl FnOnce(&mut RetainedAllocationEffect<'_>) -> R) -> R {
    let turn = bank.run_outcome(grant, 1 << 20, |effect| {
        let result = operation(effect);
        RetainedWorkOutcome { result: Ok::<_, ()>(result), work: effect.performed_work() }
    });
    bank.take_admitted();
    turn.result.unwrap()
}

fn advance_tree(
    bank: &mut EffectBank,
    cursor: &mut ArtifactCanonicalJsonTreeCursor,
    bytes: &mut [u8],
    mut grant: RetainedCloneGrant,
) -> ArtifactCanonicalJsonTreeStep {
    for quote in driven_quotes::<ArtifactCanonicalJsonTreeStep, ValueError>() {
        grant.maximum_copy_bytes = grant.maximum_copy_bytes.max(quote.copy_bytes);
        grant.maximum_depth = grant.maximum_depth.max(quote.depth);
    }
    let source = bank.source_witness().identity();
    let mut advance = bind_reply::<ArtifactCanonicalJsonTreeStep, ValueError, _>(|effect, reply| Ok(reply.complete(cursor.advance(bytes, effect))));
    effect_turn(bank, grant, |effect| drive_reply(effect, source, &mut advance).unwrap().result.unwrap())
}

fn admit_source<T: RetireOwned + Sync + 'static>(root: T, bank: &mut EffectBank) -> (RetainedCloneSource<T>, RetainedCloneProgress) {
    let mut admission = RetainedCloneSource::<T>::admission_owned(root, ());
    let mut source = None;
    let mut total = RetainedCloneProgress::default();
    for _ in 0..64 {
        if admission.terminal_is_empty() {
            break;
        }
        let turn = match admission.phase() {
            "projectionStart" => effect_turn(bank, grant(admission.owned_projection_demand().unwrap()), |effect| admission.project_owned(effect)),
            "ready" => effect_turn(bank, grant(admission.take_output_demand()), |effect| admission.take_output(&mut source, effect)),
            _ => effect_turn(bank, grant(admission.next_demand().unwrap()), |effect| admission.advance(effect)),
        };
        total = total.checked_add(turn.result.unwrap_or_else(|error| panic!("source admission: {error}")).progress()).unwrap();
    }
    assert!(admission.terminal_is_empty());
    (source.expect("source admitted"), total)
}

fn sealed<T: RetireOwned + Send + Sync + 'static>(value: T, bank: &mut EffectBank) -> SealedShared<T> {
    let mut input = Some(value);
    let mut output = None;
    effect_turn(bank, grant(SealedShared::<T>::source_admission_demand().unwrap()), |effect| {
        SealedShared::admit_from(&mut input, SharedIssuer::owned(), &mut output, effect)
    }).result.unwrap();
    output.expect("original value sealed")
}

fn rows(count: usize, prefix: &str, stride: i64, bank: &mut EffectBank) -> PersistentList<Row> {
    if count == 0 {
        return PersistentList::default();
    }
    let mut level = Vec::new();
    let page_grant = grant(PersistentList::<Row>::page_birth_demand());
    for start in (0..count).step_by(PERSISTENT_PAGE_FANOUT) {
        let length = (count - start).min(PERSISTENT_PAGE_FANOUT);
        let rows = std::array::from_fn(|slot| {
            (slot < length).then(|| {
                let rank = start + slot;
                sealed(Row { label: format!("{prefix}{rank}"), rank: rank as i64 * stride }, bank)
            })
        });
        level.push(effect_turn(bank, page_grant, |effect| PersistentList::admit_leaf(rows, length, effect)).result.ok().expect("original canonical leaf birth").0);
    }
    while level.len() > 1 {
        let mut input = level.into_iter();
        let mut next = Vec::new();
        loop {
            let children = std::array::from_fn(|_| input.next());
            if children[0].is_none() {
                break;
            }
            next.push(effect_turn(bank, page_grant, |effect| PersistentList::admit_branch(children, effect)).result.ok().expect("original canonical branch birth").0);
        }
        level = next;
    }
    level.pop().unwrap()
}

#[test]
fn original_persistent_canonical_tree_preserves_native_rows_and_exact_wire() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("🧫️fixtures/🔣️.json")).unwrap();
    let prefix = fixture["prefix"].as_str().unwrap();
    let stride = fixture["stride"].as_i64().unwrap();
    let mut bank = EffectBank::new(BankWallet::from_profile(&serde_json::json!({
        "items": 1u64 << 30,
        "copyBytes": 1u64 << 40,
        "capacityBytes": 1u64 << 40,
        "releaseBytes": 1u64 << 40,
        "depth": 1u64 << 20,
        "fuel": 1u64 << 60
    })));
    for count in fixture["lengths"].as_array().unwrap() {
        let count = count.as_u64().unwrap() as usize;
        let oracle = serde_json::to_vec(&(0..count).map(|i| serde_json::json!({"label": format!("{prefix}{i}"), "rank": i as i64 * stride})).collect::<Vec<_>>()).unwrap();
        let turn_limit = oracle.len().saturating_mul(8).max(128);
        for cut in std::iter::once(None).chain(fixture["cuts"].as_array().unwrap().iter().map(|cut| Some(cut.as_u64().unwrap() as usize))) {
            let (original, mut total) = observe(|| rows(count, prefix, stride, &mut bank));
            let (_, heap) = observe(|| {
                assert!(matches!(original.canonical_tree_node().unwrap(), Node::Array(n) if n == count));
                for index in 0..count {
                    assert_eq!(original.canonical_tree_child(index).unwrap() as *const dyn ArtifactCanonicalJsonTree as *const (), original.get(index).unwrap() as *const Row as *const ());
                }
                assert!(original.canonical_tree_child(count).is_err());
            });
            assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));

            let ((mut source, progress), heap) = observe(|| admit_source(original, &mut bank));
            assert_eq!((heap.requested_bytes, heap.released_bytes), (progress.retained_capacity_bytes, progress.released_bytes));
            total.requested_bytes += heap.requested_bytes;
            total.released_bytes += heap.released_bytes;

            let reference = source.try_borrow().unwrap().project(0, |root| root as &dyn ArtifactCanonicalJsonTree);
            let alias_grant = grant(retained_alias_demand(reference).unwrap());
            let (projection, heap) = effect_turn(&mut bank, alias_grant, |effect| observe(|| retained_alias(reference, effect).unwrap().unwrap().0));
            assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
            let ((mut cursor, _), heap) = observe(|| ArtifactCanonicalJsonTreeCursor::admit(projection, grant(ArtifactCanonicalJsonTreeCursor::constructor_demand())).unwrap_or_else(|_| panic!("original canonical cursor")));
            assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));

            let mut output = Vec::new();
            let mut turns = 0;
            while !cursor.terminal_is_empty() {
                if cut == Some(turns) {
                    cursor.begin_close();
                }
                let (demand, heap) = observe(|| cursor.next_demand().unwrap());
                assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
                let grant = grant(demand);
                let mut bytes = [0; 512];
                let (step, heap) = observe(|| advance_tree(&mut bank, &mut cursor, &mut bytes, grant));
                let progress = step.ownership.progress();
                assert!(progress.fits(grant));
                assert_eq!((heap.requested_bytes, heap.released_bytes), (progress.retained_capacity_bytes, progress.released_bytes));
                total.requested_bytes += heap.requested_bytes;
                total.released_bytes += heap.released_bytes;
                output.extend_from_slice(&bytes[..step.written_bytes]);
                turns += 1;
                assert!(turns < turn_limit, "native tree stalled");
            }
            if cut.is_none() {
                assert!(cursor.is_complete());
                assert_eq!(output, oracle);
            } else {
                assert_eq!(output, oracle[..output.len()]);
            }
            let (_, heap) = observe(|| drop(cursor));
            assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));

            for turn in 0..turn_limit {
                if source.terminal_is_empty() {
                    break;
                }
                let grant = grant(source.next_close_demand(4096).unwrap());
                let (step, heap) = observe(|| {
                    let (poll, _) = driven(&mut bank, grant, 1 << 20, |effect, reply, recover| {
                        if recover { source.recover_close_into(4096, effect, reply) } else { source.close_into(4096, effect, reply) }
                    });
                    settled(poll).unwrap().0
                });
                assert!(step.progress().fits(grant));
                assert_eq!((heap.requested_bytes, heap.released_bytes), (step.progress().retained_capacity_bytes, step.progress().released_bytes));
                total.requested_bytes += heap.requested_bytes;
                total.released_bytes += heap.released_bytes;
                assert!(turn + 1 < turn_limit || source.terminal_is_empty(), "original source retirement stalled");
            }
            let (_, heap) = observe(|| drop(source));
            assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
            assert_eq!(total.requested_bytes, total.released_bytes);
        }
    }

    let (text, mut total) = observe(|| sealed(String::from(prefix), &mut bank));
    let (_, heap) = observe(|| {
        assert!(matches!(text.canonical_tree_node().unwrap(), Node::String(value) if value.as_ptr() == text.get().as_ptr()));
        assert!(text.canonical_tree_child(0).is_err());
        assert!(text.canonical_tree_key(0).is_err());
    });
    assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
    let mut owner = ControlledRetirement::new(text).unwrap_or_else(|_| panic!("original text close"));
    for turn in 0..100_000 {
        if owner.terminal_is_empty() {
            break;
        }
        let grant = grant(owner.next_demand(4096).unwrap());
        let (step, heap) = observe(|| {
            let (poll, _) = driven(&mut bank, grant, 1 << 20, |effect, reply, recover| {
                if recover { owner.recover_step_into(4096, effect, reply) } else { owner.step_into(4096, effect, reply) }
            });
            settled(poll).unwrap().0
        });
        assert_eq!((heap.requested_bytes, heap.released_bytes), (step.progress().retained_capacity_bytes, step.progress().released_bytes));
        total.requested_bytes += heap.requested_bytes;
        total.released_bytes += heap.released_bytes;
        assert!(turn < 99_999, "original text retirement stalled");
    }
    assert_eq!(total.requested_bytes, total.released_bytes);
}
