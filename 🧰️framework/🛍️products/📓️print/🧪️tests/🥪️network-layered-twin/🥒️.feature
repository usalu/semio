@capability-network-layered-layout-twin
@oracle-dagre
@comparison-viz-probe-exact-v1
Feature: The TypeScript twin of the layered graph layout answers as the reference implementation does
  `🎚️network-layered` measures the LaTeX kernel — the layered layout ranks and orders a DAG the way a Sugiyama implementation does. This case measures the
  second subject of the same specification, the TypeScript twin `@semio-tech/print-viz-kernel`, on
  the same vectors and against the same oracle, because a kernel that exists twice is only a
  kernel if both copies answer alike.

  The platform gives a case one adapter per language and one subject per scenario, so the twin
  cannot share `🎚️network-layered`'s adapter with the LaTeX probe subject. It gets its own case instead;
  the tables below are that case's tables character for character, and the adapter reuses its
  oracle handlers unchanged, so the two subjects meet literally the same reference numbers.

  Both subjects are rounded onto the probe protocol's emission grid before they are compared, so
  the twin is held to the grid the LaTeX subject is held to and neither is given a wider target.

  @id-ranks-agree-with-dagre
  @level-quick
  @mode-differential
  Scenario: Layer assignment agrees with dagre's longest-path ranker
    Given the acyclic edges
      | source | target |
      | parse  | check  |
      | parse  | plan   |
      | check  | plan   |
      | plan   | opt    |
      | opt    | emit   |
      | emit   | link   |
      | link   | pack   |
      | check  | emit   |
    Then every node lands in the same layer as in dagre

  @id-slack-ranks-are-as-soon-as-possible
  @level-quick
  @mode-conformance
  Scenario: A node with slack is ranked as soon as possible, not as late as possible
    Given the acyclic edges and their expected layers
      | source | target | node | layer |
      | a      | b      | a    | 0     |
      | b      | c      | b    | 1     |
      | a      | d      | c    | 2     |
      |        |        | d    | 1     |
    Then every node lands in the layer the table names

  @id-barycenter-ordering
  @level-quick
  @mode-conformance
  Scenario: Barycentre sweeps put the nodes of a layer in the specified order
    Given the acyclic edges and their expected positions
      | source | target | node | layer | position |
      | r      | x      | r    | 0     | 1        |
      | r      | y      | x    | 1     | 1        |
      | r      | z      | y    | 1     | 3        |
      | x      | s      | z    | 1     | 2        |
      | z      | s      | s    | 2     | 1        |
    Then every node sits at the position the table names inside its layer
