@capability-network-force-layout
@oracle-d3-force
@comparison-viz-probe-coarse-v1
Feature: The force layout reproduces d3-force tick for tick
  `semio-viz-network`'s `force` layout is not "a force-directed layout that looks similar to d3's".
  It is d3-force's own algorithm written in expl3: the phyllotaxis start (radius `10*sqrt(0.5+i)`,
  angle `i*pi*(3-sqrt 5)`), the `alpha += (alphaTarget - alpha) * alphaDecay` schedule, the same
  per-link strength `1/min(deg s, deg t)` and bias `deg s/(deg s + deg t)`, and the velocity Verlet
  integration `x += vx *= 1 - velocityDecay` — applied in the order the forces are listed.

  Two things about the reference must be said out loud, because they decide whether the comparison
  means anything.

  **`theta` must be 0 on the oracle.** d3's `forceManyBody` approximates with a Barnes–Hut quadtree
  whose default `theta` is 0.9. This kernel evaluates the many-body force exactly, once per
  unordered pair, which is what d3 itself computes when `theta` is 0 (the accuracy test can never
  accept an internal node, so the traversal descends to every leaf). The oracle therefore calls
  `.theta(0)`; comparing against the default 0.9 would measure d3's approximation, not our
  arithmetic.

  **Only the iteration count is compared, never convergence.** `simulation.stop()` plus
  `simulation.tick(n)` is the only deterministic way to run d3, and it is what these scenarios do on
  both sides. A force simulation is a chaotic integrator: agreement is asserted after a fixed,
  small number of ticks under `viz-probe-coarse-v1` (1e-3), because the divergence between IEEE
  doubles and expl3's 16-digit fixed point grows with every tick. Measured today, twenty ticks of
  the eight-node vector still agree to thirteen significant digits; the tolerance is the budget for
  longer runs, not a description of the error.

  The layout has no randomness other than d3's own linear congruential generator (a = 1664525,
  c = 1013904223, m = 2^32, seed 1), and that generator is only ever reached through `jiggle()`,
  which fires when two coordinates are exactly equal. The vectors below are chosen so it never
  fires, so these scenarios are fully determined by the arithmetic.

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
    Then every node position agrees with d3-force

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
    Then every node position agrees with d3-force

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
    Then every node position agrees with d3-force
