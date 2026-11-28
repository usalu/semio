Feature: A renewable idle window is never frozen into a physical cleanup body cutoff
  Scenario: A publication waits longer than the idle window for a reader of its destination
    Given a renewable control whose idle window is shorter than its nonrenewable maximum
    When an accepted physical cleanup body is funded beneath it
    Then the body cutoff is the maximum minus the cleanup reserve, never the idle window
    And a declared wait longer than the idle window leaves the body control alive
