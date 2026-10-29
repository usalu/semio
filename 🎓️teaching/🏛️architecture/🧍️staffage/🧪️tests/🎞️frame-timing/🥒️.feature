Feature: Deterministic frame timing and honest video encodings
  Scenario: A clip is sampled at t = n / fps and encoded at that exact constant frame rate
    Given a looping clip of 1.2 s and a non-looping clip of 1.2 s at 25 fps
    Then the looping clip has 30 frames, the non-looping clip 31, and frame n sits at exactly n / 25 s
    When the walk cycle of the walking commuter is exported as MP4 and as WebM at 12 fps
    Then ffprobe reports 12/1 frames per second, as many frames as scheduled, a duration within one frame and equal packet spacing
    And the MP4 pixel format is yuv420p without alpha
    And the WebM carries the alpha_mode tag and decodes as yuva420p with libvpx-vp9
    And the first-party browser MP4 writer produces a stream ffmpeg decodes with the right frame count and rate
