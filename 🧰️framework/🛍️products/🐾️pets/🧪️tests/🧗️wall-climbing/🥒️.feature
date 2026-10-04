@capability-pets-wall-climbing
@oracle-pets-scipy
@comparison-pets-float-v1
Feature: Pets climb the free stretches of the surveyed side walls, hold on by their grip, slide down and mantle over the rim
  A wall is a side edge of a surveyed element from its top `y0` down to `y1`, with the air on its left
  (`side` −1) or on its right (1) (design-v2 §16, §21, MECH §5). `wallsOf(walls, keepouts, width,
  height, clearance, minimum)` is `perchesOf` turned by a quarter: of every wall whose band lies inside
  `[0, width]` it clips the extent to `[0, height]`, removes the y-extent of every keep-out that
  reaches into the band on its air side, from `WALL_LIP` to `clearance` pixels away from it, and keeps
  the stretches (pitches) at least `minimum` long, in wall order and then down the wall. The survey
  grows the box of every surface sideways and the box of every control by a few pixels, so the first
  `WALL_LIP` pixels beside a wall belong to the wall itself and block nothing. A keep-out that only
  touches an end or the band takes nothing away, a box without area blocks nothing, and no pitch has
  zero length. `wallAt(pitches, wall, y)` is the first pitch of a wall that carries `y` (ends
  included), `nearestWall(pitches, x, y)` the pitch whose nearest point is closest (the first among
  equals). `segmentHits(from, to, rect, margin)` is the line of sight of ropes and ladders
  (Liang–Barsky): a segment hits a box grown by the margin when it passes through its inside; touching
  an edge or grazing a corner is no hit, a box without area is never hit, and a segment of no length
  hits the box its point lies inside. `segmentClear` is clear of every box of a list.

  An actor clings to a pitch half its width out on the air side (`clingOf`) with its hands
  `HAND_HEIGHT` heights above its feet: between `rimOf`, where the hands reach the top, and `footOf`,
  where they hold `GRIP_BITE` above the lower end. `gripFor(perch, pitch, size)` is the hold it takes:
  over the rim from the perch that crowns the pitch (same surface, same height, carrying the ledge
  beside the rim), or from any other perch on which it can stand beside the wall with its hands on
  the pitch. `wallHolds` is the pitch that still carries it after a survey (the same wall and side,
  no more than `WALL_FOLLOW` aside), and `slipOf` the throw that takes it off the wall when none does.
  `climbStep(y, goal, ticks)` climbs up at `CLIMB_RISE` and down at `CLIMB_DESCENT` pixels per second,
  gathered over `CLIMB_RAMP` ticks by `smoothstep`, and never overshoots; `climbTicks` is how long a
  climb takes, one more than `GRIP_BUDGET` when no rested grip lasts for it; `climbPhase` is the phase
  of the clip by the distance from the top of the pitch, one cycle per two holds, so hands never
  slide. `gripStep` spends `GRIP_CLIMB` per tick of climbing and `GRIP_HANG` per tick of hanging, and
  gives `GRIP_REST` back per tick on a perch, between nothing and `GRIP_BUDGET`. `slideStep` is one
  tick of sliding, velocity first: `vy ← min(max(vy, SLIDE_START) + SLIDE_GAIN ÷ 64, SLIDE_SPEED)`,
  then `y ← min(y + vy ÷ 64, floor)`. `mantlePath(pitch, size, phase)` is `hoistPath` from the highest
  hold to the ledge: the body rises over the first `HOIST_RISE` of the way to `HOIST_HUMP` heights
  above its end, crosses over the last `HOIST_RISE` of it and settles over the rest, each part eased
  by `smoothstep`. The pitches of one wall line — the same side, no more than `WALL_FOLLOW` apart, the
  side edges of a column of cards — are one way: `crossable(from, to, size)` says whether a climber
  lunges from one to the other, wholly above or below it, both holding it somewhere, the hands
  crossing from the top of the lower one to the lowest hold of the upper one within `CROSS_REACH`
  heights; `chainOf` is the line of a pitch from the top down (upwards the reachable pitch with the
  lowest end, downwards the one with the highest top, the first among equals); `wallPath` is the whole
  way along a line tick by tick — the grab beside the wall (eased onto the hold) or the hang over the
  rim (the mantle path backwards), a climb on every pitch that gathers its speed anew, a lunge across
  every gap (`hoistPath` over `LUNGE_TICKS`) and the mantle — and `wallCost` the grip it costs, one
  climbing tick for every tick of it. `routeOf` with the gear to climb offers one leg per pitch it takes hold of: up the line from
  a hold at the foot of the pitch to the rim of the nearest pitch the target crowns, or down over the
  rim of the pitch to the nearest pitch where the target passes its foot, when the grip lasts for the
  whole way.

  THE REFERENCE is `🐍️.py` beside this file and subtracts no interval and clips no segment. Every
  committed coordinate of a layout lies on a quarter-pixel lattice; numpy boolean masks decide by
  membership of the cell centres which cells of a wall lie inside the viewport and beside no keep-out
  of its band, and the pitches are the runs of free cells (`numpy.diff`). The pitch at a height is
  read from a raster of lattice points, the nearest pitch is found by brute force over the sampled
  points of every pitch (`numpy.argmin`). A line of sight is judged by the separating axes of segment
  and box — their extents on both axes and the signs of the `numpy.cross` products of the box's
  corners with the segment — and laid over 4097 sampled points of the segment, which must agree, so
  a vector whose verdict hangs on less than a sample is refused. A climb is `numpy.cumsum` of its
  eased speeds, held at its goal; the grip after a run is a clipped line; a slide is the sum of its
  clamped speeds, held at its floor; the mantle is three cubic Hermite splines with flat ends between
  the keys of its definition (`scipy.interpolate.CubicHermiteSpline`), and so are the hang and every
  lunge of a way along a line, whose climbs are held to the sums of their eased speeds and whose grab
  to the numpy ease; which pitch lunges to which is a matrix of numpy broadcasting, and the lines are
  read off it with `numpy.argmax`. Every number the oracle answers
  is the plain binary64 value of the stated tick, refused unless the libraries reach it within 1e-9:
  the comparison grid of `pets-float-v1` is decimal (1e-9), and a number that is right to 1e-13 can
  round to the other side of it. The oracle refuses a climb that arrives within 1e-6 of a tick. The
  holds, the surveys and the routes are decided by a second reading of the design written with numpy
  comparisons — a supplement that holds the twins to one reading, on top of the geometry above. The
  subjects are `wallsOf`, `wallAt`, `nearestWall`, `segmentHits`, `segmentClear` and `WALL_LIP` of
  `🔨️modules/🏞️terrain` and the wall functions and constants of `🔨️modules/🧗️climbing` in
  `@semio-tech/pets`, and the same names in snake case of the `pets` crate. The subjects project
  their own constants, so a constant tuned in one language only, or without regenerating the vectors,
  fails the case.

  The layouts cover walls of both sides, bands partly outside the viewport and on its limits, walls
  clipped by the viewport, above it and below it, keep-outs inside the band, within the lip, reaching
  the lip and a quarter pixel beyond it, ending where the band begins and a quarter pixel inside it,
  behind the wall, touching the ends and a quarter pixel over them, nested, overlapping and abutting
  in three orders, without area, walls of no length, stretches of exactly the minimum, a clearance
  within the lip and none at all, three cards in a column as a quiz page lays them out, and twelve
  scattered layouts. The sights cover segments through, into, out of and inside a box, beside it,
  short of it and beyond it, along every edge, through every corner, a hair inside, of no length,
  grown and shrunk boxes, boxes without area and 160 scattered segments against three boxes each.

  The vectors shared://🧗️wall-climbing/🔣️.json are generated, never hand-edited, from the Python
  reference in this directory, from the repository root:
  `.venv/Scripts/python.exe .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️02/QUIZ-PETS/generate_climbing_vectors.py`
  (`.venv/bin/python` outside Windows).

  @id-constants
  @level-fundamental
  @mode-conformance
  Scenario: The tuning constants are the ones the vectors were generated with
    Given the committed vectors shared://🧗️wall-climbing/🔣️.json
    When every implementation states the lip of a wall, its climbing speeds, its grip and its slide
    Then every implementation projects the committed constants

  @id-pitches
  @level-fundamental
  @mode-differential
  Scenario: Walls are cut into pitches by the viewport and by the keep-outs in the band on their air side
    Given the committed vectors shared://🧗️wall-climbing/🔣️.json
    When the pitches of every committed layout are cut with wallsOf
    Then every implementation projects the runs of free lattice cells of every wall as its pitches, in wall order and then down the wall

  @id-stretches
  @level-fundamental
  @mode-differential
  Scenario: A pitch carries every height between its ends, the first one where two touch
    Given the committed vectors shared://🧗️wall-climbing/🔣️.json
    When every committed query is answered with wallAt
    Then every implementation projects the index of the pitch that owns the lattice point of its wall, and none in a gap, beyond the ends or on an unknown wall

  @id-nearest
  @level-fundamental
  @mode-differential
  Scenario: The nearest pitch is the one with the closest point, the first among equals
    Given the committed vectors shared://🧗️wall-climbing/🔣️.json
    When every committed point is answered with nearestWall
    Then every implementation projects the index of the pitch with the closest sampled point, and none without pitches

  @id-sights
  @level-fundamental
  @mode-differential
  Scenario: A segment hits a box only when it passes through its inside
    Given the committed vectors shared://🧗️wall-climbing/🔣️.json
    When every committed segment is tested against every box of its vector with segmentHits and against all of them with segmentClear
    Then every implementation projects the verdict of the separating axes for every box, never a hit for an edge touched, a corner grazed or a box without area, and a clear line exactly when no box is hit

  @id-holds
  @level-fundamental
  @mode-differential
  Scenario: An actor takes hold of a pitch from a perch at its foot or over the rim from the perch on top
    Given the committed vectors shared://🧗️wall-climbing/🔣️.json
    When the hold of every perch on every pitch of every committed stage is asked from gripFor, the perch on top from rimFor, and the measures of every pitch
    Then every implementation projects the hold beside the wall with the hands on the pitch, the hold at the rim for the perch that crowns it, and none where the actor cannot stand or reach

  @id-surveys
  @level-fundamental
  @mode-differential
  Scenario: A wall carries its climber through a survey while it stays within its tolerance and free under the hands
    Given the committed vectors shared://🧗️wall-climbing/🔣️.json
    When every committed survey is answered with wallHolds, and with slipOf where the wall is lost
    Then every implementation projects the index of the pitch that still carries the climber, or none and the throw away from the wall

  @id-climbs
  @level-fundamental
  @mode-differential
  Scenario: A climb gathers its speed, never overshoots, and turns the clip by the distance climbed
    Given the committed vectors shared://🧗️wall-climbing/🔣️.json
    When every committed climb is advanced tick by tick with climbStep and measured with climbTicks and climbPhase
    Then every implementation projects the ticks of the climb and, when a rested grip lasts for it, the height as the sum of the eased speeds held at the goal and the phase by the distance from the top of the pitch

  @id-grips
  @level-fundamental
  @mode-differential
  Scenario: Climbing and hanging spend the grip, resting gives it back
    Given the committed vectors shared://🧗️wall-climbing/🔣️.json
    When every committed run of efforts is advanced tick by tick with gripStep
    Then every implementation projects the grip after every run as a line clipped between nothing and the budget

  @id-slides
  @level-fundamental
  @mode-differential
  Scenario: A slide gains speed up to the fastest slide and ends on its floor
    Given the committed vectors shared://🧗️wall-climbing/🔣️.json
    When every committed slide is advanced tick by tick with slideStep
    Then every implementation projects the clamped speeds and the heights as their sum, held at the floor

  @id-mantles
  @level-fundamental
  @mode-differential
  Scenario: A mantle rises over the rim, crosses over and settles onto the ledge
    Given the committed vectors shared://🧗️wall-climbing/🔣️.json
    When every committed mantle is traced tick by tick with mantlePath and every committed hoist with hoistPath
    Then every implementation projects the feet after every tick along the three eased parts between the keys of the path, the start itself first and the end itself last

  @id-routes
  @level-fundamental
  @mode-differential
  Scenario: A climber gets from perch to perch over every wall line that joins them while its grip lasts
    Given the committed vectors shared://🧗️wall-climbing/🔣️.json
    When the ways between the two perches of every committed stage are asked from routeOf with the gear to climb
    Then every implementation projects one leg per pitch it takes hold of, up from its foot or down over its rim, with the pitch of the line where the way ends, in pitch order, and none when nothing joins them or the grip is too short

  @id-crossings
  @level-fundamental
  @mode-differential
  Scenario: A climber lunges across the gap to the next pitch of its wall line and climbs the whole line
    Given the committed vectors shared://🧗️wall-climbing/🔣️.json
    When every pair of pitches of every committed stage is asked from crossable, the line of every pitch from chainOf, and every committed way from wallPath and wallCost
    Then every implementation projects which pitch lunges to which, every line from the top down, and every way tick by tick — its climbs on the sums of their eased speeds, its grab on the numpy ease, its hang, lunges and mantle on the cubic Hermite splines — with the grip it costs
