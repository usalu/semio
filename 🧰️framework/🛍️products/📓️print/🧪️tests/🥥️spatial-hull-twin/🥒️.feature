@capability-viz-spatial-hull-twin
@oracle-d3-delaunay
@comparison-viz-probe-exact-v1
Feature: The TypeScript twin of the convex hull answers as the reference implementation does
  `🥚️spatial-hull` measures the LaTeX kernel — the convex hull of semio-viz-spatial agrees with d3-delaunay. This case measures the
  second subject of the same specification, the TypeScript twin `@semio-tech/print-viz-kernel`, on
  the same vectors and against the same oracle, because a kernel that exists twice is only a
  kernel if both copies answer alike.

  The platform gives a case one adapter per language and one subject per scenario, so the twin
  cannot share `🥚️spatial-hull`'s adapter with the LaTeX probe subject. It gets its own case instead;
  the tables below are that case's tables character for character, and the adapter reuses its
  oracle handlers unchanged, so the two subjects meet literally the same reference numbers.

  Both subjects are rounded onto the probe protocol's emission grid before they are compared, so
  the twin is held to the grid the LaTeX subject is held to and neither is given a wider target.

  @id-convex-hull
  @level-quick
  @mode-differential
  Scenario: The hull vertex cycle agrees with d3-delaunay
    Given the point set demo-points
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
    Then the twin kernel and the reference implementation agree on the hull cycle
