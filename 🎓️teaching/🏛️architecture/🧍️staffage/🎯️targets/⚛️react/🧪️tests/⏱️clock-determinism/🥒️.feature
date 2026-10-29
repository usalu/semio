Feature: A shared clock keeps figures frame-identical
  Scenario: Elapsed time is accumulated, not sampled
    Given a looping clock of 1 s at 30 frames per second playing at rate 2
    When the scheduler ticks at 1000 ms, 1250 ms, 1500 ms and 1750 ms
    Then the clock time is 0.5 s, then 0 s after wrapping at 1 s, then 0.5 s again
  Scenario: Stepping lands on exact frames
    Given a paused non-looping clock of 1 s at 30 frames per second
    When it steps 3 frames, back 5 frames and seeks 7.5 s
    Then the times are 0.1 s, 0 s and 1 s
  Scenario: Reduced motion starts paused
    Given the user prefers reduced motion
    When a clock is created
    Then it is paused until play is pressed
  Scenario: Two figures on one clock show one instant
    Given two figures of the reference walker in different views bound to one clock
    When the clock seeks to frame 10 and steps 5 frames
    Then both figures report the same time, 10/30 s and then 15/30 s
