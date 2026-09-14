@capability-viz-hierarchy-aggregates-twin
@oracle-d3-hierarchy
@comparison-viz-probe-v1
Feature: The TypeScript twin aggregates, orders and traverses hierarchies exactly as d3-hierarchy does
  `hierarchy-aggregates` measures the LaTeX kernel. This case measures the second subject of the same
  specification, the TypeScript twin `@semio-tech/print-viz-kernel`, on the same vectors and against
  the same oracle, because a kernel that exists twice is only a kernel if both copies answer alike.

  The platform gives a case one adapter per language and one subject per scenario, so the twin cannot
  share `hierarchy-aggregates`'s adapter with the LaTeX probe. It gets its own case instead; the
  vectors below are the vectors of `hierarchy-aggregates` verbatim, and the adapter reuses that case's
  oracle handlers unchanged so the two subjects are measured against literally the same reference
  numbers.

  Numbers are rounded onto the probe protocol's six-decimal emission grid on both sides, so the twin
  is held to the grid the LaTeX subject is held to and neither side is given a wider target.

  The delimited-path scenario of `hierarchy-aggregates` has no counterpart here: the twin exposes no
  path constructor, so there is nothing of the twin left to measure once the prefixes are expanded.

  @id-sum-and-count
  @level-quick
  @mode-differential
  Scenario: sum and count aggregate over the postorder the way d3 does
    Given the twin kernel and the aggregations
      | hierarchy                 | aggregate |
      | demo-hierarchy-deep       | sum       |
      | demo-hierarchy-deep       | count     |
      | demo-hierarchy-unbalanced | sum       |
      | demo-hierarchy-unbalanced | count     |
    Then the twin kernel and the reference implementation agree on every value

  @id-depth-and-height
  @level-quick
  @mode-differential
  Scenario: depth, height and the postorder match d3's own traversals
    Given the twin kernel and the hierarchies
      | hierarchy                 |
      | demo-hierarchy-deep       |
      | demo-hierarchy-unbalanced |
    Then the twin kernel and the reference implementation agree on every value

  @id-sort
  @level-quick
  @mode-differential
  Scenario: Every comparator reorders the children the way the matching d3 sort does
    Given the twin kernel and the comparators
      | hierarchy                 | sort              |
      | demo-hierarchy-deep       | value-descending  |
      | demo-hierarchy-deep       | value-ascending   |
      | demo-hierarchy-deep       | name-descending   |
      | demo-hierarchy-unbalanced | height-descending |
      | demo-hierarchy-unbalanced | height-ascending  |
    Then the twin kernel and the reference implementation agree on every value

  @id-traversal
  @level-quick
  @mode-differential
  Scenario: leaves, ancestors and links list the same nodes in the same order as d3
    Given the twin kernel and the traversals
      | hierarchy                 | traversal | node |
      | demo-hierarchy-deep       | leaves    |      |
      | demo-hierarchy-unbalanced | leaves    |      |
      | demo-hierarchy-deep       | ancestors | c3   |
      | demo-hierarchy-unbalanced | ancestors | saab |
      | demo-hierarchy-deep       | links     |      |
    Then the twin kernel and the reference implementation agree on every value
