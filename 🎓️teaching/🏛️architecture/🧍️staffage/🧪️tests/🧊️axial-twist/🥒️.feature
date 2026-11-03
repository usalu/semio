@capability-staffage-axial-twist
Feature: A bone twists about its own axis in the side, front and top view
  A pose or clip key may give a revolute bone `twistDeg`: the bone and its subtree turn about the bone's own longitudinal axis, accumulating down the chain. For a bone
  pointing up this is a yaw: a crane slews, an office chair swivels, a bicycle steers. Positive twist turns the local +y axis of the bone towards the right of the asset,
  so a bone pointing up turns counter-clockwise seen from above. The side view (elevation), the front view and the top view (plan) are the orthographic projections of
  the same three-dimensional chain, so a jib that slews 60 degrees is half as long in the side view and reaches 0.87 of its length sideways in front and plan.

  THE REFERENCE is numpy rotation matrices for the three-dimensional chain and shapely for the extents.

  Scenario: A twisted torso turns the forward arm
    Given the sample figure with the torso twisted 45, 90 and 180 degrees and the arm raised forward
    When the three views are evaluated
    Then the arm reaches 0.424264 m ahead and left, vanishes in the side view and reaches 0.6 m left, and points to the back

  Scenario: Twists accumulate down the chain
    Given a torso twisted 30 degrees and a head twisted 60 degrees
    When the head frame is solved
    Then it equals the head of a torso twisted 90 degrees

  Scenario: Clips interpolate the twist
    Given a transition from standing to the torso twisted 90 degrees
    When it is evaluated half way
    Then the torso is twisted 45 degrees

  Scenario: A shank twist turns the foot sideways
    Given the sample figure with the shank twisted by -90 degrees
    When the foot is evaluated
    Then it grows outwards by 0.22 m in front and becomes a lateral sliver in plan

  Scenario: The twist is bounded to the joints that can take it
    Given a pose that twists a bone
    When the asset is validated
    Then a revolute joint accepts up to 180 degrees unless it declares twistMinDeg and twistMaxDeg, and fixed and prismatic joints and larger angles are refused

  Scenario: A joint may declare whole turns
    Given a revolute joint with twistMinDeg -360 and twistMaxDeg 360 and a torso twisted 270 and 360 degrees with the arm raised forward
    When the frames are solved
    Then the arm reaches 0.6 m to the right at 270 degrees and 0.6 m ahead at 360 degrees

  Scenario: A third-party library recomputes the same chain and extents
    Given the poses of the vectors
    When numpy solves the three-dimensional chain and shapely measures the shapes
    Then the positions and the extents equal the evaluation

  Scenario: The tower crane slews its jib
    Given the tower crane in the parked weathervane pose
    When the three views are evaluated
    Then the jib is foreshortened in the side view and reaches sideways in plan

  Scenario: Solids follow the twisted and tilted frames
    Given an extruded slab and a revolved drum on a torso twisted 45 and 90 degrees and tilted back by 14 degrees, and a lid opened 105 degrees on a twisted body
    When the front and plan views are evaluated
    Then the slab swaps its width and depth, the drum revolves about the tilted axis and the lid parts turn with the carried axis and lateral direction

  Scenario: A solid on an elevation shape follows the twist in the side view
    Given an extruded slab and a revolved drum authored as solids on the elevation of a torso twisted 0, 45 and 90 degrees
    When the side view is evaluated
    Then the slab is 0.2, 0.424264 and 0.4 m across and the drum stays 0.2 m wide

  Scenario: A lid opened by a plain sagittal rotation revolves about its tilted axis
    Given a revolved and an extruded lid on an arm raised 105 degrees without spread or twist
    When the three views are evaluated
    Then the lid is a dome of 0.438 m by 0.297 m in every view instead of a vertical disc

  Scenario: A solid projects to one outline
    Given an L-shaped extruded bracket on a twisted torso and an upright extruded slab
    When the front view is evaluated
    Then the bracket is a single non-convex ring of the right area and the slab is exactly its rectangle
