@capability-pets-feeling-dynamics
@oracle-pets-numpy
@comparison-pets-float-v1
Feature: A mood comes with a cause, holds, fades and travels, and shows on the face and in the wishes of a pet
  Pets can have different moods (design-v2 §14, §19). A feeling is one of nine moods with an intensity in [0, 1]
  and the tick it was last stirred. `impulse(feeling, mood, amount, tick)` reinforces the same mood, soothes with
  `content` whatever is worse than content, and lets another mood replace the present one when that one is calm, or
  outranked (scared > grumpy > sad > proud = happy = playful > curious > sleepy > content), or — once its hold of
  two seconds is over — of the same rank or more than 1.2 times weaker. `settled(feeling, resting, tick)` is the
  feeling at any later tick in closed form: it holds, fades linearly at the rate of its mood and gives way to the
  resting mood of the species, which rises back to its rest level of 0.25; `settlesAt` is the tick from which
  nothing changes. `caught(mine, theirs, affinity, sociability, tick)` lets a stirred mood travel to a neighbour
  without ever growing on the way. `faceOf` and `spiritsOf` are the face of content moved towards the face of the
  mood by its intensity, `moodWeights` the multipliers on what a pet feels like doing, `encounterBias` what two
  moods do to an encounter, and `appraised`, `performed` and `drowsed` say which occasion gives which impulse to
  which character (mechanics §8.2–§8.3, content §A1).

  THE REFERENCE is numpy, written from the design text. The trajectory of a feeling is evaluated over a whole array
  of ticks at once with `numpy.maximum`, `numpy.minimum` and `numpy.where`; the tick a mood has faded is the first
  sample below the threshold by `numpy.argmax` and the tick a feeling settles the first sample at rest — neither
  solves for it as the subjects do; `numpy.diff` asserts that a trajectory only ever holds, falls at its rate or
  rises at the rate of the rest. The ranks are a boolean matrix: its transitivity is a matrix product, its tiers
  `numpy.argsort` of its column sums, and they must be the tiers of the design. A crowd is vectorised over
  everybody who catches, and beat by beat the oracle asserts that every intensity stays in [0, 1] and that nobody
  ends above the strongest of its mood (no amplification). Faces and weights are `numpy.interp`, the proneness of a
  character a `numpy.dot`. The tables themselves (ranks, rates, faces, appraisals, weights, spreads) are this
  product's own tuning, restated by the oracle so that the committed numbers, the TypeScript twin
  (`@semio-tech/pets`) and the Rust twin (`pets` crate) are held to one reading. Floats compare within 1e-9, moods,
  ticks and flags exactly.

  The vectors shared://💗️feeling-dynamics/🔣️.json are generated, never hand-edited, from the Python reference in
  this directory, from the repository root:
  `.venv/Scripts/python.exe .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️02/QUIZ-PETS/generate_feeling_vectors.py`
  (`.venv/bin/python` outside Windows).

  @id-tables
  @level-fundamental
  @mode-differential
  Scenario: The tables of the moods are those of the design
    Given the committed vectors shared://💗️feeling-dynamics/🔣️.json
    When every table and constant of the feeling module is read
    Then every implementation projects the same ranks, hold, thresholds, rates, faces, appraisals, proneness, weights, leanings and spreads

  @id-priorities
  @level-fundamental
  @mode-property
  Scenario: A mood only gives way to one that outranks it, or to its equal once its hold is over
    Given the committed vectors shared://💗️feeling-dynamics/🔣️.json
    When an impulse of 0.5 of every mood meets every mood of 0.8, inside its hold and at the tick that ends it
    Then every implementation projects the same two matrices of who replaces whom, and outranking is a strict order in the tiers content, sleepy, curious, the three joys, sad, grumpy, scared

  @id-impulses
  @level-fundamental
  @mode-differential
  Scenario: Impulses reinforce, soothe, replace or are lost
    Given the committed vectors shared://💗️feeling-dynamics/🔣️.json
    When every committed story is folded: the feeling is settled at the tick of each event and then takes its impulse
    Then every implementation projects the same feeling after every event

  @id-decays
  @level-fundamental
  @mode-differential
  Scenario: A feeling holds, fades and returns to the resting mood in closed form
    Given the committed vectors shared://💗️feeling-dynamics/🔣️.json
    When settled is taken of every committed feeling at every committed tick, and settlesAt of the feeling
    Then every implementation projects the same mood, intensity and anchor per tick and the same tick from which nothing changes

  @id-jumps
  @level-fundamental
  @mode-property
  Scenario: However time is cut, the same feeling comes out
    Given the committed vectors shared://💗️feeling-dynamics/🔣️.json
    When every committed feeling is settled through its cuts one after the other and in one step
    Then every implementation projects the same feeling both ways, and settling it again at the same tick changes nothing

  @id-contagion
  @level-fundamental
  @mode-property
  Scenario: Moods travel through a crowd and never grow on the way
    Given the committed vectors shared://💗️feeling-dynamics/🔣️.json
    When every committed crowd lives through its beats: everybody is settled, then catches from each neighbour in order, each as it stood when the beat began
    Then every implementation projects the same feelings after every beat, every intensity stays between 0 and 1, and nobody ends a beat above the strongest of its mood

  @id-faces
  @level-fundamental
  @mode-differential
  Scenario: The face is the face of content moved towards the face of the mood
    Given the committed vectors shared://💗️feeling-dynamics/🔣️.json
    When valenceOf, spiritsOf and faceOf are taken of every committed feeling
    Then every implementation projects the same valence, spirits, mouth bend, lid height, lid slant and posture drop

  @id-appraisals
  @level-fundamental
  @mode-differential
  Scenario: Every occasion gives its impulses as the character of the species takes them
    Given the committed vectors shared://💗️feeling-dynamics/🔣️.json
    When pronenessOf is taken of every committed character for every mood, and appraised, performed and drowsed of its committed feelings for every occasion, every kind of trick and every committed energy
    Then every implementation projects the same proneness and the same feelings

  @id-wishes
  @level-fundamental
  @mode-differential
  Scenario: A mood moves what a pet feels like doing
    Given the committed vectors shared://💗️feeling-dynamics/🔣️.json
    When moodWeights is taken of every mood at every committed intensity
    Then every implementation projects the same twenty-six multipliers per mood, all of them 1 for content and for an intensity of 0

  @id-leanings
  @level-fundamental
  @mode-differential
  Scenario: Two moods tip an encounter
    Given the committed vectors shared://💗️feeling-dynamics/🔣️.json
    When encounterBias is taken of every committed pair of feelings and swayedShares of the committed shares
    Then every implementation projects the same shift of the affinity, the same multipliers of greet, cuddle and squabble, the same one who shows off and the same swayed shares
