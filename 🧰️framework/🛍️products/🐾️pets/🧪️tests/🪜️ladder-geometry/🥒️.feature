@capability-pets-ladder-geometry
@oracle-pets-numpy
@comparison-pets-float-v1
Feature: Pets raise a ladder from a lower perch against the rim or the lower end of a wall, climb it rung by rung, and it topples when its wall leaves
  A ladder is a segment from a foot on a lower perch to a contact on a wall (design-v2 §16, MECH §4).
  `ladderFor(low, high, pitch, keepouts, size)` raises one against a pitch that `high` crowns, its
  contact `LADDER_TUCK` pixels under the rim; `ladderTo(low, pitch, keepouts, size)` raises one
  against the lower end of a pitch, so that a climber at its exit takes hold of the wall (`clings`) —
  the telescopic ladder reaches up to `LADDER_TALL` = 8 heights, what the cards of the real pages at
  1440 × 900 need above their footer. The foot stands where the lean (its distance from the wall ÷ the
  rise) is `LADDER_LEAN`, the 4 : 1 rule of real ladders, or as near to that as the perch lets it, at
  least `LADDER_FOOTING` from both ends of the perch. It stands only when its rise lies between
  `LADDER_SHORT` and `LADDER_TALL` heights of its owner, its contact lies on the pitch no higher than
  `LADDER_TUCK` under its top, its lean lies between `LADDER_STEEP` and `LADDER_FLAT` on the air side
  of the wall, and its line is clear of every keep-out on the air side of the wall grown by
  `LADDER_GIRTH` — keep-outs that begin within `WALL_LIP` of the wall line on the side of its element
  are the body of that element, which a survey grows past its side and the ladder leans against —,
  except the keep-outs that hold its foot or its contact: what a ladder stands on and what it leans
  against are not in its way. `ladderLength` is its length,
  `ladderRungs` the whole `RUNG_SPACING` that fit, `ladderLean` its lean, `ladderExit` the distance at
  which a climber steps over onto the perch (`LADDER_EXIT` heights before the top), `ladderLanding`
  the spot on the rim it steps onto. `ladderStep(travel, goal, ticks)` climbs along the rails, up at
  `LADDER_RISE` and down at `LADDER_DESCENT` pixels per second, gathered over `LADDER_RAMP` ticks by
  `smoothstep`, and never overshoots; `ladderAt` is the place of the feet after a distance, the foot
  itself and the top itself at the ends; `ladderPhase` turns the clip one cycle per two rungs, so
  hands and feet meet the rungs. `ladderHolds` is the ladder as it stands after a survey: its foot
  keeps its x on a perch of its surface that moved no more than `LADDER_SHIFT` up or down, its top
  follows a pitch of its wall — under its rim or at its lower end, as ladders lean — no farther than
  `LADDER_FOLLOW`, and it still stands by the rules above — or none, and it topples. A ladder leans on
  its wall, not on what lies on top of it: it stands on when the perch on the rim is gone. `spillOf` is what a toppling ladder does
  to its climber: below `TOPPLE_STEP` heights above the foot it steps off, higher up it is thrown
  clear of the wall at `TOPPLE_PUSH` pixels per second.

  THE REFERENCE is `🐍️.py` beside this file. The length of a ladder is `numpy.hypot`, its lean the
  foot's distance over the rise, its rungs the whole spacings that fit (`numpy.floor`); the place of a
  climber is judged by its distance from the foot (`numpy.hypot`) and by the `numpy.cross` product
  that keeps it on the rails; the climb is `numpy.cumsum` of its eased speeds, held at its goal;
  whether the top followed is `numpy.hypot` of its move. Whether the line of a ladder is clear is not
  clipped: the neighbouring oracle of case 🧗️wall-climbing judges segment against box by their
  separating axes and lays 4097 sampled points over it, and refuses a vector whose verdict hangs on
  less than a sample. Every number the oracle answers is the plain binary64 value of the stated
  arithmetic, refused unless numpy reaches it within 1e-9: the comparison grid of `pets-float-v1` is
  decimal (1e-9), and a number that is right to 1e-13 can round to the other side of it. The oracle
  refuses a ladder whose length lies within 1e-6 of a whole number of rung spacings, a climb that
  arrives within 1e-6 of a tick, and a top that moved within 1e-6 of what a ladder follows. Why a
  ladder is refused is part of every committed placement — no rim, too short, too tall, off the pitch,
  no footing, too steep, too flat, blocked — and of every committed lean against a wall — the same,
  and no grip where the hands at its exit miss the wall —, and the oracle must find the same reason,
  so the vectors keep covering every rule. The subjects are the ladder functions and constants of
  `🔨️modules/🧗️climbing` in `@semio-tech/pets` and the same names in snake case of the `pets` crate.
  The subjects project their own constants, so a constant tuned in one language only, or without
  regenerating the vectors, fails the case.

  The vectors shared://🪜️ladder-geometry/🔣️.json are generated, never hand-edited, from the Python
  reference in this directory, from the repository root:
  `.venv/Scripts/python.exe .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️02/QUIZ-PETS/generate_climbing_vectors.py`
  (`.venv/bin/python` outside Windows).

  @id-constants
  @level-fundamental
  @mode-conformance
  Scenario: The tuning constants are the ones the vectors were generated with
    Given the committed vectors shared://🪜️ladder-geometry/🔣️.json
    When every implementation states the leans, the rises, the measures and the speeds of a ladder
    Then every implementation projects the committed constants

  @id-placements
  @level-fundamental
  @mode-differential
  Scenario: A ladder stands at the lean of real ladders where its perch, its wall and the keep-outs let it
    Given the committed vectors shared://🪜️ladder-geometry/🔣️.json
    When the ladder of every committed placement is asked from ladderFor and measured
    Then every implementation projects the foot, the top, the length, the rungs, the lean, the exit and the landing of every ladder that stands, and none for a placement without a rim, too short, too tall, off the pitch, without footing, too steep, too flat or blocked

  @id-leans
  @level-fundamental
  @mode-differential
  Scenario: A ladder leans against the lower end of a wall so that a climber at its exit takes hold of the wall
    Given the committed vectors shared://🪜️ladder-geometry/🔣️.json
    When the ladder of every committed lean is asked from ladderTo and its exit placed with ladderAt and ladderExit
    Then every implementation projects the foot and the top of every ladder that stands and the feet at its exit, and none for a lean too short, too tall, off the pitch, without footing, too steep, blocked or out of the hands' reach of the wall

  @id-climbs
  @level-fundamental
  @mode-differential
  Scenario: A climber gathers its speed along the rails and turns the clip by the rungs it passes
    Given the committed vectors shared://🪜️ladder-geometry/🔣️.json
    When every committed climb is advanced tick by tick with ladderStep between the foot and the exit and placed with ladderAt and ladderPhase
    Then every implementation projects the distance as the sum of the eased speeds held at the goal, the feet that far from the foot on the rails, and the phase by the distance climbed

  @id-surveys
  @level-fundamental
  @mode-differential
  Scenario: A ladder follows its wall within a tolerance and topples beyond it
    Given the committed vectors shared://🪜️ladder-geometry/🔣️.json
    When every committed survey is answered with ladderHolds
    Then every implementation projects the ladder with its top on the moved wall and its foot on the moved perch — also when the perch on the rim is gone —, and none when the wall or the perch moved too far, vanished or lost its footing, or a keep-out got in the way

  @id-spills
  @level-fundamental
  @mode-differential
  Scenario: A toppling ladder lets a low climber step off and throws a high one clear of the wall
    Given the committed vectors shared://🪜️ladder-geometry/🔣️.json
    When the fate of a climber at every committed height is asked from spillOf
    Then every implementation projects none below the step height and the throw away from the wall from there on
