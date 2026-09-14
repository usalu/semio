@capability-flow-sankey-layout-twin
@oracle-d3-sankey
@comparison-viz-probe-v1
Feature: The TypeScript twin of the sankey layout reproduces d3-sankey for every node alignment
  `flow-sankey` measures the LaTeX kernel. This case measures the second subject of the same
  specification, the TypeScript twin `@semio-tech/print-viz-kernel`, on the same vectors and against
  the same oracle, because a kernel that exists twice is only a kernel if both copies answer alike.

  The platform gives a case one adapter per language and one subject per scenario, so the twin cannot
  share `flow-sankey`'s adapter with the LaTeX probe. It gets its own case instead; the vectors
  below are the vectors of `flow-sankey` verbatim, and the adapter reuses that case's oracle
  handlers unchanged so the two subjects are measured against literally the same reference numbers.

  Everything the base case says about the pipeline holds here word for word: `computeNodeValues`,
  `computeNodeDepths` and `computeNodeHeights` over the full frontier, `computeNodeLayers` with the
  four alignments, one common value scale with per-column centring, the six relaxation passes with
  `alpha = 0.99^i` and `beta = max(1 - alpha, (i+1)/iterations)`, `resolveCollisions` outwards from
  the middle node of a column and then clamped against both extents, and `computeLinkBreadths`.

  Every one of the four alignments is a scenario, because they are the only place where the layer
  assignment can diverge. The comparison covers the complete geometry — every node's `x0, y0, x1, y1`
  and every link's `y0, y1, width` — in the layout's own unit extent, so nothing about millimetres,
  themes or drawing enters the measurement. Node order is the first appearance of each endpoint in
  the edge list, source before target.

  Numbers are rounded onto the probe protocol's six-decimal emission grid on both sides, so the twin
  is held to the grid the LaTeX subject is held to and neither side is given a wider target.

  @id-align-justify
  @level-quick
  @mode-differential
  Scenario: The justify alignment agrees with d3-sankey
    Given the flow
      | source | target | value |
      | a      | c      | 12    |
      | b      | c      | 5     |
      | b      | d      | 8     |
      | c      | e      | 9     |
      | c      | f      | 8     |
      | d      | f      | 6     |
      | a      | d      | 3     |
      | e      | g      | 7     |
      | f      | g      | 10    |
    Then every node rectangle and every link band of the twin kernel agrees with d3-sankey

  @id-align-left
  @level-quick
  @mode-differential
  Scenario: The left alignment agrees with d3-sankey
    Given the flow
      | source | target | value |
      | a      | c      | 12    |
      | b      | c      | 5     |
      | b      | d      | 8     |
      | c      | e      | 9     |
      | c      | f      | 8     |
      | d      | f      | 6     |
      | a      | d      | 3     |
      | e      | g      | 7     |
      | f      | g      | 10    |
    Then every node rectangle and every link band of the twin kernel agrees with d3-sankey

  @id-align-right
  @level-quick
  @mode-differential
  Scenario: The right alignment agrees with d3-sankey
    Given the flow
      | source | target | value |
      | a      | c      | 12    |
      | b      | c      | 5     |
      | b      | d      | 8     |
      | c      | e      | 9     |
      | c      | f      | 8     |
      | d      | f      | 6     |
      | a      | d      | 3     |
      | e      | g      | 7     |
      | f      | g      | 10    |
    Then every node rectangle and every link band of the twin kernel agrees with d3-sankey

  @id-align-center
  @level-quick
  @mode-differential
  Scenario: The center alignment agrees with d3-sankey
    Given the flow
      | source | target | value |
      | a      | c      | 12    |
      | b      | c      | 5     |
      | b      | d      | 8     |
      | c      | e      | 9     |
      | c      | f      | 8     |
      | d      | f      | 6     |
      | a      | d      | 3     |
      | e      | g      | 7     |
      | f      | g      | 10    |
    Then every node rectangle and every link band of the twin kernel agrees with d3-sankey
