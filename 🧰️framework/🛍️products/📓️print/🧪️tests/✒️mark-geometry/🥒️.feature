@capability-viz-mark-geometry
@no-oracle-viz-mark-geometry
@comparison-viz-probe-v1
Feature: Every section 0 primitive emits its own geometry
  Taxonomy section 0 lists fifty-six primitives, and the old library rendered most of them as the
  same fixed glyph. `semio-viz-mark` gives every one of them real geometry, and the probe hook makes
  that checkable without rasterising anything: with `\SemioVizProbeOn` each mark writes the SVG
  path it drew, plus the point, rotation and area it was asked for.

  There is no third-party library that draws a taxonomy of print marks, so this case is a
  conformance test against the taxonomy itself: every slug listed in section 0 of
  `🖼️assets/📊️viz-taxonomy.md` must appear, and -- the part that actually
  bites -- no two of them may draw the same path. Drawn with no fill and no stroke, the record is
  pure geometry, so a primitive that quietly reuses another one's outline is caught by the
  distinctness count rather than by a reviewer's eye.

  @id-primitives
  @level-quick
  @mode-conformance
  Scenario: All fifty-six section 0 primitives are present and pairwise distinct
    Given the committed probe document shared://✒️mark-geometry/mark-geometry.tex and the section 0 slugs
      | slugs |
      | dot, circle, square, rectangle |
      | triangle, diamond, cross, plus |
      | star, custom-glyph, icon-mark, image-mark |
      | text-mark, straight-line, polyline, step-line |
      | curved-line, bezier-curve, spline, catmull-rom-spline |
      | basis-spline, cardinal-spline, monotone-spline, closed-curve |
      | polygon, filled-path, ribbon, band |
      | envelope, circular-arc, elliptical-arc, annular-arc |
      | sector, wedge, straight-connector, orthogonal-connector |
      | curved-connector, elbow-connector, bundled-connector, arrow |
      | bidirectional-arrow, rectangular-region, circular-region, polygonal-region |
      | voronoi-region, convex-hull, concave-hull, label |
      | callout, leader-line, bracket, brace |
      | highlight-region, reference-line, reference-band, reference-point |
    Then the compiled probe and the reference implementation agree on every value

  @id-rotation
  @level-quick
  @mode-conformance
  Scenario: A rotated mark keeps its point and reports its angle
    Given the committed probe document shared://✒️mark-geometry/mark-rotation.tex and the drawn rotations
      | kind, angle |
      | triangle, 0 |
      | triangle, 30 |
      | rectangle, 0 |
      | rectangle, 45 |
    Then the compiled probe and the reference implementation agree on every value
