Feature: An idle desktop overview stays within a memory budget
  The overview of the archive holds seven live figures and the grids of its pages hold many more previews. A live preview replaces its whole drawing on every
  frame, so what a page holds follows what its live previews weigh. Only the previews near the reader are live, together within a weight budget; every other
  preview is a still figure.

  Scenario: The English desktop overview idles
    Given the built site in a desktop window that chose English
    When the overview is open and nobody touches it for 40 seconds
    Then at every sample the live previews together weigh no more than the live budget
    And live previews are fewer than the previews on the page and some previews are still
    And every live preview of a card lies inside the window or within one row of it
    And the page holds no more than 400000 DOM nodes
    And the figures of the overview are drawn
