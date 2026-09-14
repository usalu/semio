@capability-viz-hierarchy-tree-twin
@capability-viz-hierarchy-cluster-twin
@oracle-d3-hierarchy
@comparison-viz-probe-v1
Feature: The TypeScript twin places tree and cluster nodes exactly as d3-hierarchy does
  `hierarchy-tree-cluster` measures the LaTeX kernel. This case measures the second subject of the
  same specification, the TypeScript twin `@semio-tech/print-viz-kernel`, on the same vectors and
  against the same oracle, because a kernel that exists twice is only a kernel if both copies answer
  alike.

  The platform gives a case one adapter per language and one subject per scenario, so the twin cannot
  share `hierarchy-tree-cluster`'s adapter with the LaTeX probe. It gets its own case instead; the
  vectors below are the vectors of `hierarchy-tree-cluster` verbatim, and the adapter reuses that
  case's oracle handlers unchanged so the two subjects are measured against literally the same
  reference numbers.

  Every scenario measures both fixtures for the reason the LaTeX case does: `demo-hierarchy-deep` is
  balanced and never exercises the contour threading of the tidy tree, while
  `demo-hierarchy-unbalanced` puts a four-level branch beside a single leaf and does.

  Numbers are rounded onto the probe protocol's six-decimal emission grid on both sides, so the twin
  is held to the grid the LaTeX subject is held to and neither side is given a wider target.

  @id-tree-size
  @level-quick
  @mode-differential
  Scenario: A tree normalised onto a size fills the frame the way d3 tree().size() does
    Given the twin kernel and the tree extents
      | hierarchy                  | mode | width | height |
      | demo-hierarchy-deep        | size | 100   | 60     |
      | demo-hierarchy-unbalanced  | size | 100   | 60     |
    Then the twin kernel and the reference implementation agree on every value

  @id-tree-node-size
  @level-quick
  @mode-differential
  Scenario: A tree with a fixed node size scales instead of normalising, as d3 tree().nodeSize() does
    Given the twin kernel and the tree extents
      | hierarchy                  | mode     | width | height |
      | demo-hierarchy-deep        | nodeSize | 12    | 20     |
      | demo-hierarchy-unbalanced  | nodeSize | 12    | 20     |
    Then the twin kernel and the reference implementation agree on every value

  @id-tree-separation
  @level-quick
  @mode-differential
  Scenario: A custom separation widens siblings and cousins as d3 tree().separation() does
    Given the twin kernel and the separations
      | hierarchy                  | mode     | siblings | cousins |
      | demo-hierarchy-deep        | size     | 1        | 2.5     |
      | demo-hierarchy-unbalanced  | nodeSize | 2        | 3       |
    Then the twin kernel and the reference implementation agree on every value

  @id-cluster-size
  @level-quick
  @mode-differential
  Scenario: A cluster puts every leaf on the last row as d3 cluster().size() does
    Given the twin kernel and the cluster extents
      | hierarchy                  | mode | width | height |
      | demo-hierarchy-deep        | size | 100   | 60     |
      | demo-hierarchy-unbalanced  | size | 100   | 60     |
    Then the twin kernel and the reference implementation agree on every value

  @id-cluster-node-size
  @level-quick
  @mode-differential
  Scenario: A cluster with a fixed node size anchors on its root as d3 cluster().nodeSize() does
    Given the twin kernel and the cluster extents
      | hierarchy                  | mode     | width | height | siblings | cousins |
      | demo-hierarchy-deep        | nodeSize | 12    | 20     |          |         |
      | demo-hierarchy-unbalanced  | nodeSize | 12    | 20     | 1.5      | 3       |
    Then the twin kernel and the reference implementation agree on every value
