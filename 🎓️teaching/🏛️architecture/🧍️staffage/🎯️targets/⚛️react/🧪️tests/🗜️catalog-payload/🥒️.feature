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
