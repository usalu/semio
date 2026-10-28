Feature: Original external BIM source capture
  Scenario Outline: Admit authored BIM before semantic projection
    Given an external BIM model and original control from bim-source-cases.json
    When source capture and domain record admission finish
    Then semantic callbacks only observe the private admitted model
    And input accessors never execute
    And finite limits and original cancellation refuse before semantic work

