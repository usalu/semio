Feature: Original named index retirement work
  Scenario: Every physical original row retains finite paid cleanup work
    Given neutral original sizes 1, 17 and 65537
    And a finite retirement budget of 1024 fixed turns plus 256 turns per row
    When the actual original persistent source closes through exact selected grants
    Then each physical receipt fits the same issued currencies
    And paid progress does not consume the separate 16-turn idle limit
    And an unchanged public receipt and selected quote cannot repeat indefinitely
    And the finite total agrees with the independent SQLite arithmetic oracle
