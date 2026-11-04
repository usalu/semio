Feature: Floating menu placement
  Scenario: Window chrome opens a flush menu
    Given the trigger belongs to a window silhouette
    When no explicit gap is supplied
    Then the menu touches the trigger and participates in shared border geometry
  Scenario: Explicit gaps and collision flips remain detached
    Given an explicit gap or insufficient space on the requested side
    When the menu is placed
    Then the gap is preserved or the menu flips without suppressing an entire border
  Scenario: Shell-owned portal
    Given a shell floating layer outside its clipped panes
    When a popover opens
    Then its content belongs to that shell layer
