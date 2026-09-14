@capability-viz-spatial-hull
@oracle-d3-delaunay
@comparison-viz-probe-exact-v1
Feature: The convex hull of semio-viz-spatial agrees with d3-delaunay
  `\SemioVizHull` is Andrew's monotone chain: the points are sorted lexicographically, the lower and
  upper chains are built by popping every vertex that does not make a left turn, and the two chains
  are concatenated without their duplicated endpoints. `d3-delaunay` exposes `delaunay.hull` — the
  hull of the same point set, derived from Delaunator's sweep. Two independent constructions of the
  same object.

  The result is a cyclic sequence and the two constructions wind in opposite directions, because
  Delaunator works in a screen frame whose y axis points down. The oracle is therefore reversed and
  both sides are rotated to start at the lexicographically smallest point; after that normalisation
  the order is significant, and a hull with a different vertex or a different length fails.

  One documented difference of policy is deliberately kept out of the comparison: a point that lies
  exactly ON a hull edge is a hull vertex for Delaunator and is not one for the monotone chain,
  whose turn test pops every non-strict left turn. `demo-points-dense` has such a point — `[51,49]`
  is exactly collinear with `[35,47]` and `[67,51]` — so that set is not part of this scenario. The
  set that is, `demo-points`, is in strictly convex position, where the two policies cannot differ.

  Indices are integers, so the comparison profile is the exact one: there is nothing to round.

  Probe document: local://hull.tex is committed.

  @id-convex-hull
  @level-quick
  @mode-differential
  Scenario: The hull vertex cycle agrees with d3-delaunay
    Given the committed probe document local://hull.tex and the point set demo-points
      | x  | y  |
      | 12 | 14 |
      | 28 | 9  |
      | 41 | 26 |
      | 19 | 33 |
      | 55 | 17 |
      | 63 | 31 |
      | 34 | 41 |
      | 8  | 27 |
      | 47 | 7  |
      | 58 | 44 |
      | 25 | 20 |
      | 39 | 13 |
    Then the compiled probe and the reference implementation agree on the hull cycle
