Feature: Neighbouring controls share one hairline
  Controls that act as one control are drawn in one frame with one divider between neighbours, never as bordered chips with a gap.

  Scenario: A button group is one shared border group
    Given a group of three grouped buttons
    Then the group is a shared border group with its own frame
    And every button is a shared border cell without a border of its own

  Scenario: A segmented choice is one shared border group
    Given a segmented choice with three options
    Then the radio group is a shared border group
    And every option is a shared border cell
    And no option draws its own left border

  Scenario: A plain notice puts its actions in one shared row
    Given a plain notice with two actions
    Then both actions are cells of one shared border group
    And the notice draws no frame of its own

  Scenario: The 3D stage toolbar is one row of cells
    Given the 3D stage stylesheet
    Then the toolbar has no gap, no group frame, no first-child rule and no 2px high-contrast border

  Scenario: Unavailable figures and collections leave edges to one owner
    Given the sheet, element, figure and gate sources
    Then none draws a dashed box and the collections never zero a border inline
