Feature: Cargo Source boundary refusal facts
  Scenario: Source is refused before allocation with bounded exact evidence
    Given an issued Source read request and a physical file
    When file slots, available bytes or expected extent refuse it
    Then the error retains the exact path, observed size and request bounds
    And neutral cases agree with the independent SQLite reason oracle
    And no Source limit is increased or compiler dispatch performed
