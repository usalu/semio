@capability-staffage-volumes-flood
Feature: The outer area of a thermal region is the surface the air from outside reaches
  `volumes-thermal-coverage` counts a face of a region's solids as outer surface (C29, C31) when the air flooded from outside through the thermal solids of the scale reaches it, or when a solid of another region
  that the air reaches covers it. The faces of a cavity that the solids close off (the bore of a capped tube, the inside of a casing of panels, a hollow knob) do not count; an open tube end, a cup or a slot of at least
  half a smallest feature lets the air in, a gap of at most a quarter feature does not. A solid enclosed in a cavity covers nothing. The flood reports its cell (`info:floodCellM`), which is a quarter feature unless the
  asset is so large that the budget of the grid raises it, and it stops when the operation is cancelled.

  THE REFERENCES are the hand-computed areas of the fixture (tubes, caps and casings) and an independent flood: trimesh voxelisation with the connected components of scipy (`🐍️.py`), fed with the very triangles of the kernel.

  Scenario: Closed cavities do not count, open ones do
    Given a tube capped at both ends, at one end and at none, a casing of six panels, the same casing with a slot of 20 mm and with a slit of 1 mm
    When the outer area of their region is measured
    Then it equals the hand value: the bore and the inside of the casing count only where the air gets in

  Scenario: Another region covers but does not hide
    Given a tube closed by caps of another region, a block on a slab of another region and a casing around a core of another region
    When the outer area of each region is measured
    Then faces covered by a solid the air reaches count, faces that look at an enclosed solid or into the closed cavity do not

  Scenario: The flood equals an independent implementation
    Given the triangles of every case
    When trimesh and scipy flood a voxel grid of the same cell
    Then the areas agree within the tolerance of the fixture

  Scenario: The rule measures a capped hollow arm like a solid one
    Given the sample figure at 1:50 with its left arm as a tube closed by two discs
    When the thermal coverage is measured
    Then the outer area of the arms equals that with a solid rod, grows by the bore when one disc is removed, the cell is reported and an aborted run throws

  Scenario: A narrow lid opening at a coarse print scale
    Given a box with 6 mm walls and a 14 mm open slit in its lid
    Then the inside of its floor exchanges heat with outside air at 1:500
    And shutting the slit encloses the same floor
