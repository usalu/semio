@capability-pets-counter-randomness
@oracle-pets-numpy
@comparison-ordered-json-v1
Feature: Counter-based randomness is bit-exact across languages
  What a pet decides next is drawn, and the same seed with the same events must yield the same frames
  in every language (design §2.4). A draw is therefore a pure function of a key of unsigned 32-bit
  integers — `[seed, stream, counter]`, the stream being the index of the species (or 0xffffffff for
  the stage) and the counter the number of decisions made so far — so the order in which actors are
  processed never changes what they draw. `randomWords(key, count)` hashes the key into a pool of four
  words and reads the pool out (design §4.2); `randomUnit(key)` is the first word divided by 2³²,
  `randomBetween(key, low, high)` is `low + (high − low) × unit`, and `randomPick(key, weights)` is the
  first index whose running sum of positive weights exceeds `unit × total` (weights that are not
  positive are never picked; −1 when no weight is positive).

  THE REFERENCE is numpy's own seed sequence: `numpy.random.SeedSequence(key).generate_state(count)`
  for the same key — the published hash-mix with a pool of four words, in a library that has never
  seen this repository. The committed keys carry one to eight words (zeros, all ones, high bits, more
  words than the pool holds, the actor and the stage shape), and the word counts run past the pool so
  its cyclic read-out is exercised. Units, ranges and picks consume only the library's words; the pick
  is `numpy.searchsorted` (side `right`) of `unit × total` in `numpy.cumsum` of the positive weights.
  Everything compares exactly: a unit is a multiple of 2⁻³², and the range bounds are integers or
  binary fractions. The subjects are `@semio-tech/pets` (`randomWords`, `randomUnit`, `randomBetween`,
  `randomPick`) and the `pets` crate (`random_words`, `random_unit`, `random_between`, `random_pick`).

  The vectors shared://🎲️counter-randomness/🔣️.json are generated, never hand-edited, from the Python
  reference in this directory, from the repository root:
  `.venv/Scripts/python.exe .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️02/QUIZ-PETS/generate_kinematics_vectors.py`
  (`.venv/bin/python` outside Windows).

  @id-words
  @level-fundamental
  @mode-differential
  Scenario: A key yields the words of numpy's seed sequence
    Given the committed vectors shared://🎲️counter-randomness/🔣️.json
    When the committed count of words is generated for every committed key with randomWords
    Then every implementation projects the same unsigned 32-bit words per key, the key [0] starting with 2968811710

  @id-units
  @level-fundamental
  @mode-differential
  Scenario: A unit draw is the first word divided by 2³²
    Given the committed vectors shared://🎲️counter-randomness/🔣️.json
    When randomUnit is taken of every committed key
    Then every implementation projects the same number in [0, 1) per key, exactly

  @id-streams
  @level-fundamental
  @mode-differential
  Scenario: An actor draws from its own stream, counter after counter
    Given the committed vectors shared://🎲️counter-randomness/🔣️.json
    When randomUnit is taken of the keys [seed, stream, 0] … [seed, stream, count − 1] of every committed stream
    Then every implementation projects the same draws in counter order, whatever any other stream has drawn

  @id-ranges
  @level-fundamental
  @mode-differential
  Scenario: A ranged draw scales the unit draw between two bounds
    Given the committed vectors shared://🎲️counter-randomness/🔣️.json
    When randomBetween is taken of every committed key between its low and its high bound
    Then every implementation projects the same number per key, exactly

  @id-picks
  @level-fundamental
  @mode-differential
  Scenario: A weighted pick follows the running sum of the positive weights
    Given the committed vectors shared://🎲️counter-randomness/🔣️.json
    When randomPick is taken of the keys [seed, stream, 0] … [seed, stream, count − 1] over every committed weight list
    Then every implementation projects the same indices in counter order, never an index whose weight is not positive, and −1 when no weight is positive
