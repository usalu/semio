Feature: The repository root is found the same way everywhere
  Scenario: The repository root is found the same way everywhere
    Given the shared workspace corpus
    When the launcher resolves each start directory
    Then it finds the nearest directory with paired markers and declared workspaces
    And a malformed package.json is refused instead of skipped
