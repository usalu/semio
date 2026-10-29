Feature: The figure component is a faithful, accessible view of one scene
  Scenario: True paper size
    Given the reference walker at 1:100 in the dark line view
    When the figure is rendered with fit paper at zoom 1
    Then its width in CSS millimetres equals the paper width of the scene and doubles at zoom 2
  Scenario: Contain fits a parent box
    When the figure is rendered with fit contain
    Then it fills the parent box, keeps the paper aspect ratio and its viewBox equals the paper scene
  Scenario: Accessible name and alternative
    Given a thermal figure in English and in German
    Then it is a focusable role img whose aria-label names asset, pose, view and scale in the chosen language
    And a visually hidden table lists every analytic value with its unit
