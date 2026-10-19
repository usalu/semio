Feature: Exact Cargo workspace package ownership
  Scenario: A nested workspace depends on a package owned by its ancestor
    Given physically nested packages with explicit distinct workspace owners
    When the nested workspace publishes its current membership
    Then each foreign-owned package directory is excluded from automatic membership
    And current members and explicit package owners remain unchanged
    And selected-only publication still reads actual ancestor ownership
    And existing authored exclusions are retained by selected publication

  Scenario: A workspace owns exact packages outside its general module prefix
    Given exact manifest inputs for the owned extension packages
    When current workspace membership is published
    Then those packages retain their owner and explicit membership
    And independent TOML and glob implementations admit every authored framework member
