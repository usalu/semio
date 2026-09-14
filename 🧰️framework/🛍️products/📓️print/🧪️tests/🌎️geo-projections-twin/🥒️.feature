@capability-geo-projection-twin
@oracle-d3-geo
@comparison-viz-probe-v1
Feature: The TypeScript twin of every projection agrees with d3-geo
  `geo-projections` measures the LaTeX kernel. This case measures the second subject of the same
  specification, the TypeScript twin `@semio-tech/print-viz-kernel`, on the same vectors and against
  the same oracle, because a kernel that exists twice is only a kernel if both copies answer alike.

  The platform gives a case one adapter per language and one subject per scenario, so the twin cannot
  share `geo-projections`'s adapter with the LaTeX probe. It gets its own case instead; the vectors
  below are the vectors of `geo-projections` verbatim, and the adapter reuses that case's oracle
  handlers unchanged so the two subjects are measured against literally the same reference numbers.

  Everything the base case says about the pipeline holds here word for word: degrees to radians, the
  three-angle rotation, the raw projection of the kind, and d3's `scaleTranslateRotate` with the
  centre offset, over the same fourteen kinds — the twelve d3 exposes as standalone constructors
  plus `albers` and the shared cylindrical-equal-area fallback the conics degenerate into. Every
  default matches d3's own default for that kind, so an unconfigured `vizGeoProjection` and an
  unconfigured d3 constructor are the same function.

  The inverse scenario of the base case has no counterpart here: the twin's `rawNaturalEarth` carries
  no `invert`, and the vector table of that scenario is not the twin's to shorten, so the twin case
  measures the forward mapping and the fitting instead.

  Numbers are rounded onto the probe protocol's nine-decimal emission grid on both sides, so the twin
  is held to the grid the LaTeX subject is held to and neither side is given a wider target.

  @id-forward-projection
  @level-quick
  @mode-differential
  Scenario: The forward mapping of every projection kind agrees with d3-geo
    Given the sample point
      | kind                  | lon | lat |
      | equirectangular       | 12  | 47  |
      | mercator              | 12  | 47  |
      | transverse-mercator   | 12  | 47  |
      | equal-earth           | 12  | 47  |
      | natural-earth1        | 12  | 47  |
      | orthographic          | 12  | 47  |
      | stereographic         | 12  | 47  |
      | gnomonic              | 12  | 47  |
      | azimuthal-equal-area  | 12  | 47  |
      | azimuthal-equidistant | 12  | 47  |
      | conic-conformal       | 12  | 47  |
      | conic-equal-area      | 12  | 47  |
      | conic-equidistant     | 12  | 47  |
      | albers                | 12  | 47  |
    Then the twin kernel and the reference implementation agree on the plane coordinates of every kind

  @id-fit-extent
  @level-quick
  @mode-differential
  Scenario: fitExtent, fitSize and fitWidth agree with d3-geo
    Given the fitting box
      | kind             | width | height |
      | equirectangular  | 100   | 60     |
      | mercator         | 100   | 60     |
      | conic-equal-area | 100   | 60     |
    Then the twin kernel and the reference implementation agree on the fitted scale and translation
