@capability-viz-shape-curves-twin
@oracle-d3-shape
@comparison-viz-probe-v1
Feature: The TypeScript twin's curve interpolators emit the control points d3-shape emits
  `shape-curves` measures the LaTeX kernel. This case measures the second subject of the same
  specification, the TypeScript twin `@semio-tech/print-viz-kernel`, on the same vectors and against
  the same oracle, because a kernel that exists twice is only a kernel if both copies answer alike.

  The platform gives a case one adapter per language and one subject per scenario, so the twin cannot
  share `shape-curves`'s adapter with the LaTeX probe. It gets its own case instead; the vectors
  below are the vectors of `shape-curves` verbatim, and the adapter reuses that case's oracle
  handlers unchanged so the two subjects are measured against literally the same reference numbers.

  Numbers are rounded onto the probe protocol's six-decimal emission grid on both sides, so the twin
  is held to the grid the LaTeX subject is held to and neither side is given a wider target.

  @id-interpolators
  @level-quick
  @mode-differential
  Scenario: The eighteen d3 interpolators agree on one six-point series
    Given the twin kernel and the series
      | points                       | curves                                                                                                                                                                                                                    |
      | 0,0; 1,3; 2,1; 3,4; 4,2; 5,5 | linear, linear-closed, step, step-before, step-after, basis, basis-open, basis-closed, bundle, cardinal, cardinal-open, cardinal-closed, catmull-rom, catmull-rom-open, catmull-rom-closed, monotone-x, monotone-y, natural |
    Then the twin kernel and the reference implementation agree on every value

  @id-parameters
  @level-quick
  @mode-differential
  Scenario: Tension, alpha and beta move the control points the way d3 moves them
    Given the twin kernel and the parameterised curves
      | curve       | parameter   |
      | cardinal    | tension=0.5 |
      | catmull-rom | alpha=0     |
      | catmull-rom | alpha=1     |
      | bundle      | beta=0.5    |
    Then the twin kernel and the reference implementation agree on every value

  @id-degenerate
  @level-quick
  @mode-differential
  Scenario: One- and two-point lines degrade exactly as d3 degrades them
    Given the twin kernel and the short series
      | curve       | points        |
      | linear      | 2,7           |
      | basis       | 2,7; 4,9      |
      | natural     | 2,7; 4,9      |
      | catmull-rom | 2,7; 4,9; 6,3 |
    Then the twin kernel and the reference implementation agree on every value
