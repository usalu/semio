Feature: Drawing Text Alignment
  Text anchors and baselines in an inferred scene preserve the native canvas placement.

  Scenario Outline: Canvas Placement Matches a Direct Native Canvas Reference
    Given a text node with anchor <anchor> and baseline <baseline>
    When the scene is painted onto a canvas
    Then the pixels equal the native canvas text reference
    Examples:
      | anchor | baseline |
      | start | alphabetic |
      | middle | middle |
      | end | top |
      | middle | bottom |

  Scenario: Authored Font Families Preserve Native Canvas Pixels
    Given text nodes with explicit monospace and serif font families
    When the same content is painted through the scene and a direct native canvas
    Then both images have identical pixels
