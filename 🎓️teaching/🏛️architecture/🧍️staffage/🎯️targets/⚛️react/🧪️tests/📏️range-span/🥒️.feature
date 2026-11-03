Feature: The display range offers a span around the air temperature
  The range control has three modes: automatic, fixed and span. The span is one number in kelvin, labelled in English and German, and it travels in the address.

  Scenario: Choosing span starts at the course span
    Given the range is automatic
    When the reader picks span
    Then the range is span with 25 K

  Scenario: Editing the span
    Given the range is a span of 25 K
    When the reader types 15 into the span field
    Then the range is span with 15 K
    And a non-positive entry changes nothing

  Scenario: Keyboard operation
    Given the span option is selected
    When the reader presses the arrow key
    Then the neighbouring mode is chosen

  Scenario: Labels in two languages without a default
    Given the language is English or German
    Then the group is labelled "Display range" or "Anzeigebereich"
    And the field is labelled with the span and its unit K

  Scenario: The address keeps the span
    Given a viewer with span 15 K
    Then the address carries span=15 and reads back to the same state
    And the cast exporter uses the same range value
