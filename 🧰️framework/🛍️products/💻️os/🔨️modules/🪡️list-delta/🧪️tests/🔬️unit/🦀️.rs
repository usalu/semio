//! 🧪️ Randomized sequence laws for the positional list delta algebra: for every random insert/remove/move/patch sequence over a
//! nested list, `absorb` of the step deltas equals sequential application, the summed delta's `inverse` restores the base, every
//! step's `inverse` restores its pre-state, and the reversed step inverses sum to the negative; a second randomized law runs the same
//! sequences over fail-closed rows and proves every displaced or abandoned row went through the cold-disposal hook.

use semio_framework_mutation_diff::ColdDiffAlgebra;
use crate::__value_derive::{FromValue, ToValue};
use crate::{ItemList, KeyOf, Parts, RowPatch};
use crate::{list_delta, row_patch};
use semio_framework_mutation_diff::ApplyCapability;

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
        semio_framework_mutation_diff::apply_diff_cold(&ItemDelta::default(), &Vec::new()).expect("the empty delta applies");
    }
    CAPABILITY.with(|cell| cell.get()).expect("the central applier handed the delta its capability")
}

impl semio_framework_mutation_diff::DiffAlgebra<Vec<Item>> for ItemDelta {

    fn is_empty(&self) -> bool {
        ItemDelta::is_empty(self)
    }
}

impl ColdDiffAlgebra<Vec<Item>> for ItemDelta {
    fn inverse(&self, base: &Vec<Item>) -> Self {
        ItemDelta::inverse(self, base)
    }
}

impl semio_framework_mutation_diff::MutationDiff<Vec<Item>> for ItemDelta {
    fn apply(&self, base: &mut Vec<Item>, capability: ApplyCapability) -> semio_framework_mutation_diff::MutationApplyResult<()> {
        CAPABILITY.with(|cell| cell.set(Some(capability)));

        let next = self.commit_onto(base, capability)
    ?;
        ItemDelta::retire_list(std::mem::replace(base, next));
        Ok(())
    }
    fn absorb(&mut self, other: Self) {
        ItemDelta::absorb(self, other)
    }

    fn apply(&self, base: &mut Vec<Item>, capability: ApplyCapability) -> semio_framework_mutation_diff::diff::MutationApplyResult<()> {
        CAPABILITY.with(|cell| cell.set(Some(capability)));
        let next = self.commit_onto(base, capability)?;
        ItemDelta::retire_list(std::mem::replace(base, next));
        Ok(())
    }
}

fn commit(delta: &ItemDelta, base: &[Item]) -> semio_framework_mutation_diff::MutationApplyResult<Vec<Item>> {
    semio_framework_mutation_diff::apply_diff_cold(delta, &base.to_vec())
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
fn commuting_rows_absorb_to_one_structurally_equal_delta() {
    let base = vec![item("a", 1), item("b", 2), item("c", 3), item("d", 4)];
    let (a_then_c, c_then_a) = {
        let mut first = ItemDelta::removal(&base, 0);
        first.absorb(ItemDelta::removal(&base[1..], 1));
        let mut second = ItemDelta::removal(&base, 2);
        second.absorb(ItemDelta::removal(&vec![base[0].clone(), base[1].clone(), base[3].clone()], 0));
        (first, second)
    };
    assert_eq!(a_then_c, c_then_a, "two removals commute: one canonical ascending-coordinate delta");
    assert_eq!(a_then_c.removed, vec![ItemRemoval { id: "a".into(), index: 0 }, ItemRemoval { id: "c".into(), index: 2 }]);
    let mut inserts = ItemDelta::insertion(0, item("x", 1));
    inserts.absorb(ItemDelta::insertion(3, item("y", 2)));
    let mut reversed = ItemDelta::insertion(2, item("y", 2));
    reversed.absorb(ItemDelta::insertion(0, item("x", 1)));
    assert_eq!(inserts, reversed, "two insertions at distinct final slots commute");
    assert_eq!(inserts.inserted.iter().map(|row| row.index).collect::<Vec<_>>(), vec![0, 3]);
    let mut first_oracle=base.clone();first_oracle.insert(0,item("x",1));first_oracle.insert(3,item("y",2));
    let mut reverse_oracle=base.clone();reverse_oracle.insert(2,item("y",2));reverse_oracle.insert(0,item("x",1));
    assert_eq!(first_oracle,reverse_oracle,"independent Vec insertion routes reach identical final rows");
    assert_eq!(commit(&inserts,&base).expect("composed inserts apply"),first_oracle);
    assert_eq!(commit(&reversed,&base).expect("reverse composed inserts apply"),reverse_oracle);
    let swap = ItemDelta::relocation(&base, 0, 1);
    let mut both = swap.clone();
    both.absorb(ItemDelta::relocation(&commit(&swap, &base).expect("move applies"), 0, 1));
    let inverse = both.inverse(&base);
    let mut summed = ItemDelta::default();
    summed.absorb(inverse.clone());
    assert_eq!(summed, inverse, "absorbing into the empty delta keeps the canonical form");
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

#[derive(Clone, Debug, PartialEq, ToValue, FromValue)]
struct IdentityRow {
    id: String,
    value: i64,
}

impl crate::Keyed for IdentityRow {
    type Key = String;
    fn key(&self) -> String { self.id.clone() }
}

impl RowPatch<IdentityRow> for IdentityRow {
    fn commit_into(&self, row: &mut IdentityRow, _: ApplyCapability) -> semio_framework_mutation_diff::diff::MutationApplyResult<()> { *row = self.clone(); Ok(()) }
    fn absorb(&mut self, later: Self) { *self = later; }
    fn inverse(&self, row: &IdentityRow) -> Self { row.clone() }
    fn is_empty(&self) -> bool { false }
}

#[derive(FromValue)]
struct IdentityPatch { target: String, row: IdentityRow }

#[derive(FromValue)]
struct IdentityVector { id: String, base: Vec<IdentityRow>, patches: Vec<IdentityPatch>, accepted: bool, expected: Vec<IdentityRow> }

#[derive(FromValue)]
struct IdentityVectors { vectors: Vec<IdentityVector> }

/// 🪪️ Whole-row replay uses immutable identities and refuses the entire diff before publishing a partial projection.
#[test]
fn stable_identity_replay_matches_portable_vectors() {
    let fixtures: IdentityVectors = semio_framework_pack_json::from_json_str(include_str!("../🧫️fixtures/🪪️stable-identity/🔣️.json"), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("portable identity vectors");
    for vector in fixtures.vectors {
        let original = vector.base.clone();
        let delta = Parts::<IdentityRow, IdentityRow> { modified: vector.patches.into_iter().map(|patch| (patch.target, patch.row)).collect(), ..Default::default() };
        let result = delta.commit_onto(&vector.base, capability());
        assert_eq!(result.is_ok(), vector.accepted, "{}", vector.id);
        let actual = result.as_ref().unwrap_or(&vector.base);
        assert_eq!(actual, &vector.expected, "{}", vector.id);
        assert_eq!(vector.base, original, "{} preserves its input", vector.id);
        if let Ok(next) = result {
            let restored = delta.inverse(&vector.base).commit_onto(&next, capability()).expect("inverse replay");
            assert_eq!(restored, vector.base, "{} exact inverse", vector.id);
        }
    }
}

#[derive(FromValue)]
struct NestedInverseVector { id: String, base: Vec<Item>, patches: Vec<ItemModification>, expected: Vec<Item> }

#[derive(FromValue)]
struct NestedInverseVectors { vectors: Vec<NestedInverseVector> }

/// 🪆️ Ordered patches of one nested row are composed before their inverse reads original positions.
#[test]
fn nested_inverse_replay_matches_portable_vectors() {
    let fixtures: NestedInverseVectors = semio_framework_pack_json::from_json_str(include_str!("../🧫️fixtures/🪆️nested-inverse/🔣️.json"), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("portable nested inverse vectors");
    for vector in fixtures.vectors {
        let delta = ItemDelta { modified: vector.patches, ..Default::default() };
        let next = commit(&delta, &vector.base).expect("ordered nested patches apply");
        assert_eq!(next, vector.expected, "{} forward projection", vector.id);
        assert_eq!(commit(&delta.inverse(&vector.base), &next).as_ref(), Ok(&vector.base), "{} exact inverse", vector.id);
        let mut sum = ItemDelta::default();
        for patch in &delta.modified {
            sum.absorb(ItemDelta::modification(patch.id.clone(), patch.patch.clone()));
        }
        assert_eq!(commit(&sum, &vector.base).as_ref(), Ok(&next), "{} composition", vector.id);
        assert_eq!(commit(&sum.inverse(&vector.base), &next).as_ref(), Ok(&vector.base), "{} composed inverse", vector.id);
    }
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
    fn commit_into(&self, row: &mut Fragile, _capability: ApplyCapability) -> semio_framework_mutation_diff::MutationApplyResult<()> {
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

#[derive(Clone, Debug, PartialEq)]
struct FragileIdentityPatch(String);

impl RowPatch<Fragile> for FragileIdentityPatch {
    fn commit_into(&self, row: &mut Fragile, _: ApplyCapability) -> semio_framework_mutation_diff::diff::MutationApplyResult<()> { row.id.clone_from(&self.0); Ok(()) }
    fn absorb(&mut self, later: Self) { *self = later; }
    fn inverse(&self, row: &Fragile) -> Self { Self(row.id.clone()) }
    fn is_empty(&self) -> bool { false }
}

/// 🧊️ An identity-changing row patch retires every cloned row when the central applier refuses it.
#[test]
fn stable_identity_refusal_retires_abandoned_rows() {
    BARE_DROPS.with(|cell| cell.set(0));
    RETIRED.with(|cell| cell.set(0));
    let base = vec![fragile("a", 1), fragile("b", 2)];
    let delta = Parts::<Fragile, FragileIdentityPatch, FragileKeys>::modification("a".into(), FragileIdentityPatch("b".into()));
    let error = delta.commit_onto(&base, capability()).expect_err("identity change refuses");
    assert_eq!(error.code, "mutation.apply.identity-change");
    assert_eq!(BARE_DROPS.with(|cell| cell.get()), 0);
    assert_eq!(RETIRED.with(|cell| cell.get()), 2);
}

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
