Feature: Public Physics Domain Windows
  Scenario Outline: A native trajectory respects its authored domain
    Given a <kind> trajectory with independently specified physical parameters
    And an omitted, ascending, reversed or default-equal x domain
    When canonical mutation, replay, inversion and inference produce the preset
    And the actual native chart is compiled in English light and German dark themes
    Then every painted trajectory vertex matches independent physical equations and D3 scales
    And omitted windows retain the native auto-fit padding

    Examples:
      | kind                  |
      | projectile-trajectory |
      | orbital-trajectory    |
      | phase-space-diagram   |
