@capability-quiz-randomness
@oracle-quiz-numpy-mt19937
@comparison-ordered-json-v1
Feature: Seeded randomness is bit-exact across languages
  Every sheet a learner sees is a pure function of the quiz and the run seed, so the seed, the
  generator, the bounded draw and the shuffle must produce the same integers in every language that
  builds a sheet (design §3): `seed(run)` is FNV-1a 32-bit over the UTF-8 bytes of the run id, the
  generator is MT19937 seeded with `init_genrand` and read as tempered u32 words, `uniform(n)` rejects
  every word at or above `2³² − (2³² mod n)` and never draws for `n = 1`, and `shuffle` is Fisher–Yates
  from the end.

  THE REFERENCE is numpy's own `MT19937` bit generator, seeded through `RandomState(seed)` (which runs
  `init_genrand` for a scalar seed) and read with `random_raw`. The oracle loads CPython's
  `random.Random` — an unrelated MT19937 implementation — with the same 624-word state and requires the
  same words, and one vector pins the C++ standard's check value: the 10000th word of a default-seeded
  `std::mt19937` is 4123659995. FNV-1a, `uniform` and `shuffle` are derived in `🐍️.py` from the design
  text and consume only the library's words. The subjects are `@semio-tech/quiz` (`fnv1a32`,
  `runSeed`, `Mt19937`, `uniformIndex`, `shuffle`) and the `quiz` crate (`fnv1a32`, `run_seed`,
  `Mt19937::next_u32`, `uniform_index`, `shuffle`).

  Every draw and every shuffle vector also projects the NEXT raw word after it, so a draw that consumed
  one word too many or too few — a missed rejection, a draw for `n = 1`, a shuffle that runs down to
  index 0 — changes the projection even when the drawn values happen to agree. The half-range bound
  `2³¹ + 1` rejects almost every second word; `2³² − 1` rejects exactly one word value.

  The vectors shared://🎲️seeded-randomness/🔣️.json are generated, never hand-edited, from the Python
  reference in this directory, from the repository root:
  `.venv/Scripts/python.exe .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️28/QUIZ-PRODUCT-AND-TEACHING-PROCTOR/generate_quiz_vectors.py`
  (`.venv/bin/python` outside Windows).

  @id-hashes
  @level-fundamental
  @mode-differential
  Scenario: FNV-1a hashes the UTF-8 bytes of a text
    Given the committed vectors shared://🎲️seeded-randomness/🔣️.json
    When every committed text is hashed with fnv1a32
    Then every implementation projects the same 32-bit hash per vector, including the empty text's offset basis 2166136261

  @id-run-seeds
  @level-fundamental
  @mode-differential
  Scenario: A run seed is the FNV-1a hash of the run id
    Given the committed vectors shared://🎲️seeded-randomness/🔣️.json
    When every committed run id is turned into a seed with runSeed
    Then every implementation projects the same seed per run id

  @id-raw-outputs
  @level-fundamental
  @mode-differential
  Scenario: MT19937 seeded with init_genrand emits the standard tempered words
    Given the committed vectors shared://🎲️seeded-randomness/🔣️.json
    When a generator is seeded per vector, skips the committed number of words and reads the committed count
    Then every implementation projects the same words, seed 5489 starting with 3499211612 and its 10000th word being 4123659995

  @id-uniform-draws
  @level-fundamental
  @mode-differential
  Scenario: Bounded draws reject the biased tail and draw nothing for a bound of one
    Given the committed vectors shared://🎲️seeded-randomness/🔣️.json
    When every committed bound is drawn in order from one generator per vector, followed by one raw word
    Then every implementation projects the same draws and the same following word

  @id-shuffles
  @level-fundamental
  @mode-differential
  Scenario: Fisher–Yates from the end permutes a sequence identically
    Given the committed vectors shared://🎲️seeded-randomness/🔣️.json
    When the indices 0 … length−1 are shuffled by one generator per vector, followed by one raw word
    Then every implementation projects the same permutation and the same following word
