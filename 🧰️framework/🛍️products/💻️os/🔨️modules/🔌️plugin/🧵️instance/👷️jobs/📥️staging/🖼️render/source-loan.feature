Feature: Original render source loans
  Scenario Outline: Read the retained actual source before child work
    Given an original render identity and its retained immutable snapshot Arc
    When a caller requests a loan for an actual snapshot pointee
    Then admission requires the original caller and exact retained pointee
    And cancellation or retirement refuses the loan before child work
    And the loan allocates and copies no model value
    And the retained source closes after all child source leases
