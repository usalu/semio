Feature: Cold native Go layout reads authored taxonomy data
  Scenario: Go build does not load unrelated taxonomy audits
    Given the workspace declares its physical taxonomy authority
    When the canonical Go projection reads its compiler input layout
    Then only schema-validated authored directory and file kind data is queried
    And source names retain their authored order and duplicate refusal
    And the full canonical loader produces the same Go layout
    And Go compiler admission and Source overlays retain their existing owners
