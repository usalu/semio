Feature: Selected Source runtime entry roles
  Scenario: All26 declared Source selections preserve executable module custody
    Given the exact authored target declarations and actual selected runtime entries
    When the task observes source as freshness data
    Then each actual runtime module remains an explicit runtimeSourceEntries member
    And existing data inputs remain data
    And independent SQLite JSON relations agree with actual manifest entries
    And the selected script or verifier owns every declared runtime entry
