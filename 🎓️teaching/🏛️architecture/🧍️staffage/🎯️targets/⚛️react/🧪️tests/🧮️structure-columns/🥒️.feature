Feature: The structure tables show every component of force and moment
  Loads, reactions and the equilibrium residual list Fx, Fy, Fz, Mx, My and Mz; joint loads add torsion; every label and unit is in English and German.

  Scenario: Components in English and German
    Given the asset page of the reference walker
    When the structure section is shown
    Then the tables name the lateral force, the moments about x, y and z and the torsion in the reader's language
    And the equilibrium residual carries newton-metre units
