Feature: Thermal view legend and values
  The thermal view paints solver values of the selected quantity and names quantity, unit, range, sign convention, conditions and assumptions.

  Scenario: Shared range
    Given a commuter at 1:100 in winter overcast conditions and the surface temperature selected
    When the thermal view is drawn
    Then the legend names the quantity and unit, states the shared comparison range minus 20 to 60 degrees Celsius and the air temperature of 2 degrees Celsius

  Scenario: Auto range
    Given the same view with the auto range
    Then the legend states that the range is normalised to the values present and not comparable between figures

  Scenario: Merged regions at 1:500
    Given the 1:500 silhouette that lists several regions
    Then its value is the area-weighted mean of those regions
