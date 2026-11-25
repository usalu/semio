Feature: An idle overview mounts a bounded number of pages
  Scenario: A grid behind the glass
    Given a grid that is not the one the reader is on and has no opened page
    When five minutes pass
    Then it has mounted none of its pages

  Scenario: The grid the reader is on
    Given the grid the reader is on, with eight pages and a budget of four
    When five minutes pass without input
    Then it has mounted at least one and at most four pages

  Scenario: Nested grids
    Given the grid the reader is on, each of whose pages holds a grid of its own
    When five minutes pass without input
    Then at most four pages are mounted in all, and none of them belongs to a nested grid
