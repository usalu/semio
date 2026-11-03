@capability-staffage-thermal
Feature: The thermal span range keeps moderate sources readable
  The absolute shared range of -20 to 60 degrees Celsius paints a 35 degree laptop in a 21 degree room pale. A span range reads the surface temperature on a symmetric
  plus and minus spanK kelvin range around the air temperature, the legend says so, and the cast manifest carries it as `thermalSpanK` (the course casts use 25 K, one value per scene).
  Every quantity other than the surface temperature keeps the shared range. The vectors live in `🧫️fixtures/🌡️thermal-span/🔣️.json`.

  THE REFERENCES are d3-scale (piecewise linear signed fraction) and ajv (the cast schema).

  Scenario: The scale spans the air temperature plus and minus the span
    Given air temperatures of 21, 27, 2 and 22 degrees Celsius with spans of 25, 25, 15 and 5 kelvin
    When the scale is built
    Then it runs from air minus span to air plus span and is centred on the air temperature

  Scenario: A moderate source reads stronger than on the shared range
    Given a 35 degree laptop in a 21 degree room
    When the signed fraction is read on the span range and on the shared range
    Then it is 0.56 against 0.359, values beyond the range saturate at 1 and the colour equals the shared colour of the same fraction

  Scenario: The legend states the span
    Given the thermal view of an open laptop with a span of 25 kelvin
    When the legend is drawn
    Then it names the range as span, runs from air minus 25 to air plus 25 and says so in English and in German

  Scenario: Other quantities keep the shared range
    Given the skin temperature and the three flux quantities with a span
    When the legend is drawn
    Then it names the shared range and its documented bounds

  Scenario: The cast manifest accepts only a positive span of the thermal view
    Given casts with a span of 25, 0, minus 5 and a text, and a line view with a span
    When ajv and the twin validate them
    Then both accept 25 and both reject the others

  Scenario: Casts of one scene share one span, different scenes may differ
    Given scenes whose thermal casts carry 25 and 25, 78 and 78, 25 and 78, and 25 and the absolute range
    When each scene is checked for mixed spans
    Then only the scenes holding two different spans are reported

  Scenario: The course manifest gives every thermal cast a deliberate span
    Given the course cast manifest
    When its thermal casts are read
    Then every one carries a positive thermalSpanK and more than one span value exists
