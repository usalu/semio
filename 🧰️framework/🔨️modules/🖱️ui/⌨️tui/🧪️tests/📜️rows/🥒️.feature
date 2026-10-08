Feature: Row Lists With Incremental Filtering
  The launcher and every list of the terminal target hold tens of thousands of rows. Typing must stay
  fast, selection must keep its identity while the filter changes, and the pointer must select before it
  activates.

  Scenario: A query selects rows whose key contains every token
    Given the shared row filter fixture with Unicode and emoji labels
    When each listed query is applied
    Then exactly the listed rows pass in row order
    And a plain substring scan over lowercase tokens selects the same rows

  Scenario: Typing stays fast at fifty thousand rows
    Given fifty thousand rows
    When a query is typed one character at a time
    Then every keystroke after the first only examines the rows the previous query kept
    And no keystroke takes longer than five milliseconds in a release build

  Scenario: The viewport follows the selection
    Given a list taller than its view
    When the selection moves down past the view and back up
    Then the view scrolls by the overshoot only and keeps its offset while the highlight moves up

  Scenario: Page, Home and End work everywhere
    Given a list with a painted page of ten rows
    When Page Down, Page Up, End and Home are pressed
    Then the selection moves by a page, to the last row and to the first row

  Scenario: A single press selects and a double press activates
    Given a list under the pointer
    When a row is pressed once and then twice
    Then the first press only selects it and the double press activates it
    And the wheel scrolls the view without moving the selection
