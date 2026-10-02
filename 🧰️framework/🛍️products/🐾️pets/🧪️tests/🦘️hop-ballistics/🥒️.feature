@capability-pets-hop-ballistics
@oracle-pets-scipy
@comparison-pets-float-v1
Feature: Pets fall onto perches and hop between them on a ballistic arc that ends on its target
  Flight is integrated tick by tick (1/64 s) with semi-implicit Euler, the velocity first (design §4.5,
  §5.2): `fallStep(y, vy)` is `vy ← min(vy + GRAVITY ÷ 64, FALL_SPEED)`, then `y ← y + vy ÷ 64`, and a
  hop adds `x ← x + vx ÷ 64`. `landingOf(perches, x, fromY, toY)` is the highest perch crossed at `x`
  on the way down between two heights, both heights and both ends of a perch included, the first among
  equals — perches are one-way platforms, nothing is crossed while rising, and a fall can never tunnel
  through a perch however fast it is.

  `hopOf(from, to)` is the hop between two points or none when out of reach. Its apex rises
  `max(HOP_CLEARANCE − min(dy, 0), |dx| × HOP_STEEPNESS)` above the launch, so it tops both ends; it is
  too high above `HOP_HEIGHT` and too far beyond `HOP_DISTANCE`. Its duration is the time of that arc,
  `sqrt(2 × rise ÷ GRAVITY) + sqrt(2 × (rise + dy) ÷ GRAVITY)`, rounded to whole ticks; it is too long
  beyond `HOP_TICKS`. Its launch velocity solves the per-tick sum, not the continuous arc:
  `vy = (dy − GRAVITY × ticks × (ticks + 1) ÷ 8192) × 64 ÷ ticks` and `vx = dx × 64 ÷ ticks`, so the
  flight ends on its target after exactly its ticks; a hop that would land faster than `FALL_SPEED` is
  a fall and is refused. `hopStep(x, y, vx, vy, to, ticks)` flies one tick and writes the target itself
  on the last one, and `hopLanding(perches, from, to, hop)` is the perch the flight really ends on —
  the first one it crosses on the way down.

  THE REFERENCE is `🐍️.py` beside this file. Below the terminal speed the ticks of semi-implicit Euler
  are samples of one continuous arc — the arc whose launch velocity is half a tick's gain larger,
  `vy + GRAVITY ÷ 128` — which `scipy.integrate.solve_ivp` integrates and reads at the tick times; at
  the terminal speed the fall continues as a straight line, and `numpy.cumsum` of the clamped speeds
  must reach the same heights. The duration of a hop is the positive root of the continuous arc with
  the stated apex (`numpy.roots`), its launch velocity the root (`scipy.optimize.brentq`) of the height
  the integrated arc misses the target by. Every height, speed and velocity the oracle answers is
  judged by those libraries within 1e-9 and answered as the plain binary64 value of the stated tick:
  the comparison grid of `pets-float-v1` is decimal (1e-9), the exact binary fractions of a flight are
  ties on it, and a number that is right to 1e-13 would round to the other side. Landings come from
  sweeps that are not the subjects' and use the integrated flight alone: the distinct perch heights
  walked from the top (`numpy.unique`), and a flight laid against every perch at once in a boolean
  matrix of ticks × perches. The oracle refuses a vector whose answer could hang on rounding: a flight
  time within 1e-6 of two tick counts, a landing speed within 1e-6 of the terminal speed, an integrated
  sample within 1e-6 of a perch height or end it has to clear. It also refuses a flight that does not
  rise first, top both of its ends by half the clearance and come down onto its target. The subjects
  are `GRAVITY`, `FALL_SPEED`, the `HOP_*` constants, `fallStep`, `landingOf`, `hopOf`, `hopStep` and
  `hopLanding` of `🔨️modules/🏞️terrain` in `@semio-tech/pets` and the same names in snake case of the
  `pets` crate. The subjects project their own constants, so a constant tuned in one language only, or
  without regenerating the vectors, fails the case.

  The vectors shared://🦘️hop-ballistics/🔣️.json are generated, never hand-edited, from the Python
  reference in this directory, from the repository root:
  `.venv/Scripts/python.exe .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️02/QUIZ-PETS/generate_terrain_vectors.py`
  (`.venv/bin/python` outside Windows).

  @id-constants
  @level-fundamental
  @mode-conformance
  Scenario: The tuning constants are the ones the vectors were generated with
    Given the committed vectors shared://🦘️hop-ballistics/🔣️.json
    When every implementation states its gravity, its terminal speed and its hop limits
    Then every implementation projects the committed constants

  @id-falls
  @level-fundamental
  @mode-differential
  Scenario: A fall gains GRAVITY ÷ 64 per tick up to the terminal speed and moves by the new speed
    Given the committed vectors shared://🦘️hop-ballistics/🔣️.json
    When every committed fall is advanced tick by tick with fallStep
    Then every implementation projects the heights of the integrated arc and of the straight line at the terminal speed, and the clamped speeds

  @id-landings
  @level-fundamental
  @mode-differential
  Scenario: The highest perch crossed on the way down is the landing
    Given the committed vectors shared://🦘️hop-ballistics/🔣️.json
    When every committed sweep is answered with landingOf
    Then every implementation projects the index of the first perch met when the perch heights are walked from the top, and none between perches, beside them or while rising

  @id-drops
  @level-fundamental
  @mode-differential
  Scenario: A fall passes the perches beside it and stops on the first one below
    Given the committed vectors shared://🦘️hop-ballistics/🔣️.json
    When every committed fall is advanced with fallStep until landingOf answers a perch or its ticks run out
    Then every implementation projects the same perch, the same number of ticks and the same final height

  @id-hops
  @level-fundamental
  @mode-differential
  Scenario: A hop is granted within reach and refused when too high, too far, too long or too fast
    Given the committed vectors shared://🦘️hop-ballistics/🔣️.json
    When the hop of every committed pair of points is asked from hopOf
    Then every implementation projects the launch velocity and the ticks of every granted hop, and none for every hop out of reach

  @id-flights
  @level-fundamental
  @mode-property
  Scenario: A granted hop rises above both of its ends and lands on its target after its ticks
    Given the committed vectors shared://🦘️hop-ballistics/🔣️.json
    When every committed granted hop is flown tick by tick with hopStep
    Then every implementation projects the feet after every tick along the integrated arc, the target itself on the last tick, and the apex

  @id-routes
  @level-fundamental
  @mode-differential
  Scenario: A hop ends on the first perch its flight crosses on the way down
    Given the committed vectors shared://🦘️hop-ballistics/🔣️.json
    When the landing of every committed hop among its perches is asked from hopLanding
    Then every implementation projects the index of the perch the flight crosses first, the launch perch or a shelf when it lies in the way, and none when the target carries nothing
