Feature: Visible Children Own Layout Space
  Scenario: Hidden children reserve neither size nor gaps
    Given a row or column with fixed and weighted children, including hidden children
    When the scene is laid out and one child is hidden or shown again
    Then hidden subtrees have empty rectangles
    And visible children own all available space without gaps for hidden siblings
    And the independent Taffy display-none layout yields the same rectangles