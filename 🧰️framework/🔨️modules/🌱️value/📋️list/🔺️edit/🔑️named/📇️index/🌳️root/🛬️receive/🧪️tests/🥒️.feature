Feature: Original Named Root Receiving

  Scenario: An owning row reader preserves the genuine original projection
    Given a published named root with its retained source and sorted key directory
    When zero-item and zero-depth row reader grants are refused without allocation
    And an admitted row reader retains the owning source's exact root projection
    And the named root is closed
    Then its projected index still authenticates against the independent reader
    And original row addresses, values and funded closure receipts remain exact

  Scenario: Explicit forward payload funding includes parent metadata
    Given a4096byte child payload and a4120byte total quote
    Then forward receiving retains the exact4120byte quote
    And close selection retains its4096byte body independently
