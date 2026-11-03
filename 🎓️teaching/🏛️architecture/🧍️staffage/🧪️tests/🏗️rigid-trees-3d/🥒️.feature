Feature: Seeded three-dimensional rigid trees
  Eight seeded trees with lateral offsets, sagittal angles, abduction, twist, uniform masses and loads with three force components and three couples; the numpy oracle composes the frames as matrix products and solves the root reaction, the joint loads and the member cuts in a right-handed frame with explicit conversion.

  Scenario: Reactions, joints and cuts
    Given a seeded tree with a fixed support at the root
    When the structure is solved
    Then every bone frame equals the oracle frame
    And the reaction, the joint force and moment vector, torsion and both bending moments match the oracle in all six components
    And all six residual components vanish
