Feature: Glyphs never dwarf the body in the 3D model (amendment D1)
  Scenario: Sizes in model space
    Given an asset with an authored volume at several scales
    When its structure and thermal scenes are built at 1:50, 1:100, 1:200 and 1:500
    Then no arrow is longer than 0.35 of the largest extent of the asset, the longest arrow is the smaller of its paper length times the denominator and that cap, the arc radius is at most half the cap and the centre-of-mass sphere has a radius of at most 0.06 of the extent
  Scenario: Stream tubes of the air flow
    Given the thermal scenes of an asset with flowing air at 1:50, 1:100, 1:200 and 1:500
    Then every stream tube is round, its radius is half the paper width of the stream at the scale (0.09 to 0.3 mm of paper) and at most 0.012 of the largest extent of the asset, and every arrow head is 1.1 by 0.75 mm of paper shrunk by the same cap
  Scenario: Paper stays paper
    Given the sizing of a drawing
    Then every glyph is its paper size at the scale and nothing is capped, so the axonometric drawing and the 2D sheets are unchanged
