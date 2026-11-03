@capability-staffage-animation
Feature: A cast shows one frame per instant of scene time
  Live playback, seeking, previews and the exporter share one pure function of cast time: frame = floor(t · fps + 1e-9); a looping cast wraps its
  time at durationS exactly, any other cast holds its last frame. The exporter gives a loop whose durationS · fps is not whole the frame rate
  count / durationS, so the frames span the period and the wrap lands on frame 0.

  Scenario: Export frame n comes back as frame n
    Given the exported schedule of a clip
    When each frame time n / fps is played back
    Then the frame shown is n

  Scenario: A loop wraps without a seam
    Given a loop of 0.1 s exported at 15 fps
    When it is played at 0.1 s
    Then it shows frame 0 again

  Scenario: A transient holds its last frame
    Given a non-looping clip
    When it is played beyond its duration
    Then it shows the last frame
