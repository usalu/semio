Feature: IFC4 selection and semantic authoring
  Scenario: A selected real product exposes its declared IFC attributes
    Given an IFC4 graph with a wall, site and project
    When the stable ifc-10 scene identity is selected through the entity interaction domain
    Then the inspector shows localized root, object, product and element attributes
    And placement and representation references navigate through the same interaction domain
    And the generic complete snapshot remains available in Details
  Scenario: Semantic input is replayable and reversible
    Given the selected wall with an optional name
    When its name is changed through the revisioned typed details input
    Then the retained snapshot action publishes the existing SetEntityArg mutation by stable entity id
    And its inverse restores the exact previous IFC value
    And clearing an optional attribute writes Unset without shifting argument positions
    And an empty string remains distinct from Unset
  Scenario: Invalid semantic values are refused before publication
    Given a malformed GlobalId, duplicate GlobalId, wrong scalar kind or overlong Unicode label
    When the semantic edit is admitted
    Then no semantic mutation is published
  Scenario: Inspection respects original request controls
    Given a selected product with forward and backward references
    When the original caller cancels or exhausts its fuel
    Then projection stops before publishing a partial inspector
