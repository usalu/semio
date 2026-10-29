Feature: The asset detail sections state scale, time, assumptions, provenance and export honestly
  Scenario: Size on paper at every preset scale
    Given the reference walker, 1.73 m high
    When the dimensions section is shown
    Then its paper table reads 3.46, 8.65, 17.3 and 34.6 mm for 1:500, 1:200, 1:100 and 1:50
    And in German the decimal separator is a comma
  Scenario: The timeline can be driven by marker and keyboard
    Given the walk clip of the reference walker at time 0
    When the second event marker is activated
    Then the timeline seeks to 0.05 seconds
    When the right arrow key is pressed on the timeline
    Then it seeks one frame, 1/30 second, ahead
  Scenario: Poses and clips are one radio group each
    Given no selection
    Then the default clip is checked
    When the arrow key is pressed on a clip card
    Then the next clip is selected
  Scenario: Export states are honest
    Given capabilities with DWG missing and WebM unsupported
    Then DWG is unavailable with its reason, WebM is unsupported with its reason, SVG is available
    And a running export shows progress and can be cancelled
  Scenario: Sources are never colour only
    Then every source tag has its own words in English and German and a citation tooltip
  Scenario: Provenance counts citations
    Then the walker's references are cited 11, 4, 2 and 1 times
