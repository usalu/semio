@capability-viz-kernel-twin-parity
@oracle-print-viz-kernel-twin
@comparison-viz-probe-v1
Feature: The LaTeX viz kernel and its TypeScript twin answer the same numbers
  Every other case in this tree measures ONE implementation against a third party. This case measures
  the two implementations of the semio visualization kernel against EACH OTHER: the subject is the
  LaTeX kernel under `🖋️latex/`, probed through `semio-viz-probe.sty`, and the reference is
  `@semio-tech/print-viz-kernel`, the TypeScript twin under `🔨️modules/📊️viz-kernel/`.

  The twin is registered as a `cross-semio-implementation` oracle, which the platform's registry
  defines as a SUPPLEMENT and explicitly not independent evidence — both implementations were written
  in this repository, from the same specification, by the same people. It can therefore never
  discharge a mutation's external-oracle requirement, and it does not try to: every algorithm below
  is separately adjudicated by d3 in its own case (`🪢️network-chord`, `⭕️network-circular-arc`,
  `🚰️flow-sankey`, `📐️scale-continuous`, `🌍️geo-projections`, `🥚️spatial-hull`) and, for the twin
  half, in that case's `-twin`. What this case adds is the one comparison neither of those makes: it
  fails the moment the two copies of the kernel drift apart, including in the places where d3 has no
  opinion at all.

  The three network and flow scenarios carry the tables of the case they double, character for
  character, so the LaTeX probe reads exactly the input the twin reads. The three remaining scenarios
  are this case's own: a probe document built here, compared against the twin call that is supposed to
  mean the same thing.

  @id-plain-matrix
  @level-quick
  @mode-differential
  Scenario: Group arcs, subgroup arcs and ribbons agree between the two kernels
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
    Then every group angle and every ribbon endpoint agrees between the two kernels

  @id-circular-equal-spacing
  @level-quick
  @mode-differential
  Scenario: Circular placement agrees between the two kernels
    Given the graph edges
      | source | target |
      | a      | b      |
      | b      | c      |
      | c      | d      |
      | d      | e      |
      | e      | f      |
      | f      | a      |
    And a radius of 10 in declaration order over the full turn
    Then every node sits at the same angle in both kernels

  @id-align-justify
  @level-quick
  @mode-differential
  Scenario: The justify alignment agrees between the two kernels
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
    Then every node rectangle and every link band agrees between the two kernels

  @id-scale-linear-and-log
  @level-quick
  @mode-differential
  Scenario: A linear and a logarithmic scale map the same inputs in both kernels
    Given the scales
      | name | kind   | domain | range |
      | lin  | linear | 0,100  | 0,180 |
      | lg   | log    | 1,1000 | 0,300 |
    And the inputs 0, 12.5, 25, 42 and 100 for the linear scale and 1, 10, 42, 100 and 1000 for the logarithmic one
    Then both kernels map every input to the same millimetre

  @id-geo-mercator
  @level-quick
  @mode-differential
  Scenario: The Mercator projection places the same points in both kernels
    Given the geographic points
      | longitude | latitude |
      | 12        | 47       |
      | 0         | 0        |
      | -74       | 40.7     |
      | 139.7     | 35.7     |
      | 9.72      | 52.37    |
    Then both kernels project every point to the same plane coordinate

  @id-spatial-hull
  @level-quick
  @mode-differential
  Scenario: The convex hull of the demo point set is the same cycle in both kernels
    Given the point set demo-points
      | x  | y  |
      | 12 | 14 |
      | 28 | 9  |
      | 41 | 26 |
      | 19 | 33 |
      | 55 | 17 |
      | 63 | 31 |
      | 34 | 41 |
      | 8  | 27 |
      | 47 | 7  |
      | 58 | 44 |
      | 25 | 20 |
      | 39 | 13 |
    Then both kernels report the same hull vertices in the same cyclic order
