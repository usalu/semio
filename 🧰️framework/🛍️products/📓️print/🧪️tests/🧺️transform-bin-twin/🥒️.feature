@capability-viz-transform-bin-twin
@oracle-d3-array
@comparison-viz-probe-v1
Feature: The TypeScript twin of the bin transform lays out the bins d3-array's bin lays out
  `transform-bin` measures the LaTeX kernel. This case measures the second subject of the same
  specification, the TypeScript twin `@semio-tech/print-viz-kernel`, on the same vectors and against
  the same oracle, because a kernel that exists twice is only a kernel if both copies answer alike.

  The platform gives a case one adapter per language and one subject per scenario, so the twin cannot
  share `transform-bin`'s adapter with the LaTeX probe. It gets its own case instead; the vectors
  below are the vectors of `transform-bin` verbatim, and the adapter reuses that case's oracle
  handlers unchanged so the two subjects are measured against literally the same reference numbers.

  Numbers are rounded onto the probe protocol's six-decimal emission grid on both sides, so the twin
  is held to the grid the LaTeX subject is held to and neither side is given a wider target.

  @id-sturges
  @level-quick
  @mode-differential
  Scenario: Without a threshold option the bin count follows Sturges' rule
    Given the twin kernel and the sample
      | table             | column | thresholds |
      | demo-distribution | value  | sturges    |
    Then the twin kernel and the reference implementation agree on every edge and count

  @id-count-five
  @level-quick
  @mode-differential
  Scenario: A requested count of five produces the niced bins d3 produces
    Given the twin kernel and the sample
      | table             | column | thresholds |
      | demo-distribution | value  | count:5    |
    Then the twin kernel and the reference implementation agree on every edge and count

  @id-count-twenty
  @level-quick
  @mode-differential
  Scenario: A requested count of twenty produces half-unit bins
    Given the twin kernel and the sample
      | table             | column | thresholds |
      | demo-distribution | value  | count:20   |
    Then the twin kernel and the reference implementation agree on every edge and count

  @id-explicit-thresholds
  @level-quick
  @mode-differential
  Scenario: An explicit threshold list keeps the data extent as the outer edges
    Given the twin kernel and the sample
      | table             | column | thresholds |
      | demo-distribution | value  | 4;6;8      |
    Then the twin kernel and the reference implementation agree on every edge and count
