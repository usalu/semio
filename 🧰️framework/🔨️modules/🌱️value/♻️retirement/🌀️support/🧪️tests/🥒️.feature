Feature: Controlled retirement support terminates on recursive type graphs

  Scenario: Every reachable unsupported leaf refuses a finite recursive conjunction
    Given the nine shared self-cycle, mutual-cycle and diamond type graphs
    When the actual support guard evaluates every reachable component
    Then the result equals independent SQLite reachability
    And no query allocates or releases heap storage

  Scenario: Query cancellation by panic restores exact synchronous ownership
    Given a live outer type frontier and a nested support callback
    When the nested callback panics and the caller catches it
    Then the original outer frontier remains active
    And subsequent complete queries observe their current capabilities
