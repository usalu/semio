@capability-viz-hierarchy-aggregates
@oracle-d3-hierarchy
@comparison-viz-probe-v1
Feature: Hierarchy aggregates, orders and traversals follow d3-hierarchy
  Every hierarchy layout reads the same four derived quantities — the aggregated value, the depth,
  the height and the child order — so they are measured before any geometry is. `semio-viz-hierarchy`
  computes them with d3's own traversals: `sum` and `count` in the postorder of `eachAfter`, the
  depth in the preorder of `eachBefore`, and `sort` as a stable merge sort of every child list.

  The traversal orders are themselves part of the contract: d3's `eachBefore` is a preorder with the
  children left to right, `eachAfter` the matching postorder, and `links()` walks the hierarchy
  breadth-first, which is a different order from both. A renderer that iterates in the wrong order
  draws the right shapes in the wrong colours, so every order is emitted as a list of identifiers
  rather than only as numbers.

  `sort` is measured on four comparators because they disagree in different ways: by value in both
  directions, by name (the string comparison, where a stable sort matters), and by height on the
  unbalanced fixture, where two children of one parent genuinely differ in height.

  The delimited-path constructor is measured too, because it builds nodes that no row names: only
  full paths carry a value, and every prefix has to appear exactly once, in first-seen order.

  @id-sum-and-count
  @level-quick
  @mode-differential
  Scenario: sum and count aggregate over the postorder the way d3 does
    Given the committed probe document local://hierarchy-aggregates.tex and the aggregations
      | hierarchy                 | aggregate |
      | demo-hierarchy-deep       | sum       |
      | demo-hierarchy-deep       | count     |
      | demo-hierarchy-unbalanced | sum       |
      | demo-hierarchy-unbalanced | count     |
    Then the compiled probe and the reference implementation agree on every value

  @id-depth-and-height
  @level-quick
  @mode-differential
  Scenario: depth, height and the postorder match d3's own traversals
    Given the committed probe document local://hierarchy-aggregates.tex and the hierarchies
      | hierarchy                 |
      | demo-hierarchy-deep       |
      | demo-hierarchy-unbalanced |
    Then the compiled probe and the reference implementation agree on every value

  @id-sort
  @level-quick
  @mode-differential
  Scenario: Every comparator reorders the children the way the matching d3 sort does
    Given the committed probe document local://hierarchy-aggregates.tex and the comparators
      | hierarchy                 | sort              |
      | demo-hierarchy-deep       | value-descending  |
      | demo-hierarchy-deep       | value-ascending   |
      | demo-hierarchy-deep       | name-descending   |
      | demo-hierarchy-unbalanced | height-descending |
      | demo-hierarchy-unbalanced | height-ascending  |
    Then the compiled probe and the reference implementation agree on every value

  @id-traversal
  @level-quick
  @mode-differential
  Scenario: leaves, ancestors and links list the same nodes in the same order as d3
    Given the committed probe document local://hierarchy-aggregates.tex and the traversals
      | hierarchy                 | traversal | node |
      | demo-hierarchy-deep       | leaves    |      |
      | demo-hierarchy-unbalanced | leaves    |      |
      | demo-hierarchy-deep       | ancestors | c3   |
      | demo-hierarchy-unbalanced | ancestors | saab |
      | demo-hierarchy-deep       | links     |      |
    Then the compiled probe and the reference implementation agree on every value

  @id-path-stratify
  @level-quick
  @mode-differential
  Scenario: A delimited path column creates every prefix once, in first-seen order
    Given the committed probe document local://hierarchy-aggregates.tex and the path table
      | hierarchy           | delimiter |
      | demo-hierarchy-path | /         |
    Then the compiled probe and the reference implementation agree on every value
