@capability-touch-session @level-unit
# language: en
Feature: Touch contacts become host steps without any domain knowledge

  Scenario: A tap presses, releases and says tap; a held finger asks for the menu at the deadline while it is still down
    Given a touch session with the long press on
    When a finger lands and lifts within 450 milliseconds inside the slop
    Then the host presses and releases once and the session says tap
    When a finger lands and is held for 450 milliseconds
    Then the host presses additively and the long press asks for a menu at once
    When that finger lifts in place
    Then the host only releases additively and the menu is not asked for a second time
    When that finger moves out of its slop afterwards
    Then the host drags a rectangle

  Scenario: An idle host sleeps until the long press is due
    Given a finger that landed at a clock time
    Then the session names the wake time 450 milliseconds later
    When the host wakes at that time and ticks the session
    Then the long press fires with no further contact event
    And a lifted, moved or second finger leaves no wake time

  Scenario: The primary press waits until the finger has decided
    Given a touch session
    When a finger moves out of its slop
    Then the host presses at the start of the touch and drags
    When a second finger lands before the first one moved
    Then the host never presses and the two fingers are the camera
    When a second finger lands after the first one dragged
    Then the host releases the drag and the two fingers are the camera
    When one finger of a pinch lifts
    Then the finger left down does not resume a single-finger gesture

  Scenario: A device where a drag never edits a handle orbits with one finger
    Given a touch session whose one-finger drag orbits
    When a finger moves out of its slop
    Then the host orbits by the movement after the slop and never presses

  Scenario: The pinch law of the native orbit controller equals the web orbit rig
    Given the orbit rows of the fixture computed by the TypeScript "applyPinchToOrbit"
    When the Rust twin applies the same pinch steps
    Then position, target and zoom agree to 1e-9
    And the wheel and drag units of the native orbit controller reproduce the same pose

  Scenario: A lost lift is forgiven
    Given a finger that never lifted
    When another finger lands more than three seconds later
    Then the old gesture is cancelled first
