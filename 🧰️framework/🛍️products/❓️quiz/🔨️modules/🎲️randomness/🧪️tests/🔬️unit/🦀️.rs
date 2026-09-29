//! 🪀️ Unit tests of the randomness: reference vectors of FNV-1a and MT19937, rejection sampling and shuffle.
//!
//! @see ../../🦀️.rs — the implementation under test

use super::*;

#[test]
fn fnv1a32_matches_the_reference_vectors() {
    assert_eq!(fnv1a32(""), 2_166_136_261);
    assert_eq!(fnv1a32("a"), 3_826_002_220);
    assert_eq!(fnv1a32("foobar"), 0xbf9c_f968);
    assert_eq!(run_seed("a"), fnv1a32("a"));
}

#[test]
fn fnv1a32_hashes_utf8_bytes() {
    let manual = "ä".bytes().fold(2_166_136_261u32, |h, b| (h ^ u32::from(b)).wrapping_mul(16_777_619));
    assert_eq!(fnv1a32("ä"), manual);
}

#[test]
fn mt19937_matches_the_reference_vectors() {
    let mut random = Mt19937::new(5489);
    assert_eq!(random.next_u32(), 3_499_211_612);
    let mut random = Mt19937::new(5489);
    let tenth_thousand = (0..10_000).map(|_| random.next_u32()).last();
    assert_eq!(tenth_thousand, Some(4_123_659_995));
}

#[test]
fn mt19937_matches_init_genrand_output_of_the_reference_code() {
    let mut random = Mt19937::new(1);
    assert_eq!([random.next_u32(), random.next_u32(), random.next_u32()], [1_791_095_845, 4_282_876_139, 3_093_770_124]);
    let mut zero = Mt19937::new(0);
    assert_eq!(zero.next_u32(), 2_357_136_044);
}

#[test]
fn uniform_index_stays_in_range_and_short_circuits_one() {
    let mut random = Mt19937::new(7);
    let before = random.clone().next_u32();
    assert_eq!(uniform_index(&mut random, 1), 0);
    assert_eq!(random.next_u32(), before);
    for n in 2..50 {
        assert!(uniform_index(&mut random, n) < n);
    }
}

#[test]
fn uniform_index_rejects_outputs_above_the_limit() {
    let n = 3usize << 30;
    let limit = (1u64 << 32) - (1u64 << 32) % (n as u64);
    let mut random = Mt19937::new(5489);
    let mut reference = Mt19937::new(5489);
    let expected = loop {
        let x = u64::from(reference.next_u32());
        if x < limit {
            break (x % n as u64) as usize;
        }
    };
    assert_eq!(uniform_index(&mut random, n), expected);
    assert_eq!(random.next_u32(), reference.next_u32());
}

#[test]
fn shuffle_is_a_seeded_permutation_that_leaves_the_input_alone() {
    let items: Vec<u32> = (0..20).collect();
    let first = shuffle(&mut Mt19937::new(42), &items);
    let second = shuffle(&mut Mt19937::new(42), &items);
    assert_eq!(first, second);
    let mut sorted = first.clone();
    sorted.sort_unstable();
    assert_eq!(sorted, items);
    assert_ne!(first, items);
    assert_eq!(items, (0..20).collect::<Vec<u32>>());
}

#[test]
fn shuffle_draws_from_the_end() {
    let mut reference = Mt19937::new(9);
    let mut expected = vec!['a', 'b', 'c', 'd'];
    for i in (1..4).rev() {
        let j = uniform_index(&mut reference, i + 1);
        expected.swap(i, j);
    }
    assert_eq!(shuffle(&mut Mt19937::new(9), &['a', 'b', 'c', 'd']), expected);
    assert!(shuffle::<u8>(&mut Mt19937::new(9), &[]).is_empty());
}

#[test]
fn shared_vectors_of_the_python_reference_hold() {
    use crate::schema::tests::{entries, fixture};
    let vectors = fixture("seeded-randomness");
    let number = |value: &serde_json::Value| value.as_u64().unwrap_or(u64::MAX);
    let numbers = |value: &serde_json::Value| entries(value).iter().map(number).collect::<Vec<_>>();
    for hash in entries(&vectors["hashes"]) {
        assert_eq!(u64::from(fnv1a32(hash["text"].as_str().unwrap_or_default())), number(&hash["hash"]), "{}", hash["id"]);
    }
    for seed in entries(&vectors["runSeeds"]) {
        assert_eq!(u64::from(run_seed(seed["run"].as_str().unwrap_or_default())), number(&seed["seed"]), "{}", seed["id"]);
    }
    let generator = |vector: &serde_json::Value| Mt19937::new(u32::try_from(number(&vector["seed"])).unwrap_or_default());
    for vector in entries(&vectors["rawOutputs"]) {
        let mut random = generator(vector);
        (0..number(&vector["skip"])).for_each(|_| {
            random.next_u32();
        });
        let outputs: Vec<u64> = (0..number(&vector["count"])).map(|_| u64::from(random.next_u32())).collect();
        assert_eq!(outputs, numbers(&vector["outputs"]), "{}", vector["id"]);
    }
    for vector in entries(&vectors["uniformDraws"]) {
        let mut random = generator(vector);
        let draws: Vec<u64> = numbers(&vector["bounds"]).iter().map(|&bound| uniform_index(&mut random, usize::try_from(bound).unwrap_or_default()) as u64).collect();
        assert_eq!(draws, numbers(&vector["expected"]["draws"]), "{}", vector["id"]);
        assert_eq!(u64::from(random.next_u32()), number(&vector["expected"]["next"]), "{}", vector["id"]);
    }
    for vector in entries(&vectors["shuffles"]) {
        let mut random = generator(vector);
        let permutation: Vec<u64> = shuffle(&mut random, &(0..number(&vector["length"])).collect::<Vec<_>>());
        assert_eq!(permutation, numbers(&vector["expected"]["permutation"]), "{}", vector["id"]);
        assert_eq!(u64::from(random.next_u32()), number(&vector["expected"]["next"]), "{}", vector["id"]);
    }
}

mod quick {
    use super::super::*;

    #[test]
    fn uniform_index_is_close_to_uniform() {
        let mut random = Mt19937::new(2024);
        let mut counts = [0usize; 6];
        for _ in 0..60_000 {
            counts[uniform_index(&mut random, 6)] += 1;
        }
        assert!(counts.iter().all(|&count| (9_500..10_500).contains(&count)), "{counts:?}");
    }
}
