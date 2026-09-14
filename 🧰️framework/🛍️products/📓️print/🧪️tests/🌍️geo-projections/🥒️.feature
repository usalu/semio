@capability-geo-projection
@oracle-d3-geo
@comparison-viz-probe-v1
Feature: Every projection of semio-viz-geo agrees with d3-geo
  `semio-viz-geo.sty` reimplements d3-geo's projection pipeline in expl3: degrees to radians, the
  three-angle rotation, the raw projection of the kind, and d3's `scaleTranslateRotate` with the
  centre offset. Fourteen kinds are implemented — the twelve d3 exposes as standalone constructors
  plus `albers` and the shared cylindrical-equal-area fallback the conics degenerate into.
  `albers-usa` is deliberately absent: it is a composite of three projections stitched by plane
  extent, which is a layout, not a projection.

  Every default matches d3's own default for that kind (scale, translate, centre, rotate, standard
  parallels and clip angle), so an unconfigured `\SemioVizProjection` and an unconfigured d3
  constructor are the same function. This feature measures exactly that: the forward mapping, the
  inverse mapping, and `fitExtent`, which is the one place where the projection and the path
  resampler have to agree before a number comes out.

  Numbers are compared under `viz-probe-v1` (1e-6). expl3 fixed-point carries about sixteen
  significant digits and the observed disagreement across all fourteen kinds is below 3e-12, so the
  profile is not the binding constraint; it is the honest resolution of the arithmetic.

  The probe documents are committed because a projection probe is a program, not a vector list:
  local://forward.tex, local://inverse.tex and local://fit.tex are all committed.

  @id-forward-projection
  @level-quick
  @mode-differential
  Scenario: The forward mapping of every projection kind agrees with d3-geo
    Given the committed probe document local://forward.tex and the sample point
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
    Then the compiled probe and the reference implementation agree on the plane coordinates of every kind

  @id-inverse-projection
  @level-quick
  @mode-differential
  Scenario: The inverse mapping of every projection kind agrees with d3-geo
    Given the committed probe document local://inverse.tex and the plane sample
      | kind                  | x   | y   |
      | equirectangular       | 500 | 130 |
      | mercator              | 500 | 130 |
      | transverse-mercator   | 500 | 130 |
      | equal-earth           | 500 | 130 |
      | natural-earth1        | 500 | 130 |
      | orthographic          | 500 | 130 |
      | stereographic         | 500 | 130 |
      | gnomonic              | 500 | 130 |
      | azimuthal-equal-area  | 500 | 130 |
      | azimuthal-equidistant | 500 | 130 |
      | conic-conformal       | 500 | 130 |
      | conic-equal-area      | 500 | 130 |
      | conic-equidistant     | 500 | 130 |
      | albers                | 500 | 130 |
    Then the compiled probe and the reference implementation agree on the degrees of every kind

  @id-fit-extent
  @level-quick
  @mode-differential
  Scenario: fitExtent, fitSize and fitWidth agree with d3-geo
    Given the committed probe document local://fit.tex and the fitting box
      | kind             | width | height |
      | equirectangular  | 100   | 60     |
      | mercator         | 100   | 60     |
      | conic-equal-area | 100   | 60     |
    Then the compiled probe and the reference implementation agree on the fitted scale and translation
