Feature: Explicit Field Control Association
  Scenario Outline: Name The Actual Authored Control
    Given the language neutral field address and optional explicit control address
    When the field association is calculated
    Then its label resolves to the actual labelable control
    And its description and error have stable unique addresses
    And a read only value does not invent a label target
