Feature: Observe Rotating Native Image Event Logs
  Scenario: A discovered service log disappears before its metadata is read
    Given a real service directory contains 64 regular event logs of 1000 bytes
    When one discovered log is removed at the observation cancellation checkpoint
    Then observation reports the remaining 63 files and 63000 bytes
    And an independent file discovery library reports the same census
    And cancellation errors propagate even when their code is ENOENT

  Scenario Outline: The service directory changes after log discovery
    Given a real service directory contains 64 regular event logs
    When the service directory is <mutation> at the observation checkpoint
    Then observation refuses rather than treating the missing logs as an empty census

    Examples:
      | mutation |
      | removed  |
      | replaced |
