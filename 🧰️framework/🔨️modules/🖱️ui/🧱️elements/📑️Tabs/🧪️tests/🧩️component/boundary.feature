Feature: Tabs preserve their painted chip boundaries
  Scenario: Only the selected tab paints a rectangular frame
    Given a tablist with selected and unselected chips
    Then the tablist is an attachment discovery group with its actual orientation
    And only the selected chip registers its full rectangular painted frame
    And no speculative attached edges or extra cell dividers are emitted
    When selection changes by keyboard
    Then frame ownership follows the newly selected chip
