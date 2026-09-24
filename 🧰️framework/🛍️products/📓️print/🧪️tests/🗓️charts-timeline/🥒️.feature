@capability-viz-charts-timeline
@no-oracle-viz-timeline-intervals
@comparison-viz-probe-v1
Feature: An interval timeline places every bar on its lane and its span
  A Gantt-shaped timeline has no d3 equivalent to adjudicate it: d3 ships scales and shapes, not a
  lane layout. What the layout owes the reader is nonetheless exact, so this case states it as
  specification vectors and measures the renderer against them.

  The rule: the time axis is a linear scale over the union of the start and the end column, mapped
  onto the plot's x range. The lanes are a band scale over the distinct lane values, running from
  the top of the plot downwards so that the first lane of the table is the first lane on the page.
  A bar spans `[x(start), x(end)]` and is centred in its lane; `mode=interval` gives it half the
  lane height, `mode=swimlane` two thirds of that half so several bars can share a lane without
  touching. Bars keep their row order, so a probe record's position in the list identifies its row.

  With `demo-lane` on the 80x40 canvas the time domain is 0..12 and the plot runs from 14 mm
  (the family widens the left padding to make room for lane labels) to 76 mm, so one time unit is
  62/12 mm; three lanes over the 8..36 range give a lane height of 28/3 mm.

  @id-interval
  @level-long
  @mode-conformance
  Scenario: Interval bars span their period and fill half their lane
    Given the committed probe document shared://🗓️charts-timeline/interval.tex
    Then the compiled probe reports the specified bars
      | x       | y       | w       | h      |
      | 14      | 29      | 15.5    | 4.6667 |
      | 29.5    | 29      | 15.5    | 4.6667 |
      | 24.3333 | 19.6667 | 25.8333 | 4.6667 |
      | 50.1667 | 19.6667 | 20.6667 | 4.6667 |
      | 55.3333 | 10.3333 | 10.3333 | 4.6667 |
      | 65.6667 | 10.3333 | 10.3333 | 4.6667 |

  @id-swimlane
  @level-long
  @mode-conformance
  Scenario: Swimlane bars keep their span and take two thirds of the interval height
    Given the committed probe document shared://🗓️charts-timeline/swimlane.tex
    Then the compiled probe reports the specified bars
      | x       | y       | w       | h      |
      | 14      | 29.7778 | 15.5    | 3.1111 |
      | 29.5    | 29.7778 | 15.5    | 3.1111 |
      | 24.3333 | 20.4444 | 25.8333 | 3.1111 |
      | 50.1667 | 20.4444 | 20.6667 | 3.1111 |
      | 55.3333 | 11.1111 | 10.3333 | 3.1111 |
      | 65.6667 | 11.1111 | 10.3333 | 3.1111 |
    # The x and w columns are identical to the interval scenario: changing the lane fill must not
    # move a bar in time. That is the property this pair of scenarios exists to protect.
