Feature: Original native child frontier custody

  Scenario: Last original alias transfers one native payload frame to its caller
    Given zero, one or several original sealed aliases
    When each existing alias returns under exact physical grants
    Then only the last alias hands off one native child
    And no nested retirement queue is allocated
    And real physical births and releases balance

  Scenario: Native child admission refuses before taking the original
    Given shared neutral native support, known birth and grant cases
    When native birth, work or depth is unavailable or underfunded
    Then the exact original owner remains in its input slot with no heap effect
    And SQLite independently confirms the admission decision
