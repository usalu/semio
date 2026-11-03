@capability-staffage-visible-feature
Feature: The print check measures the visible geometry
  A shape is part of the print only where nothing nearer paints over it. The minimum feature width is measured on the visible part of a filled
  shape: a thin piece completely behind a filled body contributes nothing, a piece partly covered is measured on the part that stays visible, and
  the paper inked by the outline of the covering shape is not visible either. Authors need no workaround (thick pens, dropped hidden pieces) for hidden parts.

  THE REFERENCE is shapely: it subtracts the union of the covering shapes grown by half their pen from the shape and applies the opening criterion.

  Scenario: A hidden thin bar has no feature
    Given a 1.0 by 0.1 mm bar completely inside a nearer filled body
    When the feature width of its visible part is measured
    Then nothing is visible and the bar contributes nothing

  Scenario: A partly visible sliver is measured on its visible part
    Given a 2 by 1 mm rectangle of which the left 1.9 mm are covered
    When the feature width of its visible part is measured
    Then it is 0.1 mm, the width of the strip that stays visible

  Scenario: A wide shape stays wide and a channel between two bodies is its own feature
    Given a 2 by 1 mm rectangle covered over half, and a 1 mm square covered on both sides leaving a 0.12 mm channel
    When their visible widths are measured
    Then they are 1 mm and 0.12 mm

  Scenario: The outline of the covering shape inks the paper next to it
    Given a 1.0 by 0.1 mm bar covered up to 0.05 mm of its end by a shape drawn with a 0.1 mm pen, and a 2 by 1 mm rectangle covered up to 0.3 mm of its end by the same pen
    When their visible parts are measured
    Then the bar is completely inked and invisible, and the rectangle keeps a strip 0.25 mm wide

  Scenario: Widening a shape widens its visible width
    Given bars of 0.06, 0.09, 0.15, 0.18 and 0.3 mm width whose left half is covered by a body drawn with a 0.1 mm pen
    When their visible widths are measured
    Then each equals the width of its bar and they increase strictly

  Scenario: Residue of coincident edges is not a drawn part
    Given a 1.0 by 0.1 mm bar covered up to 0.005 mm of its end
    When its visible part is measured
    Then the 0.0005 mm2 piece is below the area tolerance and nothing is measured

  Scenario: The suitability of an asset follows the depth order
    Given the sample figure with a hairline bar added behind the torso, sticking out of it, or in front of it
    When the suitability at 1:100 is measured
    Then only the visible hairline is a feature violation: the bar behind the torso is not, the one sticking out and the one in front are

  Scenario: A third-party library measures the same widths
    Given the same shapes and covers
    When shapely takes the difference and opens the rest
    Then its widths equal the hand-computed numbers
