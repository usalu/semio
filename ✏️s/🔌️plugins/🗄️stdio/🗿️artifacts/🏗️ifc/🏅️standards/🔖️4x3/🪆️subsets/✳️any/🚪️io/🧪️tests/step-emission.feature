Feature: Bounded Native STEP Source Emission
  Scenario Outline: Emit the Complete Admitted Graph
    Given an original graph parsed from a step-emission.json data case
    When the source position cursor emits its complete physical file
    Then every fragment contains at most fragmentBytes ASCII bytes
    And parsing the output preserves every header and entity value
    And exact integer identities and negative zero retain their bits

  Scenario: Emit a Long Unicode String
    Given an admitted property containing many non-BMP and quoted characters
    When the cursor emits all its fragments
    Then each fragment remains bounded independently of property length
    And the borrowed original property backing stays unchanged
