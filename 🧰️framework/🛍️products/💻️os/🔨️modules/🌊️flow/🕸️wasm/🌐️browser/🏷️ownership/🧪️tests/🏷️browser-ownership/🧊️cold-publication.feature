Feature: Flow declaration publication starts from source only
  Scenario: The generated declaration directory is absent
    Given the owned Flow browser ABI schema
    And no generated declaration directory
    When the declaration producer runs
    Then it creates the owned parent directories
    And its exact output is admitted by the TypeScript declaration parser
