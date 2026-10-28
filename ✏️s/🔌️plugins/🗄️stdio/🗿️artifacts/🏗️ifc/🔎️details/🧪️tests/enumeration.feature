Feature: Actual Source Enumeration Details Choices
  Scenario: Complete typed choices resolve before rendering
    Given a revision-bound IfcPropertyEnumeratedValue EnumerationReference
    When the original controlled query completes
    Then every actual allowed typed payload is retained in source order
    And display labels do not become replacement payloads
  Scenario: Multiple selected values use central declared events
    When a complete item is replaced or an allowed item is appended or removed
    Then the existing mutation and inverse authority receives the complete typed values
    And an empty list remains distinct from optional unset and its cardinality refusal
  Scenario: Source and revision custody
    When source identity or generation or revision differs or cancellation arrives
    Then no stale completed choice can publish an edit
