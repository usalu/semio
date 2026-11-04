Feature: Select shared-boundary placement
  Scenario: A window-contained listbox opens flush
    Given a combobox inside a window silhouette
    When its listbox opens with no explicit side offset
    Then the listbox meets the trigger with zero gap
    And both painted rectangles register all physical edges
    And the listbox remains in its shell portal

  Scenario: Detached and explicitly offset listboxes preserve spacing
    Given a standalone combobox or an explicit side offset
    When the listbox opens or flips to fit the viewport
    Then its measured placement preserves the requested gap
    And no extra translation moves it outside those bounds
