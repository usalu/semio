@capability-staffage-loft-shape
Feature: A loft is one body in the side elevation, the front elevation and the plan at every pose
  A part whose cross-section changes freely along its bone (an arm, a hand, a shoe, a coat, a tapering soundbox) is neither a prism nor a body of
  revolution. The `loft` shape states superelliptic sections in the frame of its bone, exactly as the `loft` solid of the volumes does; `evaluate` carries
  the sections with the full 3D frame of the bone and draws the body between every two consecutive sections in each view. Like the `member` it has no
  planar geometry and no `solid`.

  THE REFERENCES are gl-matrix (it turns the section axes by the section quaternion and carries them through the hand-written basis of the bone), the
  kernel's own tessellation of the same sections as a volume loft, shapely with numpy and scipy (the union of the convex hulls of consecutive section rings
  in each view) and ajv for the schema; the numbers of the vectors are hand-computed.

  Scenario: The hand-computed vectors give the extents of the loft in the three views
    Given a loft of three elliptic sections with the limb rotation and a loft of two sections without rotation on the left thigh of the sample figure
    When the rest pose and the stride pose are evaluated in the side elevation, the front elevation and the plan
    Then every extent equals the hand-computed number relative to the pelvis origin

  Scenario: The section frames are those of the volume loft
    Given the same sections as a loft solid of the volumes
    When the kernel tessellates it
    Then the section rings of the shape reach as far as the kernel's body in every direction

  Scenario: A third-party geometry library builds the same outlines
    Given the same sections placed by numpy and scipy with the hand-written basis of the bone
    When shapely unites the convex hulls of consecutive section rings in each view
    Then its bounds and its area equal those of the evaluation

  Scenario: The three views are one body
    Given the loft in every pose of the sample figure
    When the three views are evaluated
    Then the height range of the side and front elevations, the forward range of the side elevation and the plan and the lateral range of the front elevation and the plan are equal

  Scenario: The paint depth of a loft is its nearest point
    Given the loft at rest and in the stride
    When the depth that ranks the shape in the front and in the plan is read
    Then it is the foremost and the highest point of the section rings

  Scenario: The drawn width of a loft is the width of its body
    Given a loft whose last section tapers to a fraction of a millimetre
    When the thinnest drawing of its bone is read for the smallest feature of the volumes
    Then it is the smallest dimension of the same sections as a volume solid but for the slope of its outline, so a tapered tip never lowers the smallest feature of the bone

  Scenario: A loft that cannot be a body is refused
    Given a loft with a single section, coincident sections, a section without a positive half extent, a rotation that is no unit quaternion, a path that turns back, a solid hint or a weight on an unknown bone
    When it is validated and checked against the schema
    Then the validator reports loft-sections, loft-section, loft-fold, loft-solid or the weight finding at the shape, and ajv rejects what the schema can express

  Scenario: A loft has no paper-unit drawing
    Given a symbol in paper millimetres with a loft
    When it is evaluated or validated
    Then evaluate throws LoftInPaperUnitsError in both languages and the validator reports loft-in-paper-symbol
