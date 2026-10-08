Feature: Status Roles And Glyphs
  Tasks, rows and tabs report their state as a glyph and a colour role instead of bracketed text, so a
  tab never changes its length when its state changes and the colour is legible in both appearances.

  Scenario: Every status role is legible on every chrome surface
    Given the dark and the light appearance
    When success, warning, danger and info are drawn on the base, window, pane and panel surfaces
    Then every pair has a WCAG contrast ratio of at least 4.5 to 1
    And the WCAG reference pairs black on white and 767676 on white measure 21 and 4.54

  Scenario: A status is one cell wide in every glyph repertoire
    Given the unicode and the ascii glyph set
    When each status and each chrome symbol is measured
    Then it is exactly one cell wide and the ascii set contains only printable ASCII

  Scenario: The running status spins and every other status holds still
    Given the running status
    When four ticks pass
    Then the four spinner frames are shown in order and repeat

  Scenario: The appearance can change without losing the glyph repertoire
    Given a theme with the ascii glyph set
    When the appearance switches from dark to light
    Then the glyph set is still ascii
