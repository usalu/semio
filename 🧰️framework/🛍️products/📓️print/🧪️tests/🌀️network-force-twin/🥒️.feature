@capability-network-force-layout-twin
@oracle-d3-force
@comparison-viz-probe-coarse-v1
Feature: The TypeScript twin of the force layout reproduces d3-force tick for tick
  `network-force` measures the LaTeX kernel. This case measures the second subject of the same
  specification, the TypeScript twin `@semio-tech/print-viz-kernel`, on the same vectors and against
  the same oracle, because a kernel that exists twice is only a kernel if both copies answer alike.

  The platform gives a case one adapter per language and one subject per scenario, so the twin cannot
  share `network-force`'s adapter with the LaTeX probe. It gets its own case instead; the vectors
  below are the vectors of `network-force` verbatim, and the adapter reuses that case's oracle
  handlers unchanged so the two subjects are measured against literally the same reference numbers.

  Everything the base case says about the reference holds here word for word. `theta` is 0 on the
  oracle, so d3's Barnes–Hut approximation never enters the measurement and the many-body force is
  evaluated exactly, once per unordered pair, on both sides. Only the iteration count is compared,
  never convergence: `simulation.stop()` plus `simulation.tick(n)` is the only deterministic way to
  run d3, and the twin's `VizForceSimulation` is ticked the same fixed number of times.

  The twin carries d3's own linear congruential generator (a = 1664525, c = 1013904223, m = 2^32,
  seed 1) and reaches it only through `jiggle()`, which fires when two coordinates are exactly
  equal. The vectors below are chosen so it never fires, so these scenarios are fully determined by
  the arithmetic.

  Numbers are rounded onto the probe protocol's six-decimal emission grid on both sides, so the twin
  is held to the grid the LaTeX subject is held to and neither side is given a wider target.

  @id-phyllotaxis-start
  @level-quick
  @mode-differential
  Scenario: The initial placement is d3's phyllotaxis arrangement
    Given the graph edges
      | source | target |
      | a      | b      |
      | a      | c      |
      | b      | d      |
      | c      | d      |
      | d      | e      |
      | e      | f      |
      | f      | g      |
      | g      | h      |
      | h      | a      |
    And zero ticks are run
    Then every node sits where d3-force's initializeNodes would have put it

  @id-default-forces-20-ticks
  @level-quick
  @mode-differential
  Scenario: Twenty ticks of link, many-body and center agree with d3-force
    Given the graph edges
      | source | target |
      | a      | b      |
      | a      | c      |
      | b      | d      |
      | c      | d      |
      | d      | e      |
      | e      | f      |
      | f      | g      |
      | g      | h      |
      | h      | a      |
    And the simulation runs 20 ticks with forceLink, forceManyBody at theta 0 and forceCenter
    Then every node position of the twin kernel agrees with d3-force

  @id-collide-and-axis-forces
  @level-quick
  @mode-differential
  Scenario: The collide, x and y forces agree with d3-force
    Given the graph edges
      | source | target |
      | a      | b      |
      | b      | c      |
      | c      | d      |
      | d      | a      |
      | a      | c      |
      | e      | f      |
    And the simulation runs 12 ticks with forceLink, forceManyBody at theta 0, forceCollide, forceX and forceY
    Then every node position of the twin kernel agrees with d3-force

  @id-parameterised-link-and-charge
  @level-quick
  @mode-differential
  Scenario: A non-default link distance and charge agree with d3-force
    Given the graph edges
      | source | target |
      | a      | b      |
      | a      | c      |
      | b      | c      |
      | c      | d      |
      | d      | e      |
      | e      | a      |
    And the simulation runs 15 ticks with link distance 45, charge -70 and forceCenter
    Then every node position of the twin kernel agrees with d3-force
