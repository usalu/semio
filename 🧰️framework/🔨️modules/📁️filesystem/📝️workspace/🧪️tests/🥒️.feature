Feature: Dashboard-Owned Source Workspace
  Scenario: An Existing Unicode Document Is Edited And Saved
    Given a physical workspace and its exact loaded revision
    When a bounded UTF-8 save command is published
    Then the document and immutable command outcome contain the new text
    And the predecessor remains available for recovery
  Scenario: Another Developer Changes The Document
    Given an editor revision and a later physical replacement
    When a save command checks the predecessor
    Then it refuses the conflict and preserves the foreign bytes
  Scenario: Cancellation Arrives At The Final Checkpoint
    When cancellation is observed before publication
    Then no source bytes are published
  Scenario: Publication Succeeds But Its Outcome Cannot Be Written
    Given an authenticated staged save intent
    When source publication succeeds and the outcome write fails
    Then the command exposes a retained version for readonly inspection
    And the foreign predecessor is never removed
