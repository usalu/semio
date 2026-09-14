@capability-viz-annotation
@oracle-d3-scale
@comparison-viz-probe-v1
Feature: An annotation is anchored in data space, not in millimetres
  `\SemioVizAnnotate` accepts `at={x,y}` in plot millimetres, but the form that matters is `x=`/`y=`
  in data units: a threshold at 85, a marker at week 7, a band from 20 to 40. Those go through the
  named scales, so the annotation moves when the domain does. This case measures exactly that
  mapping, and the extent of the reference lines and bands the mapping produces.

  The scales are a linear x over 0..10 mapped to 8..118 and a linear y over 0..100 mapped to 8..58,
  which is a plot area of 110 by 50 millimetres inset by 8 — the geometry a `VizFigure` gives a
  full-width figure. A reference line spans the OPPOSITE scale's range, never the frame, so a plot
  whose axes do not touch the frame edge still gets a line that stops where the data does.

  @id-data-space-anchors
  @level-quick
  @mode-differential
  Scenario: Data coordinates are mapped through the scales d3-scale defines
    Given the committed probe document local://data-space.tex and the anchors
      | x | y  |
      | 3 | 50 |
      | 7 | 85 |
      | 8 | 70 |
    Then the compiled probe and the reference implementation agree on every anchor position

  @id-reference-extent
  @level-quick
  @mode-conformance
  Scenario: Reference lines and bands span the opposite scale's range
    Given the committed probe document local://data-space.tex
    Then the compiled probe reports the specified extents
      | key             | values         |
      | reference-line  | 8,38,118,38    |
      | reference-band  | 8,18,118,28    |
      | event-marker    | 85,8,85,58     |

  @id-bracket-normal
  @level-quick
  @mode-conformance
  Scenario: A bracket turns inward along its own perpendicular
    Given the committed probe document local://bracket.tex with from=20,12 to=50,12 depth=2
    Then the compiled probe reports the specified bracket
      | key     | values        |
      | bracket | 20,12,50,12,2 |
