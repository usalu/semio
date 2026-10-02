@capability-pets-bond-dynamics
@oracle-pets-numpy
@comparison-pets-float-v1
Feature: Bonds drift with shared history and drives move with what a pet does
  Pets interact with each other: sometimes they like each other, sometimes they have small disputes (design §1).
  How two species feel about each other is the authored affinity of their bond plus the drift of their rapport,
  `affinityOf(menagerie, rapports, a, b)`, held inside [−0.6, 1] so disputes stay small. `rapportAfter(drift,
  activity)` moves the drift with every encounter — a greet adds 0.05, a cuddle 0.1, a squabble takes 0.15 and
  the end of the sulk that follows gives 0.1 back — inside [−0.5, 0.5], and `rapportFaded(drift, ticks)` lets it
  fade back to 0, a tenth in ten minutes (design §5.5). The drives of an actor, `needsAfter(needs, activity,
  ticks, temperament)` from `needsOf(temperament)`, move linearly with what it does and stay inside [0, 1]: energy
  drains awake and returns asleep, sociability grows alone and is spent in company, curiosity grows at rest and
  is spent on the move (design §5.2).

  THE REFERENCE is numpy: every law above is clipped linear arithmetic, evaluated with `numpy.clip`,
  `numpy.sign`, `numpy.maximum` and `numpy.abs` from the design text. The subjects are `@semio-tech/pets`
  (`affinityOf`, `rapportAfter`, `rapportFaded`, `needsOf`, `needsAfter`) and the `pets` crate (`affinity_of`,
  `rapport_after`, `rapport_faded`, `needs_of`, `needs_after`). Numbers compare within 1e-9.

  The committed histories tell the stories the stage plays: friends that keep cuddling until their rapport is
  full, rivals that squabble, sulk and mend over and over without ever falling below the floor, strangers whose
  greetings are forgotten after ten quiet minutes; the committed days follow an energetic, a sleepy and a
  sociable temperament through idling, walking, company and sleep.

  The vectors shared://🤝️bond-dynamics/🔣️.json are generated, never hand-edited, from the Python reference in
  this directory, from the repository root:
  `.venv/Scripts/python.exe .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️02/QUIZ-PETS/generate_behavior_vectors.py`
  (`.venv/bin/python` outside Windows).

  @id-affinities
  @level-fundamental
  @mode-differential
  Scenario: An affinity is the authored bond plus the drift of the rapport
    Given the committed vectors shared://🤝️bond-dynamics/🔣️.json
    When affinityOf is taken of every committed pair over the committed bonds and rapports
    Then every implementation projects the same affinity per pair, in either order of the two species, 0 for strangers and for a species with itself, never below −0.6 and never above 1

  @id-rapport-steps
  @level-fundamental
  @mode-differential
  Scenario: Every encounter moves the rapport by its step
    Given the committed vectors shared://🤝️bond-dynamics/🔣️.json
    When rapportAfter is taken of every committed drift for every activity
    Then every implementation projects the same drift per activity, held inside −0.5 and 0.5, unchanged by every activity that is not an encounter or a sulk

  @id-rapport-fading
  @level-fundamental
  @mode-differential
  Scenario: A rapport fades back to nothing
    Given the committed vectors shared://🤝️bond-dynamics/🔣️.json
    When rapportFaded is taken of every committed drift after every committed number of ticks
    Then every implementation projects the same drift, a tenth closer to 0 per ten minutes and exactly 0 from then on

  @id-histories
  @level-fundamental
  @mode-differential
  Scenario: A shared history moves a bond without ever breaking it
    Given the committed vectors shared://🤝️bond-dynamics/🔣️.json
    When every committed history is folded: the drift fades over the ticks since the last event, then takes the step of the event
    Then every implementation projects the same drift and the same affinity after every event, and no affinity falls below −0.6

  @id-needs
  @level-fundamental
  @mode-differential
  Scenario: Drives move linearly with what a pet does
    Given the committed vectors shared://🤝️bond-dynamics/🔣️.json
    When needsAfter is taken of every committed need, activity, span and temperament
    Then every implementation projects the same energy, sociability and curiosity, each inside 0 and 1

  @id-days
  @level-fundamental
  @mode-differential
  Scenario: A temperament lives through a day
    Given the committed vectors shared://🤝️bond-dynamics/🔣️.json
    When the needs of every committed temperament start at needsOf and pass through every committed span in order
    Then every implementation projects the same needs after every span
