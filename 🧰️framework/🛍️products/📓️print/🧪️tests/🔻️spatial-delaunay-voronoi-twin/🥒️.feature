@capability-viz-spatial-delaunay-twin
@oracle-d3-delaunay
@comparison-viz-probe-v1
Feature: The TypeScript twin of the Delaunay triangulation and the Voronoi diagram answers as the reference implementation does
  `🔺️spatial-delaunay-voronoi` measures the LaTeX kernel — the triangulation and the Voronoi diagram of semio-viz-spatial agree with d3-delaunay. This case measures the
  second subject of the same specification, the TypeScript twin `@semio-tech/print-viz-kernel`, on
  the same vectors and against the same oracle, because a kernel that exists twice is only a
  kernel if both copies answer alike.

  The platform gives a case one adapter per language and one subject per scenario, so the twin
  cannot share `🔺️spatial-delaunay-voronoi`'s adapter with the LaTeX probe subject. It gets its own case instead;
  the tables below are that case's tables character for character, and the adapter reuses its
  oracle handlers unchanged, so the two subjects meet literally the same reference numbers.

  Both subjects are rounded onto the probe protocol's emission grid before they are compared, so
  the twin is held to the grid the LaTeX subject is held to and neither is given a wider target.

  @id-delaunay-triangles
  @level-quick
  @mode-differential
  Scenario: The triangle set agrees with d3-delaunay
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
    Then the twin kernel and the reference implementation agree on the canonical triangle set

  @id-voronoi-cells
  @level-quick
  @mode-differential
  Scenario: The clipped Voronoi cells agree with d3-delaunay
    Given the clip rectangle
      | x0 | y0 | x1 | y1 |
      | 0  | 0  | 70 | 50 |
    And the point set demo-points
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
    Then the twin kernel and the reference implementation agree on the vertices of every cell
