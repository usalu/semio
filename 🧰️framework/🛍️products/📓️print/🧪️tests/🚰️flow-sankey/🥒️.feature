@capability-flow-sankey-layout
@oracle-d3-sankey
@comparison-viz-probe-v1
Feature: The sankey layout reproduces d3-sankey for every node alignment
  `semio-viz-flow`'s `sankey` layout is d3-sankey's pipeline written in expl3, step for step:
  `computeNodeValues` (a node carries the larger of its outgoing and its incoming total),
  `computeNodeDepths` and `computeNodeHeights` (both start with *every* node in the frontier, so the
  result is the longest path from any source respectively to any sink), `computeNodeLayers` with the
  four alignments, `initializeNodeBreadths` with one common value scale and the per-column centring,
  the six relaxation passes with `alpha = 0.99^i` and `beta = max(1 - alpha, (i+1)/iterations)`,
  `resolveCollisions` outwards from the middle node of a column and then clamped against both
  extents, and finally `computeLinkBreadths`.

  Every one of the four alignments is a scenario, because they are the only place where the layer
  assignment can diverge and each of them is a different rule: `left` takes the depth, `right` takes
  `layers - 1 - height`, `justify` pushes sinks to the last layer, and `center` pulls a node with no
  incoming links back to one before the earliest of its targets.

  The comparison covers the complete geometry — every node's `x0, y0, x1, y1` and every link's
  `y0, y1, width` — in the layout's own unit extent, so nothing about millimetres, themes or
  drawing enters the measurement. Node order is the first appearance of each endpoint in the edge
  list, source before target, which is the order a caller building d3's node array from the same
  edge list gets.

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
    Then every node rectangle and every link band agrees with d3-sankey

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
    Then every node rectangle and every link band agrees with d3-sankey

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
    Then every node rectangle and every link band agrees with d3-sankey

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
    Then every node rectangle and every link band agrees with d3-sankey
