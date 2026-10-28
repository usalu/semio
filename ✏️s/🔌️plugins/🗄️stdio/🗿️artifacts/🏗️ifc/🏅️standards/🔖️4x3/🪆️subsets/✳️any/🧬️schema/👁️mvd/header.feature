Feature: Released IFC4x3 View Declarations Remain Reversible Header Edits
  Scenario Outline: Select A Released View Or Clear Existing Declarations
    Given The Description Strings From A Neutral Header Case
    And An Unchanged Implementation Level And File Metadata
    When The Explicit View Selection Plans A setFileDescription Mutation
    Then The Description Strings Equal The Neutral Expected Values
    And Central Diff Application And Exact Inversion Preserve All Other Data
    And Revision History Can Undo And Redo The Selection
    And STEP Reimport Preserves The Authored Header

  Scenario: Preserve Unknown View Names Without Substitution
    Given An Imported ViewDefinition With A Development Or Custom Name
    When Reading The Header Declaration
    Then Its Exact Name Is Retained And Its Released View Set Is Empty

  Scenario: Refuse Destructive Selection Of Malformed Declarations
    Given A Malformed ViewDefinition Header Entry
    When Selecting A Released View
    Then The Planner Refuses Without Publishing Any Edit

  Scenario: Cancel While Parsing A Long Declaration
    Given A Long ViewDefinition Entry
    When Cancellation Is Requested At An Interior Text Checkpoint
    Then Planning Stops Before Returning An Authoritative Mutation
