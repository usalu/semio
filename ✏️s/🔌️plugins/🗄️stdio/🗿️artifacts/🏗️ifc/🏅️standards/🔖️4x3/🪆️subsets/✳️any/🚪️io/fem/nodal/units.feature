Feature: Native nodal force and torque units
  Scenario: Released SI and derived dimensional words
    Given the original project unit assignment
    When one explicit nodal component is projected
    Then force requires one unscaled FORCEUNIT with dimensions 1 1 -2 0 0 0 0
    And moment requires one unscaled TORQUEUNIT with dimensions 2 1 -2 0 0 0 0
    And absent duplicate scaled or contradictory units refuse
    And a derived element with a derived-unit child refuses even with exponent zero
    And units for absent components do not invent records
  Scenario: Original receiver authority
    Given the original snapshot and native receiving owner
    Then dimensional traversal spends original grants and preserves parent stage
    And cancellation retains partial owners for original paid close
