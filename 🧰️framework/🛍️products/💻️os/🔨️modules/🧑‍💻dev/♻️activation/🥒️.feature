Feature: Completed development activation
  Scenario: Warm restoration does not reload unchanged artifacts
    Given a receipt records the completed artifact identity of each selected plugin
    When Nx completes an identical preparation with a later wall clock
    Then the receipt and plugin activation timestamps remain byte-identical

  Scenario: Completed changed artifacts receive a fresh activation identity
    Given an earlier receipt and a clock that moved backwards
    When a completed plugin artifact digest changes
    Then only that plugin receives a strictly newer activation timestamp
    And removed plugins leave the next snapshot

  Scenario: Failed preparation cannot announce partially restored artifacts
    Given the receipt exists independently of cached module directories
    When cached module files change without a new activation receipt
    Then no activation notification is emitted
    And closing the receipt observer releases its filesystem watcher

  Scenario: Nx separates preparation from runtime publication
    Given every playground has two authored preparation profiles
    Then each profile has an uncached activation target
    And its only prerequisite is the matching completed preparation
    And its runtime receipt is outside all cached outputs
    And the activation target has an editor command

  Scenario: One server consumes activated artifacts
    Given Nx has completed the selected activation
    Then each variant/profile has a continuous uncached server target
    And its only prerequisite is that activation
    And its leaf command starts the server without compiling or scheduling work
    And the server has an editor command
