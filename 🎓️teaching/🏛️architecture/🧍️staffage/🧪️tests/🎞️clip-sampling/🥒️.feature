@capability-staffage-animation
Feature: Clips are sampled deterministically at any time
  A clip key shows a pose at its time; between two keys every bone offset and the thermal binding blend by the eased weight of the earlier key
  (linear, smooth = 3t² − 2t³, in = t², out = 1 − (1 − t)², step = 0 until 1), angles linearly in degrees. Scalar tracks interpolate with their own ease
  and hold their end values. A looping clip wraps its time modulo the duration, any other clip clamps to [0, duration]. Frame n sits at t = n / fps:
  frame = floor(t · fps + 1e-9). Contacts follow the clip's half-open spans; a phase is the first one containing the time.

  THE REFERENCE for the ease curves is three.js `MathUtils.smoothstep` and d3-ease `easeQuadIn/easeQuadOut/easeLinear`; the key samples are the
  hand-computed numbers of the fixture.

  Scenario: Sampling between keys
    Given the idle-breathing clip with a torso override of 1 degree at t = 2 s
    When the clip is sampled at t = 0.5 s
    Then the torso offset is 0.15625 degrees, the smooth weight 0.25 of 1 degree

  Scenario: A looping clip repeats
    Given a looping clip of 4 s
    When it is sampled at t and at t + 4 s
    Then both states have the same pose, frame and phase

  Scenario: A transition clamps outside its duration
    Given a transition from stand to carry-bag over 1 s
    When it is sampled before 0 and after 1 s
    Then it shows the first and the last pose
