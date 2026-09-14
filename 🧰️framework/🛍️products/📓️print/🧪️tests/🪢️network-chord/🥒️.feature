@capability-network-chord-layout
@oracle-d3-chord
@comparison-viz-probe-v1
Feature: The chord layout reproduces d3-chord's group arcs and ribbon endpoints
  A chord diagram is entirely decided by its layout: the group arcs are the row sums of the matrix
  scaled onto `2*pi - padAngle*n`, each group's span is subdivided into one subgroup arc per column
  in the subgroup order, and a ribbon joins the two subgroup arcs of a cell pair, with the larger of
  the two values taken as the source. `semio-viz-network`'s `chord` layout is that algorithm, so the
  test is a straight numeric comparison with `d3-chord` — angles in radians, no drawing involved.

  The matrix is read off the graph: cell (i, j) is the sum of the weights of the edges from node i
  to node j, in the node order of the node table, which is why the vectors below are written as a
  weighted edge list. `symmetric` is left off, so the matrix stays asymmetric exactly as d3 receives
  it.

  Both sorting knobs are exercised, because they are where an implementation usually drifts: d3
  sorts the *group index* by the group totals and the *subgroup index* of every group independently
  by that row's values, and the padded scale is `max(0, tau - padAngle*n) / total` with the gap
  becoming `tau/n` only when the total is zero.

  @id-plain-matrix
  @level-quick
  @mode-differential
  Scenario: Group arcs, subgroup arcs and ribbons agree with d3-chord
    Given the weighted directed edges
      | source | target | weight |
      | n1     | n1     | 11     |
      | n1     | n2     | 3      |
      | n1     | n4     | 7      |
      | n2     | n1     | 5      |
      | n2     | n3     | 4      |
      | n3     | n2     | 6      |
      | n3     | n3     | 2      |
      | n4     | n1     | 9      |
      | n4     | n3     | 8      |
    And no padding and no sorting
    Then every group angle and every ribbon endpoint agrees with d3-chord

  @id-padded-and-sorted
  @level-quick
  @mode-differential
  Scenario: padAngle, sortGroups and sortSubgroups agree with d3-chord
    Given the weighted directed edges
      | source | target | weight |
      | n1     | n1     | 11     |
      | n1     | n2     | 3      |
      | n1     | n4     | 7      |
      | n2     | n1     | 5      |
      | n2     | n3     | 4      |
      | n3     | n2     | 6      |
      | n3     | n3     | 2      |
      | n4     | n1     | 9      |
      | n4     | n3     | 8      |
    And a pad angle of 0.05 with groups sorted descending and subgroups sorted ascending
    Then every group angle and every ribbon endpoint agrees with d3-chord
