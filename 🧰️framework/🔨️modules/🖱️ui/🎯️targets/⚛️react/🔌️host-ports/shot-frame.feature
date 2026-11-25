Feature: Icon shot dimensions belong to the crop guide
  Scenario Outline: The dimension chip stays within the crop
    Given a <shape> crop guide at pane level
    When dimensions are enabled
    Then the chip meets the guide's bottom edge
    And the chip uses panel-level glass inside the crop mask

    Examples:
      | shape     |
      | rectangle |
      | ellipse   |
