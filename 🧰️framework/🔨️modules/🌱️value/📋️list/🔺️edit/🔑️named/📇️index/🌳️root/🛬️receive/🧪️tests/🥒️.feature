Feature: Original Named Root Receiving

  Scenario: An owning row reader preserves the genuine original projection
    Given a published named root with its retained source and sorted key directory
    When zero-item and zero-depth row reader grants are refused without allocation
    And an admitted row reader retains the owning source's exact root projection
    And the named root is closed
    Then its projected index still authenticates against the independent reader
    And original row addresses, values and funded closure receipts remain exact
