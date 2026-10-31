Feature: Parcel jet model
  A parcel is a slice of a turbulent jet or plume that entrains ambient air; its motion follows the round-jet law and its ODE.

  Scenario: Momentum jet follows the closed-form law
    Given an isothermal exhaust jet of 25.5 m/s from a 50 mm nozzle in calm air
    When a parcel is followed for 0.4 s
    Then its distance, speed and radius follow u = U0 / (1 + 2 alpha x / r0) within 1 percent

  Scenario Outline: Parcels match the scipy re-integration
    Given the parcel case <case>
    When the parcel is read at several ages
    Then position, radius, speed, temperature excess and condensate match scipy solve_ivp

    Examples:
      | case            |
      | momentum-jet    |
      | hot-jet         |
      | condensing-puff |
