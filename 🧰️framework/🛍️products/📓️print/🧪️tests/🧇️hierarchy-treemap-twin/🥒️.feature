@capability-viz-hierarchy-treemap-twin
@oracle-d3-hierarchy
@comparison-viz-probe-v1
Feature: The TypeScript twin tiles treemap rectangles exactly as d3-hierarchy does
  `hierarchy-treemap` measures the LaTeX kernel. This case measures the second subject of the same
  specification, the TypeScript twin `@semio-tech/print-viz-kernel`, on the same vectors and against
  the same oracle, because a kernel that exists twice is only a kernel if both copies answer alike.

  The platform gives a case one adapter per language and one subject per scenario, so the twin cannot
  share `hierarchy-treemap`'s adapter with the LaTeX probe. It gets its own case instead; the vectors
  below are the vectors of `hierarchy-treemap` verbatim, and the adapter reuses that case's oracle
  handlers unchanged so the two subjects are measured against literally the same reference numbers.

  All six tilings are measured on the twin for the reason they are measured on the LaTeX subject: a
  tiling is a choice of algorithm rather than a style, and the padding is measured on its own because
  the outer padding shrinks a node's own rectangle while half the inner padding is pushed onto a
  per-depth stack and taken off every child.

  The custom squarify ratio of `hierarchy-treemap` has no counterpart here: the twin exposes squarify
  only at the golden ratio, so that scenario is not part of this specification's second subject.

  Numbers are rounded onto the probe protocol's six-decimal emission grid on both sides, so the twin
  is held to the grid the LaTeX subject is held to and neither side is given a wider target.

  @id-tiling
  @level-quick
  @mode-differential
  Scenario: Every d3 tiling splits demo-hierarchy-deep into the same rectangles
    Given the twin kernel and the tilings
      | tile       | width | height |
      | squarify   | 100   | 60     |
      | resquarify | 100   | 60     |
      | slice      | 100   | 60     |
      | dice       | 100   | 60     |
      | slice-dice | 100   | 60     |
      | binary     | 100   | 60     |
    Then the twin kernel and the reference implementation agree on every value

  @id-padding
  @level-quick
  @mode-differential
  Scenario: Inner and outer padding shrink the rectangles as d3 treemap padding does
    Given the twin kernel and the paddings
      | key     | padding | paddingInner | paddingTop | paddingRight | paddingBottom | paddingLeft |
      | padding | 2       |              |            |              |               |             |
      | nested  |         | 1.5          | 6          | 1            | 1             | 1           |
    Then the twin kernel and the reference implementation agree on every value

  @id-unbalanced
  @level-quick
  @mode-differential
  Scenario: An unbalanced hierarchy tiles as d3 does at every depth
    Given the twin kernel and the tilings
      | tile     | width | height |
      | squarify | 80    | 50     |
      | binary   | 80    | 50     |
    Then the twin kernel and the reference implementation agree on every value
