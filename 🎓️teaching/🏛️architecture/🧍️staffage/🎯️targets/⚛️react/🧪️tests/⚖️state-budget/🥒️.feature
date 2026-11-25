Feature: Evaluated instants stay within a weight budget
  Scenario: A live figure passes many different instants
    Given a figure whose every frame is a different instant
    When more instants are evaluated than the budget holds
    Then the kept instants never hold more evaluated points than the budget
    And an instant that left the budget is evaluated again when it is asked for

  Scenario: An instant recurs
    Given an instant that is still kept
    When it is asked for again
    Then the same evaluated state is returned and nothing is evaluated

  Scenario: Several figures share one budget
    Given two assets
    When both pass more instants than the budget holds
    Then the budget holds for both together and the most recent instant of each is still kept
