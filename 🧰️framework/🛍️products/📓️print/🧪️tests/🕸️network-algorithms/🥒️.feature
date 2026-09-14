@capability-network-graph-algorithms
@oracle-d3-array
@comparison-viz-probe-exact-v1
Feature: The graph algorithms under the layouts return the orders and labels they promise
  Every layout in `semio-viz-network` stands on four graph algorithms — degrees, connected
  components, breadth-first and depth-first traversal, and a topological order — and none of them is
  visible in a rendered figure. They are specified here directly, because a layout that ranks or
  colours by a wrong component is wrong long before anything is drawn.

  **Why the oracle is a second implementation and not a graph library.** `graphology` would be the
  obvious reference, but it is not a registered oracle of this repository and adding a package needs
  a surveyed rationale, a license and a production-reachability check. These four algorithms are
  textbook and short, so the oracle is an independent implementation written in the adapter against
  `d3-array` — a different language, a different data structure (adjacency arrays rather than
  expl3's indexed sequences) and a different author's reading of the same definitions. That is what
  a differential test needs; what it does not give is an *external* authority, so every scenario
  also states the tie-breaking rule in prose, and the rules are what both sides implement:

  – node index order is the first appearance of an endpoint in the edge list, source before target;
  – `degree` counts an edge at both of its endpoints, so a self-loop counts twice;
  – components are numbered in the order their lowest-indexed node is reached;
  – breadth-first and depth-first visit a node's neighbours in edge-declaration order, and
    depth-first is preorder;
  – the topological order is Kahn's, and among the nodes whose remaining in-degree has reached zero
    it always takes the lowest index — which is what makes it reproducible at all.

  Everything compared here is an integer, so the profile is `viz-probe-exact-v1`.

  @id-degrees
  @level-quick
  @mode-differential
  Scenario: Degree, in-degree and out-degree agree with the reference implementation
    Given the directed edges
      | source | target |
      | a      | b      |
      | a      | c      |
      | b      | d      |
      | c      | d      |
      | e      | f      |
    Then every node's degree, in-degree and out-degree agrees

  @id-connected-components
  @level-quick
  @mode-differential
  Scenario: Connected components are numbered in discovery order
    Given the directed edges
      | source | target |
      | a      | b      |
      | a      | c      |
      | b      | d      |
      | c      | d      |
      | e      | f      |
      | g      | h      |
      | h      | e      |
    Then every node carries the component number the reference implementation gives it

  @id-topological-order
  @level-quick
  @mode-differential
  Scenario: Kahn's order with the lowest-index tie-break agrees with the reference implementation
    Given the directed edges
      | source | target |
      | parse  | check  |
      | parse  | plan   |
      | check  | plan   |
      | plan   | opt    |
      | opt    | emit   |
      | check  | emit   |
      | emit   | link   |
    Then the topological order agrees

  @id-traversal-order
  @level-quick
  @mode-differential
  Scenario: Breadth-first and depth-first traversal agree with the reference implementation
    Given the directed edges
      | source | target |
      | a      | b      |
      | a      | c      |
      | b      | d      |
      | c      | d      |
      | d      | e      |
      | c      | f      |
    Then both traversal orders agree

  @id-lcg-stream
  @level-quick
  @mode-conformance
  Scenario: The seeded generator is d3's linear congruential generator
    Given the expected stream of the generator seeded with one
      | draw | state      |
      | 1    | 1015568748 |
      | 2    | 1586005467 |
      | 3    | 2165703038 |
      | 4    | 3027450565 |
      | 5    | 217083232  |
    Then the kernel's generator produces exactly that stream
