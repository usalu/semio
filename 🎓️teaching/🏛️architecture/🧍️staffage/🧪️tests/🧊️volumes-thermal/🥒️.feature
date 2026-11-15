@capability-staffage-volumes-thermal
Feature: The thermal area of a region is the outer area of the union of its own solids
  `volumes-thermal-coverage` compares `areaM2` of a region with the outer surface of the union of the region's own outermost-layer solids (C29): faces between solids of the same region and faces declared `sectionFaces`
  (also on a `{ of }` thermal reference) are left out, faces covered by other regions, a contact plane or a support stay in, a hidden region counts its full mesh area, and two solids of one region that meet within a fifth
  of the smallest feature, as two tessellations of one curved surface do, are touching. A region smaller than six squared features is left out of the total of the coarse scales on both sides (C21).

  THE REFERENCES are the hand-computed areas of a ring around a disc and trimesh surface samples (`🐍️.py`) fed with the very triangles of the kernel.

  Scenario: A disc in a ring of one region leaves no seam whatever the tessellation
    Given a ring and a disc of one region tessellated with 64, 48 and 32 segments
    When the outer area is measured
    Then it equals the hand value within 1.5 % and a real 5 mm gap keeps both faces

  Scenario: The sample figure agrees with trimesh region by region
    Given the sample figure at 1:50
    When the outer areas of its regions are measured, once with a hidden region and once with `sectionFaces` on a `{ of }` reference
    Then they equal trimesh, the body solid's own `sectionFaces` is not inherited, and a tiny region is out of the 1:500 total
