Feature: Concrete numeric and dimensional issued-owner boundaries
  Scenario: Canonical compiler-issued bodies receive exact neutral inputs
    Given the pinned IFC4X3_ADD2 program and six selected functions
    When each closed concrete boundary executes its actual owner
    Then the observed scalar or declared field projection matches the neutral result
    And original input values remain unchanged
  Scenario: Deliberate unknown results and original control refusal remain explicit
    Given the released CorrectDimensions USERDEFINED branch
    Then its result is logical unknown without inventing missing implementation
    And original cancellation, exact failures and iteration denial cannot be redirected
