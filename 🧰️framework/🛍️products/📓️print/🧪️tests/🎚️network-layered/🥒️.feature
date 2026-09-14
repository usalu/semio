@capability-network-layered-layout
@oracle-dagre
@comparison-viz-probe-exact-v1
Feature: The layered layout ranks and orders a DAG the way a Sugiyama implementation does
  `semio-viz-network`'s `layered` layout is the classical Sugiyama pipeline: back edges found by a
  DFS in node-index order are marked reversed so the graph becomes acyclic, every node is assigned
  the length of the longest path that reaches it, the nodes of each layer are then reordered by the
  barycentre of their neighbours in `sweeps` alternating downward and upward passes, and the
  cross-axis coordinate is the simple median assignment.

  **What is compared against dagre, and what deliberately is not.**

  *Ranks* are compared, exactly, as integers — that is the part of Sugiyama with a single right
  answer once the ranker is fixed, and dagre exposes `ranker: "longest-path"` so both sides run the
  same ranker. There is one documented difference and the scenarios are built around it: dagre's
  `longest-path` is *as late as possible* — it walks back from the sinks, so a node with slack is
  pushed towards its successors — while this kernel's is *as soon as possible*, which keeps every
  source on the first layer where a reader expects it. On a graph in which every node lies on some
  longest source-to-sink path the two coincide, and that is the differential scenario. The graph
  with slack gets its own conformance scenario with the expected ASAP ranks written out, so the
  difference is specified rather than hidden.

  *Ordering inside a layer* is not compared to dagre. dagre orders with the median heuristic plus a
  transpose pass; this kernel orders by barycentre with a fixed number of sweeps. Both are
  heuristics for the same NP-hard problem and neither is a reference for the other, so the ordering
  is pinned by a conformance scenario with the expected positions instead. That scenario also fixes
  the rule for a node the sweep gives nothing to average over: a node with no neighbour in the
  reference layer keeps its own current position as its barycentre, which is why the upward sweep of
  the third scenario leaves the childless middle node behind its two siblings.

  *Coordinates* are not compared to anything: dagre assigns them with Brandes–Köpf, this kernel with
  the simple median method. Only the layer index and the position inside the layer are asserted, and
  they are integers, so the comparison profile is `viz-probe-exact-v1`.

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
