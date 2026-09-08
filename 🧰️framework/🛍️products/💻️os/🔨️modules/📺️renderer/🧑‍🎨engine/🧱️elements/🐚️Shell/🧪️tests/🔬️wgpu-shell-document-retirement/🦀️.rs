
use super::*;

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
    let refused = registry.try_admit(expected).expect_err("exhausted vacancy refuses before transfer");
    assert_eq!(refused.generation(), generation);
    assert_eq!(registry.qualified_owners().count(), SHELL_DOCUMENT_RETIREMENT_CAPACITY - 1);
    registry.epochs[vacancy] = 0;
    registry.try_admit(refused).expect("exact refused owner retries without reconstruction");
    while !registry.terminal_is_empty() {
        let _ = registry.close_one();
    }
}
