@capability-viz-hierarchy-pack-twin
@oracle-d3-hierarchy
@comparison-viz-probe-v1
Feature: The TypeScript twin places and sizes packed circles exactly as d3-hierarchy does
  `hierarchy-pack` measures the LaTeX kernel. This case measures the second subject of the same
  specification, the TypeScript twin `@semio-tech/print-viz-kernel`, on the same vectors and against
  the same oracle, because a kernel that exists twice is only a kernel if both copies answer alike.

  The platform gives a case one adapter per language and one subject per scenario, so the twin cannot
  share `hierarchy-pack`'s adapter with the LaTeX probe. It gets its own case instead; the vectors
  below are the vectors of `hierarchy-pack` verbatim, and the adapter reuses that case's oracle
  handlers unchanged so the two subjects are measured against literally the same reference numbers.

  Welzl's enclosure is randomised, and the twin steps the same seeded linear congruential generator
  d3 does. The order in which that stream is consumed is therefore part of the contract for the twin
  exactly as it is for the LaTeX subject, and a twin that packed the same circles in a different order
  would land on different, equally valid, coordinates and fail here.

  Numbers are rounded onto the probe protocol's six-decimal emission grid on both sides, so the twin
  is held to the grid the LaTeX subject is held to and neither side is given a wider target.

  @id-default-radius
  @level-quick
  @mode-differential
  Scenario: A pack over sqrt(value) matches d3 pack() on a square and an oblong frame
    Given the twin kernel and the pack extents
      | key    | width | height |
      | plain  | 100   | 100    |
      | oblong | 120   | 80     |
    Then the twin kernel and the reference implementation agree on every value

  @id-padding
  @level-quick
  @mode-differential
  Scenario: Padding separates the circles as d3 pack().padding() does
    Given the twin kernel and the pack extents
      | key    | width | height | padding |
      | padded | 100   | 100    | 3       |
    Then the twin kernel and the reference implementation agree on every value

  @id-explicit-radius
  @level-quick
  @mode-differential
  Scenario: An explicit radius takes d3's single-pass branch
    Given the twin kernel and the radii
      | key      | width | height | radius |
      | value    | 100   | 100    | value  |
      | constant | 100   | 100    | 4      |
    Then the twin kernel and the reference implementation agree on every value

  @id-unbalanced
  @level-quick
  @mode-differential
  Scenario: An unbalanced hierarchy consumes the same random stream as d3
    Given the twin kernel and the pack extents
      | key    | width | height | padding |
      | plain  | 90    | 90     |         |
      | padded | 90    | 90     | 2       |
    Then the twin kernel and the reference implementation agree on every value
