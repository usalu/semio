@capability-staffage-volumes-rules
Feature: The volumes of an asset are judged per scale and per mode by ten rules with measurements
  Every preset scale of an asset has its own handcrafted volume (`volumes[scale]`: `body`, `thermal`, `structure`, `decisions`, or `unavailable` exactly where the representation is). `checkVolumes(asset, { scale?, rules? }, operation)`
  returns findings with the rule, the scale, the list and the failing measurements; `measureVolumes` returns every measurement with its limit. The ten rules: `volumes-availability`, `volumes-silhouette` (IoU at least 0.92 and
  Hausdorff at most two smallest features against the filled 2D shapes of the same scale in the elevation, the front and the plan), `volumes-bounds` (the bounding box within one feature of `dimensions`), `volumes-feature` (no solid
  thinner than the smallest feature of the scale, except thin sheets), `volumes-monotonic` (solids and triangles never grow towards coarser scales and one of them drops), `volumes-cross-scale` (IoU at least 0.85, Hausdorff
  at most two features of the coarser scale, union volume within 15 %), `volumes-thermal-coverage` (every region, layer shell and emitter has a solid, outer area within 15 % at 1:50 and 30 % coarser, inside the body box grown by
  one feature), `volumes-structure-coverage` (every eligible bone, member shape and support has a solid, members inside the silhouette, supports on the ground or at their anchor), `volumes-bones` and `volumes-solids`.
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
