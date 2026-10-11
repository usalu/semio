//! 🧪️ Randomized sequence laws for the positional list delta algebra: for every random insert/remove/move/patch sequence over a
//! nested list, `absorb` of the step deltas equals sequential application, the summed delta's `inverse` restores the base, every
//! step's `inverse` restores its pre-state, and the reversed step inverses sum to the negative; a second randomized law runs the same
//! sequences over fail-closed rows and proves every displaced or abandoned row went through the cold-disposal hook.

use crate::__value_derive::{FromValue, ToValue};
use crate::list_delta::{ItemList, KeyOf, Parts, RowPatch};
use crate::{list_delta, row_patch, ApplyCapability};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_value::RetireOwned, crate::__dsl_record_derive::DslRecord)]
#[value(rename_all = "camelCase")]
struct Part {
    id: String,
    weight: i64,
}

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_value::RetireOwned, crate::__dsl_record_derive::DslRecord)]
#[value(rename_all = "camelCase")]
struct Item {
    id: String,
    value: i64,
    label: String,
    parts: Vec<Part>,
}

row_patch! { PartPatch of Part { set { weight: i64 } } }
list_delta! { PartDelta { removal: PartRemoval, insertion: PartInsertion, relocation: PartRelocation, modification: PartModification, row: Part, patch: PartPatch, key: id } }
row_patch! { ItemPatch of Item { set { value: i64, label: String } nest { parts: PartDelta } } }
list_delta! { ItemDelta { removal: ItemRemoval, insertion: ItemInsertion, relocation: ItemRelocation, modification: ItemModification, row: Item, patch: ItemPatch, key: id } }

thread_local! {
    static CAPABILITY: std::cell::Cell<Option<ApplyCapability>> = const { std::cell::Cell::new(None) };
}

fn capability() -> ApplyCapability {
    if CAPABILITY.with(|cell| cell.get()).is_none() {
        crate::apply_diff(&ItemDelta::default(), &Vec::new()).expect("the empty delta applies");
    }
    CAPABILITY.with(|cell| cell.get()).expect("the central applier handed the delta its capability")
}

impl crate::DiffAlgebra<Vec<Item>> for ItemDelta {
    fn inverse(&self, base: &Vec<Item>) -> Self {
        ItemDelta::inverse(self, base)
    }
    fn is_empty(&self) -> bool {
        ItemDelta::is_empty(self)
    }
}

impl crate::MutationDiff<Vec<Item>> for ItemDelta {
    fn apply(&self, base: &Vec<Item>, capability: ApplyCapability) -> crate::MutationApplyResult<Vec<Item>> {
        CAPABILITY.with(|cell| cell.set(Some(capability)));
        self.commit_onto(base, capability)
    }
    fn absorb(&mut self, other: Self) {
        ItemDelta::absorb(self, other)
    }
}

fn commit(delta: &ItemDelta, base: &[Item]) -> crate::MutationApplyResult<Vec<Item>> {
    crate::apply_diff(delta, &base.to_vec())
}

struct Rng(u64);

impl Rng {
    fn next(&mut self) -> u64 {
        self.0 ^= self.0 << 13;
        self.0 ^= self.0 >> 7;
        self.0 ^= self.0 << 17;
        self.0
    }
    fn below(&mut self, bound: usize) -> usize {
        (self.next() % bound as u64) as usize
    }
}

fn item(id: &str, value: i64) -> Item {
    Item { id: id.into(), value, label: format!("label-{id}"), parts: Vec::new() }
}

fn random_step(rng: &mut Rng, state: &[Item], fresh: &mut usize, graveyard: &mut Vec<String>) -> Option<ItemDelta> {
    match rng.below(9) {
        0 | 1 => {
            let index = rng.below(state.len() + 1);
            let id = if !graveyard.is_empty() && rng.below(3) == 0 { graveyard.swap_remove(rng.below(graveyard.len())) } else { *fresh += 1; format!("k{fresh}") };
            if state.iter().any(|row| row.id == id) {
                return None;
            }
            Some(ItemDelta::insertion(index, item(&id, rng.below(50) as i64)))
        }
        2 | 3 => {
            let index = rng.below(state.len().max(1));
            let row = state.get(index)?;
            graveyard.push(row.id.clone());
            Some(ItemDelta::removal(state, index))
        }
        4 => {
            let from = rng.below(state.len().max(1));
            state.get(from)?;
            Some(ItemDelta::relocation(state, from, rng.below(state.len())))
        }
        5 => {
            let row = state.get(rng.below(state.len().max(1)))?;
            Some(ItemDelta::modification(row.id.as_str(), ItemPatch { value: Some(rng.below(50) as i64), ..ItemPatch::default() }))
        }
        6 => {
            let row = state.get(rng.below(state.len().max(1)))?;
            let index = rng.below(row.parts.len() + 1);
            *fresh += 1;
            let part = Part { id: format!("p{fresh}"), weight: rng.below(9) as i64 };
            Some(ItemDelta::modification(row.id.as_str(), ItemPatch { parts: PartDelta::insertion(index, part), ..ItemPatch::default() }))
        }
        7 => {
            let row = state.get(rng.below(state.len().max(1)))?;
            let index = rng.below(row.parts.len().max(1));
            row.parts.get(index)?;
            Some(ItemDelta::modification(row.id.as_str(), ItemPatch { parts: PartDelta::removal(&row.parts, index), ..ItemPatch::default() }))
        }
        _ => {
            let row = state.get(rng.below(state.len().max(1)))?;
            let from = rng.below(row.parts.len().max(1));
            row.parts.get(from)?;
            let patch = if rng.below(2) == 0 {
                ItemPatch { parts: PartDelta::relocation(&row.parts, from, rng.below(row.parts.len())), ..ItemPatch::default() }
            } else {
                ItemPatch { parts: PartDelta::modification(row.parts[from].id.as_str(), PartPatch { weight: Some(rng.below(9) as i64) }), ..ItemPatch::default() }
            };
            Some(ItemDelta::modification(row.id.as_str(), patch))
        }
    }
}

#[test]
fn random_edit_sequences_obey_the_delta_laws() {
    let mut rng = Rng(0x9E37_79B9_7F4A_7C15);
    for case in 0..6000 {
        let mut fresh = 0usize;
        let mut graveyard = Vec::new();
        let base: Vec<Item> = (0..rng.below(6)).map(|index| item(&format!("b{index}"), index as i64)).collect();
        let mut state = base.clone();
        let mut absorbed = ItemDelta::default();
        let mut steps = Vec::new();
        for _ in 0..rng.below(10) + 1 {
            let Some(step) = random_step(&mut rng, &state, &mut fresh, &mut graveyard) else { continue };
            let next = commit(&step, &state).unwrap_or_else(|error| panic!("case {case}: step {step:?} must apply to {state:?}: {error}"));
            assert_eq!(commit(&step.inverse(&state), &next).as_ref(), Ok(&state), "case {case}: the step inverse restores its pre-state: {step:?}");
            absorbed.absorb(step.clone());
            steps.push((state.clone(), step));
            state = next;
        }
        assert_eq!(commit(&absorbed, &base).as_ref(), Ok(&state), "case {case}: absorb of {steps:?} equals sequential application, absorbed {absorbed:?}");
        assert_eq!(commit(&absorbed.inverse(&base), &state).as_ref(), Ok(&base), "case {case}: the absorbed delta's inverse restores the base");
        let mut reverse_sum = ItemDelta::default();
        for (pre, step) in steps.iter().rev() {
            reverse_sum.absorb(step.inverse(pre));
        }
        assert_eq!(commit(&reverse_sum, &state).as_ref(), Ok(&base), "case {case}: the reversed step inverses sum to the negative");
    }
}

#[test]
fn absorb_coalesces_per_id() {
    let base = vec![item("a", 1), item("b", 2), item("c", 3)];
    let created = ItemDelta::insertion(1, item("n", 5));
    let mut cancelled = created.clone();
    cancelled.absorb(ItemDelta::removal(&commit(&created, &base).expect("create applies"), 1));
    assert!(cancelled.is_empty(), "insert∘remove is nothing: {cancelled:?}");
    let mut folded = created.clone();
    folded.absorb(ItemDelta::modification("n", ItemPatch { value: Some(9), ..ItemPatch::default() }));
    assert_eq!(commit(&folded, &base).expect("insert∘patch applies")[1].value, 9, "insert∘patch applies as one changed insert");
    let mut patched = ItemDelta::modification("a", ItemPatch { value: Some(7), ..ItemPatch::default() });
    patched.absorb(ItemDelta::modification("a", ItemPatch { label: Some("x".into()), ..ItemPatch::default() }));
    assert_eq!(patched.modified.len(), 1, "patch∘patch is one patch");
    let mut moved = ItemDelta::relocation(&base, 0, 1);
    let mid = commit(&moved, &base).expect("move applies");
    moved.absorb(ItemDelta::relocation(&mid, 1, 2));
    assert_eq!(moved.moved.len(), 1, "move∘move is one move");
    assert_eq!(commit(&moved, &base).expect("one move applies"), vec![item("b", 2), item("c", 3), item("a", 1)]);
    let mut replaced = ItemDelta::removal(&base, 1);
    replaced.absorb(ItemDelta::insertion(1, item("b", 8)));
    assert_eq!((replaced.removed.len(), replaced.inserted.len()), (1, 1), "remove∘insert is a replacement");
    assert_eq!(commit(&replaced, &base).expect("replacement applies"), vec![item("a", 1), item("b", 8), item("c", 3)]);
    let mut shifted = ItemDelta::removal(&base, 0);
    shifted.absorb(ItemDelta::removal(&base[1..], 0));
    assert_eq!(shifted.removed, vec![ItemRemoval { id: "a".into(), index: 0 }, ItemRemoval { id: "b".into(), index: 1 }], "the second removal reads the base index");
}

#[test]
fn deltas_refuse_what_they_cannot_apply() {
    let base = vec![item("a", 1)];
    assert!(commit(&ItemDelta::removal_by_id("zz", 0), &base).is_err());
    assert!(commit(&ItemDelta::modification("zz", ItemPatch::default()), &base).is_err());
    assert!(commit(&ItemDelta::insertion(0, item("a", 2)), &base).is_err());
    assert!(commit(&ItemDelta::insertion(5, item("n", 1)), &base).is_err());
    assert_eq!(ItemDelta::removal(&base, 3), ItemDelta::default(), "a removal past the end is empty");
    let _ = Parts::<Item, ItemPatch>::default().is_empty();
    assert_eq!(ItemList::<Item>::count(&base), 1);
}

// 🧊️ Fail-closed rows: a row that aborts on a bare drop. The algebra must hand every row it displaces or abandons to the cold-disposal
// hook of its key marker; a row cloned inside the algebra and dropped any other way counts as a bare drop.
thread_local! {
    static BARE_DROPS: std::cell::Cell<usize> = const { std::cell::Cell::new(0) };
    static RETIRED: std::cell::Cell<usize> = const { std::cell::Cell::new(0) };
}

#[derive(Debug)]
struct Fragile {
    id: String,
    value: i64,
    blessed: bool,
}

impl Clone for Fragile {
    fn clone(&self) -> Self {
        Self { id: self.id.clone(), value: self.value, blessed: false }
    }
}

impl PartialEq for Fragile {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self.value == other.value
    }
}

impl Drop for Fragile {
    fn drop(&mut self) {
        if !self.blessed {
            BARE_DROPS.with(|cell| cell.set(cell.get() + 1));
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
struct FragileKeys;

impl KeyOf<Fragile> for FragileKeys {
    type Key = String;
    fn key_of(row: &Fragile) -> String {
        row.id.clone()
    }
    fn retire_cold(mut row: Fragile) {
        row.blessed = true;
        RETIRED.with(|cell| cell.set(cell.get() + 1));
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
struct FragilePatch {
    value: Option<i64>,
}

impl RowPatch<Fragile> for FragilePatch {
    fn commit_into(&self, row: &mut Fragile, _capability: ApplyCapability) -> crate::MutationApplyResult<()> {
        if let Some(value) = self.value {
            row.value = value;
        }
        Ok(())
    }
    fn absorb(&mut self, later: Self) {
        self.value = later.value.or(self.value);
    }
    fn inverse(&self, row: &Fragile) -> Self {
        Self { value: self.value.map(|_| row.value) }
    }
    fn is_empty(&self) -> bool {
        self.value.is_none()
    }
}

type FragileParts = Parts<Fragile, FragilePatch, FragileKeys>;

fn fragile(id: &str, value: i64) -> Fragile {
    Fragile { id: id.into(), value, blessed: true }
}

fn bless_rows(rows: &mut [Fragile]) {
    rows.iter_mut().for_each(|row| row.blessed = true);
}

fn bless_parts(parts: &mut FragileParts) {
    parts.inserted.iter_mut().for_each(|(_, row)| row.blessed = true);
}

fn cloned_blessed(rows: &[Fragile]) -> Vec<Fragile> {
    let mut copy = rows.to_vec();
    bless_rows(&mut copy);
    copy
}

fn random_fragile_step(rng: &mut Rng, state: &[Fragile], fresh: &mut usize) -> Option<FragileParts> {
    match rng.below(6) {
        0 | 1 => {
            *fresh += 1;
            Some(FragileParts::insertion(rng.below(state.len() + 1), fragile(&format!("k{fresh}"), rng.below(50) as i64)))
        }
        2 | 3 => {
            let index = rng.below(state.len().max(1));
            state.get(index)?;
            Some(FragileParts::removal(state, index))
        }
        4 => {
            let from = rng.below(state.len().max(1));
            state.get(from)?;
            Some(FragileParts::relocation(state, from, rng.below(state.len())))
        }
        _ => {
            let row = state.get(rng.below(state.len().max(1)))?;
            Some(FragileParts::modification(row.id.clone(), FragilePatch { value: Some(rng.below(50) as i64) }))
        }
    }
}

#[test]
fn fail_closed_rows_are_retired_never_bare_dropped() {
    let capability = capability();
    let mut rng = Rng(0xD1B5_4A32_D192_ED03);
    for case in 0..6000 {
        let mut fresh = 0usize;
        let base: Vec<Fragile> = (0..rng.below(6)).map(|index| fragile(&format!("b{index}"), index as i64)).collect();
        let mut state = cloned_blessed(&base);
        let mut absorbed = FragileParts::default();
        let mut steps = Vec::new();
        for _ in 0..rng.below(10) + 1 {
            let Some(step) = random_fragile_step(&mut rng, &state, &mut fresh) else { continue };
            let mut next = step.commit_onto(&state, capability).unwrap_or_else(|error| panic!("case {case}: step must apply: {error}"));
            bless_rows(&mut next);
            let mut inverse = step.inverse(&state);
            bless_parts(&mut inverse);
            let mut restored = inverse.commit_onto(&next, capability).expect("the step inverse applies");
            bless_rows(&mut restored);
            assert_eq!(restored, state, "case {case}: the step inverse restores its pre-state");
            inverse.retire_cold();
            absorbed.absorb(step.clone());
            bless_parts(&mut absorbed);
            steps.push((cloned_blessed(&state), step));
            state = next;
        }
        let mut replayed = absorbed.commit_onto(&base, capability).expect("the absorbed delta applies");
        bless_rows(&mut replayed);
        assert_eq!(replayed, state, "case {case}: absorb equals sequential application");
        assert!(FragileParts::removal_by_id("never-existed".into(), 0).commit_onto(&base, capability).is_err());
        if !base.is_empty() {
            assert!(FragileParts::insertion(0, fragile("b0", 1)).commit_onto(&base, capability).is_err(), "case {case}: a duplicate key refuses");
        }
        assert_eq!(BARE_DROPS.with(|cell| cell.get()), 0, "case {case}: a displaced or abandoned row was dropped bare");
        absorbed.retire_cold();
        steps.into_iter().for_each(|(_, step)| step.retire_cold());
    }
    assert!(RETIRED.with(|cell| cell.get()) > 0, "the cold-disposal hook ran");
    assert_eq!(BARE_DROPS.with(|cell| cell.get()), 0);
}
