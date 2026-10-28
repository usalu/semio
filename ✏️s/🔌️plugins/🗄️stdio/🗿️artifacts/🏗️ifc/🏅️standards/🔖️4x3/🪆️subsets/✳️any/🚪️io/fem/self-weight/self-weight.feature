Feature: Exact per-case self-weight projection
  Scenario: upright implicit weight
    Given a case with true FEM selfWeight and explicit SI material and section data
    When exported to IFC4x3 and projected back
    Then SelfWeightCoefficients is [0,0,-1] without synthesized actions
    And the case flag and original explicit loads are retained
  Scenario: unsupported gravity vector
    Given a valid IFC coefficient vector outside zero or upright unit gravity
    Then analytical projection refuses without publishing a partial model
