Feature: Nonrenewable Original process timers
  Scenario: A finite birth or physical body exceeds the native timer chunk limit
    Given one representable Original deadline and its fixed wall and monotonic clocks
    When the runtime timer reaches a chunk boundary or fires early
    Then it schedules at most 2147483647 milliseconds from the actual remaining authority
    And it never renews the birth, body cutoff, or terminal cleanup deadline
  Scenario: A retired timer callback has already been queued
    When its actual owner settles or its physical body finishes draining
    Then the queued callback cannot abort or rearm the retired operation
  Scenario: An ordinary body enters its accepted terminal cleanup reserve
    Then the authored reserve is at most 60000 milliseconds
    And the reserve deadline begins only once and remains referenced until settlement
  Scenario: An accepted physical body finishes draining before its deadline
    Then its private control timer and physical cutoff timer retire
    And its Original birth and terminal cleanup deadlines remain armed until settlement
