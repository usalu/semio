@capability-staffage-lateral-rest
Feature: A bone rests in any direction and its children hang from it
  A clavicle, a pelvis half, a scapula, an axle or a handlebar runs sideways at rest, and the limb or wheel that hangs from it has its own joint axis.
  The rest placement of a bone therefore orients it fully in its parent: `angleDeg` about the joint axis w of the parent, then `abductionDeg` about its own
  normal v (positive turns the bone axis u towards w), then `twistDeg` about its own axis u (positive turns v to -w). A pose or clip offset then turns the
  bone about its own rest axes in the same order, and children are placed in the frame of their parent: R(bone) = R(parent) · R(rest) · R(offset).
  The side elevation, the front elevation and the plan are orthographic projections of that one chain; shapes, solids, anchors, the 3D node tree and the
  validator all read it. A rig whose bones all rest in the sagittal plane is solved exactly as before.

  THE REFERENCE is scipy `Rotation` (intrinsic Z·Y·X Euler angles) for the chain and the node placements, numpy for the anchors and shapely for the
  projected extents of the solids; the frames of the named cases are also computed by hand in the fixture.

  Scenario: The clavicles run laterally and the arms hang from their tips
    Given the sample figure with a clavicle on each side (abduction ±90, twist ±90 degrees) and the arms re-parented to them (angle -90, twist ±90 degrees)
    When the rig is solved at rest
    Then each clavicle reaches from 0.02 m to 0.19 m beside the symmetry plane at shoulder height and each arm hangs 0.6 m down from its tip with its joint axis to the left

  Scenario: The hanging arm equals the sagittal arm beside the symmetry plane
    Given the same figure with the arms on the torso 0.19 m beside the symmetry plane
    When both figures are evaluated in every arm pose
    Then the arm frames and the arm shapes of all three views are equal

  Scenario: An offset turns a bone about its own rest axes
    Given the figure with the shoulder girdle
    When an arm is flexed or spread, the clavicles are lifted or one is retracted
    Then the frames, the shapes and the anchors are at the hand-computed places and the arms ride on the clavicles

  Scenario: A third-party library recomputes the chain
    Given every case of the vectors, including offsets about all three axes and a scapula with a general rest placement
    When scipy solves the chain, numpy places the anchors and shapely measures the solids
    Then the frames, the rest and posed node placements, the anchors and the extents equal the product

  Scenario: The contract and the validator know the rest placement
    Given a bone with `abductionDeg` and `twistDeg`
    When the asset is validated against the schema and by the validator
    Then it conforms, values outside ±180 degrees are refused, and a flat elevation shape on a bone that is edge-on at rest is reported while a solid is accepted

  Scenario: Every consumer of the bone frames carries the lateral bone
    Given the figure with the shoulder girdle, a solid clavicle in the drawings, a capsule on the clavicle in the volume lists and a clip that lifts both shoulders by 30 degrees in half a second
    When the lifted clavicle is drawn in the three views, written to SVG, PDF, DXF and DWG, laid out on the multiview sheet, sampled frame by frame, exported as a cast at the full and the preview level, posed as a 3D model, exported as GLB and FBX, and painted in the thermal, force and moment views
    Then each output shows the clavicle in the hand-computed box of its view (svgelements, jsdom, PDFium, ezdxf, ezdwg and three read the files back)
    And the clip passes the hand-computed tips at a quarter and at half a second

  Scenario: The structure is solved through the lateral member
    Given the arm of 3.75 kg hanging from the clavicle tip 0.17 m beside the sternoclavicular joint
    When the structure is solved at rest
    Then the clavicle carries 36.77 N of shear and 6.25 N·m of bending at the joint, falling linearly to zero at the tip, and numpy finds the same joint loads in every case
    And the bending diagram is drawn in the front elevation, has no ordinate in the side elevation and is an out-of-plane mark in the plan

  Scenario: A sagittal rig is untouched
    Given the sample figure without the shoulder girdle
    When it is solved at rest
    Then no three-dimensional chain is needed and the affines equal the plain planar composition
