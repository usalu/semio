@capability-staffage-animation
Feature: Planted feet do not slide
  With root motion `plant`, the root x is integrated at 240 Hz from t = 0. While a contact span is active its anchor keeps the world x it had when the
  span began; when several spans are active the earliest planted one drives the root. A later span that outlasts the driver takes over through a smoothstep
  crossfade over the overlap, so the root position and velocity are continuous at handovers. A looping clip advances the root by `advanceM` per cycle so
  cycles concatenate seamlessly. With ground `auto` the lowest active contact anchor sits on y = 0.

  Scenario: A two-leg walker over three cycles
    Given the reference walker with 0.1 s double-support overlaps
    When the clip is sampled every 1/240 s for three cycles
    Then every planted anchor drifts less than 0.1 mm in x outside the overlaps
    And the root x never jumps at handovers or at the loop seam
    And the lowest active contact anchor stays on y = 0
