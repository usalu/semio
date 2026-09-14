@capability-network-placement-layouts-twin
@oracle-d3-array
@comparison-viz-probe-v1
Feature: The TypeScript twin of the circular and arc placements answers as the reference implementation does
  `⭕️network-circular-arc` measures the LaTeX kernel — the closed-form placements put every node exactly where their definition says. This case measures the
  second subject of the same specification, the TypeScript twin `@semio-tech/print-viz-kernel`, on
  the same vectors and against the same oracle, because a kernel that exists twice is only a
  kernel if both copies answer alike.

  Two of the base case's four scenarios are deliberately absent: the twin implements `circular` and
  `arc` but has no `shell` and no `adjacency` layout, so there is nothing of its own to measure
  there. That gap is recorded in `📓️integration.md` rather than covered by an adapter that would
  re-implement the missing layout inside the test.

  The platform gives a case one adapter per language and one subject per scenario, so the twin
  cannot share `⭕️network-circular-arc`'s adapter with the LaTeX probe subject. It gets its own case instead;
  the tables below are that case's tables character for character, and the adapter reuses its
  oracle handlers unchanged, so the two subjects meet literally the same reference numbers.

  Both subjects are rounded onto the probe protocol's emission grid before they are compared, so
  the twin is held to the grid the LaTeX subject is held to and neither is given a wider target.

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
