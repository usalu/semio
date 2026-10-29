Feature: Two-foot stance of a person-like rig
  A language-agnostic specification of the stance case; the oracle is an anastruct beam on hinge and roller in 🧫️fixtures/🧍️two-foot-stance/🔣️.json.

  Scenario: Lever rule between the feet
    Given an anatomical rig with masses, two ground contacts 0.30 m apart and the centre of mass right of the midpoint
    When the structure is solved with the arm hanging down and no hand load
    Then each foot carries the weight share given by the lever rule and the right foot carries more
    And each contact pressure equals the normal reaction divided by the footprint area

  Scenario: A hand load flips the heavier foot
    Given the arm reaching left with a 150 N downward load at the hand
    When the structure is solved
    Then the left foot carries more than the right foot
    And the joint moment at the shoulder equals the sum of vertical forces times their horizontal lever arms about the shoulder, counter-clockwise positive

  Scenario: Anatomical values are not clinical
    When the notes are read in English and German
    Then they state that joint values are simplified rigid-segment resultants, not clinically validated bone or muscle forces
