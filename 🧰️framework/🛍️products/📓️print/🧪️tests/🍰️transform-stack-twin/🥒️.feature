@capability-viz-transform-stack-twin
@oracle-d3-shape
@comparison-viz-probe-v1
Feature: The TypeScript twin of the stack transform produces the baselines d3-shape produces
  `transform-stack` measures the LaTeX kernel. This case measures the second subject of the same
  specification, the TypeScript twin `@semio-tech/print-viz-kernel`, on the same vectors and against
  the same oracle, because a kernel that exists twice is only a kernel if both copies answer alike.

  The platform gives a case one adapter per language and one subject per scenario, so the twin cannot
  share `transform-stack`'s adapter with the LaTeX probe. It gets its own case instead; the vectors
  below are the vectors of `transform-stack` verbatim, and the adapter reuses that case's oracle
  handlers unchanged so the two subjects are measured against literally the same reference numbers.

  Numbers are rounded onto the probe protocol's six-decimal emission grid on both sides, so the twin
  is held to the grid the LaTeX subject is held to and neither side is given a wider target.

  @id-order-none
  @level-quick
  @mode-differential
  Scenario: Without an order the series stack in the order they appear
    Given the twin kernel and the series
      | series | values                |
      | alpha  | 4,7,6,9,8,12,11,14    |
      | beta   | 2,3,5,4,7,6,9,8       |
      | gamma  | 6,5,3,5,2,4,3,5       |
    Then the twin kernel and the reference implementation agree on every baseline

  @id-order-ascending
  @level-quick
  @mode-differential
  Scenario: The ascending order stacks the smallest total first
    Given the twin kernel and the same series
      | order     |
      | ascending |
    Then the twin kernel and the reference implementation agree on the stacking order and every baseline

  @id-order-reverse
  @level-quick
  @mode-differential
  Scenario: The reverse order stacks the given order backwards
    Given the twin kernel and the same series
      | order   |
      | reverse |
    Then the twin kernel and the reference implementation agree on the stacking order and every baseline

  @id-offset-expand
  @level-quick
  @mode-differential
  Scenario: The expand offset rescales every column onto the unit interval
    Given the twin kernel and the same series
      | offset |
      | expand |
    Then the twin kernel and the reference implementation agree on every baseline

  @id-offset-silhouette
  @level-quick
  @mode-differential
  Scenario: The silhouette offset centres every column on the horizontal axis
    Given the twin kernel and the same series
      | offset     |
      | silhouette |
    Then the twin kernel and the reference implementation agree on every baseline

  @id-offset-diverging
  @level-quick
  @mode-differential
  Scenario: The diverging offset stacks negative values below the axis
    Given the twin kernel and the signed series
      | series | values  |
      | up     | 3,4,2   |
      | down   | -2,-5,-1|
      | mixed  | 1,-3,4  |
    Then the twin kernel and the reference implementation agree on every baseline
