@capability-staffage-animation
Feature: Looping clips can spin
  A looping clip may end with a closing key at t = durationS, which is the end of the last segment. The validator requires the closing key to equal the first key,
  except that a revolute joint without limits may differ by whole turns of 360 degrees. Sampling wraps t into [0, duration) and adds the difference of the closing
  key to the first key once per completed cycle, so the evaluated angle at t + duration differs by exactly that difference (720 degrees for two turns). Smooth
  tangents at the seam use the wrapped neighbour with the same offset, so angular velocity and the central differences of the kinematics are continuous. A looping
  clip without a closing key still wraps from its last key back to its first.

  Scenario: A wheel spins two turns per loop
    Given a wheel whose closing key is the first key plus 720 degrees
    When it is evaluated at t and at t + duration
    Then the wheel angle differs by exactly 720 degrees
    And the angular velocity is the same on both sides of the seam

  Scenario: A limited joint cannot spin
    Given a wheel joint with limits and a closing key 720 degrees away
    When the asset is validated
    Then a loop-closing issue is reported

  Scenario: Instants without contact need a ballistic phase
    Given an asset without structure supports and a clip with a gap between contact spans
    When the asset is validated
    Then an unsupported-instant issue is reported unless a ballistic phase covers the gap
