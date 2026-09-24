@capability-viz-layout-contour
@oracle-d3-contour
@comparison-viz-probe-v1
Feature: Isolines and kernel density in semio-viz-spatial agree with d3-contour
  `\SemioVizContour` walks a row-major value grid cell by cell, classifies the four corners against
  the threshold into one of sixteen marching-squares cases, places each crossing on its edge at the
  linear fraction `(t - a) / (b - a)`, and joins the resulting segments end to end into rings. That
  is the same geometry `d3-contour` produces through its case table and its `smooth` step.

  One coordinate convention differs and is reported here rather than hidden: node (i,j) sits at
  (i,j) in `semio-viz-spatial`, while `d3-contour` writes its case table on half-integer cell
  corners and therefore reports the identical isoline shifted by exactly +0.5 in both axes. The
  oracle subtracts that constant offset. It also removes the consecutive duplicate vertices d3
  emits where two segments meet exactly on a grid node, and the repeated closing vertex, neither of
  which is geometry.

  A second, larger deviation decides which thresholds are tested: `d3-contour` frames the grid with
  a border of −∞ and therefore closes an isoline that reaches the edge along that border.
  `semio-viz-spatial` does not; an isoline that leaves the grid stays an open polyline. Only
  thresholds whose isolines are strictly interior are compared — 4 and 6 on `demo-grid`, and 0.0046
  on the kernel-density grid. Threshold 2 on `demo-grid` is the boundary case and is deliberately
  excluded; the deviation is recorded in the GEO-SPATIAL status file, not papered over here.

  The kernel-density scenario measures two things at once. The grid itself is the exact Gaussian sum
  `Σ exp(−r²/2h²) / (2πh²)` — `d3-contour`'s `contourDensity` deliberately approximates that sum
  with three box blurs, so the grid values are compared against an independent implementation of the
  stated specification written in the adapter, not against d3. The isolines of that grid, which is
  what a density map actually draws, are compared against `d3-contour` proper.

  Probe documents: shared://🏔️spatial-contours-density/contours.tex and shared://🏔️spatial-contours-density/density.tex are both committed.

  @id-marching-squares
  @level-quick
  @mode-differential
  Scenario: Isoline rings of a value grid agree with d3-contour
    Given the committed probe document shared://🏔️spatial-contours-density/contours.tex and the grid demo-grid
      | width | height | thresholds |
      | 6     | 5      | 4, 6       |
    Then the compiled probe and the reference implementation agree on every isoline vertex

  @id-kernel-density
  @level-quick
  @mode-differential
  Scenario: A kernel-density grid and its isolines agree with the specification and with d3-contour
    Given the committed probe document shared://🏔️spatial-contours-density/density.tex and the density parameters
      | width | height | originX | originY | cell | bandwidth | threshold |
      | 7     | 5      | 6       | 6       | 9    | 11        | 0.0046    |
    Then the compiled probe and the reference implementation agree on every grid value and every isoline vertex
