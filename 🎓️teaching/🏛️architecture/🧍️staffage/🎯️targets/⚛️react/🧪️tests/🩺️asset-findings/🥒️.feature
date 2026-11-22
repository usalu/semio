Feature: Findings of an asset are computed off the thread that serves it

  Scenario: The check runs on a thread of its own
    Given an authored catalog asset file
    When its findings are asked for
    Then the asking thread keeps running its timers while the check runs
    And the six parts of the check are reported in order
    And the findings equal those `validateAsset` computes for the same document

  Scenario: One check per revision
    Given two readers who ask for the same revision
    Then one check runs and both hear its progress and its findings
    And a later question for that revision is answered from the kept findings without a check

  Scenario: A check nobody waits for is stopped
    Given a check that waits for a free thread and a check that runs
    When the reader of the waiting check withdraws
    Then it never reaches a thread
    When the only reader of the running check withdraws
    Then its thread is ended and the next question gets a fresh thread
    And a check one of two readers withdrew from still finishes for the other

  Scenario: The development server serves the asset before its findings
    Given the staffage asset plugin on a listening development server
    When an asset module is loaded
    Then the module is returned at once with the address of its findings and without findings
    And the module accepts its own updates, so a changed file reloads only the pages that loaded it
    When that address is read
    Then the stream carries the progress of the check and then the findings `validateAsset` computes
    And an address of a revision that is no longer on disk answers stale
    And a reader who closes the stream early withdraws the check

  Scenario: A build waits for the findings
    Given the staffage asset plugin without a listening server
    When an asset module is loaded
    Then the module carries the findings and no address
