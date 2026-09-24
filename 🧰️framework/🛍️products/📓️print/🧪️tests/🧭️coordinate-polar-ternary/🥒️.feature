@capability-viz-coordinate
@no-oracle-viz-coordinate
@comparison-viz-probe-v1
Feature: Every coordinate system maps its two data channels onto the page as specified
  `semio-viz-coordinate` is the one place a chart learns what its two data channels mean: x and y in
  cartesian, angle and radius in polar, the a and b shares of a triangle in ternary, the axis index
  and the value on it in parallel, longitude and latitude in geographic. Every family reaches the
  page through `\semio_viz_coordinate_map:nnNN`, so these vectors pin the contract every chart
  depends on.

  There is no third-party implementation to adjudicate this: d3 has no ternary, parallel or
  log-polar coordinate system, and its polar geometry lives inside the shape generators rather than
  in a mapping the caller can query. The scenarios are therefore specification vectors, computed
  from the formulas documented in the package's own Keys region, and the geographic rows exercise
  only the equirectangular fallback the hook `\semio_viz_coordinate_project:nnNN` ships with -- the
  real projection pipeline is `semio-viz-geo-projection`'s and is adjudicated by `d3-geo` there.

  Angles are radians clockwise from twelve o'clock, so a quarter turn of the polar system lands on
  the positive x axis, and the ternary triangle has its a corner at the apex.

  @id-cartesian
  @level-quick
  @mode-conformance
  Scenario: The cartesian system honours origin, domain and the flipped y axis
    Given the committed probe document shared://🧭️coordinate-polar-ternary/coordinate-polar-ternary.tex and the points
      | tag               | x   | y    | expected  |
      | cartesian-origin  | 0   | 0    | 0,0       |
      | cartesian-middle  | 0.5 | 0.25 | 40,10     |
      | cartesian-corner  | 1   | 1    | 80,40     |
      | cartesian-flipped | 0.5 | 0.25 | 50,35     |
      | cartesian-domain  | 2.5 | 0    | 20,20     |
    Then the compiled probe and the reference implementation agree on every value

  @id-polar
  @level-quick
  @mode-conformance
  Scenario: The polar and log-polar systems place angle and radius as specified
    Given the committed probe document shared://🧭️coordinate-polar-ternary/coordinate-polar-ternary.tex and the points
      | tag           | angle | radius | expected |
      | polar-quarter | 0.25  | 1      | 20,0     |
      | polar-noon    | 0     | 1      | 0,20     |
      | polar-half    | 0.5   | 0.5    | 0,-10    |
      | polar-counter | 0.25  | 0      | -8,0     |
      | logpolar-half | 0     | 0.5    | 0,14.807 |
      | logpolar-full | 0     | 1      | 0,20     |
    Then the compiled probe and the reference implementation agree on every value

  @id-ternary
  @level-quick
  @mode-conformance
  Scenario: The ternary system puts each pure share on its own corner
    Given the committed probe document shared://🧭️coordinate-polar-ternary/coordinate-polar-ternary.tex and the points
      | tag         | a   | b    | expected     |
      | ternary-a   | 1   | 0    | 30,51.961    |
      | ternary-b   | 0   | 1    | 60,0         |
      | ternary-c   | 0   | 0    | 0,0          |
      | ternary-mid | 0.5 | 0.25 | 30,25.981    |
    Then the compiled probe and the reference implementation agree on every value

  @id-parallel
  @level-quick
  @mode-conformance
  Scenario: Parallel coordinates space their axes evenly across the frame
    Given the committed probe document shared://🧭️coordinate-polar-ternary/coordinate-polar-ternary.tex and the points
      | tag            | axis | value | expected |
      | parallel-first | 1    | 0     | 0,0      |
      | parallel-third | 3    | 0.5   | 60,25    |
      | parallel-last  | 4    | 1     | 90,50    |
    Then the compiled probe and the reference implementation agree on every value

  @id-geographic
  @level-quick
  @mode-conformance
  Scenario: The geographic hook falls back to an equirectangular mapping
    Given the committed probe document shared://🧭️coordinate-polar-ternary/coordinate-polar-ternary.tex and the points
      | tag                     | lon  | lat   | expected      |
      | geographic-null         | 0    | 0     | 180,90        |
      | geographic-hannover     | 9.72 | 52.37 | 189.72,142.37 |
      | geographic-antimeridian | 180  | -90   | 360,0         |
    Then the compiled probe and the reference implementation agree on every value
