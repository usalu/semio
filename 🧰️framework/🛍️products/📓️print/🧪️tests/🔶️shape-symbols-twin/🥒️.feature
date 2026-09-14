@capability-viz-shape-symbol-twin
@oracle-d3-shape
@comparison-viz-probe-v1
Feature: The TypeScript twin draws every d3 symbol type from its own area, not from a radius
  `shape-symbols` measures the LaTeX kernel. This case measures the second subject of the same
  specification, the TypeScript twin `@semio-tech/print-viz-kernel`, on the same vectors and against
  the same oracle, because a kernel that exists twice is only a kernel if both copies answer alike.

  The platform gives a case one adapter per language and one subject per scenario, so the twin cannot
  share `shape-symbols`'s adapter with the LaTeX probe. It gets its own case instead; the vectors
  below are the vectors of `shape-symbols` verbatim, and the adapter reuses that case's oracle
  handlers unchanged so the two subjects are measured against literally the same reference numbers.

  Numbers are rounded onto the probe protocol's six-decimal emission grid on both sides, so the twin
  is held to the grid the LaTeX subject is held to and neither side is given a wider target.

  @id-symbol-paths
  @level-quick
  @mode-differential
  Scenario: Thirteen symbol types at three areas draw the paths d3-shape draws
    Given the twin kernel and the symbols
      | types                                                                                            | sizes        |
      | circle, cross, diamond, diamond2, plus, square, square2, star, times, triangle, triangle2, wye, asterisk | 64, 17.5, 200 |
    Then the twin kernel and the reference implementation agree on every value
