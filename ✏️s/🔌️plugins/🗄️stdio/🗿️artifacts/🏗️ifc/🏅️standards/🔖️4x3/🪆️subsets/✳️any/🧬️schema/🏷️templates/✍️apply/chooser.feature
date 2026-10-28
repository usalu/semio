Feature: Select a released IFC4x3 property or quantity template
  Scenario: Explicit complete release selection
    Given the pinned IFC4X3_ADD2 template release
    When the template application action is rendered
    Then its required template input is a select control
    And all 760 released names occur exactly once in release order
    And all seven carrier kinds are represented
    And each option preserves the official name as its value and data label
    And the input label supports explicit English and German locales
    And no template is selected automatically
    And application still passes the complete retained semantic transaction gate
