Feature: Current Host Capacity Grant Inheritance

  Scenario: Share Only a Current Live Grant
    Given a closed grant and the eleven authored liveness observations
    When inherited and ordinary admission inspect the same identity, PID, boot and heartbeat facts
    Then only current matching grants within the inclusive published bounds are inherited
    And a stale inherited grant enters ordinary admission without fast-path deletion
    And malformed path identifiers are refused before filesystem reads
