@capability-viz-shape-link
@oracle-d3-shape
@oracle-d3-chord
@comparison-viz-probe-v1
Feature: Links bump and ribbons close the way d3 draws them
  `\SemioVizLink` is `d3-shape/src/curve/bump.js`: a single cubic whose control points sit on the
  midpoint of the axis the link flows along, and whose radial form maps angle and radius through
  d3's own `pointRadial`, measuring the angle clockwise from twelve o'clock. A link between two
  coincident points must still emit that cubic, which is why the degenerate row is here.

  `\SemioVizRibbon` is `d3-chord/src/ribbon.js`: two arcs on the circle joined by quadratics that
  pass through the centre, with the pad angle taken off each end and the target side skipped
  entirely when it coincides with the source side -- the self-chord case.

  @id-links
  @level-quick
  @mode-differential
  Scenario: Horizontal, vertical and radial links bump on the right axis
    Given the committed probe document shared://🎀️shape-links-ribbons/shape-links-ribbons.tex and the links
      | tag        | kind       | source | target |
      | horizontal | horizontal | 10,20  | 80,60  |
      | vertical   | vertical   | 10,20  | 80,60  |
      | radial     | radial     | 0.5,30 | 2.5,90 |
      | flat       | horizontal | 10,20  | 10,20  |
    Then the compiled probe and the reference implementation agree on every value

  @id-ribbons
  @level-quick
  @mode-differential
  Scenario: Chord ribbons pad, straddle two radii and collapse onto themselves as d3-chord does
    Given the committed probe document shared://🎀️shape-links-ribbons/shape-links-ribbons.tex and the ribbons
      | tag    | radius | startAngle | endAngle | targetRadius | targetStartAngle | targetEndAngle | padAngle |
      | plain  | 100    | 0.2        | 0.9      | 100          | 2.1              | 2.9            | 0        |
      | padded | 100    | 0.2        | 0.9      | 100          | 2.1              | 2.9            | 0.06     |
      | radii  | 120    | 0.2        | 0.9      | 80           | 2.1              | 2.9            | 0        |
      | self   | 100    | 0.2        | 0.9      | 100          | 0.2              | 0.9            | 0        |
    Then the compiled probe and the reference implementation agree on every value
