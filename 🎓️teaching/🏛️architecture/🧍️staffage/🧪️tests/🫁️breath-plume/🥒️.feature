Feature: Breath plume
  The exhale jet leaves the mouth anchor along the head bone rotation plus the emitter direction and dilutes within about a metre.

  Scenario: Direction follows the bone
    Given a head bone rotated by 20 degrees and an emitter direction of -10 degrees
    When the head bone is rotated by 30 degrees more
    Then the first plume segment rotates by 30 degrees within 2 degrees

  Scenario: Looping duty reproduces the parcels
    Given an exhale period of 4 s
    When the thermal field is solved at t and t + 4 s
    Then all parcels are identical

  Scenario: Excess decays monotonically
    When the thermal field is solved
    Then the excess temperature along the streamline never increases
    And the vapour excess decays by the same ratio as the temperature excess

  Scenario Outline: Parcels stay close to the mouth
    Given a wind of <wind> m/s
    When the thermal field is solved at any time of the period
    Then no parcel is farther than <limit> m from the mouth

    Examples:
      | wind | limit |
      | 0    | 1.2   |
      | 2    | 2     |

  Scenario: Streamline against the solve_ivp oracle
    When the streamline is compared with the RK45 reference
    Then every point lies within 0.01 m of the reference polyline
