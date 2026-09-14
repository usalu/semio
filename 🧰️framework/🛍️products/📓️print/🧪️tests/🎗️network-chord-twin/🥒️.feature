@capability-network-chord-layout-twin
@oracle-d3-chord
@comparison-viz-probe-v1
Feature: The TypeScript twin of the chord layout reproduces d3-chord's group arcs and ribbon endpoints
  `network-chord` measures the LaTeX kernel. This case measures the second subject of the same
  specification, the TypeScript twin `@semio-tech/print-viz-kernel`, on the same vectors and against
  the same oracle, because a kernel that exists twice is only a kernel if both copies answer alike.

  The platform gives a case one adapter per language and one subject per scenario, so the twin cannot
  share `network-chord`'s adapter with the LaTeX probe. It gets its own case instead; the vectors
  below are the vectors of `network-chord` verbatim, and the adapter reuses that case's oracle
  handlers unchanged so the two subjects are measured against literally the same reference numbers.

  Everything the base case says about the algorithm holds here word for word: the matrix is read off
  the weighted edge list, cell (i, j) is the summed weight of the edges from node i to node j in the
  node order of the node table, the matrix stays asymmetric exactly as d3 receives it, and both
  sorting knobs are exercised because they are where an implementation usually drifts.

  Numbers are rounded onto the probe protocol's six-decimal emission grid on both sides, so the twin
  is held to the grid the LaTeX subject is held to and neither side is given a wider target.

  @id-plain-matrix
  @level-quick
  @mode-differential
  Scenario: Group arcs, subgroup arcs and ribbons agree with d3-chord
    Given the weighted directed edges
      | source | target | weight |
      | n1     | n1     | 11     |
      | n1     | n2     | 3      |
      | n1     | n4     | 7      |
      | n2     | n1     | 5      |
      | n2     | n3     | 4      |
      | n3     | n2     | 6      |
      | n3     | n3     | 2      |
      | n4     | n1     | 9      |
      | n4     | n3     | 8      |
    And no padding and no sorting
    Then every group angle and every ribbon endpoint of the twin kernel agrees with d3-chord

  @id-padded-and-sorted
  @level-quick
  @mode-differential
  Scenario: padAngle, sortGroups and sortSubgroups agree with d3-chord
    Given the weighted directed edges
      | source | target | weight |
      | n1     | n1     | 11     |
      | n1     | n2     | 3      |
      | n1     | n4     | 7      |
      | n2     | n1     | 5      |
      | n2     | n3     | 4      |
      | n3     | n2     | 6      |
      | n3     | n3     | 2      |
      | n4     | n1     | 9      |
      | n4     | n3     | 8      |
    And a pad angle of 0.05 with groups sorted descending and subgroups sorted ascending
    Then every group angle and every ribbon endpoint of the twin kernel agrees with d3-chord
