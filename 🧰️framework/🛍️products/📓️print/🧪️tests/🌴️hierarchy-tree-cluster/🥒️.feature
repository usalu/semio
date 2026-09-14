@capability-viz-hierarchy-tree
@capability-viz-hierarchy-cluster
@oracle-d3-hierarchy
@comparison-viz-probe-v1
Feature: The tree and cluster layouts place nodes exactly as d3-hierarchy does
  `semio-viz-hierarchy` implements taxonomy §78's `tree` transform as the Reingold-Tilford tidy
  algorithm with the Buchheim/Walker/Leipert linear-time improvements, command for command as
  `d3-hierarchy`'s `tree()`: a postorder first walk that gives every node a preliminary abscissa,
  an apportion step that threads the inside contours of neighbouring subtrees and moves whole
  subtrees apart on conflict, and a preorder second walk that sums the modifiers. `cluster()` is
  the simpler sibling: every leaf on the deepest row, every parent on the mean abscissa of its
  children.

  A tidy tree is only correct if the contour threading is correct, and a balanced tree never
  exercises it — the interesting shifts only happen where one subtree is deeper than its
  neighbour. Every scenario therefore measures two fixtures: `demo-hierarchy-deep`, a balanced
  three-level tree, and `demo-hierarchy-unbalanced`, whose left branch is four levels deep while
  its right branch is a single leaf.

  The two extent modes are measured separately because they are different code paths in d3:
  `size` normalises the preliminary coordinates onto a frame (and needs the left-most, right-most
  and deepest node), while `nodeSize` scales them by a fixed per-node spacing and never looks at
  the extent at all. `separation` is measured on its own because it feeds both the first walk and
  the `size` normalisation.

  Values are the node fields of the laid-out hierarchy, listed in the layout's own preorder
  (d3's `eachBefore`), so a wrong traversal order fails as loudly as a wrong coordinate.

  @id-tree-size
  @level-quick
  @mode-differential
  Scenario: A tree normalised onto a size fills the frame the way d3 tree().size() does
    Given the committed probe document local://hierarchy-tree-cluster.tex and the tree extents
      | hierarchy                  | mode | width | height |
      | demo-hierarchy-deep        | size | 100   | 60     |
      | demo-hierarchy-unbalanced  | size | 100   | 60     |
    Then the compiled probe and the reference implementation agree on every value

  @id-tree-node-size
  @level-quick
  @mode-differential
  Scenario: A tree with a fixed node size scales instead of normalising, as d3 tree().nodeSize() does
    Given the committed probe document local://hierarchy-tree-cluster.tex and the tree extents
      | hierarchy                  | mode     | width | height |
      | demo-hierarchy-deep        | nodeSize | 12    | 20     |
      | demo-hierarchy-unbalanced  | nodeSize | 12    | 20     |
    Then the compiled probe and the reference implementation agree on every value

  @id-tree-separation
  @level-quick
  @mode-differential
  Scenario: A custom separation widens siblings and cousins as d3 tree().separation() does
    Given the committed probe document local://hierarchy-tree-cluster.tex and the separations
      | hierarchy                  | mode     | siblings | cousins |
      | demo-hierarchy-deep        | size     | 1        | 2.5     |
      | demo-hierarchy-unbalanced  | nodeSize | 2        | 3       |
    Then the compiled probe and the reference implementation agree on every value

  @id-cluster-size
  @level-quick
  @mode-differential
  Scenario: A cluster puts every leaf on the last row as d3 cluster().size() does
    Given the committed probe document local://hierarchy-tree-cluster.tex and the cluster extents
      | hierarchy                  | mode | width | height |
      | demo-hierarchy-deep        | size | 100   | 60     |
      | demo-hierarchy-unbalanced  | size | 100   | 60     |
    Then the compiled probe and the reference implementation agree on every value

  @id-cluster-node-size
  @level-quick
  @mode-differential
  Scenario: A cluster with a fixed node size anchors on its root as d3 cluster().nodeSize() does
    Given the committed probe document local://hierarchy-tree-cluster.tex and the cluster extents
      | hierarchy                  | mode     | width | height | siblings | cousins |
      | demo-hierarchy-deep        | nodeSize | 12    | 20     |          |         |
      | demo-hierarchy-unbalanced  | nodeSize | 12    | 20     | 1.5      | 3       |
    Then the compiled probe and the reference implementation agree on every value
