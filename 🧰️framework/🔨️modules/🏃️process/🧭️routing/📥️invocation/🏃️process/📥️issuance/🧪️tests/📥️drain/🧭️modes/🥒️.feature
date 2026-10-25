Feature: Accepted work keeps an Original alive until it settles
  Scenario: Every drain mode of the issuance law runs in its own process birth
    Given the closed list of actual drain modes
    When each mode runs as the only Original of its own process
    Then the Original waits for its accepted child, refuses a late top-level or nested admission after it begins to drain, and retires only after the child stopped
    And an accepted failure or an interruption still rejects the Original and releases every signal handler
