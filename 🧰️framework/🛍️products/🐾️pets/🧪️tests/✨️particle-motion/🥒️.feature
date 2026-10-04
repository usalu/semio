@capability-pets-particle-motion
@oracle-pets-numpy
@comparison-pets-float-v1
Feature: Particles are pure functions of time
  Tricks, states and purring come with sparks, hearts, rain and stars (design-v2 §19). A frame must stay a pure
  function of the stage, bit for bit in every language, and a stage must be able to sleep, skip frames and wake
  up again — so no particle is ever stored. What an emitter shows at a tick follows from its numbers (`motion`,
  `count`, `life` seconds, `speed` pixels per second, `spread` of a full turn), the tick it began, the tick it
  stopped, a 32-bit key and the tick asked for: `particlesOf(emitter, origin, facing, since, until, tick, key)`.

  Randomness is a hash, not a stream: `mix(a, b) = lowbias32((a xor 0x9e3779b9) + (b + 1)·0x85ebca6b)` at 32 bits,
  and particle `index` reads its lane `lane` as `unit(mix(mix(key, index), lane))`, a multiple of 2⁻³² in [0, 1).
  A particle lives `⌊life × 64 + ½⌋` ticks. `fall`, `rise` and `drift` are continuous: particle `index` is born at
  `since + index·period + ⌊lane 0 × period⌋` with `period = ⌈life ÷ count⌉`, while that tick lies before `until`;
  when one more than `count` is alive the eldest is left out. A `burst` throws `count` particles at `since`, once.
  An `orbit` keeps `count` particles circling from `since` and fades them out over 8 ticks from `until`.
  Directions scatter over `spread` of a full turn: around straight down for `fall`, straight up for `rise` and
  `burst` (whose particles share the fan evenly), straight ahead for `drift`; an `orbit` spaces its particles
  over `spread` of a ring that takes `life` per lap at `speed`. `capped(particles, cap)` keeps the youngest (the
  earlier of two of one age), and `emitterEnds(emitter, since, until)` is the first tick from which nothing is
  alive, or nothing at all while the emitter has no end.

  THE REFERENCE is numpy. The hash is evaluated in numpy's `uint32` arithmetic, which wraps at 32 bits by
  itself, and must reproduce the five published words of the research report before anything is projected. Who
  is alive is found by a simulation that walks from the start of the emitter tick by tick with a pool of
  particles — the subject stores nothing and looks at a window of indices. Where the particles are is the same
  formulas for all of them at once with `numpy.sin` and `numpy.cos` of `2π·t`, `numpy.clip` for the eases, and
  the rational decay `1 ÷ (1 + x + 0.48x² + 0.235x³)` held to `numpy.exp(−x)` within 0.02 wherever it is used —
  and with it the path of every thrown particle to the path under a true exponential drag. The cap is a stable
  `numpy.argsort` of the ages. The subjects are `@semio-tech/pets` (`lowbias32`, `mix`, `unit`, `bornAt`,
  `particlesOf`, `capped`, `emitterEnds`) and, once it exists, the `pets` crate. Words, ages and ticks compare
  exactly, positions and envelopes within 1e-9.

  The vectors shared://✨️particle-motion/🔣️.json are generated, never hand-edited, from the Python reference in
  this directory, from the repository root:
  `.venv/Scripts/python.exe .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️02/QUIZ-PETS/generate_effects_vectors.py`
  (`.venv/bin/python` outside Windows).

  @id-hashes
  @level-fundamental
  @mode-differential
  Scenario: The hash yields the words of 32-bit integer arithmetic
    Given the committed vectors shared://✨️particle-motion/🔣️.json
    When lowbias32 is taken of every committed word and mix is folded from the left over every committed chain
    Then every implementation projects the same unsigned 32-bit word per vector, 0x688990c0 for the word 1, 0xe577f3aa for the chain [0, 0] and 0x5ab36a78 for the chain [7, 3, 5]

  @id-uniformity
  @level-fundamental
  @mode-property
  Scenario: A lane is uniform over the indices
    Given the committed vectors shared://✨️particle-motion/🔣️.json
    When the words mix(mix(key, index), lane) of every committed run of indices are summed and their units counted in sixteen equal bins
    Then every implementation projects the same sum and the same sixteen counts per run, and the mean, the variance and the chi-square of every long run are those of a uniform lane

  @id-births
  @level-fundamental
  @mode-differential
  Scenario: Births and lives follow from the emitter alone
    Given the committed vectors shared://✨️particle-motion/🔣️.json
    When bornAt is taken of the first particles of every committed emitter and the ages of particlesOf are listed for every tick from first to last
    Then every implementation projects the life, the count, the period, the births in the order of their indices and per tick the ages of the living, eldest first, never more than the count, nothing before since and no birth at or after until

  @id-motions
  @level-fundamental
  @mode-differential
  Scenario: The five motions place their particles
    Given the committed vectors shared://✨️particle-motion/🔣️.json
    When particlesOf is taken of every committed emitter at each of its committed ticks
    Then every implementation projects per tick the same particles with x, y, scale, rotation, opacity and age within the comparison tolerance, mirrored about the origin for a pet that faces left

  @id-caps
  @level-fundamental
  @mode-differential
  Scenario: A cap keeps the youngest
    Given the committed vectors shared://✨️particle-motion/🔣️.json
    When capped is taken of particles with the committed ages at the committed cap
    Then every implementation projects the positions of the same survivors in their order, the earlier of two particles of one age staying

  @id-ends
  @level-fundamental
  @mode-differential
  Scenario: An emitter names the tick from which nothing of it is alive
    Given the committed vectors shared://✨️particle-motion/🔣️.json
    When emitterEnds is taken of every committed emitter with its since and until
    Then every implementation projects the same tick, or nothing for an emitter without an end, and the simulation finds nothing alive from that tick on for any committed key
