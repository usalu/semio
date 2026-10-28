Feature: Original native template identity requirements
  Scenario: A released application is staged without invented identities
    Given an independently validated native template source case
    And its explicit identities are absent
    When original planning reaches the completed identity-requirements frontier
    Then every planner key and GlobalId requirement matches the neutral fixture
    And no mutation has been emitted
    And continued mutation planning refuses the missing exact identity roster
    And every original partial owner retires after cancellation
