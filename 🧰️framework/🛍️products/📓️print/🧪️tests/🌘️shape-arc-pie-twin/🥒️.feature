@capability-viz-shape-arc-twin
@oracle-d3-shape
@comparison-viz-probe-v1
Feature: The TypeScript twin's arc geometry and pie angles are the ones d3-shape computes
  `shape-arc-pie` measures the LaTeX kernel. This case measures the second subject of the same
  specification, the TypeScript twin `@semio-tech/print-viz-kernel`, on the same vectors and against
  the same oracle, because a kernel that exists twice is only a kernel if both copies answer alike.

  The platform gives a case one adapter per language and one subject per scenario, so the twin cannot
  share `shape-arc-pie`'s adapter with the LaTeX probe. It gets its own case instead; the vectors
  below are the vectors of `shape-arc-pie` verbatim, and the adapter reuses that case's oracle
  handlers unchanged so the two subjects are measured against literally the same reference numbers.

  Numbers are rounded onto the probe protocol's six-decimal emission grid on both sides, so the twin
  is held to the grid the LaTeX subject is held to and neither side is given a wider target.

  @id-arc-geometry
  @level-quick
  @mode-differential
  Scenario: Every branch of d3's arc generator produces the same path
    Given the twin kernel and the arcs
      | tag          | innerRadius | outerRadius | startAngle | endAngle | padAngle | padRadius | cornerRadius |
      | plain        | 0           | 100         | 0          | 1.2      | 0        |           | 0            |
      | annular      | 40          | 100         | 0.3        | 2.5      | 0        |           | 0            |
      | pad          | 40          | 100         | 0.3        | 2.5      | 0.05     |           | 0            |
      | pad-radius   | 40          | 100         | 0.3        | 2.5      | 0.05     | 120       | 0            |
      | corner       | 40          | 100         | 0.3        | 2.5      | 0        |           | 12           |
      | pad-corner   | 40          | 100         | 0.3        | 2.5      | 0.05     |           | 12           |
      | corner-solid | 0           | 100         | 0.3        | 2.5      | 0        |           | 12           |
      | full         | 40          | 100         | 0          | 6.283185307179586 | 0 |      | 0            |
      | full-disc    | 0           | 100         | 0          | 6.283185307179586 | 0 |      | 0            |
      | counter      | 40          | 100         | 2.5        | 0.3      | 0        |           | 0            |
      | collapsed    | 40          | 100         | 0          | 0.2      | 0.5      |           | 0            |
    Then the twin kernel and the reference implementation agree on every value

  @id-arc-centroid
  @level-quick
  @mode-differential
  Scenario: The arc centroid is the midpoint in both radius and angle
    Given the twin kernel and the arcs
      | tag     | innerRadius | outerRadius | startAngle | endAngle |
      | annular | 40          | 100         | 0.3        | 2.5      |
      | wedge   | 0           | 60          | 1          | 1.9      |
      | counter | 10          | 30          | 2.5        | 0.3      |
    Then the twin kernel and the reference implementation agree on every value

  @id-pie-angles
  @level-quick
  @mode-differential
  Scenario: Pie start and end angles follow d3's ordering, padding and zero handling
    Given the twin kernel and the pies
      | tag        | value   | startAngle | endAngle          | padAngle | sort       |
      | descending | 1,2,3,4 | 0          | 6.283185307179586 | 0        | descending |
      | padded     | 1,2,3,4 | 0          | 6.283185307179586 | 0.05     | descending |
      | unsorted   | 5,1,4,2 | 0.5        | 4                 | 0        | none       |
      | ascending  | 5,1,4,2 | 0          | 6.283185307179586 | 0        | ascending  |
      | all-zero   | 0,0,0   | 0          | 6.283185307179586 | 0        | descending |
    Then the twin kernel and the reference implementation agree on every value
