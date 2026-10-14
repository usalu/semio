Feature: Retained computing progress admission
  Scenario: Complete typed facts precede host mutation
    Given the shared accepted and refused progress vectors
    When physical input receives zero and one-unit grants
    Then admitted progress remains retained across grants
    And all stale identities are admitted before host mutation
    And malformed input leaves the original host unchanged
    And cancel interruption and deadline observers refuse work
    And independent JSON Schema and Serde observe the same facts
