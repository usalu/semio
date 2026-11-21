Feature: Current Host Capacity Grant Inheritance

  Scenario: Share Only a Current Live Grant
    Given a closed grant and the eleven authored liveness observations
    When inherited and ordinary admission inspect the same identity, PID, boot and heartbeat facts
    Then only current matching grants within the inclusive published bounds are inherited
    And a stale inherited grant enters ordinary admission without fast-path deletion
    And malformed path identifiers are refused before filesystem reads

  Scenario: Reclaim Stale Grants Without Interrupting Admission
    Given a grant recorded before the current boot
    When a status reader lists current grants
    Then it excludes the stale grant and preserves its file
    When admission encounters the stale grant during a Windows sharing violation
    Then the stale grant does not count toward the host reservation
    And the next admission can reclaim the file after the reader closes it

  Scenario: Count Only Live Waiters Against the Queue Limit
    Given the waiter directory reaches its file limit with a dead waiter record
    When a new request asks for admission
    Then the dead waiter does not count against the queue limit
    And admission reclaims it during the next ledger scan
