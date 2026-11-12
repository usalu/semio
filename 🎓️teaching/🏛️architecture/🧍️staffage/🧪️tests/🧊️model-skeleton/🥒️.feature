@capability-staffage-model-skeleton
Feature: The rig, its poses and its clips become a neutral node tree through the single evaluation
  Every bone of the rig is one node of the neutral model in its rest placement, parents first. The staffage joint becomes the mechanism: a revolute joint turns about the node's own +Z
  (the staffage lateral axis) within its angle limits in radians; a bone that a pose or a clip key spreads or twists, or whose joint limits a twist, is spherical about its bone axis; a
  prismatic joint slides along its axis in the parent frame; a fixed joint is fixed; the rig root is free. No kinematics is repeated: every pose and every clip frame is the bone frames
  of `evaluate`, expressed relative to the parent. A clip is sampled at its own fps with the closing key on the absolute timeline (root motion), rotation keys continuous.

  THE REFERENCES are `evaluate` (frames composed independently with three.js) for poses and clips, the kernel's own pose replay for the mechanism, and three's FBXLoader with AnimationMixer for the FBX.

  Scenario: One node per bone with the hand-computed mechanism
    Given the sample figure
    When its model is built
    Then the nodes are the bones parents first, the pelvis is free and every other joint is revolute about +Z with its limits in radians

  Scenario: Spread, twisted, sliding and fixed bones get their own joint
    Given the sample figure with a spread arm, a head with twist limits, a sliding foot and a fixed head
    When the model is built and every pose is replayed by the kernel
    Then each joint is spherical, spherical, prismatic and fixed as hand-computed, no joint limit is clamped and the replayed world matrices equal the evaluated frames

  Scenario: Every pose is the evaluated pose
    Given every named and key pose of the sample figure
    When the kernel replays the pose of the model
    Then every bone world matrix equals the frame of `evaluate` within 1 nm

  Scenario: Every clip is sampled at its fps
    Given the walk, idle and bag-swing clips at 30 fps
    When their animations are built
    Then each has ceil(duration x fps) + 1 keys from 0 to the duration, its events are the clip events, and the frames at the keys equal `evaluate`

  Scenario: Between the keys the animation stays within tolerance
    Given arbitrary times inside every clip
    When the kernel samples the animation there
    Then the bone origins are within 6 mm and the axes within 0.02 of the evaluated frames

  Scenario: Root motion and rotation continuity
    Given the planted walk clip
    When its closing key is read
    Then the root has advanced by the authored advance and no rotation key flips hemisphere

  Scenario: The FBX replays like the evaluation
    Given the FBX of each clip loaded by three's FBXLoader
    When an AnimationMixer plays it to arbitrary times
    Then every bone world position equals the evaluated one within 6 mm
