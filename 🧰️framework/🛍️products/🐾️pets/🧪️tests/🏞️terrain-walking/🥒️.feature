@capability-pets-terrain-walking
@oracle-pets-numpy
@comparison-pets-float-v1
Feature: Pets stand and walk on the free stretches of the surveyed surfaces
  A pet stands on the top edge of a card or on the floor, in the space above it (design §4.5, §5.1).
  `perchesOf(surfaces, keepouts, width, height, clearance, minimum)` takes every surface with
  `clearance ≤ y ≤ height`, clips its extent to `[0, width]`, removes the x-extent of every keep-out
  that reaches into the headroom band `[y − clearance, y)` above it and keeps the stretches that are at
  least `minimum` wide, in surface order and then by x. A keep-out that only touches an end, the band or
  the surface takes nothing away, a box without area blocks nothing, and no perch has zero width.
  `perchAt(perches, surface, x)` is the first perch of a surface that carries `x` (ends included),
  `nearestPerch(perches, x, y)` the perch whose nearest point is closest (the first among equals), and
  `strideTo(x, goal, speed)` the next x one tick (1/64 s) later at `speed` pixels per second, the goal
  itself as soon as it is within one step — a walker never overshoots and stands still without speed.

  THE REFERENCE is `🐍️.py` beside this file and subtracts no interval. Every committed coordinate of a
  layout lies on a quarter-pixel lattice; the oracle cuts the stage into lattice cells and lets numpy
  boolean masks decide by membership of the cell centres which cells of a surface lie inside the
  viewport and under no keep-out of its band; the perches are the runs of free cells (`numpy.diff`).
  Standing is read from a raster of the lattice points the perches of a surface own, the nearest perch
  is found by brute force over the sampled points of every perch (`numpy.argmin` of the squared
  distances), and a walk is judged by its closed form: after `k` ticks the walker has covered
  `min(k × speed ÷ 64, distance)`. The walk is answered as the plain binary64 values of its stated
  ticks, refused unless the closed form reaches them within 1e-9, because the comparison grid of
  `pets-float-v1` is decimal (1e-9) and a number that is right to 1e-13 can round to the other side of
  it; perches are made of the committed numbers alone and agree exactly. The subjects are `perchesOf`,
  `perchAt`, `nearestPerch` and `strideTo` of `🔨️modules/🏞️terrain` in `@semio-tech/pets` and
  `perches_of`, `perch_at`, `nearest_perch`, `stride_to` of the `pets` crate.

  The layouts cover surfaces partly and wholly outside the viewport, above the clearance line and on
  it, on the floor and below it, keep-outs ending where the band begins and a quarter pixel inside it,
  beginning on the surface and a quarter pixel above it, touching the ends and a quarter pixel over
  them, nested, overlapping and abutting in three orders, without area, zero-width surfaces, stretches
  of exactly the minimum, a stage without clearance, and twelve scattered layouts.

  The vectors shared://🏞️terrain-walking/🔣️.json are generated, never hand-edited, from the Python
  reference in this directory, from the repository root:
  `.venv/Scripts/python.exe .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️02/QUIZ-PETS/generate_terrain_vectors.py`
  (`.venv/bin/python` outside Windows).

  @id-perches
  @level-fundamental
  @mode-differential
  Scenario: Surfaces are cut into perches by the viewport, the clearance and the keep-outs above them
    Given the committed vectors shared://🏞️terrain-walking/🔣️.json
    When the perches of every committed layout are cut with perchesOf
    Then every implementation projects the runs of free lattice cells of every surface as its perches, in surface order and then by x

  @id-standing
  @level-fundamental
  @mode-differential
  Scenario: A perch carries every x between its ends, the first one where two touch
    Given the committed vectors shared://🏞️terrain-walking/🔣️.json
    When every committed query is answered with perchAt
    Then every implementation projects the index of the perch that owns the lattice point of its surface, and none in a gap, beyond the ends or on an unknown surface

  @id-nearest
  @level-fundamental
  @mode-differential
  Scenario: The nearest perch is the one with the closest point, the first among equals
    Given the committed vectors shared://🏞️terrain-walking/🔣️.json
    When every committed point is answered with nearestPerch
    Then every implementation projects the index of the perch with the closest sampled point, and none without perches

  @id-strides
  @level-fundamental
  @mode-differential
  Scenario: A walk advances speed ÷ 64 pixels per tick and ends on its goal
    Given the committed vectors shared://🏞️terrain-walking/🔣️.json
    When every committed walk is advanced tick by tick with strideTo
    Then every implementation projects the x after every tick as min(k × speed ÷ 64, distance) covered towards the goal, never beyond it
