Feature: Selected templates use the original native IFC authoring route
  Scenario Outline: One command retains each independently authored graph and inverse
    Given a source STEP and selected application from native-cases.json
    When the original decimal application is received under the caller's work grants
    And each selected carrier is reduced to existing IFC mutation leaves
    Then central mutation application equals the independently authored expected STEP graph
    And the exact generic inverse restores the complete source graph
    And complete EXPRESS validation of the expected graph reports no diagnostic

  Scenario: Missing retained semantic authority cannot publish an unchecked graph
    Given an original captured IFC4x3 artifact and a valid selected application
    When actual forward and inverse native Store wire admission finishes
    And no retained EXPRESS validation authority exists
    Then the registered native command refuses with template.semantic-admission-unavailable
    And no artifact mutation or event is published
    And all original and partially received owners close under the original retirement grants

  Scenario Outline: Cancellation retains original command and graph custody
    Given an application whose selected values own large strings and arrays
    When cancellation arrives at a frontier from native-contract.json
    Then no mutation batch is emitted
    And owned receiving, planning, codec and source leases drain through original closure

  Scenario: An immutable catalog cannot be constructed inside a small request grant
    Given the released catalog has not been initialized outside retained request work
    When a native template planner requests catalog access
    Then it refuses with template.catalog-admission-unavailable

  Scenario: Existing templates remain visible through a property-set definition select
    Given native-select.json wraps an existing attachment as IfcPropertySetDefinitionSet
    When the selected released template is requested again
    Then authoring refuses with existingTemplate before testing newly retained identities
