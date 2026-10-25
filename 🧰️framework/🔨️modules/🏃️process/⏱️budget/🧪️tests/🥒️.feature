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

  Scenario: A long native action outlives a fixed wall clock while it makes progress
    Given a native action with an idle window and a maximum derived from its declared cost
    When its child keeps using CPU time or writing output, or it sits in a declared lease or queue wait
    Then the idle window is renewed each time
    And the action is never stopped before the maximum for lack of time alone

  Scenario: A native action that does nothing is stopped after its idle window
    Given a native action whose child neither uses CPU time nor writes output nor waits in a declared wait
    When the idle window passes
    Then the action is stopped and the reason is written in every selected language
    And a heartbeat that the waiting process writes about itself does not renew the window

  Scenario: The maximum cannot be renewed
    Given a native action that makes progress without end
    When its declared maximum is reached
    Then the action is stopped as having reached its maximum
    And no progress, wait or environment setting moves that maximum
