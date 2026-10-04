@capability-pets-clearance-proof
@oracle-pets-scipy
@comparison-pets-float-v1
Feature: Pets never overlap: bodies, free places, claimed corridors, guarded strides, seatings, heads, pushes and the last resort
  At the end of every tick the bodies of any two visible pets are apart (second round of the design, §18).
  A body is the size box of a species above its feet, the hover beneath a floater, what its posture adds
  (`leaning` for a held pet, `canopied` under a parachute) and a margin of 4 px on every side
  (`bodyOf`). Two bodies overlap when their open boxes intersect — bodies that touch are apart — and
  `overlaps(bodies)` names every overlapping pair, `nearMisses(bodies, margin)` every pair that is
  apart but closer than a margin. Everything that places a body keeps a seam of 1/64 px beside its
  neighbour, so no rounding can turn placed bodies into overlapping ones.

  Free places: `slotIn(x, extent, low, high, obstacles)` is the nearest place between two ends at which
  a box keeps a seam from every obstacle whose height it shares (the left one among equals, none
  without room); `spotOn` asks it for a perch with the footprint on it, `columnOver` for the whole
  column a landing comes down through.

  Order: `guardedStride(extent, stride, obstacles)` is the farthest a body may move sideways; it stops
  a seam before the first obstacle of its height that lies ahead (by the middles of the boxes), lets a
  body out of an overlap and never further in. Nobody passes anybody: `orderOf` and `orderKept` are
  the order along a perch, and `vaults` is the first condition of a swap by a hop.

  Claims: `claimOf(owner, from, extents, span, rest)` is the swept corridor of a plan, one slice per
  `span` ticks (the smallest box around the boxes of those ticks) and the place where the owner rests
  afterwards. `claimClear(claim, bodies, claims)` holds when, tick by tick, the plan overlaps nobody:
  another owner is where its body is until its own claim begins (for ever without one), then inside
  the slices of that claim, then at its rest; so two plans may cross one place at different ticks, but
  no plan may come to rest where somebody arrives later. `freeAt` is the test of everybody who moves
  without a plan: no other body, no corridor that is left (`obstaclesOf`, `pruned`), no rest.

  Seating: `seatOf(bodies, perch, margin)` seats a perch group after a survey with the least squares of
  the moves, keeping the order, a seam between neighbours and every box without its margin on the
  perch (pool adjacent violators, clamped into the perch); whoever does not fit leaves, the last in the
  list first. `scootFraction` and `scooted` move the group by one common share of the way.

  Heads, hands and the last resort: `headUnder(before, after, owner, bodies)` is the body on whose top
  a descending body lands, like a perch crossing (the highest, the first among equals); `liftOnto`,
  `restsOn`, `slideSide`, `slideStride` and `slideOf` carry the rider and slide it off.
  `pushedOut(extent, obstacles, iterations)` pushes a held body out of what it was dragged into by the
  shortest ways along the axes, or answers nothing when its rounds are over. `mustPoof(owner, stay,
  plans, bodies, claims, tick)` holds when a pet may not stay and none of its plans is clear;
  `evicted(bodies, claims, tick)` names whoever a moving surface carried into somebody who stays.

  THE REFERENCE is `🐍️.py` beside this file; it walks no interval and no list of obstacles the way the
  subjects do. Boxes are numpy rows and overlap when the rectangle they share has a positive width
  and height. Every committed coordinate lies on the quarter-pixel lattice, so every place a subject
  may answer lies on the 1/64 px lattice: free places are found by a search over every lattice place
  (`numpy.argmin` over the admissible ones), guarded strides and slides by a walk along the lattice
  that stops at the first place too close to an obstacle. Claims are sampled densely: the slices are
  the hulls `numpy.minimum.reduceat` and `numpy.maximum.reduceat` cut out of the path, every owner
  becomes a time line of one box per tick up to a horizon beyond the last plan, and a plan is clear
  when its time line shares nothing with any other at any tick. A seating is a quadratic programme:
  it is answered as the plain binary64 result of the pooling written out in the oracle — the
  comparison grid of `pets-float-v1` is decimal (1e-9) and a number that is right to 1e-13 can round to
  the other side — and refused unless `scipy.optimize.isotonic_regression`, clipped into the perch,
  reaches the same seats within 1e-9 and `scipy.optimize.minimize` (SLSQP) on the programme itself
  within 1e-5; the leavers are whoever makes the programme infeasible. Heads are found with boolean
  masks over all tops at once. The push of a held body is the stated rule replayed and judged twice by
  numpy: the place it ends at shares nothing with any obstacle, and out of a single obstacle it is the
  shortest of the four ways along the axes found on the lattice. The last resort is read from the dense
  time lines. The subjects are the constants and the functions of `🔨️modules/🚧️clearance` in
  `@semio-tech/pets` and the same names in snake case of the `pets` crate; they project their own
  constants, so a constant tuned in one language only, or without regenerating the vectors, fails.

  The vectors shared://🚧️clearance-proof/🔣️.json are generated, never hand-edited, from the Python
  reference in this directory, from the repository root:
  `.venv/Scripts/python.exe .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️02/QUIZ-PETS/generate_clearance_vectors.py`
  (`.venv/bin/python` outside Windows).

  @id-constants
  @level-fundamental
  @mode-conformance
  Scenario: The tuning constants are the ones the vectors were generated with
    Given the committed vectors shared://🚧️clearance-proof/🔣️.json
    When every implementation states its margin, seam, steerings, patience, delay, slide speeds, lean, lane lift and scoot haste
    Then every implementation projects the committed constants

  @id-bodies
  @level-fundamental
  @mode-differential
  Scenario: A body is the size box above the feet, the hover beneath them, the posture and the margin
    Given the committed vectors shared://🚧️clearance-proof/🔣️.json
    When the body of every committed actor is built with bodyOf, leaning and canopied
    Then every implementation projects the four edges numpy adds to the feet

  @id-crowds
  @level-fundamental
  @mode-differential
  Scenario: Overlaps and near misses are the pairs whose shared rectangle has an area or whose gap is below the margin
    Given the committed vectors shared://🚧️clearance-proof/🔣️.json
    When every committed crowd is judged with overlaps and nearMisses
    Then every implementation projects the overlapping pairs and the near pairs, by the first and then the second in the list

  @id-spots
  @level-fundamental
  @mode-differential
  Scenario: A free place is the nearest lattice place that keeps a seam from every obstacle of its height
    Given the committed vectors shared://🚧️clearance-proof/🔣️.json
    When every committed place is asked from slotIn, spotOn and columnOver
    Then every implementation projects the place the search over the 1/64 px lattice finds, the left one among equals, and none where there is no room

  @id-strides
  @level-fundamental
  @mode-differential
  Scenario: A guarded stride stops a seam before the first obstacle ahead
    Given the committed vectors shared://🚧️clearance-proof/🔣️.json
    When every committed stride is cut with guardedStride
    Then every implementation projects how far the walk along the lattice gets, the whole stride where nothing lies ahead, and nothing further into an overlap

  @id-orders
  @level-fundamental
  @mode-differential
  Scenario: The order along a perch is the order of the middles, and a hop swaps two only over the top
    Given the committed vectors shared://🚧️clearance-proof/🔣️.json
    When every committed order is asked from orderOf, orderKept and vaults
    Then every implementation projects the stable order of the middles, whether it was kept, and whether the rise clears the hurdle by a seam

  @id-claims
  @level-fundamental
  @mode-differential
  Scenario: A plan is clear when its time line shares nothing with any other time line at any tick
    Given the committed vectors shared://🚧️clearance-proof/🔣️.json
    When every committed plan is claimed with claimOf and judged with claimClear, sliceAt, freeAt, obstaclesOf, pruned and released
    Then every implementation projects the hulls of its slices, where it is at the stated ticks, the verdict of the dense time lines, and what is free and left at every stated tick

  @id-seatings
  @level-fundamental
  @mode-differential
  Scenario: A perch group is seated with the least moves that keep order, seams and the perch
    Given the committed vectors shared://🚧️clearance-proof/🔣️.json
    When every committed group is seated with seatOf and scooted one tick with scootFraction and scooted
    Then every implementation projects the moves the isotonic regression and the quadratic programme agree on, the leavers, the common share and the places after one tick

  @id-heads
  @level-fundamental
  @mode-differential
  Scenario: A descending body lands on the highest head it comes down onto and slides off it
    Given the committed vectors shared://🚧️clearance-proof/🔣️.json
    When every committed landing, rest, side and slide is asked from headUnder, liftOnto, restsOn, slideSide, slideStride and slideOf
    Then every implementation projects the host and the lift onto it, whether the rider rests, its side, the strides of a slide and one guarded tick of it

  @id-pushes
  @level-fundamental
  @mode-differential
  Scenario: A held body is pushed out of its obstacles by the shortest ways along the axes
    Given the committed vectors shared://🚧️clearance-proof/🔣️.json
    When every committed body is pushed with pushedOut
    Then every implementation projects the move to a place that shares nothing with any obstacle, and none where the rounds are over before it is free

  @id-resorts
  @level-fundamental
  @mode-differential
  Scenario: A pet poofs only when it may not stay and no plan is clear, and a surface evicts only whom it carried into somebody
    Given the committed vectors shared://🚧️clearance-proof/🔣️.json
    When every committed last resort is asked from mustPoof and evicted
    Then every implementation projects the verdict of the dense time lines and the evicted owners in list order
