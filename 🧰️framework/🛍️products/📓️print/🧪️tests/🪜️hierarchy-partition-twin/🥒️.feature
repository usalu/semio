@capability-viz-hierarchy-partition-twin
@capability-viz-hierarchy-sunburst-twin
@oracle-d3-hierarchy
@comparison-viz-probe-v1
Feature: The TypeScript twin bands and angles the partition exactly as d3-hierarchy does
  `hierarchy-partition` measures the LaTeX kernel. This case measures the second subject of the same
  specification, the TypeScript twin `@semio-tech/print-viz-kernel`, on the same vectors and against
  the same oracle, because a kernel that exists twice is only a kernel if both copies answer alike.

  The platform gives a case one adapter per language and one subject per scenario, so the twin cannot
  share `hierarchy-partition`'s adapter with the LaTeX probe. It gets its own case instead; the
  vectors below are the vectors of `hierarchy-partition` verbatim, and the adapter reuses that case's
  oracle handlers unchanged so the two subjects are measured against literally the same reference
  numbers.

  The sunburst is measured here for the same reason it is measured there: it is not a second
  algorithm but the same rectangles read as (angle, radius), so the twin has to answer it with the
  partition over the extent `2π × radius` and nothing else.

  Numbers are rounded onto the probe protocol's six-decimal emission grid on both sides, so the twin
  is held to the grid the LaTeX subject is held to and neither side is given a wider target.

  @id-icicle
  @level-quick
  @mode-differential
  Scenario: Partition bands, padded bands and rounded bands match d3 partition
    Given the twin kernel and the partitions
      | key     | width | height | padding | round |
      | plain   | 100   | 60     |         | false |
      | padded  | 100   | 60     | 1.5     | false |
      | rounded | 100   | 60     |         | true  |
    Then the twin kernel and the reference implementation agree on every value

  @id-sunburst
  @level-quick
  @mode-differential
  Scenario: A partition over a full turn gives the sunburst angles and radii of d3
    Given the twin kernel and the polar extent
      | turn             | radius |
      | 6.28318530717958 | 24     |
    Then the twin kernel and the reference implementation agree on every value

  @id-unbalanced
  @level-quick
  @mode-differential
  Scenario: An unbalanced hierarchy gets its row count from the root height, as d3 does
    Given the twin kernel and the partitions
      | key   | width | height |
      | plain | 80    | 50     |
    Then the twin kernel and the reference implementation agree on every value
