@capability-pets-chemistry-rules
@oracle-pets-scipy
@comparison-pets-float-v1
Feature: Species have states and tricks, and what two pets do to each other depends on their states and moods
  Pets can have different states, their tricks are based on what the pet is, and the interaction of the pets
  with each other depends on mood and state (design-v2 §14, §19, §21). A state that `lasts` gives way to its
  `then`: `stateAt(species, state, since, tick)` says in closed form which state a pet is in, `stateEnds` when the
  present one is over. `ladderOf` is the order of the states as authored; circling a pet steps it up or down the
  rungs of the trick that answers (`rungsOf`: the states the trick lists in `from`, the whole ladder otherwise),
  held at both ends, and `stateAfterTrick` is the state a trick leaves — its `to`, or that step. `tricksFor(species,
  cue, state, feeling)` are the tricks on offer — the pet's own cues only while the mood it shows is a willing one —,
  `clickTrick` the trick of the n-th click, `whimTrick` and `showTrick` a weighted pick for the pet's own
  initiative. `trialsOf` lists the reactions of a menagerie that are due on a stage — for every ordered pair of
  actors by the place of their species, then by reaction: both traits match (a side that names no species takes
  anyone; a state held at least `held` seconds; a trick being performed; a mood by the one an actor shows: content
  while it is calm), the bodies are within `within` pixels and stand as `where` asks, the affinity of the two (the
  authored bond moved by their rapport) lies within the reaction's `affinity` bounds, no third actor that matches
  `unless` stands within `within` pixels of the second, and the reaction is not cooling for the pair — and
  `reactionsOf` folds one beat: a unit handed in decides a `chance`, every reaction that had its turn cools for
  `every` seconds, everybody is judged as it stood when the beat began, and nobody takes two states, two moods, two
  tricks or two activities, no pair two shifts of its rapport or two promised encounters in one beat (content
  §7–§8).

  THE REFERENCE is numpy and scipy, written from the design text. The state at a tick is read off the unrolled
  chain of states with `numpy.cumsum` and `numpy.searchsorted` — the subjects take whole laps off a circle instead.
  A weighted pick is `numpy.searchsorted` in `numpy.cumsum` of the weights. Whether every state of a species can be
  reached from its resting state — by tricks, by time, by reactions — is decided by
  `scipy.sparse.csgraph.breadth_first_order` on the graph of its states; a state nothing leads to fails the oracle.
  The matching is vectorised over all ordered pairs at once: bodies are boxes, gaps and overlaps are matrices built
  by broadcasting, traits are masks, and `numpy.argwhere` lists what is due in pair order. The fold of a beat is
  restated from the text. Subjects are `@semio-tech/pets` and the `pets` crate. Floats compare within 1e-9, ids,
  ticks, flags and orders exactly.

  The committed menagerie is small and made for this case: four species (a lamp whose resting state lies in the
  middle of its ladder, a cloud, a plant, a candle whose states run in a circle) and seventeen reactions that use
  every `where`, every kind of effect, a chance, a trick that is not always on offer, two reactions that reach for
  the same actor in one beat, sides of any species, a state held for a while, a trick being performed, a third
  party that bars a reaction and affinity bounds. The sample menagerie of the product
  (shared://🧬️schema-conformance/🔣️.json) is judged as it is committed at the time of the run.

  The vectors shared://⚗️chemistry-rules/🔣️.json are generated, never hand-edited, from the Python reference in
  this directory, from the repository root:
  `.venv/Scripts/python.exe .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️02/QUIZ-PETS/generate_feeling_vectors.py`
  (`.venv/bin/python` outside Windows).

  @id-relations
  @level-fundamental
  @mode-differential
  Scenario: Two bodies are near, above, below or beside each other
    Given the committed vectors shared://⚗️chemistry-rules/🔣️.json
    When nearby is taken of every committed pair of bodies at every committed reach, and seen for every where
    Then every implementation projects the same answers: the gap between the boxes decides near, a shared column above and below, a shared row without a shared column beside

  @id-states
  @level-fundamental
  @mode-differential
  Scenario: A state that lasts gives way to the next, however many ticks pass
    Given the committed vectors shared://⚗️chemistry-rules/🔣️.json
    When stateAt is taken of every committed species, state and tick, with stateEnds and lastingTicks
    Then every implementation projects the same state and the tick it began per tick, also round a circle of states for hours, and the same end of the first state

  @id-ladders
  @level-fundamental
  @mode-differential
  Scenario: Circling steps a pet up and down the rungs of its trick
    Given the committed vectors shared://⚗️chemistry-rules/🔣️.json
    When ladderOf, rungsOf, stepState and stateAfterTrick are taken of every committed species, trick and state
    Then every implementation projects the same ladder, the same rungs per trick, the same steps held at both ends and the same state after every trick

  @id-tricks
  @level-fundamental
  @mode-differential
  Scenario: A cue sets off the tricks that are on offer in a state and a mood
    Given the committed vectors shared://⚗️chemistry-rules/🔣️.json
    When tricksFor is taken of every committed species for every cue, state and committed feeling, clickTrick of every committed click, and whimTrick and showTrick of every committed unit
    Then every implementation projects the same tricks in authored order, none of its own for a pet that shows sleepiness, anger, sadness or fear, the click tricks round and round, and the pick numpy's cumsum and searchsorted find

  @id-reachability
  @level-fundamental
  @mode-property
  Scenario: Every state of every species can be reached
    Given the committed vectors shared://⚗️chemistry-rules/🔣️.json
    When the ways between the states of every committed species are collected from its tricks, its lasting states and the reactions of the menagerie
    Then every implementation projects the same edges, and scipy reaches every state from the resting one

  @id-matching
  @level-fundamental
  @mode-differential
  Scenario: The reactions that are due are found in pair order
    Given the committed vectors shared://⚗️chemistry-rules/🔣️.json
    When trialsOf is taken of every committed stage of sightings with its coolings
    Then every implementation projects the same trials in the same order: first by the species of the first actor, then of the second, then by reaction

  @id-reactions
  @level-fundamental
  @mode-differential
  Scenario: A beat of chemistry applies nothing twice and lets every reaction cool
    Given the committed vectors shared://⚗️chemistry-rules/🔣️.json
    When every committed story is folded beat by beat with reactionsOf, the coolings carried along
    Then every implementation projects the same consequences, coolings and number of units used per beat

  @id-sample
  @level-fundamental
  @mode-differential
  Scenario: The sample menagerie of the product holds together
    Given the committed vectors shared://⚗️chemistry-rules/🔣️.json
    And the sample menagerie shared://🧬️schema-conformance/🔣️.json
    When the ladder and the reachable states of every sample species are taken, and every reaction is staged with its two species standing as it asks
    Then every implementation projects the same ladders, edges and reachable states, and the same trials and consequences per staged reaction
