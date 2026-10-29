Feature: Tied canopy structural solution
  A language-agnostic specification of the tied canopy; the oracle is an anastruct truss model in 🧫️fixtures/🪢️tied-canopy/🔣️.json.

  Scenario: Tie-rod tension from a pin and a roller along the rod
    Given a 2 m canopy with a pin at the wall and a roller restraining along a rod from the tip to a wall point 1 m above
    And a self-weight UDL of 800 N/m and a 500 N point load at midspan
    When the structure is solved quasi-statically
    Then the reaction at the rod support acts only along the rod and the rod tension equals the anastruct truss axial force
    And the pin reaction equals the anastruct pin reaction
    And the canopy carries a constant axial compression equal to the horizontal rod component
    And the bending moment vanishes at both supports
