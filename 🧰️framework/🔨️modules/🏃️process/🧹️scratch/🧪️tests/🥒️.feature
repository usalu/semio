Feature: Repo-Owned Scratch Lives In One Declared Home And Never Outlives Its Work

  Scenario: Scratch Is Removed However The Work Ends
    Given a scratch directory made for some work
    When the work completes, throws or is cancelled
    Then the directory and everything below it is gone

  Scenario: Scratch Names Its Owner
    Given a scratch directory
    Then its name is the process id, a bounded prefix and a random part, matching the unit pattern of the cache ledger

  Scenario: A Process Gets A Private Temporary Directory
    Given a test or tool process
    When it starts with scratch isolation
    Then the operating system temporary directory variables point below the declared home and the directory is removed when the process ends
