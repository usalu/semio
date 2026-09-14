@capability-repo.tree.child-sorting
@no-oracle-repo-tree-filtering-and-sorting
@comparison-ordered-json-v1
Feature: Tree children carry one deterministic order
  The codebase view is assembled through maps, so the order it comes out in is decided afterwards
  by one rule and one rule only: a folder sorts before a non-folder, and otherwise the label
  decides. The rule reaches every level of the tree, not just the top one.

  The vectors come from shared://🔀️sort-vectors.json, whose first two entries restate the Go
  original's own `TestSortTreeChildren` and whose remaining entries pin what that test left open.

  @id-sorts-every-vector
  @level-fundamental
  @mode-conformance
  Scenario: Each vector sorts into the stated order
    Given the four sorting vectors
    When the host sorts the children of each tree
    Then every outline equals the stated one, folders lead their level regardless of label, and every nested level is sorted too

  @id-sorting-is-idempotent
  @level-quick
  @mode-property
  Scenario: Sorting an already sorted tree changes nothing
    Given every vector of the fixture
    When the host sorts each tree twice
    Then the second pass returns the outline of the first
