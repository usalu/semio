@capability-staffage-volumes-rules
Feature: The volumes of an asset are judged per scale and per mode by fourteen rules with measurements
  Every preset scale of an asset has its own handcrafted volume (`volumes[scale]`: `body`, `thermal`, `structure`, `decisions`, or `unavailable` exactly where the representation is). `checkVolumes(asset, { scale?, rules? }, operation)`
  returns findings with the rule, the scale, the list and the failing measurements; `measureVolumes` returns every measurement with its limit. The rules: `volumes-availability`, `volumes-silhouette` (IoU at least 0.92 and
  Hausdorff at most two smallest features against the filled 2D shapes of the same scale in the elevation, the front and the plan), `volumes-bounds` (the bounding box within one feature of `dimensions`), `volumes-feature` (no solid
  thinner than the smallest feature of the scale, each scale judged on its own, except declared thin sheets (`sheet: true`, or a thermal layer gauge), which are judged by their two large extents), `volumes-monotonic` (solids and triangles never grow towards coarser scales and one of them drops), `volumes-cross-scale` (IoU at least 0.85, Hausdorff
  at most two features of the coarser scale, union volume within 15 %), `volumes-thermal-coverage` (every region, layer shell and emitter has a solid, outer area within 15 % at 1:50 and 30 % coarser, inside the body box grown by
  one feature), `volumes-structure-coverage` (every eligible bone, member shape and support has a solid, members inside the silhouette, supports on the ground or at their anchor), `volumes-bones`, `volumes-solids`, `volumes-copied`, `volumes-slender`, `volumes-views-disagree` and `volumes-connected` (one connected body, closed load paths; scenarios in `../🧊️volumes-connected`).
  Expensive rules report progress and stop when cancelled.

  THE REFERENCES are the hand-computed cuboids of the fixture, shapely (unions and Hausdorff distances of the projected triangles) and trimesh (volumes and outer areas by point containment), fed with the very triangles of the kernel.

  Scenario: The limits are those of the design
    Given the thresholds of the design
    Then the owned limits equal them and the smallest features are 7.5, 15, 30 and 75 mm at 1:50, 1:100, 1:200 and 1:500

  Scenario: One broken variant per rule
    Given the sample figure with one defect per rule, such as a head grown past the dimensions or a foot bone without a body solid
    When the volumes are checked
    Then the rule reports a finding at the scale of the defect, in English and German, and a measured value moves the wrong way against the baseline

  Scenario: The measurements show the distance to every limit
    Given the sample figure
    When the volumes are measured
    Then every limit of the design appears with its value, so that authors see how far off they are

  Scenario: The fine-scale body area diagnostic uses the physiological tolerance
    Given the sample figure with the foot region declared as 2 square metres
    When the thermal volume coverage is checked at 1:50
    Then the foot-area finding states its applied 30 percent limit in English and German

  Scenario: Expensive rules report progress and stop
    Given a check of the silhouette
    When it runs and when the signal is aborted
    Then progress reaches its total and the aborted run throws

  Scenario: The geometry equals independent implementations
    Given cuboids with hand-computed volumes and areas and the triangles of the sample figure
    When the owned union volume, outer area, IoU and Hausdorff distance are compared with trimesh and shapely
    Then they agree within the tolerance of the raster

  Scenario: Physical tissue remains measurable when the printed drawing merges its region
    Given a 1:50 figure with a two-millimetre thermal tissue solid attached to the head
    When its region is drawn individually the paper feature rule finds the undersized solid
    When every drawing of the region on that bone merges it with adjacent tissue the digital solid retains its real diameter

  Scenario: A breath outlet retains its declared physical face dimensions
    Given a 2 mm physiological outlet on the head and a 0.64 mm normal thickness
    When comparing the thermal aperture with its declaration
    Then both the ellipsoid and the extruded oval retain their thickness at 1:50
    And untagged, misplaced, undersized and ordinary device solids retain the print feature check

  Scenario: Repeated solids match the reference tessellation
    Given the repeated solid in the sample figure and the volume reference quality
    When the solid is instanced in the pose and tessellated directly by the kernel at that quality
    Then both have the same triangle indices and all corresponding world positions agree within one nanometre

  Scenario: Translated components and their thermal references
    Given 38 cubes of side 0.1 metres, placed 0.1 or 0.2 metres apart along one axis
    When the thermal entry refers to the body geometry
    Then body and thermal use the same posed vertices and triangle indices
    And separated copies can be oriented independently
    And touching copies use the topology of the assembled mesh
    And a section projects each full-body face once across all disconnected pieces
    And both forms preserve the 2.28 square metres of triangle surface and match explicit expansion

  Scenario: An internal structural member has a measured elliptical section
    Given a narrow loft whose section area, extreme fibre and second moment agree with the mechanical member
    Then the rendered member retains its physical gauge and conflicting section data fail the feature check

  Scenario: A tapered twig shares a bone with adjacent foliage
    Given a slender bark solid and a broad leaf strip on the same bone
    Then the leaf material does not supply the width of the twig
    And a remote bark strip does not supply its width
    And a bark stroke following the twig supplies its printed width
    And an off-centre folded cleat retains its gauge behind a broad panel of the same material

  Scenario: A typified row is compared with the row it stands for through their common envelope
    Given 13 rods of 12 mm at 75 mm pitch at the finer scale and 4 posts of 75 mm at 0.275 m pitch on the same bone at the coarser scale
    When the posts are declared `envelope` and the two scales are compared
    Then the convex hull of the posts is compared with the convex hull of the rods (IoU 0.9 / 0.912 = 0.986842, as shapely finds)
    And without the declaration the posts fail against the rods

  Scenario: The centre line of the outline of a body is the body edge
    Given a plate of 1 m x 0.6 m with a rim wire of 10 mm along its edge and a closed unfilled outline of 80 mm pen around both
    When the slender wire is drawn for the comparison
    Then it keeps its 10 mm (0.6051 m² with the plate, as shapely finds) instead of the pen (0.6464 m²), because the solids fill what the outline encloses
    And around the wire alone the outline encloses a void and the wire is drawn at the pen (0.0864 m²)

  Scenario Outline: Two views must place the asset alike, not only give it the same extent
    Given the sample at 1:100 whose plan is one rectangle of the length of the side elevation, moved ahead by <shift>
    When the views are compared
    Then the extents agree and the place differs by <shift> against a limit of 15 mm, as the bounds of shapely say, and it is <verdict>

    Examples:
      | shift | verdict   |
      | 10 mm | accepted  |
      | 50 mm | a finding |
