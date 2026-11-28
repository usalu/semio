@capability-staffage-model-roundness
Feature: Round solids are round in the viewer and in every 3D export
  The 3D view, the GLB, the FBX and the resolved model JSON tessellate every scene at `qualityOf(asset, scale)`. That quality is adaptive to curvature and size: the chord deviation is at most 2 % of the smallest
  feature of the scale (0.15 mm of paper: 7.5, 15, 30 and 75 mm at 1:50, 1:100, 1:200 and 1:500) and at most 1/3200 of the largest dimension of the asset at 1:50 (a quarter of a pixel when the asset fills 800 pixels) growing with the denominator from there like the feature itself,
  a full circle whose radius exceeds one feature of the scale never has fewer than 24 segments, no circle has fewer than 12, and the segments of one circle are capped per scale (96, 64, 48, 40 and 32 from 1:20 to
  1:500) so that the triangle budgets hold and a coarser scale never has more triangles than a finer one, also for solids that the scale itself sizes (the mass and joint markers of one feature in diameter). Vertex normals are smooth across the segments of a round surface and hard only at real creases.

  THE REFERENCE is trimesh with numpy (`🐍️.py`), fed with the very triangles and normals of the kernel: it recomputes the segments from the rule, counts the meridians of every mesh, measures the largest gap between
  the mesh and the analytic circle or sphere, and the angle between every vertex normal and the analytic normal.

  Scenario: The quality of a scale follows the rule
    Given the rule of the fixture and assets from a tealight to a crane
    When the quality of every scale is asked for
    Then its chord, segment bounds and round floor are those of the rule, and the chord never falls towards a coarser scale

  Scenario: Cylinders, spheres, capsules and round lofts are round
    Given solids of radii from 1 mm to 1.2 m about one axis
    When each is tessellated at the quality of every asset and scale
    Then trimesh counts the segments of the rule, at least 24 where the radius exceeds one feature, a gap to the analytic surface within the chord unless the cap of the scale binds, smooth normals on the round surface and hard normals only at the rims

  Scenario: A coarser scale never has more triangles
    Given the same solids
    Then the triangles of every solid do not grow from 1:20 to 1:500

  Scenario: A marker sized by the scale never has more triangles at a coarser scale
    Given spheres, cylinders, capsules and round lofts of one feature in diameter in assets from 1 cm to 10 m
    When each is tessellated at the quality of 1:50, 1:100, 1:200 and 1:500
    Then its triangles do not grow, the chord does not fall, and the triangles of a sphere are those of three's SphereGeometry for the segments of the rule

  Scenario: The catalog shows round candles, valves, heads, limbs and wheels
    Given the tealight, the thermostatic radiator valve, the standing adult and the city car of the catalog
    When every scene of their multi-scene model is tessellated as the viewer and the exports do
    Then every full revolve, ellipsoid and capsule whose radius exceeds one feature of its scale has at least 24 meridians as trimesh counts them
