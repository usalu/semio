@capability-viz-guide-legend
@oracle-d3-scale
@comparison-viz-probe-v1
Feature: A legend generates its entries from a scale
  `\SemioVizLegend` never takes a list of entries: a categorical legend enumerates its scale's
  domain, a gradient or size legend enumerates its scale's ticks. That is the whole point — a legend
  that is typed out separately from the scale it explains drifts away from it. This case fixes both
  halves: the entry SET (which is the scale's, measurable against `d3-scale`) and the entry LAYOUT
  (which is the legend's own, and specified here).

  A vertical legend stacks entries downward from `at` by `swatchSize + itemGap`; a size legend
  places one circle per tick with the radius the scale maps that tick to, advancing by the circles'
  own diameters. Both are read out of the compiled probe as emitted geometry.

  @id-categorical-entries
  @level-quick
  @mode-differential
  Scenario: A swatch legend enumerates the scale domain
    Given the committed probe document shared://🗝️guide-legend/categorical.tex and the ordinal scale
      | domain    |
      | A,B,C,D,E |
    Then the compiled probe and the reference implementation agree on every legend entry

  @id-vertical-layout
  @level-quick
  @mode-conformance
  Scenario: A vertical swatch legend stacks its entries by swatch and gap
    Given the committed probe document shared://🗝️guide-legend/categorical.tex with at=4,36 swatchSize=2.6 itemGap=1.2
    Then the compiled probe reports the specified entry origins
      | key            | values                                       |
      | legend-origins | 4,33.4,4,29.6,4,25.8,4,22,4,18.2             |

  @id-size-entries
  @level-quick
  @mode-conformance
  Scenario: A size legend takes each circle's radius from the scale
    Given the committed probe document shared://🗝️guide-legend/size.tex with ticks=3 over the linear scale 0..10 mapped to 0..5
    Then the compiled probe reports the specified circle radii
      | key            | values      |
      | legend-radii   | 0,2.5,5     |
