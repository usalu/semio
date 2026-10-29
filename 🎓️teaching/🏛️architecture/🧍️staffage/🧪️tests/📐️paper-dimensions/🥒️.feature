Feature: Paper dimensions of staffage
  A 1.80 m figure measures 3.6, 9, 18 and 36 mm at 1:500, 1:200, 1:100 and 1:50.

  Scenario Outline: figure height in the written SVG
    Given a 1.80 m figure drawn at 1:<denominator> without annotations
    When the scene is written as SVG
    Then the figure path spans <millimetres> mm in user units
    And the SVG width in millimetres equals the scene width
    And librsvg renders the figure element <millimetres> mm times 10 pixels per mm within 1 pixel

    Examples:
      | denominator | millimetres |
      | 500         | 3.6         |
      | 200         | 9           |
      | 100         | 18          |
      | 50          | 36          |
