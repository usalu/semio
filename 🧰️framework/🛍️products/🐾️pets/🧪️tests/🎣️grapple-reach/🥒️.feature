@capability-pets-grapple-reach
@oracle-pets-scipy
@comparison-pets-float-v1
Feature: Pets shoot a grappling hook at an edge above, are hauled up the rope straight or swinging, and choose their way between perches by their gear
  A shot of the grappling gun leaves the muzzle, `MUZZLE_FORWARD` widths in front of the feet and
  `MUZZLE_HEIGHT` heights above them, for a point `HOOK_LIFT` above the edge of a perch (design-v2
  §16, MECH §3). `shotFor(feet, perches, keepouts, size)` tries, for every perch at least `ROPE_RISE`
  heights above the feet, its two corners, each `HOOK_INSET` inside its ends — the hook bites the
  corner, so a rope from below can rise beside the element the edge tops —, and the point between them
  nearest to the feet (the middle of a perch too narrow for that). A candidate holds when its rope is
  `ROPE_SHORT` to `ROPE_LONG` heights long (8 heights: what the cards of the real pages at 1440 × 900
  need above their footer), rises at least `ROPE_ELEVATION` per pixel of its length, and its line
  misses every keep-out grown by `ROPE_MARGIN` — except the keep-outs that hold the point of the edge
  under the hook: what a hook bites is not in its way. The candidate with the least `length +
  ROPE_DETOUR × sideways distance` wins, the first one among equals. A rope no more slanted than
  `ZIP_SLANT` (sideways distance ÷ rise) is reeled in straight (`zip`), any other swings — unless the
  swing would carry the actor into a keep-out under the hook (the line straight down from the hook,
  as long as the rope, misses no keep-out grown by `ROPE_MARGIN` but what lies under the hook): such a
  rope is hauled in straight too. `hookStep(from, to, speed, ticks)` is the hook on its straight line at a constant
  speed — `HOOK_SPEED` out, `HOOK_RETURN` back after a miss — the start itself at tick 0 and the end
  itself from `hookTicks` on. A haul begins at rest where the actor stood when the hook bit
  (`haulOf`), its feet hanging under its hands as they stood under the muzzle. `zipStep` shortens the
  rope by `ZIP_SPEED ÷ 64` per tick, gathered over `ZIP_RAMP` ticks by `smoothstep`, down to
  `REEL_LEAST` heights, the hands on the taut line; `swayStep` makes the hands the pendulum of the
  swing module's `reelStep` under the hook, down to the same rope. `shotHolds` is the shot as it holds
  after a survey — the hook follows its edge up or down by no more than `ROPE_FOLLOW` while that edge
  still carries it and the line stays clear — or none. `landingFor` is where the actor stands once it
  is up, half a width and `MANTLE_INSET` from the hook towards the middle of the perch and never
  beyond its ends; `missOf` is where a shot that is meant to miss is aimed, `ROPE_MISS_OVERSHOOT` past
  the nearer end of the perch.

  `routeOf(x, from, to, gear, size, grip, pitches, ladders, keepouts)` lists the ways an actor that
  stands at `x` on `from` can take to `to`, most preferred first (design-v2 §16): a ladder that
  stands between the two, for whoever owns any gear but a parachute; every wall line that joins them
  (gear `climb`; one leg per pitch it takes hold of, with the pitch where its way ends), while the grip
  lasts; a ladder of its own against every wall `to` crowns (gear
  `ladder`), and only where none stands; a shot at the edge of `to` (gear `grapple`) from where it
  stands, else from the point of its perch nearest to an end or to the middle of `to`, else from
  farther and farther back from either end of `to`, a width at a time while a rope could still reach
  across — where its line clears what stands under the edge. No way is none.

  THE REFERENCE is `🐍️.py` beside this file. A candidate's rope is `numpy.hypot`; whether its line is
  clear is not clipped — the neighbouring oracle of case 🧗️wall-climbing judges segment against box by
  their separating axes (`numpy.cross`) and lays 4097 sampled points over it — and the winner is
  `numpy.argmin` of the costs. A flight is the closed form `start + direction × min(speed × k ÷ 64,
  distance)`; a straight haul is `numpy.cumsum` of its eased speeds, held at the least rope, with the
  hands that far from the hook (`numpy.hypot`) on the taut line (`numpy.cross`). A swinging haul is a
  pendulum on a rope that shortens: `scipy.integrate.solve_ivp` (DOP853, tolerances 1e-12, through
  the neighbouring oracle of case 🪢️swing-dynamics) integrates `r·θ'' = −g·sin θ − γ·r·θ' − 2·r'·θ'`
  with the length the reel prescribes, from rest half a tick before the first step, and the restated
  steps must stay within the degrees every committed swing states (0.4°, MECH §3.3) while the rope is
  longer than 0.4 of its start; the rope must follow the ramped reel down to its least, the hands
  never leave its reach, and no committed swing is one the speed cap bends. Every number the oracle
  answers is the plain binary64 value of the stated arithmetic, refused unless the libraries reach it
  within 1e-9 (the swing within its stated degrees, since that approximation is far coarser than the
  comparison): the comparison grid of `pets-float-v1` is decimal (1e-9), and a number that is right to
  1e-13 can round to the other side of it. Why every candidate of a shot is refused — too low, too
  short, too long, too flat, blocked — is part of every committed shot, and the oracle must find the
  same reasons. The routes are a second reading of design-v2 §16 that composes the oracles of walls,
  ladders and ropes — a supplement that holds the twins to one reading, on top of the geometry above.
  The subjects are the rope functions and constants and `routeOf` of `🔨️modules/🧗️climbing` in
  `@semio-tech/pets` and the same names in snake case of the `pets` crate. The subjects project their
  own constants, so a constant tuned in one language only, or without regenerating the vectors, fails
  the case.

  The vectors shared://🎣️grapple-reach/🔣️.json are generated, never hand-edited, from the Python
  reference in this directory, from the repository root:
  `.venv/Scripts/python.exe .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️02/QUIZ-PETS/generate_climbing_vectors.py`
  (`.venv/bin/python` outside Windows).

  @id-constants
  @level-fundamental
  @mode-conformance
  Scenario: The tuning constants are the ones the vectors were generated with
    Given the committed vectors shared://🎣️grapple-reach/🔣️.json
    When every implementation states the speeds of the hook, the limits of the rope, the muzzle and the zip
    Then every implementation projects the committed constants

  @id-shots
  @level-fundamental
  @mode-differential
  Scenario: A shot takes the edge above that needs the least rope and detour and has a clear line
    Given the committed vectors shared://🎣️grapple-reach/🔣️.json
    When the shot of every committed stand is asked from shotFor
    Then every implementation projects the muzzle, the hook, the length and the way of reeling of the winning candidate — straight when it is steep or its swing would carry it into a keep-out under the hook —, and none when every candidate is too low, too short, too long, too flat or blocked

  @id-flights
  @level-fundamental
  @mode-differential
  Scenario: A hook flies on a straight line at a constant speed and ends on its target
    Given the committed vectors shared://🎣️grapple-reach/🔣️.json
    When every committed flight is advanced tick by tick with hookStep and timed with hookTicks
    Then every implementation projects the ticks and the hook after every tick as min(speed × k ÷ 64, distance) along its line, the target itself once it has arrived

  @id-zips
  @level-fundamental
  @mode-differential
  Scenario: A steep rope is reeled in straight, the hands on the taut line and the feet under the hands
    Given the committed vectors shared://🎣️grapple-reach/🔣️.json
    When every committed straight haul is advanced tick by tick with zipStep until the least rope is left
    Then every implementation projects the ticks, the rope as the sum of the eased speeds held at its least, the hands that far from the hook on the line of the shot, and the feet under them

  @id-swings
  @level-fundamental
  @mode-differential
  Scenario: A slanted rope swings as a pendulum on a rope that shortens
    Given the committed vectors shared://🎣️grapple-reach/🔣️.json
    When every committed swinging haul is advanced tick by tick with swayStep until the least rope is left
    Then every implementation projects the ticks, the rope along the ramped reel, the hands within the stated degrees of the integrated pendulum, and the feet under them

  @id-surveys
  @level-fundamental
  @mode-differential
  Scenario: A hook follows its edge within a tolerance and loses it beyond
    Given the committed vectors shared://🎣️grapple-reach/🔣️.json
    When every committed survey is answered with shotHolds
    Then every implementation projects the shot with its hook on the moved edge and its new length, and none when the edge moved too far, left the hook behind or vanished, or a keep-out got in the line

  @id-landings
  @level-fundamental
  @mode-differential
  Scenario: A rope lets its actor off inside the hook, and a miss is aimed past the nearer end
    Given the committed vectors shared://🎣️grapple-reach/🔣️.json
    When the landing and the miss of every committed shot are asked from landingFor and missOf
    Then every implementation projects the spot half a width and the inset from the hook towards the middle, held between the ends of the perch, and the aim past its nearer end

  @id-routes
  @level-fundamental
  @mode-differential
  Scenario: An actor reaches a perch by what it owns: a standing ladder, a wall, its own ladder, its rope
    Given the committed vectors shared://🎣️grapple-reach/🔣️.json
    When the ways between the two perches of every committed stage are asked from routeOf with the committed gear and grip
    Then every implementation projects the legs in the order of preference with where the actor walks to first, and none when nothing joins the perches or the gear does not allow it
