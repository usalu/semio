Feature: Original-owned native EXPRESS admission
  Scenario: The final population includes original and inserted entities
    Given an original captured IFC4X3_ADD2 graph and an immutable sparse mutation batch
    When the retained overlay receives one source coordinate per funded original turn
    Then replacements are visible and inserted forward references resolve
    And no alternate graph or copied original string is created

  Scenario: Semantic preparation is cancelled at each retained frontier
    Given the same original command and captured revision
    When original cancellation occurs during overlay, receiving or evaluation
    Then no mutation is published
    And every semantic owner retires before the captured source

  Scenario: Only complete valid evaluation permits publication
    Given every released EXPRESS declaration and the complete resulting graph
    When any domain, WHERE, UNIQUE, inverse, contextual bound or global rule is false or incomplete
    Then the registered original publication route refuses
