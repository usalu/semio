Feature: Released IFC SELECT membership in TYPEOF
  Scenario Outline: Query containing SELECT closure
    Given an actual named runtime type in typeof-select-cases.json
    When TYPEOF closes over the released direct SELECT member graph
    Then every containing SELECT including transitive parents is present
    And arbitrary defined aliases are never inferred from primitive value domains
    And current qualified seed and unknown behavior are preserved
  Scenario Outline: Enforce the official MaxOneColour proposition
    Given one or two distinct RGB colours assigned to an IfcFillAreaStyle
    When the actual complete released population is validated
    Then the named MaxOneColour result matches its explicit official limit

