@capability-network-placement-layouts
@oracle-d3-array
@comparison-viz-probe-v1
Feature: The closed-form placements put every node exactly where their definition says
  `circular`, `shell`, `grid`, `arc` and `adjacency` are not heuristics: each is a formula over the
  node's position in an ordering, and that formula is the whole specification. d3 has no equivalent
  to compare against — `d3-chord` places arcs of a matrix, not nodes of a graph — so these scenarios
  are conformance vectors, and `d3-array`'s `range` builds the expected sequence on the oracle side
  so the expectation is generated from the definition rather than copied from an earlier run.

  The definitions under test:

  – **circular** — the k-th node of the ordering (k counted from zero) sits at angle
    `startAngle + (endAngle - startAngle) * k / n` on a circle of `radius` around the centre, with
    the angle measured the way `sin`/`cos` measure it, so node 0 is at `(radius, 0)`.
  – **shell** — the ordering is cut into the shells `shells={…}`; inside a shell of size `m` the
    j-th node takes `startAngle + (endAngle - startAngle) * j / m`, and the shell's radius is
    `innerRadius + radiusStep * (shell - 1)`.
  – **arc** — the k-th node sits at `originX + spacing * k` on one axis, which is what makes an arc
    diagram's ordering the only thing a reader has to follow.
  – **adjacency** — no geometry at all, only the permutation: the rank of every node under the
    chosen ordering. `degree` sorts by descending degree, ties keeping the declaration order, which
    is why the two nodes of degree three below rank in the order they first appear in the edge list.

  Every scenario states its ordering explicitly, because the ordering is the half of these layouts
  that a reimplementation gets wrong.

  @id-circular-equal-spacing
  @level-quick
  @mode-conformance
  Scenario: Circular places the ordering at equal angles on the radius
    Given the graph edges
      | source | target |
      | a      | b      |
      | b      | c      |
      | c      | d      |
      | d      | e      |
      | e      | f      |
      | f      | a      |
    And a radius of 10 in declaration order over the full turn
    Then every node sits at the angle its position in the ordering defines

  @id-shell-rings
  @level-quick
  @mode-conformance
  Scenario: Shell cuts the ordering into rings of the given sizes
    Given the graph edges
      | source | target |
      | a      | b      |
      | b      | c      |
      | c      | d      |
      | d      | e      |
      | e      | f      |
      | f      | a      |
    And the shells 2 and 4 with inner radius 3 and radius step 5
    Then every node sits on the ring its position in the ordering defines

  @id-arc-spacing
  @level-quick
  @mode-conformance
  Scenario: Arc lays the ordering out along one axis
    Given the graph edges
      | source | target |
      | a      | b      |
      | b      | c      |
      | c      | d      |
      | d      | e      |
      | e      | f      |
      | f      | a      |
    And a spacing of 6 in declaration order
    Then every node sits at the axis position its place in the ordering defines

  @id-adjacency-degree-order
  @level-quick
  @mode-conformance
  Scenario: The adjacency ordering ranks the nodes by descending degree
    Given the graph edges and the expected ranks
      | source | target | node | rank |
      | a      | b      | a    | 1    |
      | a      | c      | b    | 2    |
      | a      | d      | c    | 3    |
      | b      | c      | g    | 4    |
      | b      | e      | d    | 5    |
      | a      | f      | e    | 6    |
      | g      | a      | f    | 7    |
      | g      | b      |      |      |
      | g      | c      |      |      |
    Then every node carries the rank the table names
