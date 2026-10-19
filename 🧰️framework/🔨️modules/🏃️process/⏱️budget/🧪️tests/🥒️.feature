Feature: Original process time limits and finite preparation
  Scenario: A long-lived session prepares its compiler
    Given an Original process with no session deadline
    When compiler preparation enters its requested phase
    Then preparation has the authored six-hour time limit
    And queued time consumes that same limit
    And the session retains its original lifetime

  Scenario: A caller supplies a shorter original time limit
    Given an explicit finite time limit at fresh process birth
    When a compiler preparation child is projected
    Then its deadline is no later than its original caller deadline
    And a repeated projection cannot renew its deadline

  Scenario: A child receives an existing Original envelope
    Given an incoming finite Original envelope
    And malformed ambient time limit text
    When the child is admitted
    Then its incoming policy and deadline remain unchanged

  Scenario: A process limit exceeds the native timer range
    Given a finite Original process limit above 2147483647 milliseconds
    When its child command schedules expiry
    Then every timer delay fits the native timer range
    And every check measures elapsed monotonic time from the same start
    And an early check never renews the deadline
    And cancellation retires every scheduled check
