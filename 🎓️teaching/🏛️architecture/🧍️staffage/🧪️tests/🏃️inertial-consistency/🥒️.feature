Feature: Inertial consistency of a moving body
  A language-agnostic specification; the numpy oracle lives in 🧫️fixtures/🏃️inertial-consistency/🔣️.json.

  Scenario: Reactions balance weight plus d'Alembert forces
    Given a body with segment masses and segment accelerations on two ground contacts
    When the structure is solved with dynamics "inertial"
    Then the sum of reactions equals Σ m·(g + a) in both axes and the residual is zero
    And the foot reactions equal the numpy solution

  Scenario: Friction saturates
    Given horizontal accelerations that need more friction than μ·N
    When the structure is solved
    Then each tangential reaction is capped at μ·N, the horizontal residual equals the unmet force and the notes say so

  Scenario: Flight and quasi-static twin
    Given all segments accelerating with -g and no contact, the solution has no reaction and zero residual
    And the same accelerations with dynamics "quasi-static" are ignored and reactions equal the weight
