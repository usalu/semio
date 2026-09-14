@capability-viz-charts-polar
@oracle-d3-shape
@comparison-floating-point-v1
Feature: The pie family assigns its angles exactly like d3-shape's pie generator
  Every leaf of taxonomy §6's pie and donut families — exploded, nested, semi-circle, multi-ring and
  the gauge donut — is one angle assignment seen through a different radius band, so the angles are
  the only thing that can be wrong in all of them at once. `d3-shape`'s `pie()` is the reference:
  arcs are returned in the order of the input data while the ANGLES are handed out in the sort
  order, which is the part a reimplementation gets wrong, so the sorted and the unsorted scenario
  differ only in that rule.

  The padded scenario exercises the two clamps d3 applies before the angles are computed: the total
  sweep is clamped to a full turn in either direction, and the pad angle is clamped to the sweep
  divided by the number of slices, so a pad wider than a slice cannot invert the arc. A partial
  circle with a non-zero pad angle is the only configuration where both clamps and the padded value
  scale interact, which is why it is a scenario of its own rather than a variation of the first.

  The donut scenario leaves the arithmetic and measures the composition: a donut is the same angle
  assignment rendered into an annulus, so the `geometry/arc` records must carry the same start and
  end angles as the transform emitted, with the inner and outer radius derived from the frame.

  @id-pie
  @level-quick
  @mode-differential
  Scenario: A full sorted pie reproduces d3's angles
    Given the committed probe document local://pie.tex
    Then the compiled probe and the reference implementation agree on every arc angle

  @id-pie-unsorted
  @level-quick
  @mode-differential
  Scenario: An unsorted pie hands out its angles in table order
    Given the committed probe document local://pie-unsorted.tex
    Then the compiled probe and the reference implementation agree on every arc angle

  @id-pie-padded
  @level-quick
  @mode-differential
  Scenario: A padded partial circle applies both of d3's clamps
    Given the committed probe document local://pie-padded.tex
    Then the compiled probe and the reference implementation agree on every arc angle

  @id-donut
  @level-long
  @mode-differential
  Scenario: A donut renders those angles into an annulus of the frame
    Given the committed probe document local://donut.tex and the frame
      | width | height | pad | inner |
      | 60    | 34     | 3   | 0.55  |
    Then the compiled probe and the reference implementation agree on every annular sector
