Feature: Cargo Source boundary refusal facts
  Scenario: Source is refused before allocation with bounded exact evidence
    Given an issued Source read request and a physical file
    When file slots, available bytes or expected extent refuse it
    Then the error retains the exact path, observed size and request bounds
    And neutral cases agree with the independent SQLite reason oracle
    And no Source limit is increased or compiler dispatch performed

  Scenario: One run verifies each directory once and shares unchanged Source
    Given a run-scoped reader over files in a deeply nested directory tree
    When several units read the same files and a sweep follows each unit
    Then directory lstats stay bounded per file and every unchanged file is opened once per run
    And a directory replaced after a read is refused as a concurrent edit by the next sweep
    And a changed file is read again and a cached file still obeys the extent bounds of each request
