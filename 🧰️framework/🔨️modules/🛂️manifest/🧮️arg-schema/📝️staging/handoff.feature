Feature: Original staged request context handoff
  Scenario Outline: A typed request retains its actual mounted window
    Given the real original source window and explicit locale in handoff-cases.json
    When the donated request context is checked before controller admission
    Then only the exact canonical token, window and locale are accepted
    And source revisions are independently rechecked against the actual document
