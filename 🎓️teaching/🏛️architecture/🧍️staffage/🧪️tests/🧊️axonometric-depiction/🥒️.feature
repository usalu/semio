Feature: The axonometric view depicts the 3D volume in the seven representations

  @capability-axonometric-depict
  Scenario Outline: A normal DrawingScene at every preset scale
    Given the sample figure in the stand pose
    When <view> is depicted at 1:50, 1:100, 1:200 and 1:500 with viewpoint axonometric
    Then the scene has finite geometry on the sheet, unique item ids, the isometric camera and its axis scales in the metadata
    And the result is deterministic and equal through depict and depictAsync

    Examples:
      | view        |
      | dark-line   |
      | dark-solid  |
      | light-line  |
      | light-solid |
      | thermal     |
      | forces      |
      | moments     |

  @capability-axonometric-colours
  Scenario: The colours are those of the planar views and of the 3D model
    Given the same evaluated state drawn in elevation, axonometric with flat shading, and as a neutral model
    When fills, opacities, hatch patterns, values and legends are compared per solid
    Then they are equal, and the banded solid views use the four documented bands with the brightest band exact

  @capability-axonometric-pens
  Scenario: Pens, level of detail and scale
    Given the authored pens of each preset scale and the solids that have a maxDenominator
    When lines and fills are drawn
    Then every line has the pen of its solid's role times 1, 1/2 or none by view and scale
    And a solid shown only at 1:50 is absent at 1:100, the overall dimension is the true height and the ground outline is to scale along the axes

  @capability-axonometric-pose
  Scenario: The volume is posed by the evaluated state
    Given the stand pose and a stride
    When the posed vertices are compared with the kernel's pose of the neutral model
    Then every vertex of every bone is found, and a clip instant differs from the pose

  @capability-axonometric-lines
  Scenario: Interpenetration and creases
    Given two crossing bars and a cube
    When joins and edges are found
    Then the join curves have the hand-computed length and lie on both surfaces, a face-on cube has four silhouette edges and a turned cube one crease

  @capability-axonometric-progress
  Scenario: Progress and cancellation
    Given an expensive depiction
    When it runs with a progress callback and an abort signal, synchronously and asynchronously
    Then progress is monotonic from 0 to 1000 permille with bilingual labels, an aborted signal throws its reason, and the event loop runs between steps

  @capability-axonometric-sheet
  Scenario Outline: The fourth cell of the multiview sheet
    Given a <projection>-angle sheet with the axonometric view
    When the cells are placed
    Then the axonometric cell lies in the free quadrant at the same scale and the SVG and CAD sheet state it

    Examples:
      | projection |
      | first      |
      | third      |

  @capability-axonometric-cli
  Scenario: Command line, cast manifest and Python consumer
    Given --viewpoint axonometric with --camera and --views, and a cast of the axonometric viewpoint
    When the arguments and the manifest are validated
    Then they are accepted with their file names, nonsense is refused in both languages, and the Python consumer offers the viewpoint
