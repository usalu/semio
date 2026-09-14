@capability-viz-layout-contour-twin
@oracle-d3-contour
@comparison-viz-probe-v1
Feature: The TypeScript twin of the marching-squares contours answers as the reference implementation does
  `🏔️spatial-contours-density` measures the LaTeX kernel — isolines and kernel density in semio-viz-spatial agree with d3-contour. This case measures the
  second subject of the same specification, the TypeScript twin `@semio-tech/print-viz-kernel`, on
  the same vectors and against the same oracle, because a kernel that exists twice is only a
  kernel if both copies answer alike.

  The base case's second scenario, `kernel-density`, is deliberately absent: the twin's
  `vizDensity2d` divides the kernel sum by the sample size and the LaTeX `\SemioVizDensity` does not,
  so the two are not yet the same specification. That divergence is recorded in `📓️integration.md`
  rather than hidden behind an adapter that rescales one side.

  The platform gives a case one adapter per language and one subject per scenario, so the twin
  cannot share `🏔️spatial-contours-density`'s adapter with the LaTeX probe subject. It gets its own case instead;
  the tables below are that case's tables character for character, and the adapter reuses its
  oracle handlers unchanged, so the two subjects meet literally the same reference numbers.

  Both subjects are rounded onto the probe protocol's emission grid before they are compared, so
  the twin is held to the grid the LaTeX subject is held to and neither is given a wider target.

  @id-marching-squares
  @level-quick
  @mode-differential
  Scenario: Isoline rings of a value grid agree with d3-contour
    Given the grid demo-grid
      | width | height | thresholds |
      | 6     | 5      | 4, 6       |
    Then the twin kernel and the reference implementation agree on every isoline vertex
