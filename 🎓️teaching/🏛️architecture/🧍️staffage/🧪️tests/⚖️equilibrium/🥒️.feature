Feature: Equilibrium of seeded random rigid trees
  A language-agnostic specification; the seeded trees and the numpy oracle live in 🧫️fixtures/⚖️equilibrium/🔣️.json.

  Scenario Outline: Random tree with <supports>
    Given a rigid tree of 4 to 9 bones with random masses and point loads generated with a fixed seed
    And supports of the family <supports>
    When the structure is solved quasi-statically
    Then the sum of all forces and moments about the origin is below 1e-9 relative
    And the reactions equal the numpy equilibrium solution (or the numpy equal-stiffness spring solution for three pins)
    And the joint loads balance each bone against its children
    And the cut at the start of the root member carries no resultant

    Examples:
      | supports   |
      | fixed      |
      | pin-roller |
      | three-pins |
