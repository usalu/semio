@capability-tui-shared-splitters
Feature: Adjacent tiled windows share their splitter cell
  Scenario: Weighted stacks share one edge while covering the available axis
    Given an axis and its available extent
    When equally weighted stacks are resolved
    Then adjacent stacks overlap by one cell
    And the first and last edges remain inside the available extent
    And the shared cell routes a pointer drag to the splitter
    And shared contours form connected junctions with hover taking priority over focus
    And the navbar and footer share their boundary rows with the tiled area
  Scenario: A focused splitter is accessible without a pointer
    Given a focused splitter
    When an arrow key moves it along its axis
    Then a one-cell splitter delta is emitted
