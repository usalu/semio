use super::*;

struct Xorshift(u64);

impl Xorshift {
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

fn random_edit(rng: &mut Xorshift, list: &[u32], fresh: &mut u32) -> Splice<u32, u32> {
    let cuts: Vec<(usize, u32)> = list.iter().enumerate().filter(|_| rng.below(4) == 0).map(|(index, value)| (index, *value)).collect();
    let kept = list.len() - cuts.len();
    let wanted = rng.below(4);
    let mut slots: Vec<usize> = Vec::new();
    while slots.len() < wanted {
        let slot = rng.below((kept + wanted).max(1));
        if !slots.contains(&slot) {
            slots.push(slot);
        }
    }
    let puts = slots
        .into_iter()
        .map(|slot| {
            *fresh += 1;
            (slot, *fresh)
        })
        .collect();
    Splice::new(cuts, puts)
}

fn applied(edit: &Splice<u32, u32>, list: &[u32]) -> Vec<u32> {
    edit.commit_onto(list.to_vec(), |entry, key| entry == key).expect("the edit applies")
}

#[test]
fn composing_two_edits_equals_applying_them_in_sequence() {
    let mut rng = Xorshift(0x9E37_79B9_7F4A_7C15);
    let mut fresh = 1_000;
    for round in 0..20_000 {
        let base: Vec<u32> = (0..rng.below(8) as u32).collect();
        let first = random_edit(&mut rng, &base, &mut fresh);
        let mid = applied(&first, &base);
        let second = random_edit(&mut rng, &mid, &mut fresh);
        let after = applied(&second, &mid);
        let mut both = first.clone();
        both.absorb(second.clone());
        assert_eq!(applied(&both, &base), after, "round {round}: {first:?} then {second:?} on {base:?}");
        let inverse = both.inverse(&base, |row| *row);
        assert_eq!(applied(&inverse, &after), base, "round {round}: the inverse restores the base");
    }
}

#[test]
fn the_inverse_steps_of_two_edits_sum_to_the_negative_script() {
    let mut rng = Xorshift(0xDEAD_BEEF);
    let mut fresh = 1_000;
    for round in 0..20_000 {
        let base: Vec<u32> = (0..rng.below(8) as u32).collect();
        let first = random_edit(&mut rng, &base, &mut fresh);
        let mid = applied(&first, &base);
        let second = random_edit(&mut rng, &mid, &mut fresh);
        let after = applied(&second, &mid);
        let mut both = first.clone();
        both.absorb(second.clone());
        both.settle();
        let mut sum = second.inverse(&mid, |row| *row);
        sum.absorb(first.inverse(&base, |row| *row));
        sum.settle();
        let mut negative = both.inverse(&base, |row| *row);
        negative.settle();
        assert_eq!(applied(&sum, &after), base, "round {round}");
        assert_eq!(sum, negative, "round {round}: {first:?} then {second:?} on {base:?}");
    }
}

#[test]
fn the_inverse_of_the_inverse_is_the_edit_itself() {
    let mut rng = Xorshift(42);
    let mut fresh = 1_000;
    for _ in 0..5_000 {
        let base: Vec<u32> = (0..rng.below(8) as u32).collect();
        let edit = random_edit(&mut rng, &base, &mut fresh);
        let after = applied(&edit, &base);
        assert_eq!(edit.inverse(&base, |row| *row).inverse(&after, |row| *row), edit);
    }
}

#[test]
fn replacing_carries_one_list_to_another_and_inverts_to_the_reverse_replacement() {
    let mut rng = Xorshift(7);
    for _ in 0..5_000 {
        let was: Vec<u32> = (0..rng.below(7)).map(|_| rng.below(3) as u32).collect();
        let now: Vec<u32> = (0..rng.below(7)).map(|_| rng.below(3) as u32).collect();
        let edit = Splice::replacing(&was, &now);
        assert_eq!(applied(&edit, &was), now);
        assert_eq!(edit.inverse(&was, |row| *row), Splice::replacing(&now, &was));
    }
}

#[test]
fn an_insertion_removed_again_cancels() {
    let mut edit: Splice<u32, u32> = Splice::putting(1, 9);
    edit.absorb(Splice::cutting(1, 9));
    assert!(edit.is_empty());
}

#[test]
fn a_script_that_does_not_fit_its_list_is_refused() {
    let wrong_entry: Splice<u32, u32> = Splice::cutting(0, 5);
    assert_eq!(wrong_entry.commit_onto(vec![1, 2], |entry, key| entry == key), Err(SpliceFault::CutMismatch { index: 0 }));
    let past_end: Splice<u32, u32> = Splice::cutting(4, 1);
    assert_eq!(past_end.commit_onto(vec![1, 2], |entry, key| entry == key), Err(SpliceFault::CutOutOfRange { index: 4, len: 2 }));
    let gap: Splice<u32, u32> = Splice::putting(5, 1);
    assert_eq!(gap.commit_onto(vec![1, 2], |entry, key| entry == key), Err(SpliceFault::PutOutOfRange { index: 5, len: 3 }));
}
