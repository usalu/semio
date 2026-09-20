
use super::*;

fn index_fixture() -> serde_json::Value {
    serde_json::from_str(include_str!("../../../../🧫️fixtures/♻️shell-document-retirement-index/🔣️.json")).expect("shell document retirement index fixture")
}

fn document(generation: u64, surface: &str) -> UiDocumentLease {
    let id = ui_contract::UiNodeId(1);
    let surface = SurfaceId::try_from(surface).expect("bounded shell retirement surface");
    let mut builder = ui_contract::UiDocumentBuilder::try_new(generation, surface, ui_contract::UiRevision(generation), Some(id), generation).expect("shell retirement document slot");
    builder
        .try_push(ui_contract::UiNodeRecord {
            id,
            key: UiText::try_from_str("root").expect("bounded shell retirement key"),
            component: ui_contract::Component::Separator(ui_contract::SeparatorProps {}),
            layout: Default::default(),
            style: Default::default(),
            activity: Default::default(),
            disabled: false,
            transition: None,
            accessibility: Default::default(),
            bindings: Default::default(),
            menu: None,
            children: Default::default(),
        })
        .expect("shell retirement root");
    builder.finish().expect("shell retirement document")
}

#[test]
fn shell_ninth_document_and_nonterminal_first_close_remain_in_qualified_retirement() {
    let first = document(90_001, "shell.retirement.first");
    let mut owners = UiFixedList::<UiDocumentLease, 9>::default();
    for _ in 0..7 {
        owners.try_push(first.try_alias().expect("fixed shell document alias")).expect("shell alias owner");
    }
    owners.try_push(first).expect("shell primary owner");
    owners.try_push(document(90_002, "shell.retirement.ninth")).expect("shell ninth owner");
    let mut registry = ShellDocumentRetirementRegistry::default();
    for owner in owners {
        registry.try_admit(owner).expect("absolute lease credits retain the ninth owner");
    }
    assert_eq!(registry.qualified_owners().count(), 9);
    assert!(registry.close_one());
    assert_eq!(registry.qualified_owners().count(), 9, "a false-returning first close opportunity cannot remove its exact owner");
    let mut opportunities = 1;
    while !registry.terminal_is_empty() {
        let _ = registry.close_one();
        opportunities += 1;
        assert!(opportunities < 8_192, "fixed shell document retirement must converge");
    }
}

#[test]
fn shell_absolute_refusal_returns_the_exact_max_plus_one_owner_before_mutation() {
    let mut owners = UiFixedList::<UiDocumentLease, SHELL_DOCUMENT_RETIREMENT_CAPACITY>::default();
    for ordinal in 0..UI_DOCUMENT_LEASE_SLOTS {
        let primary = document(90_100 + ordinal as u64, &format!("shell.retirement.absolute-{ordinal}"));
        for _ in 1..UI_DOCUMENT_LEASE_ALIASES {
            owners.try_push(primary.try_alias().expect("absolute fixed shell alias")).expect("absolute shell alias owner");
        }
        owners.try_push(primary).expect("absolute shell primary owner");
    }
    let expected = owners.pop().expect("absolute shell refusal owner");
    let generation = expected.generation();
    let mut registry = ShellDocumentRetirementRegistry::default();
    for owner in owners {
        registry.try_admit(owner).expect("absolute shell retirement owner");
    }
    let vacancy = registry.slots.iter().position(Option::is_none).expect("one exact refusal vacancy");
    registry.epochs[vacancy] = u64::MAX;
    assert!(registry.try_reserve_admission().is_none(), "MAX+1 preflight refuses before any owner moves");
    let refused = registry.try_admit(expected).expect_err("exhausted vacancy refuses before transfer");
    assert_eq!(refused.generation(), generation);
    assert_eq!(registry.qualified_owners().count(), SHELL_DOCUMENT_RETIREMENT_CAPACITY - 1);
    registry.epochs[vacancy] = 0;
    let admission = registry.try_reserve_admission().expect("exact refused owner admission retries");
    registry.admit_reserved(admission, refused);
    while !registry.terminal_is_empty() {
        let _ = registry.close_one();
    }
}

#[test]
fn sparse_occupied_index_preserves_cursor_vacancy_epoch_and_terminal_ownership() {
    let fixture = index_fixture();
    assert_eq!(SHELL_DOCUMENT_RETIREMENT_CAPACITY, fixture["capacity"].as_u64().unwrap() as usize);
    assert_eq!(SHELL_DOCUMENT_RETIREMENT_WORD_BITS, fixture["wordBits"].as_u64().unwrap() as usize);
    assert_eq!(SHELL_DOCUMENT_RETIREMENT_WORDS, fixture["wordCount"].as_u64().unwrap() as usize);
    assert_eq!(SHELL_DOCUMENT_RETIREMENT_INDEX_BYTES, fixture["indexBytes"].as_u64().unwrap() as usize);

    let mut registry = ShellDocumentRetirementRegistry::default();
    for (ordinal, index) in fixture["initial"]["occupied"].as_array().expect("initial occupied slots").iter().enumerate() {
        let index = index.as_u64().unwrap() as usize;
        let document = document(91_000 + ordinal as u64, &format!("shell.retirement.sparse-{index}"));
        let generation = document.generation();
        let surface = document.header().ok().map(|header| header.surface);
        registry.epochs[index] = 1;
        registry.slots[index] = Some(ShellDocumentRetirementSlot { epoch: 1, generation, surface, document });
        registry.mark_occupied(index);
    }
    registry.cursor = fixture["initial"]["cursor"].as_u64().unwrap() as usize;
    let mut admitted_generation = 92_000_u64;

    for step in fixture["steps"].as_array().expect("retirement steps") {
        let expected_index = step["slot"].as_u64().unwrap() as usize;
        match step["op"].as_str().unwrap() {
            "closePage" => {
                assert_eq!(registry.next_occupied_index(), Some(expected_index), "the occupied-word selector preserves physical cursor fairness");
                registry.cursor = (expected_index + 1) % SHELL_DOCUMENT_RETIREMENT_CAPACITY;
                if step["terminal"].as_bool().unwrap() {
                    let mut terminal = registry.slots[expected_index].take().expect("fixture terminal owner remains exact");
                    while !terminal.document.terminal_is_empty() {
                        terminal.document.close_step_with_grant(SHELL_DOCUMENT_RETIREMENT_ITEMS, SHELL_DOCUMENT_RETIREMENT_BYTES).expect("fixture terminal close remains valid");
                    }
                    registry.clear_occupied(expected_index);
                }
                assert_eq!(registry.cursor, step["nextCursor"].as_u64().unwrap() as usize);
            }
            "admit" => {
                let owner = document(admitted_generation, &format!("shell.retirement.reuse-{expected_index}"));
                admitted_generation += 1;
                registry.try_admit(owner).expect("fixture vacancy admits the exact owner");
                let admitted = registry.slots[expected_index].as_ref().expect("lowest vacancy owns the admitted document");
                assert_eq!(admitted.epoch, step["epoch"].as_u64().unwrap());
            }
            operation => panic!("unknown retirement fixture operation {operation}"),
        }
        assert_eq!(registry.occupied_count, step["occupiedCount"].as_u64().unwrap());
        assert_eq!(registry.occupied_count as usize, registry.slots.iter().flatten().count(), "bitmap count and exact owner slots remain one authority");
        for index in 0..SHELL_DOCUMENT_RETIREMENT_CAPACITY {
            let word = index / SHELL_DOCUMENT_RETIREMENT_WORD_BITS;
            let bit = 1_u64 << (index % SHELL_DOCUMENT_RETIREMENT_WORD_BITS);
            assert_eq!(registry.occupied[word] & bit != 0, registry.slots[index].is_some(), "slot {index} bitmap membership matches owner presence");
        }
    }
    assert!(registry.terminal_is_empty());
    assert!(registry.occupied.iter().all(|word| *word == 0));
}

#[test]
fn actual_sparse_close_walks_one_owner_page_per_cursor_step_and_converges() {
    let mut registry = ShellDocumentRetirementRegistry::default();
    for ordinal in 0..3 {
        registry.try_admit(document(93_000 + ordinal, &format!("shell.retirement.actual-{ordinal}"))).expect("sparse actual owner");
    }
    for (from, to) in [(0, 130), (2, 511)] {
        let owner = registry.slots[from].take().expect("relocated sparse owner");
        registry.clear_occupied(from);
        registry.epochs[to] = registry.epochs[from];
        registry.epochs[from] = 0;
        registry.slots[to] = Some(owner);
        registry.mark_occupied(to);
    }
    registry.cursor = 129;
    assert_eq!(registry.next_occupied_index(), Some(130));
    assert!(registry.close_one());
    assert_eq!(registry.cursor, 131);
    assert_eq!(registry.next_occupied_index(), Some(511));
    assert!(registry.close_one());
    assert_eq!(registry.cursor, 0);
    assert_eq!(registry.next_occupied_index(), Some(1));
    assert!(registry.close_one());
    assert_eq!(registry.cursor, 2);

    let mut opportunities = 3;
    while !registry.terminal_is_empty() {
        assert!(registry.close_one(), "a nonempty occupied index always advances one exact owner");
        opportunities += 1;
        assert!(opportunities < 8_192, "sparse indexed retirement converges without losing an owner");
    }
    assert_eq!(registry.occupied_count, 0);
    assert!(registry.slots.iter().all(Option::is_none));
    assert!(registry.occupied.iter().all(|word| *word == 0));
}
