Feature: Simply supported beam structural solution
  A language-agnostic specification of the simply supported case; the oracle is anastruct in 🧫️fixtures/🌉️simply-supported/🔣️.json.

  Scenario: Reactions, maximum moment and midspan deflection
    Given a 4 m steel beam with a pin at x = 0 and a roller restraining vertically at x = 4 m
    And a self-weight UDL of 1000 N/m and a 5000 N point load at midspan
    When the structure is solved quasi-statically
    Then both reactions equal 4500 N
    And the maximum sagging moment is q·L²/8 + P·L/4 = 7000 N·m at midspan within 0.5 % of anastruct
    And the shear drops by 5000 N across the point load
    And the midspan deflection equals 5qL⁴/384EI + PL³/48EI within 0.5 % of anastruct
    And the uniform self-weight is discretised into at least 16 sub-forces
