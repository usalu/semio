Feature: Complete string clone output custody
  Scenario: Reserved and partially copied text remains owned by its cursor
    Given an admitted original UTF8 string and a finite copy budget
    When reservation or copying progresses without completion
    Then readiness is false and taking yields no output
    And only the complete owned payload may be taken once
  Scenario: Closing retires the complete payload instead of transferring it
    Given a complete string clone whose output was not taken
    When closure begins
    Then readiness is false and taking yields no output
    And every cancelled and completed cursor closes with exact physical receipts

  Scenario: Closing quotes one selected physical transition
    Given a bound String cursor with retained source aliases
    When every close turn receives exactly its quoted currencies
    Then the cursor reaches empty custody within the finite close bound
    And no transition substitutes independent zero-copy observations for selected handoff work
    And the shared grant vectors agree with the SQLite currency oracle
