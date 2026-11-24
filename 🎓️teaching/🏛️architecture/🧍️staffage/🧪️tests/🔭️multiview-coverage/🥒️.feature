@capability-staffage-multiview-coverage
Feature: Every staffage is covered in the side elevation, the front elevation and the plan for every scale, mode, pose, clip instant and transition
  The coverage matrix enumerates asset × scale (the four presets, the ends of every valid range, every declared key, every alternative) × viewpoint (elevation, front, plan) × mode
  (dark line, dark solid, light line, light solid, thermal, forces, moments). Every cell is ok, declared unavailable with a complete reason, or fail. A cell that no runner produced is a failure.
  The frame is left-handed: x ahead, y up, z left. The side elevation is seen from the asset's right, the front from ahead (paper x = z), the plan from above (paper up = z).
  All views of one instant come from one evaluated state. Scale maps metres to paper millimetres only: mm = m · 1000 / denominator.

  THE REFERENCES are third-party: svgelements recomputes the bounds of exported SVG paths and of hatched areas, scipy the Hausdorff distance of derived-view detection, shapely the gap of the printability vector, the distance of an anchor to drawn shapes and the extents of bone shapes.

  Scenario: The universe of cells follows the declarations
    Given an asset with presets, valid ranges and alternatives
    When the universe is enumerated
    Then it holds every scale entry times three viewpoints times seven modes

  Scenario: A cell nobody produced fails
    Given an empty set of produced cells
    When the matrix is assembled
    Then every cell is a failure named cell-missing

  Scenario: Clips and transitions are sampled with their extremes
    Given a clip with keyframes and a long clip
    When the instants are chosen
    Then they contain start, keyframes, mid-transitions, extremes, at least eight uniform instants (sixteen for clips longer than 600 s) and the end
    And every pose goes to the rest pose plus three seeded random pairs at 0, 0.25, 0.5, 0.75 and 1

  Scenario: Missing and half-available scales are named failures
    Given a figure with an undecided scale, an empty front view and a declared unavailable scale
    When the matrix is built
    Then the undecided scale is representation-missing, the empty front is unavailable-view-missing for the front only

  Scenario: Known dimensions reach the paper in all three views
    Given a rectangle 0.25 m by 1.80 m in every view
    When it is drawn at 1:50 and 1:200
    Then it measures 5 by 36 mm and 1.25 by 9 mm in all three views and sits at the handed positions about the origin

  Scenario: A held object sits on the side of its hand
    Given the left and right hand anchors of catalog assets
    When the front and plan are drawn
    Then a right-hand object is left of the origin in the front and below it in the plan, a left-hand object the opposite

  Scenario: Contact and root motion are one state
    Given a planting walk cycle
    When its end and loop cycles are evaluated
    Then planted anchors do not move, the root advances by the authored distance, timeS is absolute and clipTimeS the phase

  Scenario: Thermal emitters move with the body and the transport persists
    Given a walking figure with respiratory emitters
    When the transport is solved at several times and scrubbed
    Then the emitters follow the root and the same time gives the same field

  Scenario: Magnitudes do not depend on the view
    Given thermal, forces and moments of one pose
    When the three viewpoints are drawn
    Then every region and member carries the same peak value and the legends are identical

  Scenario: Out-of-plane force and moment are carried
    Given a load with fz and a moment about x
    When the state is evaluated and the force view drawn
    Then the world force carries them and the front and plan differ from the unloaded drawing

  Scenario: Coarse scales are measured against the print profile
    Given two bars with an ink gap of 0.10 mm at 1:200
    When the profile requires 0.15 mm
    Then the representation is unsuitable with the measured gap, and authored at 1:50

  Scenario: Switching views and scrubbing are stateless
    Given a clip drawn at many times in many orders
    When the views are switched repeatedly
    Then every scene is identical to its first rendering

  Scenario: Exports are deterministic
    Given two independent runs of one instant
    When SVG, PDF and DXF of the three viewpoints are written
    Then the bytes are identical and the viewpoints differ

  Scenario: A derived view is detected
    Given a front that is a copy, mirror, 90 degree rotation or scaled copy of the elevation
    When derived-view detection runs
    Then it is flagged unless it is allowlisted with a reason, and handcrafted outlines are not

  Scenario: The paper box reads fills and outline runs
    Given a view whose outermost part is drawn unfilled
    When the paper box of the view is measured
    Then it spans the fills and the outline runs of the shapes and equals the dimensions on the paper
    And an independent SVG parser finds the same box

  Scenario: An anchor may lie on a descendant bone
    Given a contact bound to the frame that carries a wheel drawn on its own bone below it
    When the anchor landmarks are tested
    Then the anchor passes on the shapes of its bone and of its descendant bones
    And an anchor on an ancestor, on another branch or in the air is a finding with its distance

  Scenario: The hard per-bone tier holds for a shape and its twin
    Given a bone whose single shapes in two views carry the same id, and one whose single shapes carry different ids
    When their extents along the shared axis disagree
    Then the twin is a sync-bone failure and the two different parts are reported as group-drift

  Scenario: A strong member diagram is never colour-only
    Given the bending-moment, shear and axial diagrams of a loaded figure
    When they are drawn with patterns on
    Then the area and the line-of-sight ring of a member above 15 % of the ordinate maximum are hatched by sign, diagonal for negative and dots for positive
    And an unhatched thermal, forces or moments scene passes only when its legend states the unhatched share in English and German

  Scenario: The generator survives a long run
    Given a run folder with one record per finished asset
    When the run is cut and started again
    Then the finished assets are skipped, an asset whose document changed is run again and shards of separate invocations merge
    And an asset that crashed or timed out fails its cells by name with the message, never as cell-missing

  Scenario: The authors' gate checks one asset in seconds
    Given one asset and the check functions of the matrix
    When it is scanned
    Then every finding names its code, pose, scale, views and, where it has one, bone, anchor and delta
    And a row counts the findings per family, synchrony at rest and in other poses, and anchors by the place they lie

  Scenario: A truthful band copy is allowlisted with its reason
    Given a view that is the same drawing in two bands because the part is prismatic or plain
    When the allowlist names the asset, the view and those bands with a physical reason in English and German
    Then a band copy between two of the named bands is no finding, every other band and view still is, and an entry without both texts allows nothing
