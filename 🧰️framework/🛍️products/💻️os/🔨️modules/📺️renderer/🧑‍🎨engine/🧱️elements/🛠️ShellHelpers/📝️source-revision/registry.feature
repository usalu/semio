Feature: Exact document source owners survive focus changes
  Scenario: Background work remains pending across focus switches
    Given an admitted source work hold on document A
    When the user focuses document B and returns to document A
    Then document A retains the same source lifetime and hold
    And document B has an independent admitted identity

  Scenario: Plugin replacement retires only its actual source owners
    Given active documents using different plugin handles
    When one exact handle retires
    Then its captures and work holds are retired
    And a coincident replacement handle acquires a new source lifetime
    And the retired handle cannot acquire a new source lifetime
