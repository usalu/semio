@capability-pets-swing-dynamics
@oracle-pets-scipy
@comparison-pets-float-v1
Feature: A hanging pet swings like a pendulum, keeps its amplitude, follows the hand and is thrown with the hand's velocity
  A pet that the learner picks up dangles under the pointer, a pet under a parachute sways under its
  canopy, and a pet on a grappling rope swings while it reels the rope in (design-v2 §16, §17; MECH §1,
  §3.3). All three are one step, `swingStep(anchorBefore, anchorNow, bob, previous, length, gravity,
  damping, rope)`, taken once per tick of 1/64 s: the point flies freely (it keeps `damping` of its last
  step and falls by `gravity ÷ 4096`), and is then pulled back along the radius it had before the step,
  by the smaller root of the quadratic that puts it at `length` from the anchor of this tick (SHAKE). A
  slack rope pulls nothing; a rod always does. Around it: `coneClamp` keeps a held pet on its rod and
  within 60° of straight down, `followStep` carries the grip to the pointer by a critically damped
  spring, `hangStep` is the three together, `leanOf` is the tilt of the hanging body in turns,
  `ringVelocity` is the pointer's velocity from its last seven samples, `releaseVelocity` and
  `throwVelocity` are what a release throws, and `reelStep` shortens a rope under two guards.

  THE REFERENCE is `🐍️.py` beside this file. What the step approximates is a pendulum, and
  `scipy.integrate.solve_ivp` (DOP853, tolerances 1e-12) integrates the pendulum itself — with a fixed
  or an accelerated pivot, with a prescribed shortening length, and with the drag a damping factor
  stands for (the step damps the displacement of the tick before, so a factor `d` is the drag
  `128·(1 − d) ÷ (1 + d)` per second under a gravity `2 ÷ (1 + d)` times as strong). Every committed
  swing starts on the integrated pendulum (its place one tick earlier is read off the integration) and
  states its tolerance in degrees: 0.4° over three seconds for ropes and canopies, 1.5° over half a
  second for the stiff pendulum of a held pet without damping (its frequency is off by 0.2 %), 0.8° over
  the two seconds its damped swing lives. The approximation is far coarser than the comparison tolerance
  of 1e-9, so the oracle restates the step in Python's IEEE doubles, refuses every vector whose restated
  swing leaves the integrated pendulum by more than it states, and projects the restated positions,
  which the subjects must reproduce. A free swing must keep its amplitude within 0.05° over a minute;
  as a negative control the usual shortcut — step freely, then pull the point back onto the circle — is
  run from the same start and must lose more than 10°, or the oracle refuses to answer, because a
  judgement that cannot tell the two steps apart judges nothing.

  The rest is judged by numpy and scipy in the same way and projected as restated: the clamp by
  `numpy.arctan2` and the sine and cosine of the cone's edge; the follow spring by
  `numpy.linalg.matrix_power` of its one-tick matrix, by its eigenvalues (no overshoot) and by
  `scipy.linalg.expm` of the continuous spring of a half-life of 0.06 s; every lean by `numpy.arctan2`
  within 2e-6 turns, and the peak lean of a drag that stays off the cone by the continuous system of
  pointer, spring and pendulum within 2°; the pointer's velocity by the slope of `numpy.polyfit(deg=2)`
  at the newest of the same seven samples (the weights 7, −2, −7, −8, −5, 2, 13 over 28 are exactly
  those of that fit, shown in rational arithmetic); a slack rope by the closed form of a free flight;
  a reeled rope by the pendulum with that length; the guards of the reel by `numpy.cumsum` of the
  ramped reel speed and by the same swing without the cap. The oracle refuses a vector whose answer
  could hang on rounding: a point within 1e-6 of the cone's edge, a release within 1e-6 of the least
  or the most speed, a flight that grazes the reach of its rope. The subjects are `swingStep`,
  `coneClamp`, `followStep`, `hangOf`, `hangStep`, `leanOf`, `ringVelocity`, `releaseVelocity`,
  `throwVelocity`, `reelStep` and the tuning constants of `🔨️modules/🪢️swing` in `@semio-tech/pets`,
  and the same names in snake case of the `pets` crate. The subjects project their own constants, so a
  constant tuned in one language only, or without regenerating the vectors, fails the case.

  One scenario is a supplement, not third-party evidence: `bit-patterns` projects the sixteen
  hexadecimal digits of single steps, which holds the twins to each other bit for bit where the
  tolerance of the other scenarios would hide a last-bit difference.

  The vectors shared://🪢️swing-dynamics/🔣️.json are generated, never hand-edited, from the Python
  reference in this directory, from the repository root:
  `.venv/Scripts/python.exe .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️02/QUIZ-PETS/generate_swing_vectors.py`
  (`.venv/bin/python` outside Windows).

  @id-constants
  @level-fundamental
  @mode-conformance
  Scenario: The tuning constants are the ones the vectors were generated with
    Given the committed vectors shared://🪢️swing-dynamics/🔣️.json
    When every implementation states its constants of the hang, the follow spring, the release, the throw and the reel
    Then every implementation projects the committed constants, the weights being exactly those of the least-squares parabola and the follow spring free of overshoot

  @id-rod-swings
  @level-fundamental
  @mode-differential
  Scenario: A point on a rod below a fixed anchor swings like a pendulum
    Given the committed vectors shared://🪢️swing-dynamics/🔣️.json
    When every committed swing is advanced tick by tick with swingStep from its bob and its place one tick earlier
    Then every implementation projects the same positions at the committed ticks, which stay within the stated degrees of the integrated pendulum

  @id-amplitude-drift
  @level-quick
  @mode-property
  Scenario: A free swing keeps its amplitude for a minute
    Given the committed vectors shared://🪢️swing-dynamics/🔣️.json
    When every committed free swing is advanced by 3840 ticks and its least height below the anchor is taken in every window of 640 ticks
    Then every implementation projects the same least heights and the same end, the amplitude never drifts by more than 0.05°, and the shortcut that pulls the point back onto the circle loses more than 10° from the same start

  @id-moving-anchors
  @level-fundamental
  @mode-differential
  Scenario: A point on a rod follows an anchor that moves
    Given the committed vectors shared://🪢️swing-dynamics/🔣️.json
    When a point that hangs at rest is advanced with swingStep while its anchor travels the committed path, one place per tick
    Then every implementation projects the same positions at the committed ticks, which stay within the stated degrees of the pendulum with that accelerated pivot

  @id-slack-ropes
  @level-fundamental
  @mode-property
  Scenario: A slack rope pulls nothing and a taut one holds
    Given the committed vectors shared://🪢️swing-dynamics/🔣️.json
    When every committed point on a rope is advanced tick by tick with swingStep
    Then every implementation projects the same position after every tick: the free flight until the rope pulls at the committed tick, and never beyond the reach of the rope afterwards

  @id-reeled-swings
  @level-fundamental
  @mode-differential
  Scenario: A swing on a shortening rope speeds up like a pendulum on a shortened string
    Given the committed vectors shared://🪢️swing-dynamics/🔣️.json
    When every committed swing is advanced with swingStep on a rope of length − rate × tick ÷ 64
    Then every implementation projects the same positions at the committed ticks, which stay within the stated degrees of the pendulum with that prescribed length

  @id-reel-guards
  @level-fundamental
  @mode-property
  Scenario: A reeled swing never shortens its rope below the least length and never winds up
    Given the committed vectors shared://🪢️swing-dynamics/🔣️.json
    When every committed pet is advanced with reelStep, its age counting the ticks reeled before
    Then every implementation projects the same position and rope length at the committed ticks and the same fastest step, the rope following the ramped reel speed down to its least and the step never exceeding the cap by more than the reel takes in

  @id-cone-clamps
  @level-fundamental
  @mode-differential
  Scenario: A held pet stays on its rod and inside its cone
    Given the committed vectors shared://🪢️swing-dynamics/🔣️.json
    When every committed point is passed through coneClamp
    Then every implementation projects the same point: untouched inside the cone and on the rod, on the edge of the cone on its own side beyond it, drawn in along its direction when farther than the rod

  @id-follow-springs
  @level-fundamental
  @mode-differential
  Scenario: The grip follows the pointer without overshoot
    Given the committed vectors shared://🪢️swing-dynamics/🔣️.json
    When every committed grip is advanced towards its target with followStep by each committed number of ticks
    Then every implementation projects the place and velocity the power of the one-tick matrix yields

  @id-drag-leans
  @level-fundamental
  @mode-differential
  Scenario: A held pet leans against the drag and settles
    Given the committed vectors shared://🪢️swing-dynamics/🔣️.json
    When a pet is picked up with hangOf and advanced with hangStep along every committed pointer path, and leanOf is taken after every tick
    Then every implementation projects the same leans at the committed ticks, the same peak lean and the same tick from which the lean stays below 3°, an ordinary drag peaking where the continuous system does and a flick at the 60° of the cone

  @id-release-velocities
  @level-fundamental
  @mode-differential
  Scenario: The velocity of the pointer at the release is the slope of a parabola through its last seven samples
    Given the committed vectors shared://🪢️swing-dynamics/🔣️.json
    When ringVelocity and releaseVelocity are taken of every committed ring of samples
    Then every implementation projects the same pointer velocity and the same throw: nothing once the pointer stopped or below the least speed, the most speed along its direction above it, and the rise cut

  @id-throw-velocities
  @level-fundamental
  @mode-differential
  Scenario: A held pet is thrown with its own velocity and half of what the grip lacks
    Given the committed vectors shared://🪢️swing-dynamics/🔣️.json
    When throwVelocity is taken of every committed held pet and its ring of samples
    Then every implementation projects the same throw

  @id-bit-patterns
  @level-fundamental
  @mode-differential
  Scenario: A single step has the same 64 bits in every language
    Given the committed vectors shared://🪢️swing-dynamics/🔣️.json
    When swingStep is taken once of every committed state and both coordinates are written as the sixteen hexadecimal digits of their IEEE bit patterns
    Then every implementation projects the same digits per step
