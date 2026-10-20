Feature: Stable owned record projection
  Scenario: Wide output transfers its stable candidate owner
    Given a 2560-byte record with 1280-byte fields and a 4096-byte per-turn limit
    When its cursor allocates a funded uninitialized candidate
    And places one completed field per funded transition
    Then birth charges candidate capacity instead of copying uninitialized payload
    And final output transfers only the owning pointer
    And all cuts retire initialized fields and candidate capacity explicitly

  Scenario: Unfunded placement retains the same owner and target
    Given a firstparty output and an uninitialized field destination
    When any required placement currency is underfunded
    Then the same output owner is returned
    And the destination and physical heap remain unchanged

  Scenario: Larger inline extents require sufficient admission
    Given an output whose candidate or field placement exceeds the selected limit
    When a caller quotes the operation
    Then the complete extent remains visible and cannot be silently reduced
