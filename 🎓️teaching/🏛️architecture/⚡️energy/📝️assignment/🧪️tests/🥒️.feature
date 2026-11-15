Feature: Heating and cooling finger exercise
  Scenario: Complete bilingual self-contained assignment
    Given the slide assumptions and an explicitly fictional teaching building
    When the German and English assignment sources are published
    Then each has heating demand, heat pump electricity, cost, solar cooling and duct sizing tasks
    And every added numerical assumption is identified as an exercise assumption
    And no historical deadline or unavailable submission link is presented as current
  Scenario: Dimensional and numerical consistency
    Given the language-neutral heating and solar design cases
    When the formulas are evaluated independently with mathjs
    Then the results agree with the declared expected values
