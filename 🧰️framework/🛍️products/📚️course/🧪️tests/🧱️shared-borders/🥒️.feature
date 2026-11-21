Feature: Single border ownership in the course

  Scenario: Results show a score histogram
    Given a result with ten score bins
    Then the histogram owns one continuous 1 CSS pixel baseline
    And the individual bins have no border

  Scenario: Results show a crowd answer table
    Given a crowd answer table inside its figure
    Then the padded table belongs to the figure without a separate frame

  Scenario: A page picker appears in the narrow layout
    Given a course overview in a short touch viewport
    Then the form owns its bottom border
    And the page selector and submit button share one divider

  Scenario: Pet actions appear in a responsive grid
    Given the learner opens the pet controls
    Then the actions are cells of one border group
    And adjacent cells share one divider

  Scenario: A task asks for a number and unit
    Given a task with a numeric guess and selectable units
    Then the guess is a cell of its task row
    And the number and unit share one separator
