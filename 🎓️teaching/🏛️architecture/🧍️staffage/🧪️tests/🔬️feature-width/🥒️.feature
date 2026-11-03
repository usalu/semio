@capability-staffage-feature-width
Feature: The minimum feature width of a small filled shape is its true width
  The print check measures the thickness of every filled shape. A shape with no thin part has the width of twice its largest inscribed radius; a finger,
  spoke or sliver that exceeds the tolerance is the width of that thin part. The result scales with the shape: a hand 0.18 mm wide is not reported as
  exactly the minimum feature of the profile, and the verdict of an asset follows the representation that is measured, also after it was changed in place.

  THE REFERENCE is shapely: it erodes the shape with a buffer and bisects, or opens it and applies the same area criterion.

  Scenario: Small shapes are measured to a few micrometres
    Given rectangles, a 64-gon and a 3 by 2 mm rectangle with hand-computed widths, and the same hand enlarged three times
    When their feature width is measured with a minimum feature of 0.2 mm
    Then every width equals the hand-computed number within 3 % or 4 µm

  Scenario: A thin finger of a palm is its own feature
    Given a 0.4 mm palm with a 0.5 mm long and 0.1 mm wide finger
    When the feature width is measured
    Then it is 0.1 mm

  Scenario: Widths grow with the shape
    Given strips and rectangles of 0.05, 0.09, 0.15 and 0.18 mm and a circle of 0.2 mm
    When their widths are compared
    Then they increase strictly, without a plateau at the minimum feature

  Scenario: The verdict follows a representation changed in place
    Given the suitability of an authored representation was measured
    When a hairline shape (0.003 mm thick, drawn with a 0.1 mm pen) is added to the same representation object and it is measured again
    Then the second result reports the feature violation of the new shape

  Scenario: The measurement reuses its raster buffers without changing a width
    Given eight shapes measured in order, in reverse order and after the retained buffers were released
    When their feature widths are compared
    Then every width is identical, so a buffer left over from a larger raster never leaks into a smaller one

  Scenario: A third-party library measures the same widths
    Given the same shapes
    When shapely erodes or opens them
    Then its widths equal the hand-computed numbers
