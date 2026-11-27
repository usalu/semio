@capability-staffage-model-silhouette
Feature: The volume agrees with the authored silhouettes
  In its presentation pose the volume projected into the elevation, front and plan covers the authored closed shapes (open strokes stroked with their pen width) with an IoU of at least
  0.80 and fills `dimensions` within 8 % on every axis (the authoring gate of volume-drafting). The sample figure and three catalog assets (a table, a bed, a beam) are checked here; the whole catalog is measured
  by the ticket report.

  THE REFERENCES are polygon-clipping (exact union and intersection areas) and the dimensions of the asset; an owned scanline raster must agree with the exact IoU.

  Scenario: The extent fills the dimensions
    Given an asset in its presentation pose
    When the solids are tessellated and carried to the world
    Then width, height and depth are within 8 % of `dimensions`

  Scenario: Each view agrees
    Given the authored shapes and the projected solids of a view
    When their exact IoU is computed
    Then it is at least 0.80 and the owned raster IoU is within 0.03 of it

  Scenario: A gathered parasol carries its cloth and both heat-exchange faces through the sewn radial pleats
    Given eight fabric gores with three pleats per gore
    When the open and furled poses are measured at each digital scale
    Then each gore and its two thermal skins have the bounds of their physical fold transforms
    And the canopy spans more than 2.4 metres open and less than 0.65 metres furled

  Scenario: A drawn extrusion names a missing joint
    Given a canopy drawing bound to a joint absent from the rig
    When the asset is validated
    Then validation reports the invalid weight on its drawing

  Scenario: An inextensible fan joins two moving rays
    Given a four-facet fan with a 48 degree flat cut on a tilted crown
    When the two rays meet or open by 1, 8, 16, 32 or 48 degrees
    Then each 12 degree facet retains its area and meets the next facet at its hem
    And the outer two seams follow their driving rays

  Scenario: A folded sheet is drawn edge-on in a second view
    Given a 600 by 340 millimetre panel of 2 millimetres thickness
    When its elevation, front and plan are projected for the drawing
    Then the smallest drawn width of its bone is 2 millimetres

  Scenario: A long mesh review releases its oldest cached geometry
    Given two distinct tessellations and more than 40 MiB of additional geometry
    When one original mesh is read repeatedly during the review
    Then the hot mesh is reused and the cold mesh is regenerated with identical positions and indices

  Scenario: A cloth profile blends two joints with different inverse bind matrices
    Given a fabric facet with a quarter of its weight on one rib and three quarters on the next
    When the frame folds
    Then the three-dimensional inspector and elevation drawing agree with the independently transformed vertices

  Scenario: A goat horn is judged at its own scale
    Given the authored goat horns at 1:500 and the thinnest drawing of their bone
    When no horn section is thinner than the limit of that bone
    Then the feature rule finds nothing, whatever the horn is translated or named
    And a horn thinned below the limit is found once with its measured thickness

  Scenario: Scale preserves the working dimensions of household components
    Given the catalog fixture of timber stock, turned dowel, valve knob, bin floor, cover glass and steel wand dimensions
    When each body mesh is tessellated at every digital scale
    Then its bounding interval along the specified local axis has the independently specified dimension
