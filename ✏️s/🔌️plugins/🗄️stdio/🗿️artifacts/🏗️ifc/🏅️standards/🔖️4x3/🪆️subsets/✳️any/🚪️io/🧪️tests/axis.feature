Feature: Normative Cartesian placement admission
  Scenario Outline: Evaluate original admitted IFC placement vectors
    Given an IFC4X3_ADD2 Cartesian placement in axis-cases.json
    When the placement is evaluated after central graph admission
    Then valid frames match the prescribed column-major matrix
    And invalid provision, coordinate kind, dimension or zero/parallel directions refuse
    And finite extreme directions normalize without overflow or subnormal underflow

