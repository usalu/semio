Feature: Bounded Parser Source Progress
  Scenario: Source preflight remains readable during a large workspace inspection
    Given parser Source operations retain their actual Original invocation
    When their shared neutral progress vectors are inspected
    Then the first checkpoint and at most one update per second are published
    And changing inspection phases does not renew the publication interval
    And every original checkpoint still yields and checks cancellation
    And the published counter sequence agrees with an independent SQLite oracle
