Feature: Source-owned action staging under asynchronous retained form updates
  Scenario: Only a current resident control can change ephemeral arguments
    Given a form token with exact u64 generation and a full source revision
    And an ordinary input with an authored text binding
    When its original current UI intent carries partial decimal text
    Then only the authored field changes and no action is executed
    And event-supplied paths, fixed values and tokens are ignored
    And another surface, revision, node, key, trigger, action or replay cannot edit
  Scenario: Rapid text remains visible while validation is pending
    When successive preserve-layout text intents use the same resident publication
    Then every exact draft remains visible and increments the exact generation
    And Execute remains disabled until the newest generation is published ready
    And an older response cannot replace the draft or enable Execute
  Scenario: Layout and owner changes retire old controls
    When a choice, reset, source revision change or close occurs
    Then the previous control cannot edit or display an optimistic value
    And a late response cannot recreate a closed owner
    And two windows maintain independent generations, drafts and Execute readiness
