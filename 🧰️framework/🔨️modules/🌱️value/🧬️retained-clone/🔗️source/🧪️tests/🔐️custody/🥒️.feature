Feature: Exact original authority handback
  Scenario: Close the current projection before transferring the original
    Given one live original authority and its projection alias
    When a caller requests a handback quote
    Then all currencies belong to the same current alias transition
    And a denied grant preserves the quote and original identity
    When the projection is terminal
    Then one complete quote describes the original transfer and physical release
    And transferred or closing authority refuses a new handback quote
