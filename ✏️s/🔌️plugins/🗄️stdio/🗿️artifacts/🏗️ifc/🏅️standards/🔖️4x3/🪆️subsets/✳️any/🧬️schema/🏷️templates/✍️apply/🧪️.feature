Feature: Apply Selected Published IFC4X3_ADD2 Templates
  The application is an owned command that publishes one admitted sparse diff.
  Identities and GlobalIds are retained command data.

  Scenario Outline: Attach a selected template to its released carrier
    Given an IFC4X3_ADD2 graph and the applicable target from cases.json
    And one explicitly selected entry of the released template
    And the exact identity keys returned by the controlled planner
    When applyTemplate completes structural and EXPRESS admission
    Then the carrier and attachment match the carrier row in cases.json
    And performance properties attach to a performance history controlling the target
    And type properties and quantities extend HasPropertySets
    And material and profile properties directly reference their target
    And applying the admitted inverse restores the entire source graph

    Examples:
      | template                        |
      | Pset_ActionRequest              |
      | Pset_ActuatorPHistory            |
      | Pset_ActuatorTypeCommon          |
      | Pset_MaterialCombustion          |
      | Pset_ProfileArbitraryDoubleT     |
      | Qto_ActuatorBaseQuantities       |
      | Qto_ArealStratumBaseQuantities   |

  Scenario: All released simple property and quantity forms
    Given the twelve selected forms listed in cases.json
    When each form is authored and independently authored with IfcOpenShell
    Then complete entity populations agree
    And both STEP documents pass IfcOpenShell EXPRESS validation

  Scenario: Exact selected values and unit references
    Given exact signed integer or Binary64 values in their declared template domains
    Then positive measures reject nonpositive values
    And quantity counts preserve integer values
    And table columns preserve their distinct defining and defined domains
    And table columns have equal nonzero cardinality and unique defining values
    And explicit physical units match the aliased measure unit type
    And bounds contain the selected set point
    And a named enumeration is reused only with its entire literal list and exact unit

  Scenario: Atomic publication with caller controls
    Given an owned source and expected event revision
    When capture, planning, copying or semantic admission is cancelled
    Or the event revision changes before publication
    Then no part of the prepared template graph is published
    And no template event is appended
    And successful undo and redo use the admitted exact inverse and diff

  Scenario: Closed finite commands
    Then application and diff commands reject unknown fields
    And captured data reject prototypes and accessor properties
    And generic inverse diffs reject overlapping targets and dangling references
    And synchronous template publication refuses missing async semantic admission
