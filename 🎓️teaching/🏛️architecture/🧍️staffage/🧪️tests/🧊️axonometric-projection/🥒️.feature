Feature: The axonometric camera is the fourth camera of the depiction
  The side elevation, the front elevation and the plan have one camera each; the axonometric view is the same
  family seen from an azimuth and an elevation, scaled so that the vertical axis is true to scale.

  @capability-axonometric-camera
  Scenario Outline: The camera equals the hand-computed vectors
    Given the camera <name> of the vectors in 🧫️fixtures/🧊️axonometric-projection
    When its axes, normalisation, axis scales and projected points are computed
    Then they equal the hand-computed numbers to 1e-12

    Examples:
      | name            |
      | isometric       |
      | dimetric        |
      | free-30-20      |
      | side-elevation  |
      | front-elevation |

  @capability-axonometric-scale
  Scenario: The axes that are to scale are the ones the camera states
    Given the isometric camera and the ISO 5456-3 dimetric camera
    When one metre is projected along the ahead, up and left axes
    Then the isometric picture shows 1 : 1 : 1 and the dimetric picture 1 : 1 : 1/2, the vertical axis always 1
    And a diagonal is not to scale

  @capability-axonometric-oracle
  Scenario: An independent implementation projects the same points
    Given three's OrthographicCamera looking at the origin from the eye of each camera in the neutral frame
    When forty scattered points are carried through its inverse world matrix
    Then x, y and the depth equal the axonometric projection after the normalisation, to 1e-9

  @capability-axonometric-one-source
  Scenario: The planar cameras and the kernel viewer share the projection
    Given the side and front elevations and the viewer's axonometric preset
    When the same points are projected through each
    Then the planar pictures equal the axonometric camera at azimuth 90 and 0 degrees and elevation 0, and the viewer's isometric view equals the isometric camera
