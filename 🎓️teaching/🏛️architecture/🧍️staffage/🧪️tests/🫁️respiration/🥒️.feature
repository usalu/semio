Feature: Respiration
  Breathing is a continuous phase over the absolute timeline with half-sine flow whose integral is the tidal volume.

  Scenario Outline: Flow integral equals the tidal volume
    Given the breathing pattern <pattern>
    When the flow is integrated over one breath
    Then the exhaled and the inhaled volume equal the tidal volume
    And scipy quad of the same half-sine agrees

    Examples:
      | pattern |
      | rest    |
      | active  |
      | mixed   |

  Scenario: Patterns change without a phase jump
    Given a clip whose breathing track switches the pattern at 7 s and 14 s
    When the phase is sampled every millisecond
    Then it never decreases and never advances faster than the rate in effect
    And a new pattern starts at a breath boundary where the lung is at functional residual capacity

  Scenario: The phase continues across clip loops
    Given a looping walk clip of 1.2 s
    When the phase is sampled over 40 s
    Then no loop boundary resets or jumps it

  Scenario: Chest motion follows lung volume
    Given chest motion of 1.5 degrees at the resting tidal volume
    When the lung holds three times that volume
    Then the chest has rotated by 4.5 degrees

  Scenario: Respiratory heat matches CoolProp
    Given the exhaled state of 34 degrees and 95 percent humidity into 20 degrees and 50 percent
    When the respiratory heat is computed
    Then sensible and latent heat agree with CoolProp within 2 percent
    And their magnitude agrees with the ASHRAE regression used by the thermal solver
