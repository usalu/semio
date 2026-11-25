@capability-staffage-representation
Feature: Print suitability is measured on the inked paper geometry
  A representation is suitable for a print profile when its pens and hatch pens are at least the minimum stroke, its hatch spacing at least the minimum hatch spacing, every filled
  shape is at least the minimum feature wide, and no white gap between separate inked areas (or negative-space sliver inside the inked silhouette) is narrower than the minimum gap.
  Outlines that touch, cross or whose strokes overlap are joined on purpose and never count as gaps.

  Scenario: Well separated shapes pass
    Given rectangles 0.25 m wide spaced 0.25 m apart
    When they are measured under office-laser at 1:500, 1:200, 1:100 and 1:50
    Then every scale is suitable

  Scenario: Touching and overlapping ink is a join
    Given two rectangles whose outlines touch, and two whose strokes overlap without touching
    When they are measured
    Then no gap violation is reported

  Scenario: An ink gap of exactly zero is a join whatever the rounding
    Given two rectangles whose centre lines are one pen width apart at 1:20 to 1:500, so the ink between them is exactly zero
    When they are measured
    Then no gap violation is reported, and a pair 0.02 mm farther apart reports a gap of 0.02 mm

  Scenario: A white sliver between separate shapes
    Given two rectangles whose ink gap is 0.105 mm at 1:500
    When they are measured
    Then a gap violation names both shapes

  Scenario: A negative-space sliver inside one outline
    Given one closed shape with a slit whose white width is 0.14 mm at 1:50
    When it is measured
    Then a gap violation names the negative space and the position

  Scenario: Thin parts, pens and hatches
    Given a 0.1 mm wide filled rectangle, a 0.05 mm pen and a 0.2 mm hatch spacing
    When they are measured
    Then feature, stroke and hatch violations are reported

  Scenario: Each drawing is measured at its shared print boundary
    Given adjacent drawings with an 18 mm flange at 1:100 and a 9 mm web at 1:200
    When each drawing is printed at 1:150 with a 0.05 mm pen
    Then the flange measures 0.17 mm and the web measures 0.11 mm on paper
