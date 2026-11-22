Feature: Compact catalog payload

  Scenario: The shipped revision is the revision of the authored text
    Given a catalog asset file
    When it is served under the staffage asset query
    Then its revision equals the revision of the authored file text

  Scenario: The shipped asset is the authored asset
    Given a catalog asset file
    When it is served under the staffage asset query
    Then its parsed json equals the asset of the file and carries no indentation
    And its validation findings equal those computed from the shipped JSON

  Scenario: Each scale can be read directly from the shipped text
    Given a catalog asset containing volume geometry at four scales
    Then each indexed character range parses to the corresponding authored scale
    And the remainder of the asset parses independently of its volumes

  Scenario: Large models travel as compressed data
    Given the authored JSON of a catalog model
    When the viewer loads its compressed payload
    Then a native decoder recovers the exact authored header and each independently compressed scale
