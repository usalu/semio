Feature: Explicit optional nodal load components
  Scenario: Authored zero is a populated degree of freedom
    Given a nodal load with one selected degree of freedom and a finite binary64 value
    When it passes through IFC export, STEP writing, parsing and import
    Then the selected value and signed-zero bits remain exact
    And the five inactive optional force components remain unset
  Scenario: Foreign component presence is explicit
    Given populated zero and nonzero force or moment components interleaved with unset components
    When analytical records are projected
    Then every populated component creates one source-ordered canonical load record
    And absent or unresolved components do not become invented zeros
  Scenario: Original authority and exact inverse
    Given a source-owned action and its actual dimensioned force or torque unit
    When a canonical component edit is inverted
    Then the exact graph and projected component records are restored
    And original cancellation or caller failure prevents publication
