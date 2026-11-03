@capability-staffage-lateral-spread
Feature: A bone spreads sideways in the front and the top view
  The sagittal rig only knows forward and up, so a wing spread sideways, an arm raised sideways or a leg straddled would not read in the front and plan views.
  A pose or clip key may give a revolute bone `abductionDeg`: after its own sagittal rotation the bone turns out of the sagittal plane about its normal (positive
  towards the left of the asset, +z), added down the chain like `angleDeg`. A point at distance s along the bone is then s·sin φ to the left and keeps s·cos φ of its
  sagittal reach; the children ride on the spread bone. Only the front and plan views move: a shape follows the displacement of the origin of its bone, and a
  `followTip` rect or poly stretches from the displacement of the origin to that of the tip. The elevation is the sagittal pose and never changes.

  THE REFERENCE is numpy rotation matrices for the three-dimensional chain and shapely for the extents.

  Scenario: An arm spread sideways reaches out and up in the front view
    Given the sample figure with the left arm spread 60 degrees
    When the front and plan views are evaluated
    Then the front arm reaches 0.519615 m sideways and ends 0.3 m higher and the plan arm grows outwards by the same lateral reach while the elevation is unchanged

  Scenario: Children ride on a spread parent
    Given the sample figure with the left thigh spread 90 degrees
    When a rigid shape of the shank is evaluated
    Then it is 0.45 m further left and 0.45 m higher in the front and 0.45 m further left in plan

  Scenario: A bone along the viewing axis grows outwards
    Given the sample figure with the left foot spread 90 degrees
    When the foot is evaluated
    Then it grows outwards by 0.22 m in the front view and turns into a lateral sliver in plan

  Scenario: Clips interpolate the spread
    Given a transition from standing to the arm spread 60 degrees
    When it is evaluated half way
    Then the arm is spread 30 degrees

  Scenario: The spread is bounded to the joints that can take it
    Given a pose that spreads a bone
    When the asset is validated
    Then a revolute joint accepts up to 180 degrees and fixed and prismatic joints and larger angles are refused

  Scenario: A third-party library recomputes the same displacements and extents
    Given the poses of the vectors
    When numpy solves the three-dimensional chain and shapely measures the shapes
    Then the displacements and the extents equal the evaluation

  Scenario: The herring gull spreads its wings in soaring and landing
    Given the herring gull in the soaring and landing poses
    When the front and plan views are evaluated
    Then the wing tips are wider than the standing bird in both views and the near and far wing are mirrored

  Scenario: A bone beside the symmetry plane pivots about its own lateral position
    Given the sample figure whose arm bone sits 0.25 m to the left of the symmetry plane
    When the arm is evaluated unspread and spread 60 degrees
    Then the unspread shapes keep their authored placement, the spread ones pivot about 0.25 m, and the validator refuses a bone outside half of the lateral dimension
