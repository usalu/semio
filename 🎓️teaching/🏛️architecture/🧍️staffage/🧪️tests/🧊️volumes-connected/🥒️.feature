@capability-staffage-volumes-connected
Feature: The solids of a volume touch, and each scale is judged for its own smallest feature
  `volumes-connected` measures with the very triangles of the kernel whether the body of a scale is one object in the default pose and whether every load path of the structure reaches a support. Parts are the copies of the solids;
  two touch when their surfaces come within a fifth of the smallest feature of the scale or one lies inside the other. A group of body parts apart from the largest must rest on the ground or at a contact or support anchor, and
  keep more than one feature of clearance and not be tied by a movable joint of the rig to a group it does not touch; a group of structure parts that is not only mass markers must hold a support solid or rest likewise (in a list without support solids the largest group is the structure). No flag, role or tag switches a part off. `volumes-feature` judges every scale on its own (a solid
  identical to a 1:50 solid is no exception at a coarser scale) and lets a thin sheet pass only when it is declared (`sheet: true`, or a thermal layer gauge) and broad enough for its thickness.

  A `member` solid of the structure list may stand for several connected bones that it names in `members` when each of them lies in it or within one smallest feature of it in the default pose; a `mass` marker is never the nearest solid of a structure finding.

  THE REFERENCES are the hand-derived distances, groups and gaps of the fixture, and trimesh (closest points, containment) with scipy (connected components of the contact graph), fed with the same triangles.

  Scenario: The distance between two triangles is exact
    Given pairs of triangles with a hand-derived distance, face to face, vertex to face, edge to edge, vertex to vertex, crossing and sharing an edge
    When the owned distance is measured
    Then it equals the hand value to a nanometre

  Scenario: Solids are grouped by true contact and not by their boxes
    Given cuboids face to face, inside and outside the tolerance, corner to edge, a cube in the empty corner of the box of a diagonal bar, crossing bars, a cube inside a cube and a repeated post with one hovering copy
    When the parts are grouped with the contact tolerance of 1:50
    Then the groups, the failing groups, their reasons, gaps and nearest solids equal the hand values

  Scenario: A load path ends in a support
    Given a portal with a mass marker, a column that stops short of the beam, a bracket that reaches nothing, a frame that ends in the air over its support, a frame standing on the ground and a list without support solids
    When the structure parts are judged
    Then the mass marker may hang in the air, the short column is near, the bracket and the frame over its support have an open load path, the standing frame has none, and in the list without support solids the largest group is the structure that a stray member must join

  Scenario: The contact graph equals an independent implementation
    Given the triangles of every case
    When trimesh measures the closest distances and containment and scipy labels the connected components
    Then the owned groups are the same partition and the owned gaps agree

  Scenario: The sample figure is connected and its defects are found
    Given the sample figure
    Then `volumes-connected` reports nothing at any scale
    When its head is lifted off the neck, a shoe is set beside the foot within one feature or a member is pulled off its joint
    Then the rule names the solid, the gap and the nearest solid in English and German at that scale

  Scenario: A thin sheet is judged by its two large extents
    Given a feather vane, a narrow strip, a lump, boards and discs with and without a declared gauge, two rib hoops of equal section and a washer
    Then only the solids whose middle extent is at least the feature and twenty times their thickness, at their declared gauge, are sheets
    And a ring is as broad as its band: neither hoop is a sheet whatever its diameter, the washer is

  Scenario: A thin sheet passes only when it is declared
    Given a broad thin plate, a lump, a strip, a ribbon, a board and a block added to the sample figure, with and without `sheet: true`, and a cotton plate bound to the garment layer
    Then the undeclared plate fails `volumes-feature` with the hint to declare `sheet: true`, at 1:50 and at 1:500
    And the declared plate passes at 1:50 and with the same thickness at 1:500
    And a plate at the gauge of its thermal layer passes without the flag and fails off that gauge
    And a declared lump, strip, ribbon or block fails for declaring a sheet that is none
    And a solid that declares `sheet` and `slender` fails
    And the feature rule lets the vane pass at 1:50 and still reports the lump

  Scenario: A coarse scale gains nothing from carrying a 1:50 solid
    Given a 20 mm cube authored identically at 1:50 and at 1:500
    Then the feature rule passes it at 1:50 and reports it at 1:500, where the smallest feature is 75 mm

  Scenario: The rule is deterministic and can be cancelled
    Given the sample figure
    Then two runs give identical measurements and an aborted signal stops the run

  Scenario: A hollow vessel is judged by its overall extents
    Given a compressor shell with a 3.5 mm wall as a revolve of its closed wall loop, a casing panel of 3.5 mm and a pellet of 3.5 mm at 1:50
    Then the shell and the panel pass the feature rule and the pellet is reported

  Scenario: The rule raster scales with the object
    Given a part of 60 mm at 1:50 and two rectangles that differ by 3 mm in length
    Then the comparison raster has at least 256 cells along the long side and never a cell below 0.05 mm
    And the IoU of the rectangles equals the hand value on that raster and misses it on a quarter-feature raster

  Scenario: A mass marker is no load path
    Given a bracket that reaches nothing with a mass marker nearer to it than the column, and a post on the ground with a mass marker over it
    When the structure parts are judged
    Then the gap of the bracket is measured to the column and the column is named, never the mass marker
    And the post beside the mass marker is no finding
    And a supported group that passes within one feature of the rest is worded as a load path of its own beside the structure, in English and German

  Scenario: One member solid spans several bones
    Given a rod 20 x 300 x 20 mm and bones inside it, beyond its end, beside it, at a right angle to it and over its edge
    When the distance of each bone from the rod is measured at nine points from joint to tip
    Then it equals the hand value and the value of trimesh
    Given the sample figure with one slender rod for pelvis, torso and head that names torso and head in `members`
    Then every one of the three bones has its member solid and neither `volumes-feature` nor `volumes-slender` reports the rod at its true section
    And the same rod without `members` leaves torso and head without a member solid
    And the torso member takes the pelvis in at 1:500, where the pelvis lies within one feature of it, and not at 1:50
    And a member that names a bone it does not reach, a bone that is not joined to it in the rig, its own bone, an unknown bone, or a mass marker that names bones is reported under `volumes-structure-coverage` in English and German

  Scenario: A long member keeps its true section at every scale
    Given a femur of 390 mm with its true shaft and a finger bone of 80 mm at 1:500
    Then the femur declared slender as a structure member passes the feature rule and the slender rule
    And the femur without the declaration is reported by the feature rule
    And the finger bone and a femur in the body list are not slender
    And two rib hoops of equal section and different diameter get the same verdict: slender by their centre line when declared, reported by the feature rule when not, and never offered `sheet: true`
