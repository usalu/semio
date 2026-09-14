@capability-viz-spatial-hexbin-twin
@oracle-d3-hexbin
@comparison-viz-probe-v1
Feature: The TypeScript twin of hexagonal binning answers as the reference implementation does
  `🐝️spatial-hexbin` measures the LaTeX kernel — hexagonal binning in semio-viz-spatial agrees with d3-hexbin. This case measures the
  second subject of the same specification, the TypeScript twin `@semio-tech/print-viz-kernel`, on
  the same vectors and against the same oracle, because a kernel that exists twice is only a
  kernel if both copies answer alike.

  The platform gives a case one adapter per language and one subject per scenario, so the twin
  cannot share `🐝️spatial-hexbin`'s adapter with the LaTeX probe subject. It gets its own case instead;
  the tables below are that case's tables character for character, and the adapter reuses its
  oracle handlers unchanged, so the two subjects meet literally the same reference numbers.

  Both subjects are rounded onto the probe protocol's emission grid before they are compared, so
  the twin is held to the grid the LaTeX subject is held to and neither is given a wider target.

  @id-hexagonal-binning
  @level-quick
  @mode-differential
  Scenario: Bin centres and counts agree with d3-hexbin
    Given the radii
      | radius |
      | 6      |
      | 10     |
    Then the twin kernel and the reference implementation agree on every bin centre and count
