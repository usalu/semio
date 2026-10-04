@capability-pets-parachute-descent
@oracle-pets-scipy
@comparison-pets-float-v1
Feature: A pet that would land hard opens its parachute, descends at a gentle speed, steers, sways and flares
  A pet that falls from high up — dropped by the learner, thrown, or shaken off a card — opens a
  parachute instead of hitting the ground (design-v2 §16; MECH §2). `impactSpeed(vy, height)` is the
  speed a fall would land with, `sqrt(vy² + 2 × GRAVITY × height)`, never beyond `FALL_SPEED`;
  `chuteOpens(vy, height)` holds when the pet falls at `CHUTE_OPENING` or faster, has `CHUTE_HEADROOM`
  or more below its feet and would land harder than `HARD_LANDING` — from rest that is a drop of 102 px
  or more, about two body heights. The pet then keeps falling for `CHUTE_REFLEX` ticks, and
  `canopyOf(feet, vx, vy, chute)` opens the canopy of `chuteOf(height)` above it. Under the canopy one
  tick is `chuteStep(canopy, chute, target, remaining, wind)`: the descent closes the gap to the
  terminal speed by `CHUTE_FACTOR` (`vy ← terminal + (vy − terminal) × factor`), the sideways speed
  closes `CHUTE_STEER_EASE` of the gap to the speed it wants (`CHUTE_STEER_GAIN` times the distance to
  the target, held within the reach of the chute), the canopy moves by `vx + wind` and by `vy` times
  `flareOf(remaining, flare)` (all of the speed above the flare height, half of it at the touch), and
  the pet swings below it on its cords by `swingStep` of case 🪢️swing-dynamics. `chuteWind(ticks,
  phase)` is a slow sine.

  THE REFERENCE is `🐍️.py` beside this file. The impact speed is confirmed by
  `scipy.integrate.solve_ivp` with an event at the landing. The descent is a first-order lag: its tick
  samples are `vt + (v0 − vt) × Fᵏ` with `F = exp(−1 ÷ (64 × 0.18))` (`numpy.exp`, `numpy.power`, and
  `solve_ivp` on the lag itself), the heights their running sum times the flare (`numpy.cumsum`,
  `numpy.interp`). The steering is the power of its one-tick matrix while it is off its limit
  (`numpy.linalg.matrix_power`) and a lag towards the limit while it is on it. The wind is
  `numpy.sin`. The sway is the pendulum below a pivot in uniform motion, integrated by `solve_ivp`
  with the drag the damping factor stands for; it must stay within 0.4° over three seconds, and a pet
  whose sway has died must trail where drag and gravity balance. A whole drop — fall, decision, reflex,
  canopy, flare, touch — is integrated once more as one continuous system with an event at the
  landing; every committed drop must land within one tick of it and at its descent speed within
  6 px/s, and touch at between half and all of that speed. The oracle projects its own restatement of
  the subjects' arithmetic once the libraries have confirmed it, because a tick-wise step and a
  differential equation agree far less closely than the comparison tolerance of 1e-9. It refuses a
  vector whose answer could hang on rounding: a predicted impact within 1e-6 of a hard landing that is
  not exactly on it, a steering that enters or leaves its limit on the way. The subjects are
  `impactSpeed`, `chuteOpens`, `chuteOf`, `canopyOf`, `flareOf`, `chuteWind`, `chuteStep` and the
  `CHUTE_*` constants and `HARD_LANDING` of `🔨️modules/🪢️swing` in `@semio-tech/pets` (with `fallStep`,
  `GRAVITY` and `FALL_SPEED` of `🔨️modules/🏞️terrain`), and the same names in snake case of the `pets`
  crate. The subjects project their own constants, so a constant tuned in one language only, or
  without regenerating the vectors, fails the case.

  The vectors shared://🪂️parachute-descent/🔣️.json are generated, never hand-edited, from the Python
  reference in this directory, from the repository root:
  `.venv/Scripts/python.exe .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️02/QUIZ-PETS/generate_swing_vectors.py`
  (`.venv/bin/python` outside Windows).

  @id-constants
  @level-fundamental
  @mode-conformance
  Scenario: The tuning constants are the ones the vectors were generated with
    Given the committed vectors shared://🪂️parachute-descent/🔣️.json
    When every implementation states its gravity, its terminal speed, its hard landing and the constants of its parachute
    Then every implementation projects the committed constants, the factor being the exponential of its time constant and a hard landing the end of a fall of 100 px from rest

  @id-impact-speeds
  @level-fundamental
  @mode-differential
  Scenario: The speed a fall would land with
    Given the committed vectors shared://🪂️parachute-descent/🔣️.json
    When impactSpeed is taken of every committed speed and height
    Then every implementation projects the same speed, the terminal speed at most, and the speed itself on and below the landing

  @id-chute-triggers
  @level-fundamental
  @mode-differential
  Scenario: A parachute opens only for a fast fall from high enough that would land hard
    Given the committed vectors shared://🪂️parachute-descent/🔣️.json
    When chuteOpens is asked for every committed speed and height
    Then every implementation projects the same answer: no below the opening speed, no without headroom, no for an impact of exactly a hard landing, yes just beyond each

  @id-chute-measures
  @level-fundamental
  @mode-differential
  Scenario: A parachute is sized by the body height
    Given the committed vectors shared://🪂️parachute-descent/🔣️.json
    When chuteOf is taken of every committed body height
    Then every implementation projects the same terminal speed, reach, flare height and cord length

  @id-flares
  @level-fundamental
  @mode-differential
  Scenario: A canopy slows evenly to half of its speed over the flare height
    Given the committed vectors shared://🪂️parachute-descent/🔣️.json
    When flareOf is taken of every committed height above the landing and flare height
    Then every implementation projects the same share: 1 at the flare height and above, one half at the touch and below

  @id-winds
  @level-fundamental
  @mode-differential
  Scenario: The wind is a slow sine of the tick
    Given the committed vectors shared://🪂️parachute-descent/🔣️.json
    When chuteWind is taken of every committed tick and phase
    Then every implementation projects the same push in pixels per second

  @id-canopy-descents
  @level-fundamental
  @mode-differential
  Scenario: A canopy approaches its terminal speed and steers towards its target
    Given the committed vectors shared://🪂️parachute-descent/🔣️.json
    When a canopy is opened with canopyOf above every committed pet and advanced with chuteStep, the height above the landing shrinking by what the feet have descended and the wind taken from chuteWind where a phase is committed
    Then every implementation projects the same canopy and feet at the committed ticks: the speeds of the lag, the heights of their running sum, the sideways motion of its closed form, the feet at the cords' length below the canopy

  @id-canopy-sways
  @level-fundamental
  @mode-differential
  Scenario: A pet sways below its canopy like a pendulum and trails behind a drifting one
    Given the committed vectors shared://🪂️parachute-descent/🔣️.json
    When every committed pet below a canopy in uniform motion is advanced with chuteStep
    Then every implementation projects the same feet at the committed ticks, which stay within the stated degrees of the integrated pendulum

  @id-drops
  @level-fundamental
  @mode-differential
  Scenario: A drop ends softly with a parachute and hard without
    Given the committed vectors shared://🪂️parachute-descent/🔣️.json
    When every committed pet falls with fallStep, asks chuteOpens before every tick where it has a parachute, opens its canopy CHUTE_REFLEX ticks after the answer and descends with chuteStep until its feet reach the landing
    Then every implementation projects the same tick of the decision (0 for none), the same tick of the landing, the same speed of the touch, the same descent speed and the same impact predicted without a parachute, the landing within one tick of the continuous system
